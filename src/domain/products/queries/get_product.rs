/*! Contains the `GetProductQuery` type. */

use crate::domain::{
    Error,
    infra::*,
    products::*,
};

/** Input for a `GetProductQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetProduct {
    pub id: ProductId,
}

impl QueryArgs for GetProduct {
    type Output = Result<Option<Product>, Error>;
}

/** Default implementation for a `GetProductQuery`. */
async fn execute(query: GetProduct, store: impl ProductStore) -> Result<Option<Product>, Error> {
    let product = store.get_product(query.id)?;

    Ok(product)
}

impl Resolver {
    /** Get a product. */
    pub fn get_product_query(&self) -> impl Query<GetProduct> {
        self.query(|resolver, query: GetProduct| async move {
            let store = resolver.product_store();

            execute(query, store).await
        })
    }
}
