/**
Transactional value storage.

This module sketches out a storage API that uses transactions to coordinate updates to
independent data stores. The design assumes data for a given transaction will be technically
observable (such as being written to disk or some external database) before the transaction
itself is committed. The `TransactionStore` keeps track of whether or not the data associated
with a given transaction should be surfaced to callers or not.
*/

use std::{
    error,
    collections::{hash_map, HashMap},
    ops::Drop,
    sync::{Arc, Mutex, RwLock},
};

pub type Error = Box<dyn error::Error + Send + Sync>;

use uuid::Uuid;

/**
An identifier for a transaction.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TransactionId(Uuid);

struct TransactionEntry {
    status: TransactionStatus,
}

enum TransactionStatus {
    Active,
    Cancelled,
}

/**
An active transaction.
*/
pub struct Transaction {
    id: TransactionId,
    complete_guard: Option<Box<dyn FnOnce()>>,
}

impl Drop for Transaction {
    fn drop(&mut self) {
        if let Some(complete_guard) = self.complete_guard.take() {
            complete_guard()
        }
    }
}

impl Transaction {
    /**
    Get the id associated with this transaction.

    The id can be used to connect changed data with a transaction.
    */
    pub fn id(&self) -> TransactionId {
        self.id
    }
}

/**
A store that tracks the state of active transactions.
*/
#[derive(Clone)]
pub struct TransactionStore {
    active: Arc<Mutex<HashMap<TransactionId, TransactionEntry>>>,
}

impl Default for TransactionStore {
    fn default() -> Self {
        TransactionStore::new()
    }
}

impl TransactionStore {
    /**
    Create a new store.

    This currently assumes that any transaction ids belong to committed transactions.
    */
    pub fn new() -> Self {
        TransactionStore {
            active: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /**
    Begin a new transaction that will be tracked by this store.
    */
    pub fn begin(&self) -> Transaction {
        let mut transactions = self.active.lock().unwrap();

        let id = Uuid::new_v4();

        transactions.insert(TransactionId(id), TransactionEntry { status: TransactionStatus::Active });

        Transaction {
            id: TransactionId(id),
            complete_guard: {
                let transactions = self.clone();

                Some(Box::new(move || {
                    let id = TransactionId(id);
                    let mut transactions = transactions.active.lock().unwrap();

                    if let Some(transaction) = transactions.get_mut(&id) {
                        transaction.status = TransactionStatus::Cancelled;
                    }
                }))
            }
        }
    }

    /**
    Commit a transaction, making its changes atomically observable.
    */
    pub fn commit(&self, mut transaction: Transaction) {
        drop(transaction.complete_guard.take());

        let mut transactions = self.active.lock().unwrap();

        // NOTE: Only removing transactions when they commit means we'll eventually run out of
        // space if they fail. In a degenerate scenario where everything fails this might not
        // take very long. We could avoid this by tracking whether or not transactions are still
        // reachable and whether or not their ids appear in any data stores.
        let _ = transactions.remove(&transaction.id);
    }

    /**
    Cancel a transaction, ensuring its changes can never be observed.
    */
    pub fn cancel(&self, mut transaction: Transaction) {
        drop(transaction.complete_guard.take());

        let mut transactions = self.active.lock().unwrap();

        if let Some(transaction) = transactions.get_mut(&transaction.id) {
            transaction.status = TransactionStatus::Cancelled;
        }
    }

    /**
    Whether or not a given transaction was committed.
    */
    pub fn is_committed(&self, id: TransactionId) -> bool {
        let transactions = self.active.lock().unwrap();

        // If a transaction is missing then it was committed
        !transactions.contains_key(&id)
    }

    /**
    Whether or not a given transaction was cancelled.
    */
    pub fn is_cancelled(&self, id: TransactionId) -> bool {
        let transactions = self.active.lock().unwrap();

        transactions.get(&id).map(|transaction| matches!(transaction.status, TransactionStatus::Cancelled)).unwrap_or(false)
    }
}

/**
An identifier for a value.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}

/**
A version for a value.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(Uuid);

impl Version {
    pub fn new() -> Self {
        Version(Uuid::new_v4())
    }
}

struct TransactionalValue<T> {
    current: Option<(TransactionId, Version, T)>,
    prior: Option<(TransactionId, Version, T)>,
}

/**
A generic value store for transactional values.

This store can participate in transactions with other disconnected stores.
*/
pub struct TransactionValueStore<T> {
    transactions: TransactionStore,
    data: RwLock<HashMap<Id, TransactionalValue<T>>>,
}

impl<T> TransactionValueStore<T>
where
    T: Clone,
{
    /**
    Create a new transactional value store.
    */
    pub fn new(transactions: TransactionStore) -> Self {
        TransactionValueStore {
            transactions,
            data: RwLock::new(HashMap::new()),
        }
    }

    /**
    Get a reference to the underlying transaction store.
     */
    pub fn transactions(&self) -> &TransactionStore {
        &self.transactions
    }

    /**
    Get a value for the given id.

    This will also return the current version of the value that will be needed to update it.
    */
    pub fn get(&self, id: Id) -> Option<(Version, T)> {
        let data = self.data.read().unwrap();

        if let Some(existing) = data.get(&id) {
            if let Some((existing_transaction, existing_version, ref existing_value)) = existing.current {
                if self.transactions.is_committed(existing_transaction) {
                    return Some((existing_version, existing_value.clone()));
                }

                if let Some((prior_transaction, prior_version, ref prior_value)) = existing.prior {
                    assert!(self.transactions.is_committed(prior_transaction));

                    return Some((prior_version, prior_value.clone()));
                }
            }
        }

        None
    }

    /**
    Set a value for the given id.

    Changes are associated with an active transaction and not observable until the transaction
    is committed. If another transaction attempts to set this same value in the meantime it will
    fail with a version mismatch.
    */
    pub fn set(&self, transaction: &Transaction, id: Id, old_version: Option<Version>, new_version: Version, new_value: T) -> Result<(), Error> {
        let mut data = self.data.write().unwrap();

        match data.entry(id) {
            hash_map::Entry::Occupied(mut occupied) => {
                let existing = occupied.get_mut();

                match &mut existing.current {
                    Some((existing_transaction, existing_version, existing_value)) => {
                        // If the transaction for the current value was cancelled then look at the version
                        // of the prior value instead
                        let version_matches = if self.transactions.is_cancelled(*existing_transaction) {
                            old_version == existing.prior.as_ref().map(|(prior_transaction, prior_version, _)| {
                                assert!(self.transactions.is_committed(*prior_transaction));

                                *prior_version
                            })
                        }
                        // If the transaction for the current value is active or committed then use it
                        // Checking the version of an active (uncommitted) transaction guarantees only
                        // a single transaction can set a new value at a time
                        else {
                            old_version == Some(*existing_version)
                        };

                        if !version_matches {
                            return Err(Error::from("version mismatch"));
                        }

                        // Set the new value to the prior one
                        // Consumers won't see this value unless the transaction is committed
                        let old_transaction = std::mem::replace(existing_transaction, transaction.id());
                        let old_version = std::mem::replace(existing_version, new_version);
                        let old_value = std::mem::replace(existing_value, new_value);

                        existing.prior = Some((old_transaction, old_version, old_value));
                    }
                    None => existing.current = Some((transaction.id(), new_version, new_value))
                }
            }
            hash_map::Entry::Vacant(vacant) => {
                vacant.insert(TransactionalValue {
                    current: Some((transaction.id(), new_version, new_value)),
                    prior: None,
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_transaction_is_not_committed() {
        let store = TransactionStore::new();

        let transaction = store.begin();

        assert!(!store.is_committed(transaction.id()));
    }

    #[test]
    fn cancelled_transaction_is_not_committed() {
        let store = TransactionStore::new();

        let transaction = store.begin();
        let id = transaction.id();

        store.cancel(transaction);

        assert!(!store.is_committed(id));
    }

    #[test]
    fn leaked_transaction_is_not_committed() {
        let store = TransactionStore::new();

        let transaction = store.begin();
        let id = transaction.id();

        std::mem::forget(transaction);

        assert!(!store.is_committed(id));
    }

    #[test]
    fn committed_transaction_is_committed() {
        let store = TransactionStore::new();

        let transaction = store.begin();
        let id = transaction.id();

        store.commit(transaction);

        assert!(store.is_committed(id));
    }

    #[test]
    fn existing_id_in_fresh_store_is_committed() {
        // Simulate reading an existing transaction id from persistent storage
        // and checking with a new transaction store whether or not it was committed
        let id = TransactionId(Uuid::new_v4());

        // NOTE: This means if we terminate in the middle of committing a transaction
        // then on restart the store will see a partial commit. This could be worked around
        // by persisting the state of the transaction store itself so it reloads partial
        // transactions too. Since our app doesn't actually persist any data though we
        // haven't tried to make this bullet-proof
        let store = TransactionStore::new();

        assert!(store.is_committed(id));
    }

    #[test]
    fn transaction_value_store_empty_get() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        assert!(store.get(id).is_none());
    }

    #[test]
    fn transaction_value_store_set_get() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store.set(&transaction, id, None, version, String::from("1")).unwrap();
        store.transactions.commit(transaction);

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(version, current_version);
        assert_eq!("1", current_value);
    }

    #[test]
    fn transaction_value_store_get_during_transaction() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store.set(&transaction, id, None, version, String::from("1")).unwrap();

        assert!(store.get(id).is_none());
    }

    #[test]
    fn transaction_value_store_cancel_get() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store.set(&transaction, id, None, version, String::from("1")).unwrap();
        store.transactions.cancel(transaction);

        assert!(store.get(id).is_none());
    }

    #[test]
    fn transaction_value_store_set_cancel_get() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        // Set an initial value
        let transaction = store.transactions.begin();
        store.set(&transaction, id, None, version, String::from("1")).unwrap();
        store.transactions.commit(transaction);

        let old_version = version;
        let version = Version::new();

        // Try set a new value, but cancel instead of commit
        let transaction = store.transactions.begin();
        store.set(&transaction, id, Some(old_version), version, String::from("2")).unwrap();
        store.transactions.cancel(transaction);

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(old_version, current_version);
        assert_eq!("1", current_value);

        let version = Version::new();

        // Try set a new value again, but this time succeed
        let transaction = store.transactions.begin();
        store.set(&transaction, id, Some(old_version), version, String::from("3")).unwrap();
        store.transactions.commit(transaction);

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(version, current_version);
        assert_eq!("3", current_value);
    }

    #[test]
    fn transaction_value_store_multi_set_get() {
        let transactions = TransactionStore::new();

        let store1 = TransactionValueStore::<String>::new(transactions.clone());
        let store2 = TransactionValueStore::<String>::new(transactions.clone());

        let transaction = transactions.begin();

        let id1 = Id::new();
        let version1 = Version::new();
        let id2 = Id::new();
        let version2 = Version::new();

        // Transactions apply across stores
        store1.set(&transaction, id1, None, version1, String::from("a1")).unwrap();
        store2.set(&transaction, id2, None, version2, String::from("a2")).unwrap();

        assert!(store1.get(id1).is_none());
        assert!(store2.get(id2).is_none());

        transactions.commit(transaction);

        let (current_version1, current_value1) = store1.get(id1).unwrap();
        let (current_version2, current_value2) = store2.get(id2).unwrap();

        assert_eq!(version1, current_version1);
        assert_eq!("a1", current_value1);
        assert_eq!(version2, current_version2);
        assert_eq!("a2", current_value2);
    }
}
