use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{ProductData, ProductStore};

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
    where TStore: ProductStore
{
    move |query: GetProduct| {
        let ProductData { id, title, .. } = store.get(query.id)?.ok_or("not found")?.into_data();

        Ok(GetProductResult {
            id: id,
            title: title
        })
    }
}

impl Resolver {
    pub fn get_product_query(&self) -> impl GetProductQuery {
        let store = self.product_store();

        get_product_query(store)
    }
}
