/*! Contains the `GetCustomerQuery` type. */

use crate::domain::{
    customers::*,
    infra::*,
    Error,
};

/** Input for a `GetCustomerQuery`. */
#[derive(Deserialize)]
pub struct GetCustomer {
    pub id: CustomerId,
}

impl QueryArgs for GetCustomer {
    type Output = Result<Option<Customer>, Error>;
}

async fn execute(query: GetCustomer, store: impl CustomerStore) -> Result<Option<Customer>, Error> {
    let customer = store.get_customer(query.id)?;

    Ok(customer)
}

impl Resolver {
    /** Get a customer. */
    pub fn get_customer_query(&self) -> impl Query<GetCustomer> {
        self.query(|resolver, query: GetCustomer| async move {
            let store = resolver.customer_store();

            execute(query, store).await
        })
    }
}
