use auto_impl::auto_impl;

use domain::Resolver;
use domain::customers::{Customer, CustomerId, CustomerStore};

pub type GetCustomerQueryError = String;
pub type GetCustomerQueryResult = Result<Customer, GetCustomerQueryError>;

#[derive(Deserialize)]
pub struct GetCustomer {
    pub id: CustomerId,
}

#[auto_impl(Fn)]
pub trait GetCustomerQuery {
    fn get_customer(&self, query: GetCustomer) -> Result<Customer, GetCustomerQueryError>;
}

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
