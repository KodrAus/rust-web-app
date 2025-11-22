/*! Persistent order storage. */

use std::{
    collections::HashSet,
    vec::IntoIter,
};

use crate::{
    domain::{
        Error,
        error,
        orders::*,
    },
    store::*,
};

/** A place to persist and fetch order entities. */
#[auto_impl(&, Arc)]
pub(in crate::domain) trait OrderStore {
    fn get_line_item(
        &self,
        id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error>;
    fn set_line_item(&self, transaction: &Transaction, order: OrderLineItem) -> Result<(), Error>;

    fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error>;
    fn set_order(&self, transaction: &Transaction, order: Order) -> Result<(), Error>;
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
    orders: TransactionValueStore<(OrderData, HashSet<LineItemId>)>,
    line_items: TransactionValueStore<LineItemData>,
}

impl OrderStore for InMemoryStore {
    fn get_line_item(
        &self,
        id: OrderId,
        line_item_id: LineItemId,
    ) -> Result<Option<OrderLineItem>, Error> {
        if let Some((version, (order_data, item_ids))) = self.orders.get(id) {
            assert_eq!(version, order_data.version.into());

            // Check that the line item is part of the order
            if !item_ids.contains(&line_item_id) {
                return Err(error::msg("line item not found"));
            }

            // Find the line item
            let (version, line_item_data) = self
                .line_items
                .get(line_item_id)
                .ok_or_else(|| error::msg("line item not found"))?;

            assert_eq!(version, line_item_data.version.into());

            Ok(Some(OrderLineItem::from_data(order_data, line_item_data)))
        } else {
            Ok(None)
        }
    }

    fn set_line_item(&self, transaction: &Transaction, order: OrderLineItem) -> Result<(), Error> {
        let (order_id, mut order_item_data) = order.into_data();
        let line_item_id = order_item_data.id;

        // Check that the line item is part of the order
        {
            let (_, (_, item_ids)) = self
                .orders
                .get(order_id)
                .ok_or_else(|| error::msg("order not found"))?;

            if !item_ids.contains(&line_item_id) {
                return Err(error::msg("line item not found"));
            }
        }

        self.line_items.set(
            transaction,
            line_item_id,
            Some(order_item_data.version),
            order_item_data.version.next(),
            order_item_data,
        )?;

        Ok(())
    }

    fn get_order(&self, id: OrderId) -> Result<Option<Order>, Error> {
        if let Some((version, (order_data, line_items))) = self.orders.get(id) {
            assert_eq!(version, order_data.version.into());

            let items_data = self
                .line_items
                .get_all(|line_item| line_items.contains(&line_item.id))
                .map(|(version, line_item_data)| {
                    assert_eq!(version, line_item_data.version.into());

                    line_item_data
                });

            Ok(Some(Order::from_data(order_data, items_data)))
        } else {
            Ok(None)
        }
    }

    fn set_order(&self, transaction: &Transaction, order: Order) -> Result<(), Error> {
        let (mut order_data, line_items_data) = order.into_data();
        let id = order_data.id;
        let order_item_ids = line_items_data.iter().map(|item| item.id).collect();

        // Update the order
        self.orders.set(
            transaction,
            id,
            Some(order_data.version),
            order_data.version.next(),
            (order_data, order_item_ids),
        )?;

        // Update each of its line items
        for mut line_item_data in line_items_data {
            let id = line_item_data.id;

            self.line_items.set(
                transaction,
                id,
                Some(line_item_data.version),
                line_item_data.version.next(),
                line_item_data,
            )?;
        }

        Ok(())
    }
}

impl OrderStoreFilter for InMemoryStore {
    #[allow(clippy::needless_collect)]
    fn filter<F>(&self, predicate: F) -> Result<Iter, Error>
    where
        F: Fn(&OrderData) -> bool,
    {
        let orders: Vec<_> = self
            .orders
            .get_all(|(data, _)| predicate(data))
            .map(|(_, (data, _))| data)
            .collect();

        Ok(orders.into_iter())
    }
}

pub(in crate::domain) fn in_memory_store(transaction_store: TransactionStore) -> InMemoryStore {
    InMemoryStore {
        orders: TransactionValueStore::new(transaction_store.clone()),
        line_items: TransactionValueStore::new(transaction_store),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        orders::model::test_data::OrderBuilder,
        products::model::test_data::default_product,
    };

    #[test]
    fn test_in_memory_store() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        store
            .set_order(
                &Transaction::none(),
                OrderBuilder::new().id(order_id).build(),
            )
            .unwrap();

        // Add a product to the order
        let mut order = store.get_order(order_id).unwrap().unwrap();
        order
            .add_product(line_item_id, &default_product(), 1)
            .unwrap();
        store.set_order(&Transaction::none(), order).unwrap();

        // Update the product in the order
        let mut line_item = store
            .get_line_item(order_id, line_item_id)
            .unwrap()
            .unwrap();
        line_item.set_quantity(5).unwrap();
        store
            .set_line_item(&Transaction::none(), line_item)
            .unwrap();

        // Get the product with the order
        let (_, line_items) = store.get_order(order_id).unwrap().unwrap().into_data();

        assert_eq!(1, line_items.len());
        assert_eq!(5, line_items[0].quantity);
    }

    #[test]
    fn add_order_twice_fails_concurrency_check() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();

        // Create an order in the store
        store
            .set_order(
                &Transaction::none(),
                OrderBuilder::new().id(order_id).build(),
            )
            .unwrap();

        // Attempting to create a second time fails optimistic concurrency check
        assert!(
            store
                .set_order(
                    &Transaction::none(),
                    OrderBuilder::new().id(order_id).build()
                )
                .is_err()
        );
    }

    #[test]
    fn set_order_item_twice_fails_concurrency_check() {
        let store = in_memory_store(Default::default());

        let order_id = OrderId::new();
        let line_item_id = LineItemId::new();

        // Create an order in the store
        let order = OrderBuilder::new()
            .id(order_id)
            .add_product(default_product(), move |line_item| {
                line_item.id(line_item_id)
            })
            .build();

        store.set_order(&Transaction::none(), order).unwrap();

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

        store
            .set_line_item(&Transaction::none(), line_item_a)
            .unwrap();

        assert!(
            store
                .set_line_item(&Transaction::none(), line_item_b)
                .is_err()
        );
    }
}
