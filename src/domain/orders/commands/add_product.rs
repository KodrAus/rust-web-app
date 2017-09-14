use auto_impl::auto_impl;

use domain::Resolver;
use domain::id::IdProvider;
use domain::products::ProductId;
use domain::products::queries::{GetProduct, GetProductQuery};
use domain::orders::{LineItemId, LineItemData, IntoLineItem, OrderId, OrderStore,
                     OrderLineItemStore};

pub type AddProductError = String;

#[derive(Clone, Deserialize)]
pub struct AddProduct {
    pub id: OrderId,
    pub product_id: ProductId,
    pub quantity: u32,
}

#[auto_impl(FnMut)]
pub trait AddProductCommand {
    fn add_product(&mut self, command: AddProduct) -> Result<LineItemId, AddProductError>;
}

pub fn add_product_command<TStore, TLineItemStore, TLineItemIdProvider, TGetProduct>(
    order_store: TStore,
    line_item_store: TLineItemStore,
    id_provider: TLineItemIdProvider,
    query: TGetProduct,
) -> impl AddProductCommand
where
    TStore: OrderStore,
    TLineItemStore: OrderLineItemStore,
    TLineItemIdProvider: IdProvider<LineItemData>,
    TGetProduct: GetProductQuery,
{
    move |command: AddProduct| if let Some(order) = order_store.get(command.id)? {
        let id = match order.into_line_item(command.product_id) {
            IntoLineItem::InOrder(mut line_item) => {
                let id = {
                    let (_, line_item) = line_item.to_data();
                    line_item.id
                };

                line_item.set_quantity(command.quantity)?;
                line_item_store.set(line_item)?;

                id
            }
            IntoLineItem::NotInOrder(mut order) => {
                let id = id_provider.id()?;
                let product = query.get_product(GetProduct { id: command.product_id })?;

                order.add_product(id, &product, command.quantity)?;

                order_store.set(order)?;

                id
            }
        };

        Ok(id)
    } else {
        Err("not found")?
    }
}

impl Resolver {
    pub fn add_product_command(&self) -> impl AddProductCommand {
        let order_store = self.orders().order_store();
        let item_store = self.orders().line_item_store();
        let id_provider = self.orders().line_item_id_provider();

        let get_product = self.get_product_query();

        add_product_command(order_store, item_store, id_provider, get_product)
    }
}

#[cfg(test)]
mod tests {
    use domain::orders::model::store::in_memory_store;
    use domain::orders::*;
    use super::*;

    #[test]
    fn add_item_if_not_in_order() {
        unimplemented!();
    }

    #[test]
    fn update_quantity_if_in_order() {
        unimplemented!();
    }
}
