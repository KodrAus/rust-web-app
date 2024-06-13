/*! Contains the `GetOrderWithProductsQuery` type. */

use crate::domain::{
    error,
    infra::*,
    orders::*,
    products::*,
    Error,
};

/** Input for a `GetOrderWithProductsQuery`. */
#[derive(Serialize, Deserialize)]
pub struct GetOrderWithProducts {
    pub id: OrderId,
}

/** An order with a product summary for each of its line items. */
#[derive(Serialize)]
pub struct OrderWithProducts {
    pub id: OrderId,
    pub line_items: Vec<ProductLineItem>,
}

/** An individual line item with a product summary. */
#[derive(Serialize)]
pub struct ProductLineItem {
    pub line_item_id: LineItemId,
    pub product_id: ProductId,
    pub title: String,
    pub price: Currency,
    pub quantity: u32,
}

impl QueryArgs for GetOrderWithProducts {
    type Output = Result<Option<OrderWithProducts>, Error>;
}

/** Default implementation for a `GetOrderWithProductsQuery`. */
async fn execute(
    query: GetOrderWithProducts,
    store: impl OrderStore,
    products_query: impl Query<GetProductSummaries>,
) -> Result<Option<OrderWithProducts>, Error> {
    let (order, line_items) = match store.get_order(query.id)? {
        Some(order) => order.into_data(),
        None => return Ok(None),
    };

    let products = {
        let product_ids = line_items.iter().map(|l| l.product_id).collect();
        products_query.execute(GetProductSummaries { ids: product_ids })
    }
    .await?;

    let line_items = line_items
        .into_iter()
        .map(|line_item| {
            products
                .iter()
                .find(|p| p.id == line_item.product_id)
                .map(|product| ProductLineItem {
                    line_item_id: line_item.id,
                    product_id: product.id,
                    title: product.title.to_owned(),
                    price: product.price,
                    quantity: line_item.quantity,
                })
                .ok_or_else(|| error::bad_input("missing product for line item"))
        })
        .collect::<std::result::Result<Vec<_>, _>>()?;

    Ok(Some(OrderWithProducts {
        id: order.id,
        line_items,
    }))
}

impl Resolver {
    /** Get an order along with product data for each of its line items. */
    pub fn get_order_with_products_query(&self) -> impl Query<GetOrderWithProducts> {
        self.query(|resolver, query: GetOrderWithProducts| async move {
            let store = resolver.order_store();
            let products_query = resolver.get_product_summaries_query();

            execute(query, store, products_query).await
        })
    }
}
