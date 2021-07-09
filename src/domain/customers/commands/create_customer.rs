/*! Contains the `CreateCustomerCommand` type. */

use crate::domain::{
    customers::*,
    infra::*,
    Error,
};

type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateCustomerCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateCustomer {
    pub id: CustomerId,
}

impl CommandArgs for CreateCustomer {
    type Output = Result;
}

impl CreateCustomer {
    async fn execute(
        &mut self,
        transaction: ActiveTransaction,
        store: impl CustomerStore,
    ) -> Result {
        debug!("creating customer `{}`", self.id);

        let customer = {
            if store.get_customer(self.id)?.is_some() {
                err!("customer `{}` already exists", self.id)?
            } else {
                Customer::new(self.id)?
            }
        };

        store.set_customer(transaction.get(), customer)?;

        info!("customer `{}` created", self.id);

        Ok(())
    }
}

impl Resolver {
    /** Create a customer. */
    pub fn create_customer_command(&self) -> impl Command<CreateCustomer> {
        self.command(|resolver, mut command: CreateCustomer| async move {
            let store = resolver.customer_store();
            let active_transaction = resolver.active_transaction();

            command.execute(active_transaction, store).await
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

        let mut create = CreateCustomer {
            id: CustomerId::new(),
        };

        create
            .clone()
            .execute(ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(create
            .execute(ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
