/*! Contains the `CreateProductCommand` type. */

use crate::domain::{
    infra::*,
    products::*,
    Error,
};

/** Input for a `CreateProductCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct CreateProduct {
    pub id: ProductId,
    pub title: String,
    pub price: Currency,
}

impl CommandArgs for CreateProduct {
    type Output = Result<(), Error>;
}

/** Default implementation for a `CreateProductCommand`. */
async fn execute(
    command: CreateProduct,
    transaction: ActiveTransaction,
    store: impl ProductStore,
) -> Result<(), Error> {
    let product = {
        if store.get_product(command.id)?.is_some() {
            err!("product {id: command.id} already exists")?
        } else {
            Product::new(command.id, command.title, command.price)?
        }
    };

    store.set_product(transaction.get(), product)?;

    Ok(())
}

impl Resolver {
    /** Create a product. */
    pub fn create_product_command(&self) -> impl Command<CreateProduct> {
        self.command(|resolver, command: CreateProduct| async move {
            let store = resolver.product_store();
            let active_transaction = resolver.active_transaction();

            execute(command, active_transaction, store).await
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::products::model::store::in_memory_store;

    #[tokio::test]
    async fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateProduct {
            id: ProductId::new(),
            title: "Test Product".into(),
            price: Currency::usd(100),
        };

        execute(create.clone(), ActiveTransaction::none(), &store)
            .await
            .unwrap();

        assert!(execute(create, ActiveTransaction::none(), &store)
            .await
            .is_err());
    }
}
