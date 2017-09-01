use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::orders::{LineItemData, LineItemId, Order, OrderData, OrderId, OrderLineItem};

pub type Error = String;

#[auto_impl(Arc)]
pub trait OrderLineItemStore {
    fn get(&self, id: OrderId, line_item_id: LineItemId) -> Result<Option<OrderLineItem>, Error>;
    fn set(&self, order: OrderLineItem) -> Result<(), Error>;
}

#[auto_impl(Arc)]
pub trait OrderStore {
    fn get(&self, order_id: OrderId) -> Result<Option<Order>, Error>;
    fn set(&self, order: Order) -> Result<(), Error>;
}

pub(in domain) struct InMemoryStore {
    orders: RwLock<HashMap<OrderId, (OrderData, HashSet<LineItemId>)>>,
    order_items: RwLock<HashMap<LineItemId, LineItemData>>,
}

impl OrderLineItemStore for InMemoryStore {
    fn get(
        &self,
        order_id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error> {
        let orders = self.orders.read().map_err(|_| "not good!")?;

        if let Some(&(ref data, ref item_ids)) = orders.get(&order_id) {
            let order_items = self.order_items.read().map_err(|_| "not good!")?;

            // Check that the line item is part of the order
            if !item_ids.contains(&line_item_id) {
                Err("line item not found")?
            }

            // Find the line item
            let item_data = order_items
                .values()
                .find(|item_data| item_data.id == line_item_id)
                .cloned()
                .ok_or("line item not found")?;

            Ok(Some(OrderLineItem::from_data(data.clone(), item_data)))
        } else {
            Ok(None)
        }
    }

    fn set(&self, order: OrderLineItem) -> Result<(), Error> {
        let (order_id, mut order_item_data) = order.into_data();
        let line_item_id = order_item_data.id;

        let orders = self.orders.read().map_err(|_| "not good!")?;

        // Check that the line item is part of the order
        let &(_, ref item_ids) = orders.get(&order_id).ok_or("order not found")?;
        if !item_ids.contains(&line_item_id) {
            Err("line item not found")?
        }

        let mut order_items = self.order_items.write().map_err(|_| "not good!")?;

        match order_items.entry(line_item_id) {
            Entry::Vacant(entry) => {
                order_item_data.version.next();
                entry.insert(order_item_data);
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.version != order_item_data.version {
                    Err("optimistic concurrency fail")?
                }

                order_item_data.version.next();
                *entry = order_item_data;
            }
        }

        Ok(())
    }
}

impl OrderStore for InMemoryStore {
    fn get(&self, id: OrderId) -> Result<Option<Order>, Error> {
        let orders = self.orders.read().map_err(|_| "not good!")?;

        if let Some(&(ref data, ref item_ids)) = orders.get(&id) {
            let order_items = self.order_items.read().map_err(|_| "not good!")?;

            let items_data = order_items
                .values()
                .filter(|item_data| item_ids.iter().any(|id| *id == item_data.id))
                .cloned();

            Ok(Some(Order::from_data(data.clone(), items_data)))
        } else {
            Ok(None)
        }
    }

    fn set(&self, order: Order) -> Result<(), Error> {
        let (mut order_data, order_items_data) = order.into_data();
        let id = order_data.id;
        let order_item_ids = order_items_data.iter().map(|item| item.id).collect();

        // Update the order
        let mut orders = self.orders.write().map_err(|_| "not good!")?;
        match orders.entry(id) {
            Entry::Vacant(entry) => {
                order_data.version.next();
                entry.insert((order_data, order_item_ids));
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.0.version != order_data.version {
                    Err("optimistic concurrency fail")?
                }

                order_data.version.next();
                *entry = (order_data, order_item_ids);
            }
        }

        // Insert the line items
        let mut order_items = self.order_items.write().map_err(|_| "not good!")?;
        for mut data in order_items_data {
            let id = data.id;

            data.version.next();
            order_items.insert(id, data);
        }

        Ok(())
    }
}

pub(in domain) fn in_memory_store() -> InMemoryStore {
    InMemoryStore {
        orders: RwLock::new(HashMap::new()),
        order_items: RwLock::new(HashMap::new()),
    }
}

pub fn order_store() -> impl OrderStore {
    in_memory_store()
}

#[cfg(test)]
mod tests {
    use domain::orders::*;
    use domain::customers::*;
    use domain::products::*;
    use super::*;

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();
        let order_store: &OrderStore = &store;
        let line_item_store: &OrderLineItemStore = &store;

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        {
            let order = Order::new(order_id, &Customer::new(1)).unwrap();
            order_store.set(order).unwrap();
        }
        // Add a product to the order
        {
            let mut order = order_store.get(order_id).unwrap().unwrap();
            order
                .add_product(
                    line_item_id,
                    &Product::new(ProductId::new(), "Some product", 1f32).unwrap(),
                    1,
                )
                .unwrap();
            order_store.set(order).unwrap();
        }
        // Update the product in the order
        {
            let mut order = line_item_store
                .get(order_id, line_item_id)
                .unwrap()
                .unwrap();
            order.set_quantity(5).unwrap();
            line_item_store.set(order).unwrap();
        }
        // Get the product with the order
        {
            let (_, line_items) = order_store.get(order_id).unwrap().unwrap().into_data();

            assert_eq!(1, line_items.len());
            assert_eq!(5, line_items[0].quantity);
        }
    }

    #[test]
    fn add_order_twice_fails_concurrency_check() {
        let store = in_memory_store();
        let order_store: &OrderStore = &store;

        let order_id = OrderId::new();
        let customer = Customer::new(1);

        // Create an order in the store
        order_store.set(Order::new(order_id, &customer).unwrap()).unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(order_store.set(Order::new(order_id, &customer).unwrap()).is_err());
    }

    #[test]
    fn set_order_item_twice_fails_concurrency_check() {
        let store = in_memory_store();
        let order_store: &OrderStore = &store;
        let line_item_store: &OrderLineItemStore = &store;
        
        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        {
            let customer = Customer::new(1);
            let product = Product::new(ProductId::new(), "A title", 3f32).unwrap();

            let mut order = Order::new(order_id, &customer).unwrap();
            order.add_product(line_item_id, &product, 1).unwrap();

            order_store.set(order).unwrap();
        }
        // Attempting to update a line item twice fails optimistic concurrency check
        {
            let get_item = || line_item_store.get(order_id, line_item_id).unwrap().unwrap();
            let mut line_item_a = get_item();
            let mut line_item_b = get_item();

            line_item_a.set_quantity(3).unwrap();
            line_item_b.set_quantity(2).unwrap();

            line_item_store.set(line_item_a).unwrap();

            assert!(line_item_store.set(line_item_b).is_err());
        }
    }
}
