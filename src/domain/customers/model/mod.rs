/*! Contains the `Customer` entity. */

use crate::domain::{
    entity::Entity,
    error::Error,
    id::{Id, IdProvider, NextId},
    version::Version,
    Resolver,
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
        Customer { data: data }
    }

    pub fn to_data(&self) -> &CustomerData {
        &self.data
    }

    pub fn into_data(self) -> CustomerData {
        self.data
    }

    pub fn new<TId>(id_provider: TId) -> Result<Self, Error>
    where
        TId: IdProvider<CustomerData>,
    {
        let id = id_provider.id()?;

        Ok(Customer::from_data(CustomerData {
            id: id,
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
    pub fn customer_id_provider(&self) -> impl IdProvider<CustomerData> {
        NextId::<CustomerData>::new()
    }
}
