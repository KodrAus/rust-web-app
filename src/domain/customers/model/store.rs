/*! Persistent customer storage. */
use auto_impl::auto_impl;

use std::{
    collections::{hash_map::Entry, HashMap},
    sync::RwLock,
};

use crate::domain::{
    customers::{Customer, CustomerData, CustomerId},
    error::{err_msg, Error},
};

/** A place to persist and fetch customers. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait CustomerStore {
    fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error>;
    fn set_customer(&self, customer: Customer) -> Result<(), Error>;
}

/* A test in-memory customer store. */
pub(in crate::domain) type InMemoryStore = RwLock<HashMap<CustomerId, CustomerData>>;

impl CustomerStore for InMemoryStore {
    fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error> {
        let customers = self.read().map_err(|_| err_msg("not good!"))?;

        if let Some(data) = customers.get(&id) {
            Ok(Some(Customer::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set_customer(&self, customer: Customer) -> Result<(), Error> {
        let mut data = customer.into_data();
        let id = data.id;

        let mut customers = self.write().map_err(|_| err_msg("not good!"))?;

        match customers.entry(id) {
            Entry::Vacant(entry) => {
                data.version.next();
                entry.insert(data);
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.version != data.version {
                    Err(err_msg("optimistic concurrency fail"))?
                }

                data.version.next();
                *entry = data;
            }
        }

        Ok(())
    }
}

pub(in crate::domain) fn in_memory_store() -> InMemoryStore {
    RwLock::new(HashMap::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::customers::{model::test_data::CustomerBuilder, *};

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
        assert!(store
            .set_customer(CustomerBuilder::new().id(id).build())
            .is_err());
    }
}
