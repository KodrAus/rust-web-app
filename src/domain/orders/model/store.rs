use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::orders::{Order, OrderData, LineItem, LineItemData, OrderLineItemAggregate, OrderLineItemsAggregate};

pub type Error = String;

#[auto_impl(Arc)]
pub trait OrderStore {
    fn get(&self, id: i32) -> Result<Option<Order>, Error>;
    fn set(&self, order: Order) -> Result<(), Error>;
}

#[auto_impl(Arc)]
pub trait OrderLineItemAggregateStore {
    fn get(&self, id: i32, line_item_id: i32) -> Result<Option<OrderLineItemAggregate>, Error>;
    fn set(&self, order: OrderLineItemAggregate) -> Result<(), Error>;
}

#[auto_impl(Arc)]
pub trait OrderLineItemsAggregateStore {
    fn get(&self, order_id: i32) -> Result<Option<OrderLineItemsAggregate>, Error>;
    fn set(&self, order: OrderLineItemsAggregate) -> Result<(), Error>;
}

pub struct InMemoryStore {
    orders: RwLock<BTreeMap<i32, (OrderData, Vec<i32>)>>,
    order_items: RwLock<BTreeMap<i32, LineItemData>>
}

impl OrderStore for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<Order>, Error> {
        let orders = self
            .orders
            .read()
            .map_err(|_| "not good!")?;

        if let Some(&(ref data, _)) = orders.get(&id) {
            Ok(Some(Order::from_data(data.clone())))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, order: Order) -> Result<(), Error> {
        let data = order.into_data();
        let id = data.id;

        let mut orders = self
            .orders
            .write()
            .map_err(|_| "not good!")?;

        match orders.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert((data, vec![]));
            },
            Entry::Occupied(mut entry) => {
                let mut entry = entry.get_mut();
                entry.0 = data;
            }
        }

        Ok(())
    }
}

impl OrderLineItemAggregateStore for InMemoryStore {
    fn get(&self, order_id: i32, line_item_id: i32) -> Result<Option<OrderLineItemAggregate>, Error> {
        let orders = self
            .orders
            .read()
            .map_err(|_| "not good!")?;

        if let Some(&(ref data, ref item_ids)) = orders.get(&order_id) {
            let order_items = self
                .order_items
                .read()
                .map_err(|_| "not good!")?;

            if !item_ids.iter().any(|id| *id == line_item_id) {
                Err("line item not found")?
            }
            
            let item_data = order_items
                .values()
                .find(|item_data| item_data.product_id == line_item_id)
                .cloned()
                .ok_or("line item not found")?;
            
            Ok(Some(OrderLineItemAggregate::from_data(data.clone(), item_data)))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, order: OrderLineItemAggregate) -> Result<(), Error> {
        let (order_data, order_item_data) = order.into_data();
        let order_id = order_data.id;
        let order_item_id = order_item_data.product_id;

        let mut orders = self
            .orders
            .write()
            .map_err(|_| "not good!")?;

        match orders.entry(order_id) {
            Entry::Vacant(entry) => {
                entry.insert((order_data, vec![order_item_id]));
            },
            Entry::Occupied(mut entry) => {
                let mut entry = entry.get_mut();
                entry.0 = order_data;

                let mut order_items = self
                    .order_items
                    .write()
                    .map_err(|_| "not good!")?;

                order_items.insert(order_item_id, order_item_data);
            }
        }

        Ok(())
    }
}

impl OrderLineItemsAggregateStore for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<OrderLineItemsAggregate>, Error> {
        let orders = self
            .orders
            .read()
            .map_err(|_| "not good!")?;

        if let Some(&(ref data, ref item_ids)) = orders.get(&id) {
            let order_items = self
                .order_items
                .read()
                .map_err(|_| "not good!")?;
            
            let items_data = order_items
                .values()
                .filter(|item_data| item_ids.iter().any(|id| *id == item_data.product_id))
                .cloned();
            
            Ok(Some(OrderLineItemsAggregate::from_data(data.clone(), items_data)))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, order: OrderLineItemsAggregate) -> Result<(), Error> {
        let (order_data, order_items_data) = order.into_data();
        let id = order_data.id;
        let order_item_ids = order_items_data.iter().map(|item| item.product_id).collect();

        let mut orders = self
            .orders
            .write()
            .map_err(|_| "not good!")?;

        match orders.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert((order_data, order_item_ids));
            },
            Entry::Occupied(mut entry) => {
                let mut entry = entry.get_mut();
                *entry = (order_data, order_item_ids);

                let mut order_items = self
                    .order_items
                    .write()
                    .map_err(|_| "not good!")?;

                for data in order_items_data {
                    let id = data.product_id;

                    order_items.insert(id, data);
                }
            }
        }

        Ok(())
    }
}

pub fn in_memory_store() -> InMemoryStore {
    InMemoryStore {
        orders: RwLock::new(BTreeMap::new()),
        order_items: RwLock::new(BTreeMap::new()),
    }
}

pub fn order_store() -> impl OrderStore {
    in_memory_store()
}

#[cfg(test)]
mod tests {
    use domain::orders::*;
    use super::*;

    #[test]
    fn test_in_memory_store() {
        use domain::products::Product;

        let store = in_memory_store();
        let order_store: &OrderStore = &store;
        let line_items_store: &OrderLineItemsAggregateStore = &store;
        let line_item_store: &OrderLineItemAggregateStore = &store;

        let order_id = 76i32;
        let product_id = 245i32;

        let order = Order::from_data(OrderData {
            id: order_id,
            customer_id: 1,
            _private: (),
        });

        order_store.set(order).unwrap();

        let product = Product::new(product_id, "Some product", 1f32).unwrap();

        let mut order = line_items_store.get(order_id).unwrap().unwrap();

        order.add_product(&product, 1).unwrap();

        line_items_store.set(order).unwrap();

        let mut order = line_item_store.get(order_id, product_id).unwrap().unwrap();

        order.set_quantity(5).unwrap();

        line_item_store.set(order).unwrap();

        let (_, line_items) = line_items_store.get(order_id).unwrap().unwrap().into_data();

        assert_eq!(1, line_items.len());
        assert_eq!(5, line_items[0].quantity);
    }
}