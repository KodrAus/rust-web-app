/*! Contains the `GetCustomerQuery` type. */

use auto_impl::auto_impl;

use domain::Resolver;
use domain::customers::{Customer, CustomerId, CustomerStore};

pub type Error = String;
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
pub fn get_customer_query<TStore>(store: TStore) -> impl GetCustomerQuery
where
    TStore: CustomerStore,
{
    move |query: GetCustomer| {
        let customer = store.get_customer(query.id)?.ok_or("not found")?;

        Ok(customer)
    }
}

impl Resolver {
    pub fn get_customer_query(&self) -> impl GetCustomerQuery {
        let store = self.customers().customer_store();

        get_customer_query(store)
    }
}
