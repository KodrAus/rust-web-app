use auto_impl::auto_impl;

use super::infra::{Resolver, Store};

#[derive(Serialize)]
pub struct GetProductResult {
    pub id: i32,
    pub title: String,
}

pub type QueryError = String;

pub struct GetProduct {
    pub id: i32
}

#[auto_impl(Fn)]
pub trait GetProductQuery {
    fn get_product(&self, query: GetProduct) -> Result<GetProductResult, QueryError>;
}

pub fn get_product_query<TStore>(store: TStore) -> impl GetProductQuery 
    where TStore: Store
{
    move |query: GetProduct| {
        let product = store.get(query.id)?.ok_or("not found")?;

        Ok(GetProductResult {
            id: product.id(),
            title: product.title().to_owned()
        })
    }
}

impl Resolver {
    pub fn get_product_query(&self) -> impl GetProductQuery {
        let store = self.store();

        get_product_query(store)
    }
}