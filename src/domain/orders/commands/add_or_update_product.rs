/*! Contains the `AddOrUpdateProductCommand` type. */

use crate::domain::{
    error,
    infra::*,
    orders::*,
    products::*,
    Error,
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
    transaction: ActiveTransaction,
    store: impl OrderStore,
    id: impl IdProvider<LineItemData>,
    query: impl GetProductQuery,
) -> impl AddOrUpdateProductCommand {
    move |command: AddOrUpdateProduct| {
        debug!(
            "updating product `{}` in order `{}`",
            command.product_id, command.id
        );

        if let Some(order) = store.get_order(command.id)? {
            let id = match order.into_line_item_for_product(command.product_id) {
                IntoLineItem::InOrder(mut line_item) => {
                    debug!(
                        "updating existing product `{}` in order `{}`",
                        command.product_id, command.id
                    );

                    let (_, &LineItemData { id, .. }) = line_item.to_data();

                    line_item.set_quantity(command.quantity)?;
                    store.set_line_item(transaction.get(), line_item)?;

                    id
                }
                IntoLineItem::NotInOrder(mut order) => {
                    debug!(
                        "adding new product `{}` to order `{}`",
                        command.product_id, command.id
                    );

                    let id = id.get()?;
                    let product = query
                        .get_product(GetProduct {
                            id: command.product_id,
                        })?
                        .ok_or_else(|| error::bad_input("product not found"))?;

                    order.add_product(id, &product, command.quantity)?;
                    store.set_order(transaction.get(), order)?;

                    id
                }
            };

            info!(
                "updated product `{}` in order `{}`",
                command.product_id, command.id
            );

            Ok(id)
        } else {
            Err(error::bad_input("not found"))
        }
    }
}

impl Resolver {
    pub fn add_or_update_product_command(&self) -> impl AddOrUpdateProductCommand {
        let order_store = self.order_store();
        let active_transaction = self.active_transaction();

        let id = self.line_item_id();

        let get_product = self.get_product_query();

        add_or_update_product_command(active_transaction, order_store, id, get_product)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        orders::model::{
            store::in_memory_store,
            test_data::OrderBuilder,
        },
        products::{
            model::test_data::ProductBuilder,
            queries::get_product::Result as QueryResult,
        },
    };

    #[test]
    fn add_item_if_not_in_order() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let quantity = 3;

        store
            .set_order(
                ActiveTransaction::none().get(),
                OrderBuilder::new().id(order_id).build(),
            )
            .unwrap();

        let mut cmd = add_or_update_product_command(
            ActiveTransaction::none(),
            &store,
            NextLineItemId::new(),
            |_| {
                let product: QueryResult = Ok(Some(ProductBuilder::new().id(product_id).build()));
                product
            },
        );

        let line_item_id = cmd
            .add_or_update_product(AddOrUpdateProduct {
                id: order_id,
                product_id,
                quantity,
            })
            .unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(quantity, line_item.quantity);
    }

    #[test]
    fn update_quantity_if_in_order() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let line_item_id = LineItemId::new();
        let quantity = 3;

        let order = OrderBuilder::new()
            .id(order_id)
            .add_product(
                ProductBuilder::new().id(product_id).build(),
                move |line_item| line_item.id(line_item_id),
            )
            .build();

        store
            .set_order(ActiveTransaction::none().get(), order)
            .unwrap();

        let mut cmd = add_or_update_product_command(
            ActiveTransaction::none(),
            &store,
            NextLineItemId::new(),
            |_| {
                let product: QueryResult = Ok(Some(ProductBuilder::new().id(product_id).build()));
                product
            },
        );

        let updated_line_item_id = cmd
            .add_or_update_product(AddOrUpdateProduct {
                id: order_id,
                product_id,
                quantity,
            })
            .unwrap();

        let (_, line_item) = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap()
            .into_data();

        assert_eq!(line_item_id, updated_line_item_id);
        assert_eq!(quantity, line_item.quantity);
    }
}
