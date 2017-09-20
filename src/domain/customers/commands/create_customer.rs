use auto_impl::auto_impl;

use domain::Resolver;
use domain::customers::{Customer, CustomerId, CustomerStore};

pub type CreateCustomerError = String;

#[derive(Clone, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

#[auto_impl(FnMut)]
pub trait CreateCustomerCommand {
    fn create_customer(&mut self, command: CreateCustomer) -> Result<(), CreateCustomerError>;
}

pub fn create_customer_command<TStore>(store: TStore) -> impl CreateCustomerCommand
where
    TStore: CustomerStore,
{
    move |command: CreateCustomer| {
        let customer = {
            if store.get_customer(command.id)?.is_some() {
                Err("already exists")?
            } else {
                Customer::new(command.id)?
            }
        };

        store.set_customer(customer)?;

        Ok(())
    }
}

impl Resolver {
    pub fn create_customer_command(&self) -> impl CreateCustomerCommand {
        let store = self.customers().customer_store();

        create_customer_command(store)
    }
}

#[cfg(test)]
mod tests {
    use domain::customers::model::store::in_memory_store;
    use domain::customers::*;
    use super::*;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let create = CreateCustomer {
            id: CustomerId::new(),
        };

        let mut cmd = create_customer_command(&store);

        cmd.create_customer(create.clone()).unwrap();

        assert!(cmd.create_customer(create).is_err());
    }
}
