use std::{
    ops::Drop,
    sync::Arc,
};

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
    transaction: Option<Arc<Transaction>>,
    on_drop: OnDrop,
    store: Option<TransactionStore>,
}

impl Drop for ActiveTransaction {
    fn drop(&mut self) {
        // If this is the last active reference to the transaction then try perform its `on_drop` action
        // If a transaction is manually committed or cancelled then it won't have values here to use
        if let (Some(transaction), Some(store)) = (self.transaction.take(), self.store.take()) {
            if let Ok(transaction) = Arc::try_unwrap(transaction) {
                match self.on_drop {
                    OnDrop::Commit => store.commit(transaction),
                    OnDrop::Cancel => store.cancel(transaction),
                }
            }
        }
    }
}

#[derive(Clone, Copy)]
pub(in crate::domain::infra::transactions) enum OnDrop {
    Commit,
    Cancel,
}

impl ActiveTransaction {
    pub(in crate::domain::infra::transactions) fn begin(
        store: TransactionStore,
        on_drop: OnDrop,
    ) -> Self {
        let transaction = Arc::new(store.begin());

        ActiveTransaction {
            transaction: Some(transaction),
            on_drop,
            store: Some(store),
        }
    }

    pub(in crate::domain) fn get(&self) -> &Transaction {
        self.transaction.as_ref().expect("missing transaction")
    }

    /**
    Commit the transaction, making its changes observable.

    There must be no other callers holding on to this transaction when it's committed.
    If there are it will return an error instead of committing.
    */
    pub fn commit(mut self) -> Result<(), Error> {
        match Arc::try_unwrap(self.transaction.take().expect("missing transaction")) {
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
        if let Ok(transaction) =
            Arc::try_unwrap(self.transaction.take().expect("missing transaction"))
        {
            if let Some(store) = self.store.take() {
                store.cancel(transaction);
            }
        }
    }

    #[cfg(test)]
    pub(in crate::domain) fn none() -> Self {
        ActiveTransaction {
            transaction: Some(Arc::new(Transaction::none())),
            on_drop: OnDrop::Commit,
            store: None,
        }
    }
}
