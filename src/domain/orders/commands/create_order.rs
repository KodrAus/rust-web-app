/*! Contains the `CreateOrderCommand` type. */

use crate::domain::{
    customers::*,
    error,
    infra::*,
    orders::*,
    Error,
};

/** Input for a `CreateOrderCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateOrder {
    pub id: OrderId,
    pub customer_id: CustomerId,
}

impl CommandArgs for CreateOrder {
    type Output = Result<(), Error>;
}

async fn execute(
    command: CreateOrder,
    transaction: ActiveTransaction,
    store: impl OrderStore,
    customer_query: impl Query<GetCustomer>,
) -> Result<(), Error> {
    let order = {
        if store.get_order(command.id)?.is_some() {
            err!("order {order_id: command.id} already exists")?
        } else {
            let customer = customer_query
                .execute(GetCustomer {
                    id: command.customer_id,
                })
                .await?
                .ok_or_else(|| error::bad_input("customer not found"))?;

            Order::new(command.id, &customer)?
        }
    };

    store.set_order(transaction.get(), order)?;

    Ok(())
}

impl Resolver {
    /** Create an order. */
    pub fn create_order_command(&self) -> impl Command<CreateOrder> {
        self.command(|resolver, command: CreateOrder| async move {
            let store = resolver.order_store();
            let active_transaction = resolver.active_transaction();

            let customer_query = resolver.get_customer_query();

            execute(command, active_transaction, store, customer_query).await
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

        let create = CreateOrder {
            id: OrderId::new(),
            customer_id,
        };

        execute(
            create.clone(),
            ActiveTransaction::none(),
            &store,
            &customer_query,
        )
        .await
        .unwrap();

        assert!(execute(
            create.clone(),
            ActiveTransaction::none(),
            &store,
            &customer_query
        )
        .await
        .is_err());
    }
}
