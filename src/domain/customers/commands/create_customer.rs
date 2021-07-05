/*! Contains the `CreateCustomerCommand` type. */

use crate::domain::{
    customers::*,
    infra::*,
    Error,
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
    transaction: ActiveTransaction,
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

        store.set_customer(transaction.get(), customer)?;

        info!("customer `{}` created", command.id);

        Ok(())
    }
}

impl Resolver {
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl CreateCustomerCommand {
        let store = self.customer_store();
        let active_transaction = self.active_transaction();

        create_customer_command(active_transaction, store)
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::customers::model::store::in_memory_store;

    use super::*;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateCustomer {
            id: CustomerId::new(),
        };

        let mut cmd = create_customer_command(ActiveTransaction::none(), &store);

        cmd.create_customer(create.clone()).unwrap();

        assert!(cmd.create_customer(create).is_err());
    }
}
