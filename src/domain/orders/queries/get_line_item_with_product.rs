/*! Contains the `GetLineItemWithProductQuery` type. */

use crate::domain::{
    infra::*,
    orders::*,
    products::{
        GetProduct,
        ProductId,
    },
    Error,
};

/** Input for a `GetLineItemWithProductQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetLineItemWithProduct {
    pub id: OrderId,
    pub line_item_id: LineItemId,
}

#[derive(Serialize)]
pub struct LineItemWithProduct {
    pub order_id: OrderId,
    pub line_item_id: LineItemId,
    pub product_id: ProductId,
    pub title: Option<String>,
    pub original_price: Option<Currency>,
    pub price: Currency,
    pub quantity: u32,
}

impl QueryArgs for GetLineItemWithProduct {
    type Output = Result<Option<LineItemWithProduct>, Error>;
}

/** Default implementation for a `GetLineItemWithProductQuery`. */
async fn execute(
    query: GetLineItemWithProduct,
    store: impl OrderStore,
    product_query: impl Query<GetProduct>,
) -> Result<Option<LineItemWithProduct>, Error> {
    let line_item = store.get_line_item(query.id, query.line_item_id)?;

    let Some(line_item) = line_item else {
        return Ok(None);
    };

    let (_, line_item) = line_item.into_data();

    let product = product_query
        .execute(GetProduct {
            id: line_item.product_id,
        })
        .await?;

    let (title, original_price) = if let Some(product) = product {
        let product = product.into_data();

        (Some(product.title), Some(product.price))
    } else {
        (None, None)
    };

    Ok(Some(LineItemWithProduct {
        order_id: query.id,
        line_item_id: query.line_item_id,
        product_id: line_item.product_id,
        title,
        original_price,
        price: line_item.price,
        quantity: line_item.quantity,
    }))
}

impl Resolver {
    /** Get a line item with its associated product. */
    pub fn get_line_item_with_product_query(&self) -> impl Query<GetLineItemWithProduct> {
        self.query(|resolver, query: GetLineItemWithProduct| async move {
            let store = resolver.order_store();
            let product_query = resolver.get_product_query();

            execute(query, store, product_query).await
        })
    }
}
