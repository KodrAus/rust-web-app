/*! Persistent storage for products. */

use std::vec::IntoIter;

use crate::{
    domain::{
        Error,
        products::*,
    },
    store::*,
};

/* A place to persist and fetch product entities. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait ProductStore {
    fn get_product(&self, id: ProductId) -> Result<Option<Product>, Error>;
    fn set_product(&self, transaction: &Transaction, product: Product) -> Result<(), Error>;
}

/**
An additional store for fetching multiple product records at a time.

This trait is an implementation detail that lets us fetch more than one product.
It will probably need to be refactored or just removed when we add a proper database.
The fact that it's internal to `domain::products` though means the scope of breakage is a bit smaller.
Commands and queries that depend on `ProductStoreFilter` won't need to break their public API.
*/
#[auto_impl(&, Arc)]
pub(in crate::domain) trait ProductStoreFilter {
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&ProductData) -> bool;
}

pub(in crate::domain) type Iter = IntoIter<ProductData>;

/** A test in-memory product store. */
pub(in crate::domain) struct InMemoryStore(TransactionValueStore<ProductData>);

impl ProductStore for InMemoryStore {
    fn get_product(&self, id: ProductId) -> Result<Option<Product>, Error> {
        if let Some((version, data)) = self.0.get(id) {
            assert_eq!(version, data.version.into());

            Ok(Some(Product::from_data(data)))
        } else {
            Ok(None)
        }
    }

    fn set_product(&self, transaction: &Transaction, product: Product) -> Result<(), Error> {
        let mut data = product.into_data();
        let id = data.id;

        self.0.set(
            transaction,
            id,
            Some(data.version),
            data.version.next(),
            data,
        )?;

        Ok(())
    }
}

impl ProductStoreFilter for InMemoryStore {
    #[allow(clippy::needless_collect)]
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&ProductData) -> bool,
    {
        let products: Vec<_> = self.0.get_all(predicate).map(|(_, data)| data).collect();

        Ok(products.into_iter())
    }
}

pub(in crate::domain::products) fn in_memory_store(
    transaction_store: TransactionStore,
) -> InMemoryStore {
    InMemoryStore(TransactionValueStore::new(transaction_store))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::products::model::test_data;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store(Default::default());

        let id = ProductId::new();

        // Create a product in the store
        let product = test_data::ProductBuilder::new().id(id).build();
        store.set_product(&Transaction::none(), product).unwrap();

        // Get the product from the store
        let found = store.get_product(id).unwrap().unwrap();
        assert_eq!(id, found.data.id);
    }

    #[test]
    fn add_product_twice_fails_concurrency_check() {
        let store = in_memory_store(Default::default());

        let id = ProductId::new();

        // Create a product in the store
        store
            .set_product(
                &Transaction::none(),
                test_data::ProductBuilder::new().id(id).build(),
            )
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(
            store
                .set_product(
                    &Transaction::none(),
                    test_data::ProductBuilder::new().id(id).build()
                )
                .is_err()
        );
    }
}
