use crate::{
    domain::{
        infra::*,
        Error,
    },
    store::TransactionStore,
};

#[derive(Clone)]
pub(in crate::domain) struct TransactionsResolver {
    transaction_store: Register<TransactionStore>,
    active_transaction: Register<ActiveTransaction>,
}

impl Default for TransactionsResolver {
    fn default() -> Self {
        TransactionsResolver {
            transaction_store: Register::once(|_| TransactionStore::new()),
            active_transaction: Register::factory(|_| {
                // By default, each call to get an active transaction will receive a fresh one
                // that isn't transactional at all
                ActiveTransaction::none()
            }),
        }
    }
}

impl App {
    /**
    Begin a transaction and return a resolver that uses it.

    Any commands that are resolved from the returned resolver will participate in the returned transaction.
    The transaction will need to be completed before it will commit.
    */
    pub fn transaction<T, E>(&self, f: impl FnOnce(Resolver) -> Result<T, E>) -> Result<T, E>
    where
        E: From<Error>,
    {
        let resolver = self
            .root_resolver
            .with_active_transaction(Register::once(|resolver| {
                ActiveTransaction::begin(resolver.transaction_store())
            }));

        let transaction = resolver.active_transaction();
        let r = f(resolver)?;
        transaction.commit()?;

        Ok(r)
    }

    /**
    Begin a transaction and return a resolver that uses it.

    Any commands that are resolved from the returned resolver will participate in the returned transaction.
    The transaction will need to be completed before it will commit.
    */
    pub async fn transaction2<F, O, T, E>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(Resolver) -> O,
        O: ::std::future::Future<Output = Result<T, E>>,
        E: From<Error>,
    {
        let resolver = self
            .root_resolver
            .with_active_transaction(Register::once(|resolver| {
                ActiveTransaction::begin(resolver.transaction_store())
            }));

        let transaction = resolver.active_transaction();
        let r = f(resolver).await?;
        transaction.commit()?;

        Ok(r)
    }
}

impl Resolver {
    pub(in crate::domain) fn transaction_store(&self) -> TransactionStore {
        self.resolve(&self.transactions_resolver.transaction_store)
    }

    pub(in crate::domain) fn active_transaction(&self) -> ActiveTransaction {
        self.resolve(&self.transactions_resolver.active_transaction)
    }

    pub(in crate::domain) fn with_active_transaction(
        &self,
        active_transaction: Register<ActiveTransaction>,
    ) -> Resolver {
        Resolver {
            transactions_resolver: TransactionsResolver {
                transaction_store: self.transactions_resolver.transaction_store.clone(),
                active_transaction,
            },
            products_resolver: self.products_resolver.clone(),
            orders_resolver: self.orders_resolver.clone(),
            customers_resolver: self.customers_resolver.clone(),
        }
    }
}
