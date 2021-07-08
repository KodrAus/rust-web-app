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
#[auto_impl(FnOnce)]
pub trait CreateCustomerCommand {
    fn create_customer(self, command: CreateCustomer) -> Future<Result>;
}

pub(in crate::domain) async fn create_customer(
    command: CreateCustomer,
    transaction: ActiveTransaction,
    store: impl CustomerStore,
) -> Result {
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

impl Resolver {
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl CreateCustomerCommand + Send + 'static {
        let store = self.customer_store();
        let active_transaction = self.active_transaction();

        move |command: CreateCustomer| create_customer(command, active_transaction, store).boxed()
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::customers::model::store::in_memory_store;

    use super::*;

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateCustomer {
            id: CustomerId::new(),
        };

        create_customer(create.clone(), ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(create_customer(create, ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
