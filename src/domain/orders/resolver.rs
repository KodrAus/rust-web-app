/*! Contains the `OrdersResolver` type. */

use std::sync::Arc;

use crate::domain::{
    infra::*,
    orders::model::store::{
        self,
        InMemoryStore,
        OrderStore,
        OrderStoreFilter,
    },
};

/**
Resolver for orders.

The `OrdersResolver` type wraps private implementation details and exposes them as traits within the `orders` module.
*/
pub(in crate::domain) struct OrdersResolver {
    order_store: Register<Arc<InMemoryStore>>,
}

impl Default for OrdersResolver {
    fn default() -> Self {
        OrdersResolver {
            order_store: Register::once(|_| Arc::new(store::in_memory_store())),
        }
    }
}

impl Resolver {
    pub(in crate::domain::orders) fn order_store(&self) -> impl OrderStore {
        self.resolve(&self.orders_resolver.order_store)
    }

    pub(in crate::domain::orders) fn order_store_filter(&self) -> impl OrderStoreFilter {
        self.resolve(&self.orders_resolver.order_store)
    }
}
