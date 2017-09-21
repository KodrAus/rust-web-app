/*! Contains the `OrdersResolver` type. */

use std::sync::Arc;

use domain::orders::model::store as order_store;

/**
Resolver for orders.

The `OrdersResolver` type wraps private implementation details and exposes them as traits within the `orders` module.
*/
pub struct OrdersResolver {
    order_store: Arc<order_store::InMemoryStore>,
}

impl Default for OrdersResolver {
    fn default() -> Self {
        OrdersResolver {
            order_store: Arc::new(order_store::in_memory_store()),
        }
    }
}

impl OrdersResolver {
    pub(in domain::orders) fn order_store(&self) -> impl order_store::OrderStore {
        self.order_store.clone()
    }
}
