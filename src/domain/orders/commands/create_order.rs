use auto_impl::auto_impl;

use domain::customers::Customer;
use domain::orders::{Order, OrderId, OrderStore, Resolver};

pub type CreateOrderError = String;

#[derive(Clone, Deserialize)]
pub struct CreateOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
}

#[auto_impl(FnMut)]
pub trait CreateOrderCommand {
    fn create_order<'a>(&mut self, command: CreateOrder) -> Result<(), CreateOrderError>;
}

pub fn create_order_command<TStore, TCustomerStore>(store: TStore, customer_store: TCustomerStore) -> impl CreateOrderCommand
where
    TStore: OrderStore,
    TCustomerStore: CustomerStore,
{
    move |command: CreateOrder| {
        let order = {
            if store.get(command.id)?.is_some() {
                Err("already exists")?
            } else {
                let customer = customer_store.get(command.customer_id)?;
                Order::new(command.id, &customer)?
            }
        };

        store.set(order)?;

        Ok(())
    }
}

impl Resolver {
    pub fn create_order_command(&self) -> impl CreateOrderCommand {
        let store = self.order_store();

        create_order_command(store)
    }
}

#[cfg(test)]
mod tests {
    use domain::customers::model::test_data::default_customer;
    use domain::orders::model::store::in_memory_store;
    use domain::orders::*;
    use super::*;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let customer = default_customer();

        let create = CreateOrder {
            id: OrderId::new(),
        };

        let mut cmd = create_order_command(&store);

        cmd.create_order(create.clone(), &customer).unwrap();

        assert!(cmd.create_order(create, &customer).is_err());
    }
}
