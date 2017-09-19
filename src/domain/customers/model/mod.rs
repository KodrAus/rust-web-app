/*!
Entities for customers.
*/

use domain::entity::Entity;
use domain::id::{Id, IdProvider, NextId};
use domain::version::Version;

pub mod store;

pub type CustomerId = Id<CustomerData>;
pub type NextCustomerId = NextId<CustomerData>;
pub type CustomerVersion = Version<CustomerData>;

pub type CustomerError = String;

#[cfg(test)]
pub mod test_data;

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomerData {
    pub id: CustomerId,
    pub version: CustomerVersion,
    _private: (),
}

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

    pub fn new<TId>(id_provider: TId) -> Result<Self, CustomerError>
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
    type Error = CustomerError;
}
