/*! Contains the `ProductsResolver` type. */

use std::sync::Arc;

use crate::{
    domain::products::model::store as product_store,
    store::StoreResolver,
};

/**
Resolver for products.

The `ProductsResolver` type wraps private implementation details and exposes them as traits within the `products` module.
*/
pub struct ProductsResolver {
    product_store: Arc<product_store::InMemoryStore>,
}

impl ProductsResolver {
    pub(in crate::domain) fn new(store_resolver: &StoreResolver) -> Self {
        ProductsResolver {
            product_store: Arc::new(product_store::in_memory_store()),
        }
    }
}

impl ProductsResolver {
    pub(in crate::domain::products) fn product_store(&self) -> impl product_store::ProductStore {
        self.product_store.clone()
    }

    pub(in crate::domain::products) fn product_store_filter(
        &self,
    ) -> impl product_store::ProductStoreFilter {
        self.product_store.clone()
    }
}
