/*! Contains the `GetOrderSummariesForCustomerQuery` type. */

use crate::domain::{
    Error,
    customers::*,
    infra::*,
    orders::*,
};

/** Input for a `GetOrderSummariesForCustomerQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetOrderSummariesForCustomer {
    pub id: CustomerId,
}

/** An individual order summary. */
#[derive(Serialize)]
pub struct OrderSummary {
    pub id: OrderId,
}

impl QueryArgs for GetOrderSummariesForCustomer {
    type Output = Result<Vec<OrderSummary>, Error>;
}

/** Default implementation for a `GetOrderSummariesForCustomerQuery`. */
async fn execute(
    query: GetOrderSummariesForCustomer,
    store: impl OrderStoreFilter,
) -> Result<Vec<OrderSummary>, Error> {
    store
        .filter(|o| o.customer_id == query.id)?
        .map(|o| Ok(OrderSummary { id: o.id }))
        .collect()
}

impl Resolver {
    /** Get a summary for all orders associated with a customer. */
    pub fn get_order_summaries_for_customer_query(
        &self,
    ) -> impl Query<GetOrderSummariesForCustomer> {
        self.query(|resolver, query: GetOrderSummariesForCustomer| async move {
            let store = resolver.order_store_filter();

            execute(query, store).await
        })
    }
}
