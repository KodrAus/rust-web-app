/*! Persistent order storage. */

use auto_impl::auto_impl;

use std::{
    collections::{
        hash_map::Entry,
        HashMap,
        HashSet,
    },
    sync::RwLock,
    vec::IntoIter,
};

use crate::domain::{
    error::{
        self,
        Error,
    },
    orders::{
        LineItemData,
        LineItemId,
        Order,
        OrderData,
        OrderId,
        OrderLineItem,
    },
};

/** A place to persist and fetch order entities. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait OrderStore {
    fn get_line_item(
        &self,
        id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error>;
    fn set_line_item(&self, order: OrderLineItem) -> Result<(), Error>;

    fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error>;
    fn set_order(&self, order: Order) -> Result<(), Error>;
}

/**
An additional store for fetching multiple order records at a time.

This trait is an implementation detail that lets us fetch more than one order.
It will probably need to be refactored or just removed when we add a proper database.
The fact that it's internal to `domain::orders` though means the scope of breakage is a bit smaller.
Commands and queries that depend on `OrderStoreFilter` won't need to break their public API.
*/
#[auto_impl(&, Arc)]
pub(in crate::domain) trait OrderStoreFilter {
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&OrderData) -> bool;
}

pub(in crate::domain) type Iter = IntoIter<OrderData>;

/** A test in-memory order store. */
pub(in crate::domain) struct InMemoryStore {
    data: RwLock<InMemoryStoreInner>,
}

struct InMemoryStoreInner {
    orders: HashMap<OrderId, (OrderData, HashSet<LineItemId>)>,
    line_items: HashMap<LineItemId, LineItemData>,
}

impl OrderStore for InMemoryStore {
    fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error> {
        let store_data = self.data.read().map_err(|_| error::msg("not good!"))?;

        if let Some(&(ref order_data, ref item_ids)) = store_data.orders.get(&id) {
            let items_data = store_data
                .line_items
                .values()
                .filter(|item_data| item_ids.iter().any(|id| *id == item_data.id))
                .cloned();

            Ok(Some(Order::from_data(order_data.clone(), items_data)))
        } else {
            Ok(None)
        }
    }

    fn set_order(&self, order: Order) -> Result<(), Error> {
        let mut store_data = self.data.write().map_err(|_| error::msg("not good!"))?;

        let (mut order_data, line_items_data) = order.into_data();
        let id = order_data.id;
        let order_item_ids = line_items_data.iter().map(|item| item.id).collect();

        // Update the order
        match store_data.orders.entry(id) {
            Entry::Vacant(entry) => {
                order_data.version.next();
                entry.insert((order_data, order_item_ids));
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.0.version != order_data.version {
                    Err(error::msg("optimistic concurrency fail"))?
                }

                order_data.version.next();
                *entry = (order_data, order_item_ids);
            }
        }

        // Insert the line items
        for mut data in line_items_data {
            let id = data.id;

            data.version.next();
            store_data.line_items.insert(id, data);
        }

        Ok(())
    }

    fn get_line_item(
        &self,
        id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error> {
        let store_data = &self.data.read().map_err(|_| error::msg("not good!"))?;

        if let Some(&(ref data, ref item_ids)) = store_data.orders.get(&id) {
            // Check that the line item is part of the order
            if !item_ids.contains(&line_item_id) {
                Err(error::msg("line item not found"))?
            }

            // Find the line item
            let item_data = store_data
                .line_items
                .values()
                .find(|item_data| item_data.id == line_item_id)
                .cloned()
                .ok_or(error::msg("line item not found"))?;

            Ok(Some(OrderLineItem::from_data(data.clone(), item_data)))
        } else {
            Ok(None)
        }
    }

    fn set_line_item(&self, order: OrderLineItem) -> Result<(), Error> {
        let mut store_data = self.data.write().map_err(|_| error::msg("not good!"))?;

        let (order_id, mut order_item_data) = order.into_data();
        let line_item_id = order_item_data.id;

        // Check that the line item is part of the order
        {
            let &(_, ref item_ids) = store_data
                .orders
                .get(&order_id)
                .ok_or(error::msg("order not found"))?;
            if !item_ids.contains(&line_item_id) {
                Err(error::msg("line item not found"))?
            }
        }

        match store_data.line_items.entry(line_item_id) {
            Entry::Vacant(entry) => {
                order_item_data.version.next();
                entry.insert(order_item_data);
            }
            Entry::Occupied(mut entry) => {
                let entry = entry.get_mut();
                if entry.version != order_item_data.version {
                    Err(error::msg("optimistic concurrency fail"))?
                }

                order_item_data.version.next();
                *entry = order_item_data;
            }
        }

        Ok(())
    }
}

impl OrderStoreFilter for InMemoryStore {
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&OrderData) -> bool,
    {
        let store_data = &self.data.read().map_err(|_| error::msg("not good!"))?;

        let orders: Vec<_> = store_data
            .orders
            .values()
            .filter(|o| predicate(&o.0))
            .map(|o| o.0.clone())
            .collect();

        Ok(orders.into_iter())
    }
}

pub(in crate::domain) fn in_memory_store() -> InMemoryStore {
    InMemoryStore {
        data: RwLock::new(InMemoryStoreInner {
            orders: HashMap::new(),
            line_items: HashMap::new(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        orders::{
            model::test_data::OrderBuilder,
            *,
        },
        products::model::test_data::default_product,
    };

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        store
            .set_order(OrderBuilder::new().id(order_id).build())
            .unwrap();

        // Add a product to the order
        let mut order = store.get_order(order_id).unwrap().unwrap();
        order
            .add_product(line_item_id, &default_product(), 1)
            .unwrap();
        store.set_order(order).unwrap();

        // Update the product in the order
        let mut line_item = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap();
        line_item.set_quantity(5).unwrap();
        store.set_line_item(line_item).unwrap();

        // Get the product with the order
        let (_, line_items) = store.get_order(order_id).unwrap().unwrap().into_data();

        assert_eq!(1, line_items.len());
        assert_eq!(5, line_items[0].quantity);
    }

    #[test]
    fn add_order_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let order_id = OrderId::new();

        // Create an order in the store
        store
            .set_order(OrderBuilder::new().id(order_id).build())
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(store
            .set_order(OrderBuilder::new().id(order_id).build())
            .is_err());
    }

    #[test]
    fn set_order_item_twice_fails_concurrency_check() {
        let store = in_memory_store();

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        let order = OrderBuilder::new()
            .id(order_id)
            .add_product(default_product(), move |line_item| {
                line_item.id(line_item_id)
            })
            .build();

        store.set_order(order).unwrap();

        // Attempting to update a line item twice fails optimistic concurrency check
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
