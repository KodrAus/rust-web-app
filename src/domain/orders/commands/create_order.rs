/*! Contains the `CreateOrderCommand` type. */

use auto_impl::auto_impl;

use crate::domain::{
    customers::{
        queries::{GetCustomer, GetCustomerQuery},
        CustomerId,
    },
    error::{err_msg, Error},
    orders::{Order, OrderId, OrderStore},
    Resolver,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateOrderCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
}

/** Create a new order. */
#[auto_impl(FnMut)]
pub trait CreateOrderCommand {
    fn create_order<'a>(&mut self, command: CreateOrder) -> Result;
}

/** Default implementation for a `CreateOrderCommand`. */
pub(in crate::domain) fn create_order_command(
    store: impl OrderStore,
    query: impl GetCustomerQuery,
) -> impl CreateOrderCommand {
    move |command: CreateOrder| {
        debug!("creating order `{}`", command.id);

        let order = {
            if store.get_order(command.id)?.is_some() {
                err!("order `{}` already exists", command.id)?
            } else {
                let customer = query.get_customer(GetCustomer {
                    id: command.customer_id,
                })?;

                Order::new(command.id, &customer)?
            }
        };

        store.set_order(order)?;

        info!("created order `{}`", command.id);

        Ok(())
    }
}

impl Resolver {
    pub fn create_order_command(&self) -> impl CreateOrderCommand {
        let store = self.orders().order_store();
        let query = self.get_customer_query();

        create_order_command(store, query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        customers::{
            model::test_data::CustomerBuilder, queries::get_customer::Result as QueryResult,
        },
        orders::{model::store::in_memory_store, *},
    };

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let customer_id = CustomerId::new();

        let create = CreateOrder {
            id: OrderId::new(),
            customer_id: customer_id,
        };

        let mut cmd = create_order_command(&store, move |_| {
            let customer: QueryResult = Ok(CustomerBuilder::new().id(customer_id).build());
            customer
        });

        cmd.create_order(create.clone()).unwrap();

        assert!(cmd.create_order(create).is_err());
    }
}
