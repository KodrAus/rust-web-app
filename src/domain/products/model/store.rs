use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::products::{Product, ProductData, ProductId};

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

pub(in domain::products) type InMemoryStore = RwLock<HashMap<ProductId, ProductData>>;

impl ProductStore for InMemoryStore {
    fn get(&self, id: ProductId) -> Result<Option<Product>, Error> {
        let products = self.read().map_err(|_| "not good!")?;

        if let Some(data) = products.get(&id) {
            Ok(Some(Product::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set(&self, product: Product) -> Result<(), Error> {
        let mut data = product.into_data();
        let id = data.id;

        let mut products = self.write().map_err(|_| "not good!")?;

        match products.entry(id) {
            Entry::Vacant(entry) => {
                data.version.next();
                entry.insert(data);
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.version != data.version {
                    Err("optimistic concurrency fail")?
                }

                data.version.next();
                *entry = data;
            }
        }

        Ok(())
    }
}

impl<'a> ProductStore for &'a InMemoryStore {
    fn get(&self, id: ProductId) -> Result<Option<Product>, Error> {
        (*self).get(id)
    }

    fn set(&self, product: Product) -> Result<(), Error> {
        (*self).set(product)
    }
}

pub(in domain::products) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
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

        // Create a product in the store
        {
            let product = Product::new(id, "Some title", 1.5f32).unwrap();
            store.set(product).unwrap();
        }
        // Get the product from the store
        {
            let found = store.get(id).unwrap().unwrap();
            assert_eq!(id, found.data.id);
        }
    }

    #[test]
    fn add_product_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let id = ProductId::new();

        // Create a product in the store
        store.set(Product::new(id, "Some title", 1.5f32).unwrap()).unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(store.set(Product::new(id, "Some title", 1.5f32).unwrap()).is_err());
    }
}
