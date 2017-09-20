use std::sync::Arc;

use domain::id::{IdProvider, NextId};
use domain::customers::CustomerData;
use domain::customers::model::store as customer_store;

/// Resolver for customers.
///
/// The `Resolver` type wraps private implementation details and exposes them as traits.
pub struct Resolver {
    customer_store: Arc<customer_store::InMemoryStore>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            customer_store: Arc::new(customer_store::in_memory_store()),
        }
    }
}

impl Resolver {
    pub(in domain) fn customer_store(&self) -> impl customer_store::CustomerStore {
        self.customer_store.clone()
    }

    pub fn customer_id_provider(&self) -> impl IdProvider<CustomerData> {
        NextId::<CustomerData>::new()
    }
}
