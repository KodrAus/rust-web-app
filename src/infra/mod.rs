use std::collections::{HashMap, BTreeMap};
use std::sync::{Arc, RwLock};
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Product
}

pub type Store = Arc<HashMap<Type, RwLock<BTreeMap<i32, Value>>>>;

pub struct Resolver {
    store: Store
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            store: Arc::new({
                let mut stores = HashMap::new();

                stores.insert(Type::Product, RwLock::new(BTreeMap::new()));
                
                stores
            })
        }
    }
}

impl Resolver {
    pub fn store(&self) -> Store {
        self.store.clone()
    }
}