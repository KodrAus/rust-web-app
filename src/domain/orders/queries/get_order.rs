/*! Contains the `GetOrderQuery` type. */

use auto_impl::auto_impl;

use crate::domain::{
    error::Error,
    orders::{
        Order,
        OrderId,
        OrderStore,
    },
    Resolver,
};

pub type Result = ::std::result::Result<Option<Order>, Error>;

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
pub(in crate::domain) fn get_order_query(store: impl OrderStore) -> impl GetOrderQuery {
    move |query: GetOrder| {
        let order = store.get_order(query.id)?;

        Ok(order)
    }
}

impl Resolver {
    pub fn get_order_query(&self) -> impl GetOrderQuery {
        let store = self.orders().order_store();

        get_order_query(store)
    }
}
