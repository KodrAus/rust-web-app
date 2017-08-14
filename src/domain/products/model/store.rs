use std::collections::BTreeMap;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::products::{Product, ProductId, ProductData};

pub type Error = String;

// TODO: A trait for iterating over products? It'll be a leaky abstraction, but necessary for queries until there's a db
// Maybe just `pub(in domain::products)`?

// Should have:
// - `Iterator<Item = &ProductData>
// - `Vec<Product>: FromIterator<Item = &ProductData>`

#[auto_impl(Arc)]
pub trait ProductStore {
    fn get(&self, id: ProductId) -> Result<Option<Product>, Error>;
    fn set(&self, product: Product) -> Result<(), Error>;
}

pub(in domain) type InMemoryStore = RwLock<BTreeMap<ProductId, ProductData>>;

impl ProductStore for InMemoryStore {
    fn get(&self, id: ProductId) -> Result<Option<Product>, Error> {
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

pub(in domain) fn in_memory_store() -> InMemoryStore {
    RwLock::new(BTreeMap::new())
}

pub fn product_store() -> impl ProductStore {
    in_memory_store()
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::products::*;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();

        let id = ProductId::new();
        let product = Product::new(id, "Some title", 1.5f32).unwrap();
        store.set(product).unwrap();

        let found = store.get(id).unwrap().unwrap();

        assert_eq!(id, found.data.id);
    } 
}
