use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::sync::RwLock;
use auto_impl::auto_impl;

use domain::orders::{Order, OrderData, OrderItem, OrderItemData, OrderWithItems};

pub type Error = String;

#[auto_impl(Arc)]
pub trait OrderStore {
    fn get(&self, id: i32) -> Result<Option<Order>, Error>;
    fn set(&self, order: Order) -> Result<(), Error>;
}

#[auto_impl(Arc)]
pub trait OrderWithItemsStore {
    fn get(&self, id: i32) -> Result<Option<OrderWithItems>, Error>;
    fn set(&self, order: OrderWithItems) -> Result<(), Error>;
}

pub(in domain::orders) struct InMemoryStore {
    orders: RwLock<BTreeMap<i32, (OrderData, Vec<i32>)>>,
    order_items: RwLock<BTreeMap<i32, OrderItemData>>
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

impl OrderWithItemsStore for InMemoryStore {
    fn get(&self, id: i32) -> Result<Option<OrderWithItems>, Error> {
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
                .filter(|item_data| item_ids.iter().any(|id| *id == item_data.id))
                .cloned();
            
            Ok(Some(OrderWithItems::from_data(data.clone(), items_data)))
        }
        else {
            Ok(None)
        }
    }

    fn set(&self, order: OrderWithItems) -> Result<(), Error> {
        let (order_data, order_items_data) = order.into_data();
        let id = order_data.id;
        let order_item_ids = order_items_data.iter().map(|item| item.id).collect();

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
                    let id = data.id;

                    order_items.insert(id, data);
                }
            }
        }

        Ok(())
    }
}

pub(in domain::orders) fn in_memory_store() -> InMemoryStore {
    InMemoryStore {
        orders: RwLock::new(BTreeMap::new()),
        order_items: RwLock::new(BTreeMap::new()),
    }
}

pub fn order_store() -> impl OrderStore {
    in_memory_store()
}
