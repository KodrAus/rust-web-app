use std::sync::Arc;

use crate::{
    domain::error::Error,
    store::{
        Transaction,
        TransactionStore,
    },
};

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
        &*self.transaction
    }

    pub fn commit(self) -> Result<(), Error> {
        match Arc::try_unwrap(self.transaction) {
            Ok(transaction) => {
                if let Some(store) = self.store {
                    store.commit(transaction);
                }

                Ok(())
            }
            Err(_) => Err(Error::from("transaction is still in use")),
        }
    }

    pub fn cancel(self) {
        if let Ok(transaction) = Arc::try_unwrap(self.transaction) {
            if let Some(store) = self.store {
                store.cancel(transaction);
            }
        }
    }

    #[cfg(not(test))]
    pub(in crate::domain::infra::transactions) fn none() -> Self {
        ActiveTransaction {
            transaction: Arc::new(Transaction::none()),
            store: None,
        }
    }

    #[cfg(test)]
    pub fn none() -> Self {
        ActiveTransaction {
            transaction: Arc::new(Transaction::none()),
            store: None,
        }
    }
}
