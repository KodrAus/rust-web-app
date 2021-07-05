use crate::{
    domain::{
        infra::{
            transactions::OnDrop,
            *,
        },
        Error,
    },
    store::TransactionStore,
};

#[derive(Clone)]
pub struct TransactionsResolver {
    transaction_store: Register<TransactionStore>,
    active_transaction: Register<ActiveTransaction>,
}

impl Default for TransactionsResolver {
    fn default() -> Self {
        TransactionsResolver {
            transaction_store: Register::once(|_| TransactionStore::new()),
            active_transaction: Register::factory(|resolver| {
                // By default, each call to get an active transaction will receive a fresh one
                // that will implicitly commit on drop
                ActiveTransaction::begin(resolver.transaction_store(), OnDrop::Commit)
            }),
        }
    }
}

impl Resolver {
    pub(in crate::domain) fn transaction_store(&self) -> TransactionStore {
        self.resolve(&self.transactions_resolver.transaction_store)
    }

    pub(in crate::domain) fn active_transaction(&self) -> ActiveTransaction {
        self.resolve(&self.transactions_resolver.active_transaction)
    }

    /**
    Begin a transaction and return a resolver that uses it.

    Any commands that are resolved from the returned resolver will participate in the returned transaction.
    The transaction will need to be completed before it will commit.
    */
    pub fn transaction<T>(&self, f: impl FnOnce(Resolver) -> Result<T, Error>) -> Result<T, Error> {
        let resolver = Resolver {
            transactions_resolver: TransactionsResolver {
                transaction_store: self.transactions_resolver.transaction_store.clone(),
                active_transaction: Register::once(|resolver| {
                    ActiveTransaction::begin(resolver.transaction_store(), OnDrop::Cancel)
                }),
            },
            products_resolver: self.products_resolver.clone(),
            orders_resolver: self.orders_resolver.clone(),
            customers_resolver: self.customers_resolver.clone(),
        };

        let transaction = resolver.active_transaction();
        let r = f(resolver)?;
        transaction.commit()?;

        Ok(r)
    }
}
