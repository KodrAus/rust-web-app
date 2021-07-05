/*! Contains the `CustomersResolver` type. */

use std::sync::Arc;

use crate::domain::{
    customers::model::store::{
        self,
        CustomerStore,
        InMemoryStore,
    },
    infra::*,
};

/**
Resolver for customers.

The `CustomersResolver` type wraps private implementation details and exposes them as traits within the `customers` module.
*/
#[derive(Clone)]
pub(in crate::domain) struct CustomersResolver {
    customer_store: Register<Arc<InMemoryStore>>,
}

impl Default for CustomersResolver {
    fn default() -> Self {
        CustomersResolver {
            customer_store: Register::once(|resolver| {
                Arc::new(store::in_memory_store(resolver.transaction_store()))
            }),
        }
    }
}

impl Resolver {
    pub(in crate::domain::customers) fn customer_store(&self) -> impl CustomerStore {
        self.resolve(&self.customers_resolver.customer_store)
    }
}
