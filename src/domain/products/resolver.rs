/*! Contains the `ProductsResolver` type. */

use std::sync::Arc;

use domain::products::model::store as product_store;

/**
Resolver for products.

The `ProductsResolver` type wraps private implementation details and exposes them as traits within the `products` module.
*/
pub struct ProductsResolver {
    product_store: Arc<product_store::InMemoryStore>,
}

impl Default for ProductsResolver {
    fn default() -> Self {
        ProductsResolver {
            product_store: Arc::new(product_store::in_memory_store()),
        }
    }
}

impl ProductsResolver {
    pub(in domain::products) fn product_store(&self) -> impl product_store::ProductStore {
        self.product_store.clone()
    }

    pub(in domain::products) fn product_store_filter(&self) -> impl product_store::ProductStoreFilter {
        self.product_store.clone()
    }
}
