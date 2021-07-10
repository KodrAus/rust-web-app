/*! Contains the `GetCustomerWithOrdersQuery` type. */

use crate::domain::{
    customers::*,
    infra::*,
    orders::*,
    Error,
};

type Result = ::std::result::Result<Option<CustomerWithOrders>, Error>;

/** Input for a `GetCustomerWithOrdersQuery`. */
#[derive(Deserialize)]
pub struct GetCustomerWithOrders {
    pub id: CustomerId,
}

/** An order with a order summary for each of its line items. */
#[derive(Serialize)]
pub struct CustomerWithOrders {
    pub id: CustomerId,
    pub orders: Vec<CustomerOrder>,
}

/** An individual order. */
#[derive(Serialize)]
pub struct CustomerOrder {
    pub id: OrderId,
}

impl QueryArgs for GetCustomerWithOrders {
    type Output = Result;
}

async fn execute(
    query: GetCustomerWithOrders,
    store: impl CustomerStore,
    orders_query: impl GetOrderSummariesForCustomerQuery,
) -> Result {
    let customer = match store.get_customer(query.id)? {
        Some(customer) => customer.into_data(),
        None => return Ok(None),
    };

    let orders = orders_query
        .get_order_summaries_for_customer(GetOrderSummariesForCustomer { id: query.id })?;

    Ok(Some(CustomerWithOrders {
        id: customer.id,
        orders: orders
            .into_iter()
            .map(|order| CustomerOrder { id: order.id })
            .collect(),
    }))
}

impl Resolver {
    /** Get a customer along with all of their orders. */
    pub fn get_customer_with_orders_query(&self) -> impl Query<GetCustomerWithOrders> {
        self.query(|resolver, query: GetCustomerWithOrders| async move {
            let store = resolver.customer_store();
            let orders_query = resolver.get_order_summaries_for_customer_query();

            execute(query, store, orders_query).await
        })
    }
}
