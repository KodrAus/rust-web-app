use auto_impl::auto_impl;

use domain::Resolver;
use domain::orders::{Order, OrderId, OrderStore};

pub type GetOrderQueryError = String;
pub type GetOrderQueryResult = Result<Order, GetOrderQueryError>;

#[derive(Deserialize)]
pub struct GetOrder {
    pub id: OrderId,
}

#[auto_impl(Fn)]
pub trait GetOrderQuery {
    fn get_order(&self, query: GetOrder) -> Result<Order, GetOrderQueryError>;
}

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
