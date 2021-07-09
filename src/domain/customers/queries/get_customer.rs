/*! Contains the `GetCustomerQuery` type. */

use crate::domain::{
    customers::*,
    infra::*,
    Error,
};

type Result = ::std::result::Result<Option<Customer>, Error>;

/** Input for a `GetCustomerQuery`. */
#[derive(Deserialize)]
pub struct GetCustomer {
    pub id: CustomerId,
}

impl QueryArgs for GetCustomer {
    type Output = Result;
}

impl GetCustomer {
    async fn execute(&self, store: impl CustomerStore) -> Result {
        let customer = store.get_customer(self.id)?;

        Ok(customer)
    }
}

impl Resolver {
    /** Get a customer. */
    pub fn get_customer_query(&self) -> impl Query<GetCustomer> {
        self.query(|resolver, query: GetCustomer| async move {
            let store = resolver.customer_store();

            query.execute(store).await
        })
    }
}
