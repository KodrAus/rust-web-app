/*! Contains the `CreateOrderCommand` type. */

use auto_impl::auto_impl;

use domain::customers::CustomerId;
use domain::customers::queries::{GetCustomer, GetCustomerQuery};
use domain::orders::{Order, OrderId, OrderStore};
use domain::Resolver;

pub type Error = String;
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
pub fn create_order_command<TStore, TGetCustomer>(store: TStore, query: TGetCustomer) -> impl CreateOrderCommand
where
    TStore: OrderStore,
    TGetCustomer: GetCustomerQuery,
{
    move |command: CreateOrder| {
        let order = {
            if store.get_order(command.id)?.is_some() {
                Err("already exists")?
            } else {
                let customer = query.get_customer(GetCustomer {
                    id: command.customer_id,
                })?;
                Order::new(command.id, &customer)?
            }
        };

        store.set_order(order)?;

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
    use domain::customers::*;
    use domain::customers::queries::get_customer::Result as QueryResult;
    use domain::customers::model::test_data::CustomerBuilder;
    use domain::orders::model::store::in_memory_store;
    use domain::orders::*;
    use super::*;

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
