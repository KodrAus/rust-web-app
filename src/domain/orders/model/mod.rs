pub mod store;

use domain::products::{Product, ProductData};

pub type OrderError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct OrderData {
    pub id: i32,
    _private: (),
}

pub struct Order {
    data: OrderData
}

#[derive(Clone, Serialize, Deserialize)]
pub struct LineItemData {
    pub id: i32,
    pub product_id: i32,
    pub price: f32,
    _private: (),
}

pub struct LineItem {
    data: LineItemData
}

// TODO: How do we get the line items for an order if we need them elsewhere?
pub struct OrderLineItemsAggregate {
    order: Order,
    order_items: Vec<LineItem>,
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

    pub fn new(id: i32) -> Self {
        Order::from_data(OrderData { 
            id: id, 
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
}

impl OrderLineItemsAggregate {
    fn from_data<TItems>(order: OrderData, items: TItems) -> Self 
        where TItems: IntoIterator<Item = LineItemData>
    {
        let order = Order::from_data(order);
        let items = items.into_iter().map(|item| LineItem::from_data(item)).collect();

        OrderLineItemsAggregate {
            order: order,
            order_items: items
        }
    }

    pub fn into_data(self) -> (OrderData, Vec<LineItemData>) {
        (self.order.into_data(), self.order_items.into_iter().map(|item| item.into_data()).collect())
    }

    pub fn contains_product(&self, product_id: i32) -> bool {
        self.order_items.iter().any(|item| item.data.product_id == product_id)
    }

    // TODO: Should we depend on a product entity directly? Seems like the point of having them, but it does couple things
    pub fn add_product(&mut self, product: &Product) -> Result<(), OrderError> {        
        let &ProductData { id, .. } = product.to_data();

        if !self.contains_product(id) {
            let order_item = LineItem::from_data(LineItemData {
                id: 1,
                product_id: id,
                price: 1f32,
                _private: ()
            });

            self.order_items.push(order_item);

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
        let product = Product::new(1, "A title").unwrap();

        let order_data = OrderData {
            id: 1,
            _private: (),
        };

        let mut order = OrderLineItemsAggregate::from_data(order_data, vec![]);

        order.add_product(&product).unwrap();

        assert_eq!(1, order.order_items.len());
        assert!(order.contains_product(1));
    }
}