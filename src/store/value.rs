use std::{
    collections::{
        hash_map,
        HashMap,
    },
    sync::RwLock,
};

use uuid::Uuid;

use crate::store::{
    transaction::{
        Transaction,
        TransactionId,
        TransactionStore,
    },
    Error,
};

/**
An identifier for a transactional value.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }

    pub(crate) fn from_raw(id: Uuid) -> Id {
        Id(id)
    }

    pub(crate) fn into_raw(self) -> Uuid {
        self.0
    }
}

/**
A version for a transactional value.

Versions are independent, so there's nothing that connects the current version
of a value to its previous one.
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Version(Uuid);

impl Version {
    pub fn new() -> Self {
        Version(Uuid::new_v4())
    }

    pub(crate) fn from_raw(version: Uuid) -> Version {
        Version(version)
    }

    pub(crate) fn into_raw(self) -> Uuid {
        self.0
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

    The store will use the given transaction store to keep track of the current
    observable state of its values.
    */
    pub fn new(transactions: TransactionStore) -> Self {
        TransactionValueStore {
            transactions,
            data: RwLock::new(HashMap::new()),
        }
    }

    /**
    Get a reference to the underlying transaction store.

    The transaction store can be used to begin the transactions needed to make changes.
    */
    pub fn transactions(&self) -> &TransactionStore {
        &self.transactions
    }

    /**
    Get a value for the given id.

    This will also return the current version of the value that will be needed to update it.
    */
    pub fn get(&self, id: impl Into<Id>) -> Option<(Version, T)> {
        let id = id.into();

        let data = self.data.read().unwrap();

        Self::get_sync(id, &self.transactions, &*data)
            .map(|(version, value)| (version, value.clone()))
    }

    /**
    Get all values that match a given filter.
    */
    pub fn get_all(
        &self,
        mut filter: impl FnMut(&T) -> bool,
    ) -> impl Iterator<Item = (Version, T)> {
        let data = self.data.read().unwrap();

        data.keys()
            .filter_map(|id| Self::get_sync(*id, &self.transactions, &*data))
            .filter_map(|(version, value)| {
                if filter(value) {
                    Some((version, value.clone()))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    fn get_sync<'a>(
        id: Id,
        transactions: &TransactionStore,
        data: &'a HashMap<Id, TransactionalValue<T>>,
    ) -> Option<(Version, &'a T)> {
        if let Some(existing) = data.get(&id) {
            if let Some((existing_transaction, existing_version, ref existing_value)) =
                existing.current
            {
                if transactions.is_committed(existing_transaction) {
                    return Some((existing_version, existing_value));
                }

                if let Some((prior_transaction, prior_version, ref prior_value)) = existing.prior {
                    assert!(transactions.is_committed(prior_transaction));

                    return Some((prior_version, prior_value));
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

    The old version is ignored if the value doesn't currently exist.
    */
    pub fn set(
        &self,
        transaction: &Transaction,
        id: impl Into<Id>,
        old_version: Option<impl Into<Version>>,
        new_version: impl Into<Version>,
        new_value: T,
    ) -> Result<(), Error> {
        let id = id.into();
        let old_version = old_version.map(Into::into);
        let new_version = new_version.into();

        assert_ne!(
            old_version,
            Some(new_version),
            "a new value must use a different version"
        );

        let mut data = self.data.write().unwrap();

        match data.entry(id) {
            hash_map::Entry::Occupied(mut occupied) => {
                let existing = occupied.get_mut();

                match &mut existing.current {
                    // If the value already exists then we need to update it, without making
                    // that new version visible to anybody currently looking at the value.
                    // We do this by updating a pair of values: one for the new version of the
                    // value and one for the prior version. While this transaction is active,
                    // callers will get the prior value, but will perform their version checks
                    // against the current. Since versions are independent that means a conflicting
                    // transaction can't clobber this one if it got in first. It won't know what
                    // version it should be using to update the current value set by the other transaction.
                    Some((existing_transaction, existing_version, existing_value)) => {
                        // First, we need to check the versions to make sure they line up

                        // If the existing value is not for a cancelled transaction
                        // then use it to check the version. This means an active transaction
                        // that sets a value will prevent any other transactions from setting
                        // that same value
                        let version_to_check =
                            if !self.transactions.is_cancelled(*existing_transaction) {
                                Some(*existing_version)
                            }
                            // If the existing value is for a cancelled transaction then use
                            // the prior version to check. This prevents a cancelled transaction
                            // from blocking the value from ever being set again
                            else {
                                existing.prior.as_ref().map(
                                    |(prior_transaction, prior_version, _)| {
                                        assert!(self.transactions.is_committed(*prior_transaction));

                                        *prior_version
                                    },
                                )
                            };

                        if old_version != version_to_check {
                            return Err(Error::from("version mismatch"));
                        }

                        // Now, we're going to set the value

                        // If the existing value is for a committed transaction then move it
                        // into the prior value and set the new value in its place
                        if self.transactions.is_committed(*existing_transaction) {
                            let old_transaction =
                                std::mem::replace(existing_transaction, transaction.id());
                            let old_version = std::mem::replace(existing_version, new_version);
                            let old_value = std::mem::replace(existing_value, new_value);

                            existing.prior = Some((old_transaction, old_version, old_value));
                        }
                        // If the existing value is for an active or cancelled transaction then
                        // update it without touching the prior value
                        else {
                            *existing_transaction = transaction.id();
                            *existing_version = new_version;
                            *existing_value = new_value;
                        }
                    }
                    // If the value doesn't exist then set it
                    // We explicitly don't check the old version for `None` here to make life easier
                    // for consumers that can't tell whether they're looking at the first version
                    // of a value or not
                    None => existing.current = Some((transaction.id(), new_version, new_value)),
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
    fn existing_id_in_fresh_store_is_committed() {
        // Simulate reading an existing transaction id from persistent storage
        // and checking with a new transaction store whether or not it was committed
        let id = TransactionId::new();

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
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();
        store.transactions.commit(transaction);

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(version, current_version);
        assert_eq!("1", current_value);
    }

    #[test]
    fn transaction_value_store_set_ignores_old_version_initially() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();

        let r = store.set(
            &transaction,
            id,
            Some(Version::new()),
            version,
            String::from("1"),
        );

        assert!(r.is_ok());
    }

    #[test]
    fn transaction_value_store_get_during_transaction() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();

        assert!(store.get(id).is_none());
    }

    #[test]
    fn transaction_value_store_cancel_get() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();
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
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();
        store.transactions.commit(transaction);

        let old_version = version;

        for _ in 0..10 {
            // Try set a new value, but cancel instead of commit
            let transaction = store.transactions.begin();
            store
                .set(
                    &transaction,
                    id,
                    Some(old_version),
                    Version::new(),
                    String::from("2"),
                )
                .unwrap();
            store.transactions.cancel(transaction);
        }

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(old_version, current_version);
        assert_eq!("1", current_value);

        let version = Version::new();

        // Try set a new value again, but this time succeed
        let transaction = store.transactions.begin();
        store
            .set(
                &transaction,
                id,
                Some(old_version),
                version,
                String::from("3"),
            )
            .unwrap();
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
        store1
            .set(
                &transaction,
                id1,
                None::<Version>,
                version1,
                String::from("a1"),
            )
            .unwrap();
        store2
            .set(
                &transaction,
                id2,
                None::<Version>,
                version2,
                String::from("a2"),
            )
            .unwrap();

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

    #[test]
    fn transaction_value_store_set_get_empty_transaction() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = Transaction::none();
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();

        // An empty transaction doesn't need to be committed
        // The transaction store never sees it

        let (current_version, current_value) = store.get(id).unwrap();

        assert_eq!(version, current_version);
        assert_eq!("1", current_value);
    }

    #[test]
    fn err_transaction_value_store_set_version_mismatch() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();
        store.transactions.commit(transaction);

        let transaction = store.transactions.begin();

        // Attempting to set the value with a mismatched current version will fail
        let r = store.set(
            &transaction,
            id,
            None::<Version>,
            Version::new(),
            String::from("2"),
        );

        assert!(r.is_err());
    }

    #[test]
    fn err_multi_transaction_value_store_set() {
        let store = TransactionValueStore::<String>::new(TransactionStore::new());

        let id = Id::new();
        let version = Version::new();

        let transaction = store.transactions.begin();
        store
            .set(
                &transaction,
                id,
                None::<Version>,
                version,
                String::from("1"),
            )
            .unwrap();
        store.transactions.commit(transaction);

        let transaction1 = store.transactions.begin();

        store
            .set(
                &transaction1,
                id,
                Some(version),
                Version::new(),
                String::from("2"),
            )
            .unwrap();

        let transaction = store.transactions.begin();

        // Attempting to set the value from concurrent transactions will fail
        let r = store.set(
            &transaction,
            id,
            Some(version),
            Version::new(),
            String::from("3"),
        );

        assert!(r.is_err());
    }
}
