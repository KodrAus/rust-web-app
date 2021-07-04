use crate::{
    domain::infra::*,
    store::TransactionStore,
};

pub struct TransactionsResolver {
    transaction_store: Register<TransactionStore>,
    active_transaction: Register<ActiveTransaction>,
}

impl Default for TransactionsResolver {
    fn default() -> Self {
        TransactionsResolver {
            transaction_store: Register::once(|_| TransactionStore::new()),
            active_transaction: Register::factory(|resolver| {
                ActiveTransaction::begin(resolver.transaction_store())
            }),
        }
    }
}

impl Resolver {
    pub(in crate::domain) fn transaction_store(&self) -> TransactionStore {
        self.resolve(&self.transactions_resolver.transaction_store)
    }

    pub fn active_transaction(&self) -> ActiveTransaction {
        self.resolve(&self.transactions_resolver.active_transaction)
    }
}
