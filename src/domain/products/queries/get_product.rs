use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{Product, ProductId, ProductStore};

pub type GetProductQueryError = String;
pub type GetProductQueryResult = Result<Product, GetProductQueryError>;

#[derive(Deserialize)]
pub struct GetProduct {
    pub id: ProductId,
}

#[auto_impl(Fn)]
pub trait GetProductQuery {
    fn get_product(&self, query: GetProduct) -> Result<Product, GetProductQueryError>;
}

pub fn get_product_query<TStore>(store: TStore) -> impl GetProductQuery
where
    TStore: ProductStore,
{
    move |query: GetProduct| {
        let product = store.get_product(query.id)?.ok_or("not found")?;

        Ok(product)
    }
}

impl Resolver {
    pub fn get_product_query(&self) -> impl GetProductQuery {
        let store = self.products().product_store();

        get_product_query(store)
    }
}
