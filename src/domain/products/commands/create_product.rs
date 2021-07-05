/*! Contains the `CreateProductCommand` type. */

use crate::domain::{
    infra::*,
    products::*,
    Error,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateProductCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateProduct {
    pub id: ProductId,
    pub title: String,
    pub price: Currency,
}

/** Create a new product. */
#[auto_impl(FnMut)]
pub trait CreateProductCommand {
    fn create_product(&mut self, command: CreateProduct) -> Result;
}

/** Default implementation for a `CreateProductCommand`. */
pub(in crate::domain) fn create_product_command(
    transaction: ActiveTransaction,
    store: impl ProductStore,
) -> impl CreateProductCommand {
    move |command: CreateProduct| {
        debug!("creating product `{}`", command.id);

        let product = {
            if store.get_product(command.id)?.is_some() {
                err!("product `{}` already exists", command.id)?
            } else {
                Product::new(command.id, command.title, command.price)?
            }
        };

        store.set_product(transaction.get(), product)?;

        info!("created product `{}`", command.id);

        Ok(())
    }
}

impl Resolver {
    /** Create a product. */
    pub fn create_product_command(&self) -> impl CreateProductCommand {
        let store = self.product_store();
        let active_transaction = self.active_transaction();

        create_product_command(active_transaction, store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::products::model::store::in_memory_store;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store(Default::default());

        let create = CreateProduct {
            id: ProductId::new(),
            title: "Test Product".into(),
            price: Currency::usd(100),
        };

        let mut cmd = create_product_command(ActiveTransaction::none(), &store);

        cmd.create_product(create.clone()).unwrap();

        assert!(cmd.create_product(create).is_err());
    }
}
