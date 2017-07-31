pub(in domain) mod store;

use domain::products::{Product, ProductData};
use domain::customers::{Customer, CustomerData};

// TODO: OrderItemsAggregate and OrderItemAggregate are silly names

pub type OrderError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: i32,
    pub customer_id: i32,
    _private: (),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemData {
    pub product_id: i32,
    pub price: f32,
    pub quantity: u32,
    _private: (),
}

pub struct Order {
    data: OrderData
}

pub struct LineItem {
    data: LineItemData
}

// TODO: How do we get the line items for an order if we need them elsewhere?
// TODO: Should we store `LineItem`s or `LineItemData`. If we want to return `&mut LineItem` then that'll be problematic.
// But I don't think we should be able to return a `&mut LineItem` because of `mem::replace`.
pub struct OrderLineItemsAggregate {
    order: Order,
    line_items: Vec<LineItemData>,
}

pub struct OrderLineItemAggregate {
    order: Order,
    line_item: LineItem,
}

impl Order {
    fn from_data(data: OrderData) -> Self {
        Order {
            data: data
        }
    }

    pub fn into_data(self) -> OrderData {
        self.data
    }

    pub fn to_data(&self) -> &OrderData {
        &self.data
    }

    pub fn new(id: i32, customer: &Customer) -> Self {
        let &CustomerData { id: customer_id, .. } = customer.to_data();

        Order::from_data(OrderData { 
            id: id, 
            customer_id: customer_id,
            _private: () 
        })
    }
}

impl LineItem {
    fn from_data(data: LineItemData) -> Self {
        LineItem {
            data: data
        }
    }

    pub fn into_data(self) -> LineItemData {
        self.data
    }

    pub fn to_data(&self) -> &LineItemData {
        &self.data
    }
}

impl OrderLineItemAggregate {
    fn from_data(order: OrderData, line_item: LineItemData) -> Self {
        let order = Order::from_data(order);
        let line_item = LineItem::from_data(line_item);

        OrderLineItemAggregate {
            order: order,
            line_item: line_item
        }
    }

    pub fn into_data(self) -> (OrderData, LineItemData) {
        (self.order.into_data(), self.line_item.into_data())
    }

    pub fn to_data(&self) -> (&OrderData, &LineItemData) {
        (self.order.to_data(), &self.line_item.to_data())
    }

    pub fn set_quantity(&mut self, quantity: u32) -> Result<(), OrderError> {
        if quantity == 0 {
            Err("quantity must be greater than 0")?
        }

        self.line_item.data.quantity = quantity;

        Ok(())
    }
}

impl OrderLineItemsAggregate {
    fn from_data<TItems>(order: OrderData, line_items: TItems) -> Self 
        where TItems: IntoIterator<Item = LineItemData>
    {
        let order = Order::from_data(order);
        let line_items = line_items.into_iter().collect();

        OrderLineItemsAggregate {
            order: order,
            line_items: line_items
        }
    }

    pub fn into_data(self) -> (OrderData, Vec<LineItemData>) {
        (self.order.into_data(), self.line_items)
    }

    pub fn to_data(&self) -> (&OrderData, &[LineItemData]) {
        (self.order.to_data(), &self.line_items)
    }

    pub fn contains_product(&self, product_id: i32) -> bool {
        self.line_items.iter().any(|item| item.product_id == product_id)
    }

    pub fn add_product(&mut self, product: &Product, quantity: u32) -> Result<(), OrderError> {
        if quantity == 0 {
            Err("quantity must be greater than 0")?
        }

        let &ProductData { id, price, .. } = product.to_data();

        // TODO: Where does LineItemId come from? Just use product id?
        if !self.contains_product(id) {
            let order_item = LineItemData {
                product_id: id,
                price: price,
                quantity: quantity,
                _private: ()
            };

            self.line_items.push(order_item);

            Ok(())
        }
        else {
            Err("product is already in order")?
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::products::*;

    #[test]
    fn add_item_to_order() {
        let product = Product::new(1, "A title", 1f32).unwrap();

        let order_data = OrderData {
            id: 1,
            customer_id: 1,
            _private: (),
        };

        let mut order = OrderLineItemsAggregate::from_data(order_data, vec![]);

        order.add_product(&product, 1).unwrap();

        assert_eq!(1, order.line_items.len());
        assert!(order.contains_product(1));
    }
}
