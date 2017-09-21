use std::sync::Arc;

use domain::id::{IdProvider, NextId};
use domain::products::ProductData;
use domain::products::model::store as product_store;

/// Resolver for products.
///
/// The `Resolver` type wraps private implementation details and exposes them as traits.
pub struct Resolver {
    product_store: Arc<product_store::InMemoryStore>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            product_store: Arc::new(product_store::in_memory_store()),
        }
    }
}

impl Resolver {
    pub(in domain) fn product_store(&self) -> impl product_store::ProductStore {
        self.product_store.clone()
    }

    pub(in domain::products) fn product_store_filter(&self) -> impl product_store::ProductStoreFilter {
        self.product_store.clone()
    }

    pub fn product_id_provider(&self) -> impl IdProvider<ProductData> {
        NextId::<ProductData>::new()
    }
}
