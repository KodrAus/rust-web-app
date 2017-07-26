mod store;

pub use self::store::*;

use std::sync::Arc;

pub struct Resolver {
    store: Arc<store::InMemoryStore>
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            store: Arc::new(store::in_memory_store())
        }
    }
}
