/*! Contains the `GetCustomerWithOrdersQuery` type. */

use auto_impl::auto_impl;

use crate::domain::{
    customers::{
        CustomerId,
        CustomerStore,
    },
    error::Error,
    orders::{
        GetOrderSummariesForCustomer,
        GetOrderSummariesForCustomerQuery,
        OrderId,
    },
    Resolver,
};

pub type Result = ::std::result::Result<Option<CustomerWithOrders>, Error>;

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

/** Get a customer along with their orders. */
#[auto_impl(Fn)]
pub trait GetCustomerWithOrdersQuery {
    fn get_customer_with_orders(&self, query: GetCustomerWithOrders) -> Result;
}

/** Default implementation for a `GetCustomerWithOrdersQuery`. */
pub(in crate::domain) fn get_customer_with_orders_query(
    store: impl CustomerStore,
    orders_query: impl GetOrderSummariesForCustomerQuery,
) -> impl GetCustomerWithOrdersQuery {
    move |query: GetCustomerWithOrders| {
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
}

impl Resolver {
    pub fn get_customer_with_orders_query(&self) -> impl GetCustomerWithOrdersQuery {
        let store = self.customers().customer_store();
        let query = self.get_order_summaries_for_customer_query();

        get_customer_with_orders_query(store, query)
    }
}
