use std::{
    collections::HashMap,
    ops::Drop,
    sync::{
        Arc,
        Mutex,
    },
};

use uuid::Uuid;

/**
An identifier for a transaction.

This values are intended to be persisted to disk to record what
transaction specific changes belonged to.

Transaction ids are independent, so there's nothing connecting the id of an active
transaction to the one that was created immediately before it.
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

The transaction needs to be passed by reference to methods that want to
make changes to values. A transaction can either be committed to make its
changes observable, or it can be cancelled to revert them. Both methods take
full ownership of the transaction so it can no longer be used.
*/
pub struct Transaction {
    id: TransactionId,
    complete_guard: Option<Box<dyn FnOnce() + Send + Sync>>,
}

impl Transaction {
    /**
    An "empty" transaction that makes all changes immediately observable.
    */
    pub(crate) fn none() -> Self {
        Transaction {
            id: TransactionId(Uuid::default()),
            complete_guard: None,
        }
    }
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
    */
    pub fn id(&self) -> TransactionId {
        self.id
    }
}

/**
A store that tracks the state of active transactions.

The store needs to be consulted to tell whether or not a given transaction is active,
committed, or cancelled. Multiple users can share the same store to track the same set
of transactions.
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

    The store is initially empty, so it will consider any transaction ids it
    encounters committed.
    */
    pub fn new() -> Self {
        TransactionStore {
            active: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /**
    Begin a new transaction that will be tracked by this store.

    The transaction will need to be passed back to this store to commit or cancel.
    */
    pub fn begin(&self) -> Transaction {
        let mut transactions = self.active.lock().unwrap();

        let id = Uuid::new_v4();

        transactions.insert(
            TransactionId(id),
            TransactionEntry {
                status: TransactionStatus::Active,
            },
        );

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
            },
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

        transactions
            .get(&id)
            .map(|transaction| matches!(transaction.status, TransactionStatus::Cancelled))
            .unwrap_or(false)
    }
}

impl TransactionId {
    #[cfg(test)]
    pub(in crate::store) fn new() -> Self {
        TransactionId(Uuid::new_v4())
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
}
