/*! Persistent storage for products. */

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::RwLock;

use domain::products::{Product, ProductData, ProductId};

pub type Error = String;

// `syn` doesn't recognise `pub(restricted)`, so we re-export the store
mod re_export {
    use std::vec::IntoIter;
    use auto_impl::auto_impl;

    use domain::products::{Product, ProductData, ProductId};
    use super::Error;

    /* A place to persist and fetch product entities. */
    #[auto_impl(&, Arc)]
    pub trait ProductStore {
        fn get_product(&self, id: ProductId) -> Result<Option<Product>, Error>;
        fn set_product(&self, product: Product) -> Result<(), Error>;
    }

    /**
    An additional store for fetching multiple product records at a time.

    This trait is an implementation detail that lets us fetch more than one product.
    It will probably need to be refactored or just removed when we add a proper database.
    The fact that it's internal to `domain::products` though means the scope of breakage is a bit smaller.
    Commands and queries that depend on `ProductStoreFilter` won't need to break their public API.
    */
    #[auto_impl(&, Arc)]
    pub trait ProductStoreFilter {
        fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
        where
            F: Fn(&ProductData) -> bool;
    }

    pub type Iter = IntoIter<ProductData>;
}

pub(in domain::products) use self::re_export::{Iter, ProductStore, ProductStoreFilter};

/** A test in-memory product store. */
pub(in domain::products) type InMemoryStore = RwLock<HashMap<ProductId, ProductData>>;

impl ProductStore for InMemoryStore {
    fn get_product(&self, id: ProductId) -> Result<Option<Product>, Error> {
        let products = self.read().map_err(|_| "not good!")?;

        if let Some(data) = products.get(&id) {
            Ok(Some(Product::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set_product(&self, product: Product) -> Result<(), Error> {
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

impl ProductStoreFilter for InMemoryStore {
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&ProductData) -> bool,
    {
        let products: Vec<_> = self.read()
            .map_err(|_| "not good!")?
            .values()
            .filter(|p| predicate(*p))
            .cloned()
            .collect();

        Ok(products.into_iter())
    }
}

pub(in domain::products) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}

/** Default implementation for a `ProductStore`. */
pub fn product_store() -> impl ProductStore {
    in_memory_store()
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::products::*;
    use domain::products::model::test_data;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();

        let id = ProductId::new();

        // Create a product in the store
        let product = test_data::ProductBuilder::new().id(id).build();
        store.set_product(product).unwrap();

        // Get the product from the store
        let found = store.get_product(id).unwrap().unwrap();
        assert_eq!(id, found.data.id);
    }

    #[test]
    fn add_product_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let id = ProductId::new();

        // Create a product in the store
        store
            .set_product(test_data::ProductBuilder::new().id(id).build())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(
            store
                .set_product(test_data::ProductBuilder::new().id(id).build())
                .is_err()
        );
    }
}
