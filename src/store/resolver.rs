use crate::store::TransactionStore;

/**
Resolver for products.

The `StoreResolver` type wraps private implementation details and exposes them as traits within the `products` module.
 */
pub struct StoreResolver {
    transaction_store: TransactionStore,
}

impl Default for StoreResolver {
    fn default() -> Self {
        StoreResolver {
            transaction_store: TransactionStore::new(),
        }
    }
}

impl StoreResolver {
    pub(crate) fn transaction_store(&self) -> TransactionStore {
        self.transaction_store.clone()
    }
}
