/*! Persistent customer storage. */

use crate::{
    domain::{
        customers::*,
        Error,
    },
    store::*,
};

/** A place to persist and fetch customers. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait CustomerStore {
    fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error>;
    fn set_customer(&self, transaction: &Transaction, customer: Customer) -> Result<(), Error>;
}

pub(in crate::domain) struct InMemoryStore(TransactionValueStore<CustomerData>);

impl CustomerStore for InMemoryStore {
    fn get_customer(&self, id: CustomerId) -> Result<Option<Customer>, Error> {
        if let Some((version, data)) = self.0.get(id) {
            assert_eq!(version, data.version.into());

            Ok(Some(Customer::from_data(data)))
        } else {
            Ok(None)
        }
    }

    fn set_customer(&self, transaction: &Transaction, customer: Customer) -> Result<(), Error> {
        let mut data = customer.into_data();
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

pub(in crate::domain) fn in_memory_store(transaction_store: TransactionStore) -> InMemoryStore {
    InMemoryStore(TransactionValueStore::new(transaction_store))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::customers::model::test_data::CustomerBuilder;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store(Default::default());

        let id = CustomerId::new();

        // Create a customer in the store
        store
            .set_customer(&Transaction::none(), CustomerBuilder::new().id(id).build())
            .unwrap();

        // Get the customer from the store
        let found = store.get_customer(id).unwrap().unwrap();
        assert_eq!(id, found.data.id);
    }

    #[test]
    fn add_customer_twice_fails_concurrency_check() {
        let store = in_memory_store(Default::default());

        let id = CustomerId::new();

        // Create a customer in the store
        store
            .set_customer(&Transaction::none(), CustomerBuilder::new().id(id).build())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(store
            .set_customer(&Transaction::none(), CustomerBuilder::new().id(id).build())
            .is_err());
    }
}
