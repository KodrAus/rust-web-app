use std::sync::Arc;

use crate::{
    domain::error::Error,
    store::{
        Transaction,
        TransactionStore,
    },
};

/**
An active transaction that may implicitly commit or cancel on drop.

A transaction is needed to make changes to entities, but callers don't necessarily need to
manage the transaction themselves.
*/
#[derive(Clone)]
pub struct ActiveTransaction {
    transaction: Arc<Transaction>,
    store: Option<TransactionStore>,
}

impl ActiveTransaction {
    pub(in crate::domain::infra::transactions) fn begin(store: TransactionStore) -> Self {
        let transaction = Arc::new(store.begin());

        ActiveTransaction {
            transaction,
            store: Some(store),
        }
    }

    pub(in crate::domain) fn get(&self) -> &Transaction {
        &self.transaction
    }

    /**
    Commit the transaction, making its changes observable.

    There must be no other callers holding on to this transaction when it's committed.
    If there are it will return an error instead of committing.
    */
    pub fn commit(mut self) -> Result<(), Error> {
        match Arc::try_unwrap(self.transaction) {
            Ok(transaction) => {
                if let Some(store) = self.store.take() {
                    store.commit(transaction);
                }

                Ok(())
            }
            Err(_) => Err(Error::from("transaction is still in use")),
        }
    }

    /**
    Cancel the transaction, reverting its changes.

    There must be no other callers holding on to this transaction when it's cancelled.
    If there are it will return an error instead of cancelling.
    */
    pub fn cancel(mut self) {
        if let Ok(transaction) = Arc::try_unwrap(self.transaction) {
            if let Some(store) = self.store.take() {
                store.cancel(transaction);
            }
        }
    }

    pub(in crate::domain) fn none() -> Self {
        ActiveTransaction {
            transaction: Arc::new(Transaction::none()),
            store: None,
        }
    }
}
