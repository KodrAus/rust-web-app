/*! Contains the `GetProductSummariesQuery` type. */

use crate::domain::{
    Error,
    infra::*,
    products::*,
};

/** Input for a `GetProductSummariesQuery`. */
#[derive(Serialize, Deserialize)]
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

impl QueryArgs for GetProductSummaries {
    type Output = Result<Vec<ProductSummary>, Error>;
}

/** Default implementation for a `GetProductSummariesQuery`. */
async fn execute(
    query: GetProductSummaries,
    store: impl ProductStoreFilter,
) -> Result<Vec<ProductSummary>, Error> {
    store
        .filter(|p| query.ids.iter().any(|id| p.id == *id))?
        .map(|p| {
            Ok(ProductSummary {
                id: p.id,
                title: p.title,
                price: p.price,
            })
        })
        .collect()
}

impl Resolver {
    /** Get some summary info for a set of products by id. */
    pub fn get_product_summaries_query(&self) -> impl Query<GetProductSummaries> {
        self.query(|resolver, query: GetProductSummaries| async move {
            let store = resolver.product_store_filter();

            execute(query, store).await
        })
    }
}
