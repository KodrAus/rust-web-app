/*! Contains the `GetProductSummariesQuery` type. */

use crate::domain::{
    infra::*,
    products::*,
    Error,
};

pub type Result = ::std::result::Result<Vec<ProductSummary>, Error>;

/** Input for a `GetProductSummariesQuery`. */
#[derive(Deserialize)]
pub struct GetProductSummaries {
    pub ids: Vec<ProductId>,
}

/** An individual product summary. */
#[derive(Serialize)]
pub struct ProductSummary {
    pub id: ProductId,
    pub title: String,
    pub price: Currency,
}

/** Get a collection of product summaries. */
#[auto_impl(Fn)]
pub trait GetProductSummariesQuery {
    fn get_product_summaries(&self, query: GetProductSummaries) -> Result;
}

/** Default implementation for a `GetProductSummariesQuery`. */
pub(in crate::domain) fn get_product_summaries_query(
    store: impl ProductStoreFilter,
) -> impl GetProductSummariesQuery {
    move |query: GetProductSummaries| {
        let products = store
            .filter(|p| query.ids.iter().any(|id| p.id == *id))?
            .map(|p| ProductSummary {
                id: p.id,
                title: p.title,
                price: p.price,
            })
            .collect();

        Ok(products)
    }
}

impl Resolver {
    /** Get some summary info for a set of products by id. */
    pub fn get_product_summaries_query(&self) -> impl GetProductSummariesQuery {
        let store = self.product_store_filter();

        get_product_summaries_query(store)
    }
}
