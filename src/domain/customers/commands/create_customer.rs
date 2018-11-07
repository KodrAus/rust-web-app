/*! Contains the `CreateCustomerCommand` type. */

use auto_impl::auto_impl;

use crate::domain::{
    customers::{Customer, CustomerId, CustomerStore},
    error::{err_msg, Error},
    Resolver,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateCustomerCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

/** Create a customer. */
#[auto_impl(FnMut)]
pub trait CreateCustomerCommand {
    fn create_customer(&mut self, command: CreateCustomer) -> Result;
}

/** Default implementation for a `CreateCustomerCommand`. */
pub(in crate::domain) fn create_customer_command(
    store: impl CustomerStore,
) -> impl CreateCustomerCommand {
    move |command: CreateCustomer| {
        debug!("creating customer `{}`", command.id);

        let customer = {
            if store.get_customer(command.id)?.is_some() {
                err!("customer `{}` already exists", command.id)?
            } else {
                Customer::new(command.id)?
            }
        };

        store.set_customer(customer)?;

        info!("customer `{}` created", command.id);

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
    use crate::domain::customers::{model::store::in_memory_store, *};

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
