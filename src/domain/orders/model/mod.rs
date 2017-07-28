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
pub struct OrderItemData {
    pub id: i32,
    pub product_id: i32,
    pub price: f32,
    _private: (),
}

pub struct OrderItem {
    data: OrderItemData
}

pub struct OrderWithItems {
    order: Order,
    order_items: Vec<OrderItem>,
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

impl OrderItem {
    fn from_data(data: OrderItemData) -> Self {
        OrderItem {
            data: data
        }
    }

    pub fn into_data(self) -> OrderItemData {
        self.data
    }
}

impl OrderWithItems {
    fn from_data<TItems>(order: OrderData, items: TItems) -> Self 
        where TItems: IntoIterator<Item = OrderItemData>
    {
        let order = Order::from_data(order);
        let items = items.into_iter().map(|item| OrderItem::from_data(item)).collect();

        OrderWithItems {
            order: order,
            order_items: items
        }
    }

    pub fn into_data(self) -> (OrderData, Vec<OrderItemData>) {
        (self.order.into_data(), self.order_items.into_iter().map(|item| item.into_data()).collect())
    }

    pub fn contains_product(&self, product_id: i32) -> bool {
        self.order_items.iter().any(|item| item.data.product_id == product_id)
    }

    pub fn add_product(&mut self, product: Product) -> Result<(), OrderError> {        
        let ProductData { id, .. } = product.into_data();

        if !self.contains_product(id) {
            let order_item = OrderItem::from_data(OrderItemData {
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

        let mut order = OrderWithItems::from_data(order_data, vec![]);

        order.add_product(product).unwrap();

        assert_eq!(1, order.order_items.len());
        assert!(order.contains_product(1));
    }
}