/*! Contains the `CreateOrderCommand` type. */

use crate::domain::{
    customers::*,
    error,
    infra::*,
    orders::*,
    Error,
};

type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateOrderCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
}

impl CommandArgs for CreateOrder {
    type Output = Result;
}

impl CreateOrder {
    async fn execute(
        &mut self,
        transaction: ActiveTransaction,
        store: impl OrderStore,
        customer_query: impl Query<GetCustomer>,
    ) -> Result {
        debug!("creating order `{}`", self.id);

        let order = {
            if store.get_order(self.id)?.is_some() {
                err!("order `{}` already exists", self.id)?
            } else {
                let customer = customer_query
                    .execute(GetCustomer {
                        id: self.customer_id,
                    })
                    .await?
                    .ok_or_else(|| error::bad_input("customer not found"))?;

                Order::new(self.id, &customer)?
            }
        };

        store.set_order(transaction.get(), order)?;

        info!("created order `{}`", self.id);

        Ok(())
    }
}

impl Resolver {
    /** Create an order. */
    pub fn create_order_command(&self) -> impl Command<CreateOrder> {
        self.command(|resolver, mut command: CreateOrder| async move {
            let store = resolver.order_store();
            let active_transaction = resolver.active_transaction();

            let customer_query = resolver.get_customer_query();

            command
                .execute(active_transaction, store, customer_query)
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::{
        customers::model::test_data::CustomerBuilder,
        orders::model::store::in_memory_store,
    };

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let customer_id = CustomerId::new();

        let customer_query = |_| async { Ok(Some(CustomerBuilder::new().id(customer_id).build())) };

        let mut create = CreateOrder {
            id: OrderId::new(),
            customer_id,
        };

        create
            .execute(ActiveTransaction::none(), &store, &customer_query)
            .await
            .unwrap();

        assert!(create
            .execute(ActiveTransaction::none(), &store, &customer_query)
            .await
            .is_err());
    }
}
