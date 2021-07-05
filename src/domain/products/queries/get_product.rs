/*! Contains the `GetProductQuery` type. */

use crate::domain::{
    infra::*,
    products::*,
    Error,
};

pub type Result = ::std::result::Result<Option<Product>, Error>;

/** Input for a `GetProductQuery`. */
#[derive(Deserialize)]
pub struct GetProduct {
    pub id: ProductId,
}

/** Get a product entity. */
#[auto_impl(Fn)]
pub trait GetProductQuery {
    fn get_product(&self, query: GetProduct) -> Result;
}

/** Default implementation for a `GetProductQuery`. */
pub(in crate::domain) fn get_product_query(store: impl ProductStore) -> impl GetProductQuery {
    move |query: GetProduct| {
        let product = store.get_product(query.id)?;

        Ok(product)
    }
}

impl Resolver {
    /** Get a product. */
    pub fn get_product_query(&self) -> impl GetProductQuery {
        let store = self.product_store();

        get_product_query(store)
    }
}
