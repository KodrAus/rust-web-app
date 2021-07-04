/*! Contains the `ProductsResolver` type. */

use std::sync::Arc;

use crate::domain::{
    infra::*,
    products::model::store::{
        self,
        InMemoryStore,
        ProductStore,
        ProductStoreFilter,
    },
};

/**
Resolver for products.

The `ProductsResolver` type wraps private implementation details and exposes them as traits within the `products` module.
*/
pub(in crate::domain) struct ProductsResolver {
    product_store: Register<Arc<InMemoryStore>>,
}

impl Default for ProductsResolver {
    fn default() -> Self {
        ProductsResolver {
            product_store: Register::once(|_| Arc::new(store::in_memory_store())),
        }
    }
}

impl Resolver {
    pub(in crate::domain::products) fn product_store(&self) -> impl ProductStore {
        self.resolve(&self.products_resolver.product_store)
    }

    pub(in crate::domain::products) fn product_store_filter(&self) -> impl ProductStoreFilter {
        self.resolve(&self.products_resolver.product_store)
    }
}
