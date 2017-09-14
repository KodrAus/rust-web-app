use auto_impl::auto_impl;

use domain::Resolver;
use domain::id::IdProvider;
use domain::products::ProductId;
use domain::products::queries::{GetProduct, GetProductQuery};
use domain::orders::{LineItemId, LineItemData, IntoLineItem, OrderId, OrderStore};

pub type AddOrUpdateProductError = String;

#[derive(Clone, Deserialize)]
pub struct AddOrUpdateProduct {
    pub id: OrderId,
    pub product_id: ProductId,
    pub quantity: u32,
}

#[auto_impl(FnMut)]
pub trait AddOrUpdateProductCommand {
    fn add_or_update_product(&mut self, command: AddOrUpdateProduct) -> Result<LineItemId, AddOrUpdateProductError>;
}

pub fn add_or_update_product_command<TStore, TLineItemIdProvider, TGetProduct>(
    store: TStore,
    id_provider: TLineItemIdProvider,
    query: TGetProduct,
) -> impl AddOrUpdateProductCommand
where
    TStore: OrderStore,
    TLineItemIdProvider: IdProvider<LineItemData>,
    TGetProduct: GetProductQuery,
{
    move |command: AddOrUpdateProduct| {
        if let Some(order) = store.get_order(command.id)? {
            let id = match order.into_line_item_for_product(command.product_id) {
                IntoLineItem::InOrder(mut line_item) => {
                    let id = {
                        let (_, line_item) = line_item.to_data();
                        line_item.id
                    };

                    line_item.set_quantity(command.quantity)?;
                    store.set_line_item(line_item)?;

                    id
                }
                IntoLineItem::NotInOrder(mut order) => {
                    let id = id_provider.id()?;
                    let product = query.get_product(GetProduct { id: command.product_id })?;

                    order.add_product(id, &product, command.quantity)?;
                    store.set_order(order)?;

                    id
                }
            };

            Ok(id)
        } else {
            Err("not found")?
        }
    }
}

impl Resolver {
    pub fn add_or_update_product_command(&self) -> impl AddOrUpdateProductCommand {
        let order_store = self.orders().order_store();
        let id_provider = self.orders().line_item_id_provider();

        let get_product = self.get_product_query();

        add_or_update_product_command(order_store, id_provider, get_product)
    }
}

#[cfg(test)]
mod tests {
    use domain::customers::*;
    use domain::products::*;
    use domain::orders::model::store::in_memory_store;
    use domain::orders::*;
    use super::*;

    #[test]
    fn add_item_if_not_in_order() {
        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let quantity = 3;

        let order = Order::new(order_id, &Customer::new(1)).unwrap();

        let store = in_memory_store();
        store.set_order(order).unwrap();
        
        let mut cmd = add_or_update_product_command(&store, NextLineItemId::new(), |_| Product::new(product_id, "A title", 1.5f32));

        let line_item_id = cmd.add_or_update_product(AddOrUpdateProduct {
            id: order_id,
            product_id: product_id,
            quantity: quantity
        }).unwrap();

        let (_, line_item) = store.get_line_item(order_id, line_item_id).unwrap().unwrap().into_data();

        assert_eq!(quantity, line_item.quantity);
    }

    #[test]
    fn update_quantity_if_in_order() {
        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let line_item_id = LineItemId::new();
        let quantity = 3;

        let product = Product::new(product_id, "A title", 1.5f32).unwrap();
        let mut order = Order::new(order_id, &Customer::new(1)).unwrap();
        order.add_product(line_item_id, &product, 1).unwrap();

        let store = in_memory_store();
        store.set_order(order).unwrap();
        
        let mut cmd = add_or_update_product_command(&store, NextLineItemId::new(), |_| Product::new(product_id, "A title", 1.5f32));

        let updated_line_item_id = cmd.add_or_update_product(AddOrUpdateProduct {
            id: order_id,
            product_id: product_id,
            quantity: quantity
        }).unwrap();

        let (_, line_item) = store.get_line_item(order_id, line_item_id).unwrap().unwrap().into_data();

        assert_eq!(line_item_id, updated_line_item_id);
        assert_eq!(quantity, line_item.quantity);
    }
}
