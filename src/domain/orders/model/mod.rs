/*!
Contains the `Order` and `OrderLineItem` entities.

The separation between `Order` and `OrderLineItem` is kind of arbitrary, and may end up being a bit of a nuisance.
If this becomes the case then rather than coupling the two together even more, we should make sure they're separated.

The main idea right now is that `OrderLineItem` is a _subset_ of `Order` for a single product.
This kind of suggests it shouldn't have an id of its own, and instead should be a composite of `(OrderId, ProductId)`.
We'll probably need to come back here one day to work this out properly.
*/

use std::convert::{TryFrom, TryInto};

pub mod store;

#[cfg(test)]
pub mod test_data;

use domain::Resolver;
use domain::error::{err_msg, Error};
use domain::entity::Entity;
use domain::id::{Id, IdProvider, NextId};
use domain::version::Version;
use domain::products::{Product, ProductData, ProductId};
use domain::customers::{Customer, CustomerData, CustomerId};

pub type OrderId = Id<OrderData>;
pub type NextOrderId = NextId<OrderData>;
pub type OrderVersion = Version<OrderData>;
pub type LineItemId = Id<LineItemData>;
pub type NextLineItemId = NextId<LineItemData>;
pub type LineItemVersion = Version<LineItemData>;

/**
An order item quantity.

Quantities must be greater than zero.
*/
pub struct Quantity(u32);

impl TryFrom<u32> for Quantity {
    type Error = Error;

    fn try_from(quantity: u32) -> Result<Self, Self::Error> {
        if quantity < 1 {
            Err(err_msg("quantity must be greater than 0"))?
        }

        Ok(Quantity(quantity))
    }
}

/** Data for an order. */
#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: OrderId,
    pub version: OrderVersion,
    pub customer_id: CustomerId,
    _private: (),
}

/** Data for a single order line item. */
#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemData {
    pub id: LineItemId,
    pub version: LineItemVersion,
    pub product_id: ProductId,
    pub price: f32,
    pub quantity: u32,
    _private: (),
}

/**
An order and its line items.

Products can be added to an order as a line item.
*/
pub struct Order {
    order: OrderData,
    line_items: Vec<LineItemData>,
}

/**
An order and one of its line items.

Properties on the line item can be updated.
*/
pub struct OrderLineItem {
    order: OrderData,
    line_item: LineItemData,
}

/**
An attempt to turn an order into a line item.

If the line item was in the order then the result is `InOrder`.
If the line item was not in the order then the result is `NotInOrder`.
*/
pub enum IntoLineItem {
    InOrder(OrderLineItem),
    NotInOrder(Order),
}

impl OrderLineItem {
    pub(self) fn from_data(order: OrderData, line_item: LineItemData) -> Self {
        OrderLineItem {
            order: order,
            line_item: line_item,
        }
    }

    pub fn into_data(self) -> (OrderId, LineItemData) {
        (self.order.id, self.line_item)
    }

    pub fn to_data(&self) -> (OrderId, &LineItemData) {
        (self.order.id, &self.line_item)
    }

    pub fn set_quantity<TQuantity>(&mut self, quantity: TQuantity) -> Result<(), Error>
    where
        TQuantity: TryInto<Quantity, Error = Error>,
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

    pub fn into_line_item_for_product(self, product_id: ProductId) -> IntoLineItem {
        if !self.contains_product(product_id) {
            IntoLineItem::NotInOrder(self)
        } else {
            let Order {
                order, line_items, ..
            } = self;

            let item = line_items
                .into_iter()
                .find(|item| item.product_id == product_id)
                .unwrap();

            IntoLineItem::InOrder(OrderLineItem::from_data(order, item))
        }
    }

    pub fn new<TId>(id_provider: TId, customer: &Customer) -> Result<Self, Error>
    where
        TId: IdProvider<OrderData>,
    {
        let id = id_provider.id()?;
        let &CustomerData {
            id: customer_id, ..
        } = customer.to_data();

        let order_data = OrderData {
            id: id,
            version: OrderVersion::default(),
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

    pub fn add_product<TId, TQuantity>(&mut self, id_provider: TId, product: &Product, quantity: TQuantity) -> Result<(), Error>
    where
        TId: IdProvider<LineItemData>,
        TQuantity: TryInto<Quantity, Error = Error>,
    {
        let &ProductData {
            id: product_id,
            price,
            ..
        } = product.to_data();

        if self.contains_product(product_id) {
            Err(err_msg("product is already in order"))?
        }

        let id = id_provider.id()?;
        let line_item = LineItemData {
            id: id,
            version: LineItemVersion::default(),
            product_id: product_id,
            price: price,
            quantity: quantity.try_into()?.0,
            _private: (),
        };

        self.line_items.push(line_item);

        Ok(())
    }
}

impl Entity for Order {
    type Id = OrderId;
    type Version = OrderVersion;
    type Data = OrderData;
    type Error = Error;
}

impl Entity for OrderLineItem {
    type Id = LineItemId;
    type Version = LineItemVersion;
    type Data = LineItemData;
    type Error = Error;
}

impl Resolver {
    pub fn order_id_provider(&self) -> impl IdProvider<OrderData> {
        NextId::<OrderData>::new()
    }

    pub fn line_item_id_provider(&self) -> impl IdProvider<LineItemData> {
        NextId::<LineItemData>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::orders::model::test_data::default_order;
    use domain::products::model::test_data::{default_product, ProductBuilder};
    use domain::customers::model::test_data::default_customer;

    #[test]
    fn add_item_to_order() {
        let order_id = OrderId::new();
        let product_id = ProductId::new();
        let order_item_id = LineItemId::new();
        let product = ProductBuilder::new().id(product_id).build();

        let customer = default_customer();

        let mut order = Order::new(order_id, &customer).unwrap();

        order.add_product(order_item_id, &product, 1).unwrap();

        assert_eq!(1, order.line_items.len());
        assert!(order.contains_product(product_id));
    }

    #[test]
    fn quantity_must_be_greater_than_0() {
        let mut order = default_order();
        let product = default_product();

        assert!(order.add_product(LineItemId::new(), &product, 0).is_err());

        order.add_product(LineItemId::new(), &product, 1).unwrap();

        let (order_data, mut line_item_data) = order.into_data();
        let mut order = OrderLineItem::from_data(order_data, line_item_data.pop().unwrap());

        assert!(order.set_quantity(0).is_err());
    }

    #[test]
    fn product_must_not_be_in_order_when_adding() {
        let mut order = default_order();
        let product = default_product();

        order.add_product(LineItemId::new(), &product, 1).unwrap();

        assert!(order.add_product(LineItemId::new(), &product, 1).is_err());
    }
}
