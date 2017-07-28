use std::sync::Arc;

use super::model::store::*;

pub struct Resolver {
    store: Arc<InMemoryStore>
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            store: Arc::new(in_memory_store())
        }
    }
}

impl Resolver {
    pub fn order_store(&self) -> impl OrderStore {
        self.store.clone()
    }

    pub fn order_with_items_store(&self) -> impl OrderWithItemsStore {
        self.store.clone()
    }
}
