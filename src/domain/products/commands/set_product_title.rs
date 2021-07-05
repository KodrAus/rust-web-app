/*! Contains the `SetProductTitleCommand`. */

use crate::domain::{
    error,
    infra::*,
    products::*,
    Error,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `SetProductTitleCommand`. */
#[derive(Clone, Deserialize)]
pub struct SetProductTitle {
    pub id: ProductId,
    pub title: String,
}

/** Set a new title for a product. */
#[auto_impl(FnMut)]
pub trait SetProductTitleCommand {
    fn set_product_title(&mut self, command: SetProductTitle) -> Result;
}

/** Default implementation for a `SetProductTitleCommand`. */
pub(in crate::domain) fn set_product_title_command(
    transaction: ActiveTransaction,
    store: impl ProductStore,
) -> impl SetProductTitleCommand {
    move |command: SetProductTitle| {
        debug!(
            "updating product `{}` title to {:?}",
            command.id, command.title
        );

        let product = {
            if let Some(mut product) = store.get_product(command.id)? {
                product.set_title(command.title)?;

                product
            } else {
                return Err(error::msg("not found"));
            }
        };

        store.set_product(transaction.get(), product)?;

        info!("updated product `{}` title", command.id);

        Ok(())
    }
}

impl Resolver {
    /** Set an existing product's title. */
    pub fn set_product_title_command(&self) -> impl SetProductTitleCommand {
        let store = self.product_store();
        let active_transaction = self.active_transaction();

        set_product_title_command(active_transaction, store)
    }
}
