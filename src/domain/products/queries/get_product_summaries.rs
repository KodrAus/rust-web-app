use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{Product, ProductId, ProductStoreFilter};

pub type GetProductSummariesQueryError = String;
pub type GetProductSummariesQueryResult = Result<Product, GetProductSummariesQueryError>;

#[derive(Deserialize)]
pub struct GetProductSummaries {
    pub ids: Vec<ProductId>,
}

#[derive(Serialize)]
pub struct ProductSummary {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
}

#[auto_impl(Fn)]
pub trait GetProductSummariesQuery {
    fn get_product_summaries(&self, query: GetProductSummaries) -> Result<Vec<ProductSummary>, GetProductSummariesQueryError>;
}

pub fn get_product_summaries_query<TStore>(store: TStore) -> impl GetProductSummariesQuery
where
    TStore: ProductStoreFilter,
{
    move |query: GetProductSummaries| {
        let products = store
            .filter(|p| query.ids.iter().any(|id| p.id == *id))?
            .map(|p| {
                ProductSummary {
                    id: p.id,
                    title: p.title,
                    price: p.price,
                }
            })
            .collect();

        Ok(products)
    }
}

impl Resolver {
    pub fn get_product_summaries_query(&self) -> impl GetProductSummariesQuery {
        let store = self.products().product_store_filter();

        get_product_summaries_query(store)
    }
}
