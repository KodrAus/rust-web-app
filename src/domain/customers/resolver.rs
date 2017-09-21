/*! Contains the `CustomersResolver` type. */

use std::sync::Arc;

use domain::customers::model::store as customer_store;

/**
Resolver for customers.

The `CustomersResolver` type wraps private implementation details and exposes them as traits within the `customers` module.
*/
pub struct CustomersResolver {
    customer_store: Arc<customer_store::InMemoryStore>,
}

impl Default for CustomersResolver {
    fn default() -> Self {
        CustomersResolver {
            customer_store: Arc::new(customer_store::in_memory_store()),
        }
    }
}

impl CustomersResolver {
    pub(in domain::customers) fn customer_store(&self) -> impl customer_store::CustomerStore {
        self.customer_store.clone()
    }
}
