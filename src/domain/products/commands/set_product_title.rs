/*! Contains the `SetProductTitleCommand`. */

use crate::domain::{
    Error,
    error,
    infra::*,
    products::*,
};

/** Input for a `SetProductTitleCommand`. */
#[derive(Clone, Serialize, Deserialize)]
pub struct SetProductTitle {
    pub id: ProductId,
    pub title: String,
}

impl CommandArgs for SetProductTitle {
    type Output = Result<(), Error>;
}

/** Default implementation for a `SetProductTitleCommand`. */
async fn execute(
    command: SetProductTitle,
    transaction: ActiveTransaction,
    store: impl ProductStore,
) -> Result<(), Error> {
    let product = {
        if let Some(mut product) = store.get_product(command.id)? {
            product.set_title(command.title)?;

            product
        } else {
            return Err(error::msg("not found"));
        }
    };

    store.set_product(transaction.get(), product)?;

    Ok(())
}

impl Resolver {
    /** Set an existing product's title. */
    pub fn set_product_title_command(&self) -> impl Command<SetProductTitle> {
        self.command(|resolver, command: SetProductTitle| async move {
            let store = resolver.product_store();
            let active_transaction = resolver.active_transaction();

            execute(command, active_transaction, store).await
        })
    }
}
