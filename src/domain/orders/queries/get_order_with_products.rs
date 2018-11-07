/*! Contains the `GetOrderWithProductsQuery` type. */

use auto_impl::auto_impl;

use crate::domain::{
    error::{err_msg, Error},
    orders::{LineItemId, OrderId, OrderStore},
    products::{GetProductSummaries, GetProductSummariesQuery, ProductId},
    Resolver,
};

pub type Result = ::std::result::Result<OrderWithProducts, Error>;

/** Input for a `GetOrderWithProductsQuery`. */
#[derive(Deserialize)]
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
    pub price: f32,
    pub quantity: u32,
}

/** Get an order along with a product summary for each line item. */
#[auto_impl(Fn)]
pub trait GetOrderWithProductsQuery {
    fn get_order_with_products(&self, query: GetOrderWithProducts) -> Result;
}

/** Default implementation for a `GetOrderWithProductsQuery`. */
pub(in crate::domain) fn get_order_with_products_query(
    store: impl OrderStore,
    products_query: impl GetProductSummariesQuery,
) -> impl GetOrderWithProductsQuery {
    move |query: GetOrderWithProducts| {
        let (order, line_items) = store
            .get_order(query.id)?
            .ok_or(err_msg("not found"))?
            .into_data();
        let products = {
            let product_ids = line_items.iter().map(|l| l.product_id).collect();
            products_query.get_product_summaries(GetProductSummaries { ids: product_ids })
        }?;

        let line_items = line_items
            .into_iter()
            .filter_map(|line_item| {
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
            })
            .collect();

        Ok(OrderWithProducts {
            id: order.id,
            line_items: line_items,
        })
    }
}

impl Resolver {
    pub fn get_order_with_products_query(&self) -> impl GetOrderWithProductsQuery {
        let store = self.orders().order_store();
        let query = self.get_product_summaries_query();

        get_order_with_products_query(store, query)
    }
}
