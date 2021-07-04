/*! Contains the `Customer` entity. */

use crate::domain::{
    infra::*,
    Error,
};

pub mod store;

pub type CustomerId = Id<CustomerData>;
pub type NextCustomerId = NextId<CustomerData>;
pub type CustomerVersion = Version<CustomerData>;

#[cfg(test)]
pub mod test_data;

/** Data for a customer. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CustomerData {
    pub id: CustomerId,
    pub version: CustomerVersion,
    _private: (),
}

/** A customer. */
pub struct Customer {
    data: CustomerData,
}

impl Customer {
    pub(self) fn from_data(data: CustomerData) -> Self {
        Customer { data }
    }

    pub fn to_data(&self) -> &CustomerData {
        &self.data
    }

    pub fn into_data(self) -> CustomerData {
        self.data
    }

    pub fn new<TId>(id: TId) -> Result<Self, Error>
    where
        TId: IdProvider<CustomerData>,
    {
        let id = id.get()?;

        Ok(Customer::from_data(CustomerData {
            id,
            version: CustomerVersion::default(),
            _private: (),
        }))
    }
}

impl Entity for Customer {
    type Id = CustomerId;
    type Version = CustomerVersion;
    type Data = CustomerData;
    type Error = Error;
}

impl Resolver {
    pub fn customer_id(&self) -> impl IdProvider<CustomerData> {
        NextId::<CustomerData>::new()
    }
}
