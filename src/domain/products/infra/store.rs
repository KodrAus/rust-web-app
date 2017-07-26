use std::collections::BTreeMap;
use std::sync::RwLock;
use serde_json::{self, Value};
use auto_impl::auto_impl;

use domain::products::{Resolver, Product};

pub type Error = String;

#[auto_impl(Arc)]
pub trait Store {
    fn get(&self, id: i32) -> Result<Option<Product>, Error>;
    fn set(&self, product: Product) -> Result<(), Error>;
}

pub(super) type InMemoryStore = RwLock<BTreeMap<i32, Value>>;

impl Store for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<Product>, Error> {
        let products = self
            .read()
            .map_err(|_| "not good!")?;

        if let Some(raw_product) = products.get(&id) {
            let product = serde_json::from_value(raw_product.clone())
                .map_err(|_| "unexpected product value")?;

            Ok(Some(product))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, product: Product) -> Result<(), Error> {
        let id = product.id();

        let mut products = self
            .write()
            .map_err(|_| "not good!")?;

        let raw_product = serde_json::to_value(product)
            .map_err(|_| "unexpected product value")?;
        
        products.insert(id, raw_product);

        Ok(())
    }
}

pub(super) fn in_memory_store() -> InMemoryStore {
    RwLock::new(BTreeMap::new())
}

pub fn store() -> impl Store {
    in_memory_store()
}

impl Resolver {
    pub fn store(&self) -> impl Store {
        self.store.clone()
    }
}
