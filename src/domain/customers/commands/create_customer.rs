/*! Contains the `CreateCustomerCommand` type. */

use crate::domain::{
    customers::*,
    error,
    infra::*,
    Error,
};

/** Input for a `CreateCustomerCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

impl CommandArgs for CreateCustomer {
    type Output = Result<(), Error>;
}

async fn execute(
    command: CreateCustomer,
    transaction: ActiveTransaction,
    store: impl CustomerStore,
) -> Result<(), Error> {
    let customer = {
        if store.get_customer(command.id)?.is_some() {
            return Err(error::emit(emit::event!(
                "customer {id: command.id} already exists"
            )));
        } else {
            Customer::new(command.id)?
        }
    };

    store.set_customer(transaction.get(), customer)?;

    Ok(())
}

impl Resolver {
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl Command<CreateCustomer> {
        self.command(|resolver, command: CreateCustomer| async move {
            let store = resolver.customer_store();
            let active_transaction = resolver.active_transaction();

            execute(command, active_transaction, store).await
        })
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

        execute(create.clone(), ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(execute(create, ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
