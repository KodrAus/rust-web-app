use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{GetProductSummaries, GetProductSummariesQuery, ProductId};
use domain::orders::{LineItemId, Order, OrderId, OrderStore};

pub type GetOrderWithProductsQueryError = String;
pub type GetOrderWithProductsQueryResult = Result<Order, GetOrderWithProductsQueryError>;

#[derive(Deserialize)]
pub struct GetOrderWithProducts {
    pub id: OrderId,
}

#[derive(Serialize)]
pub struct OrderWithProducts {
    pub id: OrderId,
    pub line_items: Vec<ProductLineItem>,
}

#[derive(Serialize)]
pub struct ProductLineItem {
    pub line_item_id: LineItemId,
    pub product_id: ProductId,
    pub title: String,
    pub price: f32,
    pub quantity: u32,
}

#[auto_impl(Fn)]
pub trait GetOrderWithProductsQuery {
    fn get_order_with_products(&self, query: GetOrderWithProducts) -> Result<OrderWithProducts, GetOrderWithProductsQueryError>;
}

pub fn get_order_with_products_query<TStore, TQuery>(store: TStore, products_query: TQuery) -> impl GetOrderWithProductsQuery
where
    TStore: OrderStore,
    TQuery: GetProductSummariesQuery,
{
    move |query: GetOrderWithProducts| {
        let (order, line_items) = store.get_order(query.id)?.ok_or("not found")?.into_data();
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
                    .map(|product| {
                        ProductLineItem {
                            line_item_id: line_item.id,
                            product_id: product.id,
                            title: product.title.to_owned(),
                            price: product.price,
                            quantity: line_item.quantity,
                        }
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
