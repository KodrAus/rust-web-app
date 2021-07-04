use std::sync::Arc;

use crate::{
    domain::{
        error::Error,
        Resolver,
    },
    store::{
        Transaction,
        TransactionStore,
    },
};

#[auto_impl(&, Arc)]
pub trait ActiveTransactionProvider {
    fn active(&self) -> ActiveTransaction;
}

#[derive(Clone)]
pub struct ActiveTransaction {
    transaction: Arc<Transaction>,
    store: Option<TransactionStore>,
}

impl ActiveTransactionProvider for ActiveTransaction {
    fn active(&self) -> ActiveTransaction {
        self.clone()
    }
}

impl ActiveTransaction {
    fn begin(store: TransactionStore) -> Self {
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

    #[cfg(test)]
    fn none() -> Self {
        ActiveTransaction {
            transaction: Arc::new(Transaction::none()),
            store: None,
        }
    }
}

impl ActiveTransactionProvider for TransactionStore {
    fn active(&self) -> ActiveTransaction {
        ActiveTransaction::begin(self.clone())
    }
}

#[cfg(test)]
pub(in crate::domain) struct NoTransaction;

#[cfg(test)]
impl ActiveTransactionProvider for NoTransaction {
    fn active(&self) -> ActiveTransaction {
        ActiveTransaction::none()
    }
}

impl Resolver {
    pub fn active_transaction_provider(&self) -> impl ActiveTransactionProvider {
        self.store_resolver().transaction_store().clone()
    }
}
