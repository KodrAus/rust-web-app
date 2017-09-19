use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::RwLock;

use domain::customers::{Customer, CustomerData, CustomerId};

pub type Error = String;

// `syn` doesn't recognise `pub(restricted)`, so we re-export the store
mod re_export {
    use auto_impl::auto_impl;

    use domain::customers::{Customer, CustomerId};
    use super::Error;

    #[auto_impl(Arc)]
    pub trait CustomerStore {
        fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error>;
        fn set_customer(&self, customer: Customer) -> Result<(), Error>;
    }

    impl<'a, T> CustomerStore for &'a T
    where
        T: CustomerStore,
    {
        fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error> {
            (*self).get_customer(id)
        }

        fn set_customer(&self, customer: Customer) -> Result<(), Error> {
            (*self).set_customer(customer)
        }
    }
}

pub(in domain::customers) use self::re_export::CustomerStore;

/// A test in-memory customer store.
pub type InMemoryStore = RwLock<HashMap<CustomerId, CustomerData>>;

impl CustomerStore for InMemoryStore {
    fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error> {
        let customers = self.read().map_err(|_| "not good!")?;

        if let Some(data) = customers.get(&id) {
            Ok(Some(Customer::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set_customer(&self, customer: Customer) -> Result<(), Error> {
        let mut data = customer.into_data();
        let id = data.id;

        let mut customers = self.write().map_err(|_| "not good!")?;

        match customers.entry(id) {
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

pub fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}

pub fn customer_store() -> impl CustomerStore {
    in_memory_store()
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::customers::*;
    use domain::customers::model::test_data::CustomerBuilder;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();

        let id = CustomerId::new();

        // Create a customer in the store
        store
            .set_customer(CustomerBuilder::new().id(id).build())
            .unwrap();

        // Get the customer from the store
        let found = store.get_customer(id).unwrap().unwrap();
        assert_eq!(id, found.data.id);
    }

    #[test]
    fn add_customer_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let id = CustomerId::new();

        // Create a customer in the store
        store
            .set_customer(CustomerBuilder::new().id(id).build())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(
            store
                .set_customer(CustomerBuilder::new().id(id).build())
                .is_err()
        );
    }
}
