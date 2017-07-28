use std::collections::BTreeMap;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::products::{Resolver, Product, ProductData};

pub type Error = String;

#[auto_impl(Arc)]
pub trait Store {
    fn get(&self, id: i32) -> Result<Option<Product>, Error>;
    fn set(&self, product: Product) -> Result<(), Error>;
}

pub(in domain::products) type InMemoryStore = RwLock<BTreeMap<i32, ProductData>>;

impl Store for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<Product>, Error> {
        let products = self
            .read()
            .map_err(|_| "not good!")?;

        if let Some(data) = products.get(&id) {
            Ok(Some(Product::from_data(data.clone())))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, product: Product) -> Result<(), Error> {
        let data = product.into_data();
        let id = data.id;

        let mut products = self
            .write()
            .map_err(|_| "not good!")?;

        products.insert(id, data);

        Ok(())
    }
}

pub(in domain::products) fn in_memory_store() -> InMemoryStore {
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
