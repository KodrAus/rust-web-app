/*! Contains the `GetOrderSummariesForCustomerQuery` type. */

use auto_impl::auto_impl;

use domain::Resolver;
use domain::error::Error;
use domain::customers::CustomerId;
use domain::orders::{OrderId, OrderStoreFilter};

pub type Result = ::std::result::Result<Vec<OrderSummary>, Error>;

/** Input for a `GetOrderSummariesForCustomerQuery`. */
#[derive(Deserialize)]
pub struct GetOrderSummariesForCustomer {
    pub id: CustomerId,
}

/** An individual order summary. */
#[derive(Serialize)]
pub struct OrderSummary {
    pub id: OrderId,
}

/** Get a collection of order summaries for a customer. */
#[auto_impl(Fn)]
pub trait GetOrderSummariesForCustomerQuery {
    fn get_order_summaries_for_customer(&self, query: GetOrderSummariesForCustomer) -> Result;
}

/** Default implementation for a `GetOrderSummariesForCustomerQuery`. */
pub fn get_order_summaries_for_customer_query(store: impl OrderStoreFilter) -> impl GetOrderSummariesForCustomerQuery {
    move |query: GetOrderSummariesForCustomer| {
        let orders = store
            .filter(|o| o.customer_id == query.id)?
            .map(|o| OrderSummary { id: o.id })
            .collect();

        Ok(orders)
    }
}

impl Resolver {
    pub fn get_order_summaries_for_customer_query(&self) -> impl GetOrderSummariesForCustomerQuery {
        let store = self.orders().order_store_filter();

        get_order_summaries_for_customer_query(store)
    }
}
