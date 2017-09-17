use std::sync::Arc;

use domain::id::{IdProvider, NextId};
use domain::orders::{LineItemData, OrderData};
use domain::orders::model::store as order_store;

/// Resolver for orders.
///
/// The `Resolver` type wraps private implementation details and exposes them as traits.
pub struct Resolver {
    order_store: Arc<order_store::InMemoryStore>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            order_store: Arc::new(order_store::in_memory_store()),
        }
    }
}

impl Resolver {
    pub(in domain) fn order_store(&self) -> impl order_store::OrderStore {
        self.order_store.clone()
    }

    pub fn order_id_provider(&self) -> impl IdProvider<OrderData> {
        NextId::<OrderData>::new()
    }

    pub fn line_item_id_provider(&self) -> impl IdProvider<LineItemData> {
        NextId::<LineItemData>::new()
    }
}
