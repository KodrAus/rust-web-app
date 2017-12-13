/*! Contains the `GetCustomerQuery` type. */

use auto_impl::auto_impl;

use domain::Resolver;
use domain::error::{err_msg, Error};
use domain::customers::{Customer, CustomerId, CustomerStore};

pub type Result = ::std::result::Result<Customer, Error>;

/** Input for a `GetCustomerQuery`. */
#[derive(Deserialize)]
pub struct GetCustomer {
    pub id: CustomerId,
}

/** Get a customer entity. */
#[auto_impl(Fn)]
pub trait GetCustomerQuery {
    fn get_customer(&self, query: GetCustomer) -> Result;
}

/** Default implementation for a `GetCustomerQuery`. */
pub fn get_customer_query(store: impl CustomerStore) -> impl GetCustomerQuery {
    move |query: GetCustomer| {
        let customer = store.get_customer(query.id)?.ok_or(err_msg("not found"))?;

        Ok(customer)
    }
}

impl Resolver {
    pub fn get_customer_query(&self) -> impl GetCustomerQuery {
        let store = self.customers().customer_store();

        get_customer_query(store)
    }
}
