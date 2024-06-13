/*! Contains the `GetOrderQuery` type. */

use crate::domain::{
    infra::*,
    orders::*,
    Error,
};

/** Input for a `GetOrderQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetOrder {
    pub id: OrderId,
}

impl QueryArgs for GetOrder {
    type Output = Result<Option<Order>, Error>;
}

/** Default implementation for a `GetOrderQuery`. */
async fn execute(query: GetOrder, store: impl OrderStore) -> Result<Option<Order>, Error> {
    Ok(store.get_order(query.id)?)
}

impl Resolver {
    /** Get an order. */
    pub fn get_order_query(&self) -> impl Query<GetOrder> {
        self.query(|resolver, query: GetOrder| async move {
            let store = resolver.order_store();

            execute(query, store).await
        })
    }
}
