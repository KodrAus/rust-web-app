use std::convert::{TryFrom, TryInto};

pub mod id;
pub mod store;

pub use self::id::*;

use domain::products::{Product, ProductData, ProductId};
use domain::customers::{Customer, CustomerData};

pub type OrderError = String;

/// An order item quantity.
pub struct Quantity(u32);

impl TryFrom<u32> for Quantity {
    type Error = OrderError;

    fn try_from(quantity: u32) -> Result<Self, Self::Error> {
        if quantity < 1 {
            Err("quantity must be greater than 0")?
        }

        Ok(Quantity(quantity))
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: OrderId,
    pub customer_id: i32,
    _private: (),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemData {
    pub id: LineItemId,
    pub product_id: ProductId,
    pub price: f32,
    pub quantity: u32,
    _private: (),
}

/// An order and its line items.
///
/// Products can be added to an order as a line item, so long as it isn't already there.
pub struct Order {
    order: OrderData,
    line_items: Vec<LineItemData>,
}

/// An order and one of its line items.
///
/// Properties on the line item can be updated.
pub struct OrderLineItem {
    order: OrderData,
    line_item: LineItemData,
}

impl OrderLineItem {
    pub(self) fn from_data(order: OrderData, line_item: LineItemData) -> Self {
        OrderLineItem {
            order: order,
            line_item: line_item,
        }
    }

    pub fn into_data(self) -> (OrderData, LineItemData) {
        (self.order, self.line_item)
    }

    pub fn to_data(&self) -> (&OrderData, &LineItemData) {
        (&self.order, &self.line_item)
    }

    pub fn set_quantity<TQuantity>(&mut self, quantity: TQuantity) -> Result<(), OrderError>
    where
        TQuantity: TryInto<Quantity, Error = OrderError>,
    {
        self.line_item.quantity = quantity.try_into()?.0;

        Ok(())
    }
}

impl Order {
    pub(self) fn from_data<TItems>(order: OrderData, line_items: TItems) -> Self
    where
        TItems: IntoIterator<Item = LineItemData>,
    {
        let line_items = line_items.into_iter().collect();

        Order {
            order: order,
            line_items: line_items,
        }
    }

    pub fn into_data(self) -> (OrderData, Vec<LineItemData>) {
        (self.order, self.line_items)
    }

    pub fn to_data(&self) -> (&OrderData, &[LineItemData]) {
        (&self.order, &self.line_items)
    }

    pub fn new<TId>(id: TId, customer: &Customer) -> Result<Self, OrderError>
    where
        TId: OrderIdProvider,
    {
        let id = id.order_id()?;
        let &CustomerData {
            id: customer_id, ..
        } = customer.to_data();

        let order_data = OrderData {
            id: id,
            customer_id: customer_id,
            _private: (),
        };

        Ok(Order::from_data(order_data, vec![]))
    }

    pub fn contains_product(&self, product_id: ProductId) -> bool {
        self.line_items
            .iter()
            .any(|item| item.product_id == product_id)
    }

    pub fn add_product<TId, TQuantity>(
        &mut self,
        id: TId,
        product: &Product,
        quantity: TQuantity,
    ) -> Result<(), OrderError>
    where
        TId: LineItemIdProvider,
        TQuantity: TryInto<Quantity, Error = OrderError>,
    {
        let &ProductData {
            id: product_id,
            price,
            ..
        } = product.to_data();

        if self.contains_product(product_id) {
            Err("product is already in order")?
        }

        let id = id.line_item_id()?;
        let line_item = LineItemData {
            id: id,
            product_id: product_id,
            price: price,
            quantity: quantity.try_into()?.0,
            _private: (),
        };

        self.line_items.push(line_item);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::products::*;
    use domain::customers::*;

    #[test]
    fn add_item_to_order() {
        let product_id = ProductId::new();
        let product = Product::new(product_id, "A title", 1f32).unwrap();

        let customer = Customer::new(1);

        let order_id = OrderId::new();
        let mut order = Order::new(order_id, &customer).unwrap();

        let order_item_id = LineItemId::new();
        order.add_product(order_item_id, &product, 1).unwrap();

        assert_eq!(1, order.line_items.len());
        assert!(order.contains_product(product_id));
    }
}
