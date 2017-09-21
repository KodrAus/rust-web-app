/*! Contains the `GetOrderQuery` type. */

use auto_impl::auto_impl;

use domain::Resolver;
use domain::orders::{Order, OrderId, OrderStore};

pub type Error = String;
pub type Result = ::std::result::Result<Order, Error>;

/** Input for a `GetOrderQuery`. */
#[derive(Deserialize)]
pub struct GetOrder {
    pub id: OrderId,
}

/** Get an order entity. */
#[auto_impl(Fn)]
pub trait GetOrderQuery {
    fn get_order(&self, query: GetOrder) -> Result;
}

/** Default implementation for a `GetOrderQuery`. */
pub fn get_order_query<TStore>(store: TStore) -> impl GetOrderQuery
where
    TStore: OrderStore,
{
    move |query: GetOrder| {
        let order = store.get_order(query.id)?.ok_or("not found")?;

        Ok(order)
    }
}

impl Resolver {
    pub fn get_order_query(&self) -> impl GetOrderQuery {
        let store = self.orders().order_store();

        get_order_query(store)
    }
}
