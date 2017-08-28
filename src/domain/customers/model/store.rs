use std::collections::BTreeMap;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::customers::{Customer, CustomerData};

pub type Error = String;

#[auto_impl(Arc)]
pub trait CustomerStore {
    fn get(&self, id: i32) -> Result<Option<Customer>, Error>;
    fn set(&self, customer: Customer) -> Result<(), Error>;
}

pub type InMemoryStore = RwLock<BTreeMap<i32, CustomerData>>;

impl CustomerStore for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<Customer>, Error> {
        let customers = self.read().map_err(|_| "not good!")?;

        if let Some(data) = customers.get(&id) {
            Ok(Some(Customer::from_data(data.clone())))
        } else {
            Ok(None)
        }
    }

    fn set(&self, customer: Customer) -> Result<(), Error> {
        let data = customer.into_data();
        let id = data.id;

        let mut customers = self.write().map_err(|_| "not good!")?;

        customers.insert(id, data);

        Ok(())
    }
}

pub fn in_memory_store() -> InMemoryStore {
    RwLock::new(BTreeMap::new())
}

pub fn customer_store() -> impl CustomerStore {
    in_memory_store()
}
