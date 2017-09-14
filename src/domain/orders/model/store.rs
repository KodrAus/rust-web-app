/*!
This module contains an implementation of some persistent storage for orders and order line items.
The separation between the two entities is kind of arbitrary, and may end up being a bit of a nuisance.
If this becomes the case then rather than coupling the two together even more, we should make sure they're separated.
*/

use std::collections::{HashMap, HashSet};
use std::collections::hash_map::Entry;
use std::sync::RwLock;

use domain::orders::{LineItemData, LineItemId, Order, OrderData, OrderId, OrderLineItem};

pub type Error = String;

// `syn` doesn't recognise `pub(restricted)`, so we re-export the store
mod re_export {
    use auto_impl::auto_impl;

    use domain::orders::{LineItemId, Order, OrderId, OrderLineItem};
    use super::Error;

    #[auto_impl(Arc)]
    pub trait OrderStore {
        fn get_line_item(
            &self,
            id: OrderId,
            line_item_id: LineItemId,
        ) -> Result<Option<OrderLineItem>, Error>;
        fn set_line_item(&self, order: OrderLineItem) -> Result<(), Error>;

        fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error>;
        fn set_order(&self, order: Order) -> Result<(), Error>;
    }

    impl<'a, T> OrderStore for &'a T where T: OrderStore {
        fn get_line_item(
            &self,
            id: OrderId,
            line_item_id: LineItemId,
        ) -> Result<Option<OrderLineItem>, Error> {
            (*self).get_line_item(id, line_item_id)
        }

        fn set_line_item(&self, order: OrderLineItem) -> Result<(), Error> {
            (*self).set_line_item(order)
        }

        fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error> {
            (*self).get_order(id)
        }

        fn set_order(&self, order: Order) -> Result<(), Error> {
            (*self).set_order(order)
        }
    }
}

pub(in domain::orders) use self::re_export::OrderStore;

pub(in domain) struct InMemoryStore {
    orders: RwLock<HashMap<OrderId, (OrderData, HashSet<LineItemId>)>>,
    order_items: RwLock<HashMap<LineItemId, LineItemData>>,
}

impl OrderStore for InMemoryStore {
    fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error> {
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

    fn set_order(&self, order: Order) -> Result<(), Error> {
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

    fn get_line_item(
        &self,
        id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error> {
        let orders = self.orders.read().map_err(|_| "not good!")?;

        if let Some(&(ref data, ref item_ids)) = orders.get(&id) {
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

    fn set_line_item(&self, order: OrderLineItem) -> Result<(), Error> {
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

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        {
            let order = Order::new(order_id, &Customer::new(1)).unwrap();
            store.set_order(order).unwrap();
        }
        // Add a product to the order
        {
            let mut order = store.get_order(order_id).unwrap().unwrap();
            order
                .add_product(
                    line_item_id,
                    &Product::new(ProductId::new(), "Some product", 1f32).unwrap(),
                    1,
                )
                .unwrap();
            store.set_order(order).unwrap();
        }
        // Update the product in the order
        {
            let mut order = store
                .get_line_item(order_id, line_item_id)
                .unwrap()
                .unwrap();
            order.set_quantity(5).unwrap();
            store.set_line_item(order).unwrap();
        }
        // Get the product with the order
        {
            let (_, line_items) = store.get_order(order_id).unwrap().unwrap().into_data();

            assert_eq!(1, line_items.len());
            assert_eq!(5, line_items[0].quantity);
        }
    }

    #[test]
    fn add_order_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let customer = Customer::new(1);

        // Create an order in the store
        store
            .set_order(Order::new(order_id, &customer).unwrap())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(
            store
                .set_order(Order::new(order_id, &customer).unwrap())
                .is_err()
        );
    }

    #[test]
    fn set_order_item_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        {
            let customer = Customer::new(1);
            let product = Product::new(ProductId::new(), "A title", 3f32).unwrap();

            let mut order = Order::new(order_id, &customer).unwrap();
            order.add_product(line_item_id, &product, 1).unwrap();

            store.set_order(order).unwrap();
        }
        // Attempting to update a line item twice fails optimistic concurrency check
        {
            let get_item = || {
                store
                    .get_line_item(order_id, line_item_id)
                    .unwrap()
                    .unwrap()
            };
            let mut line_item_a = get_item();
            let mut line_item_b = get_item();

            line_item_a.set_quantity(3).unwrap();
            line_item_b.set_quantity(2).unwrap();

            store.set_line_item(line_item_a).unwrap();

            assert!(store.set_line_item(line_item_b).is_err());
        }
    }
}
