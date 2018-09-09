/*! Contains the `AddOrUpdateProductCommand` type. */

use auto_impl::auto_impl;

use crate::domain::{
    error::{err_msg, Error},
    id::IdProvider,
    orders::{IntoLineItem, LineItemData, LineItemId, OrderId, OrderStore},
    products::{
        queries::{GetProduct, GetProductQuery},
        ProductId,
    },
    Resolver,
};

pub type Result = ::std::result::Result<LineItemId, Error>;

/** Input for an `AddOrUpdateProductCommand`. */
#[derive(Clone, Deserialize)]
pub struct AddOrUpdateProduct {
    pub id: OrderId,
    pub product_id: ProductId,
    pub quantity: u32,
}

/** Add or update a product line item on an order. */
#[auto_impl(FnMut)]
pub trait AddOrUpdateProductCommand {
    fn add_or_update_product(&mut self, command: AddOrUpdateProduct) -> Result;
}

/** Default implementation for an `AddOrUpdateProductCommand`. */
pub(in crate::domain) fn add_or_update_product_command(
    store: impl OrderStore,
    id_provider: impl IdProvider<LineItemData>,
    query: impl GetProductQuery,
) -> impl AddOrUpdateProductCommand {
    move |command: AddOrUpdateProduct| {
        if let Some(order) = store.get_order(command.id)? {
            let id = match order.into_line_item_for_product(command.product_id) {
                IntoLineItem::InOrder(mut line_item) => {
                    let (_, &LineItemData { id, .. }) = line_item.to_data();

                    line_item.set_quantity(command.quantity)?;
                    store.set_line_item(line_item)?;

                    id
                }
                IntoLineItem::NotInOrder(mut order) => {
                    let id = id_provider.id()?;
                    let product = query.get_product(GetProduct {
                        id: command.product_id,
                    })?;

                    order.add_product(id, &product, command.quantity)?;
                    store.set_order(order)?;

                    id
                }
            };

            Ok(id)
        } else {
            Err(err_msg("not found"))?
        }
    }
}

impl Resolver {
    pub fn add_or_update_product_command(&self) -> impl AddOrUpdateProductCommand {
        let order_store = self.orders().order_store();
        let id_provider = self.line_item_id_provider();

        let get_product = self.get_product_query();

        add_or_update_product_command(order_store, id_provider, get_product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        orders::{
            model::{store::in_memory_store, test_data::OrderBuilder},
            *,
        },
        products::{
            model::test_data::ProductBuilder, queries::get_product::Result as QueryResult, *,
        },
    };

    #[test]
    fn add_item_if_not_in_order() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let quantity = 3;

        store
            .set_order(OrderBuilder::new().id(order_id).build())
            .unwrap();

        let mut cmd = add_or_update_product_command(&store, NextLineItemId::new(), |_| {
            let product: QueryResult = Ok(ProductBuilder::new().id(product_id).build());
            product
        });

        let line_item_id = cmd
            .add_or_update_product(AddOrUpdateProduct {
                id: order_id,
                product_id: product_id,
                quantity: quantity,
            }).unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(quantity, line_item.quantity);
    }

    #[test]
    fn update_quantity_if_in_order() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let line_item_id = LineItemId::new();
        let quantity = 3;

        let order = OrderBuilder::new()
            .id(order_id)
            .add_product(
                ProductBuilder::new().id(product_id).build(),
                move |line_item| line_item.id(line_item_id),
            ).build();

        store.set_order(order).unwrap();

        let mut cmd = add_or_update_product_command(&store, NextLineItemId::new(), |_| {
            let product: QueryResult = Ok(ProductBuilder::new().id(product_id).build());
            product
        });

        let updated_line_item_id = cmd
            .add_or_update_product(AddOrUpdateProduct {
                id: order_id,
                product_id: product_id,
                quantity: quantity,
            }).unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(line_item_id, updated_line_item_id);
        assert_eq!(quantity, line_item.quantity);
    }
}
