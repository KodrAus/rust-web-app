/*! Contains the `CreateProductCommand` type. */

use auto_impl::auto_impl;

use crate::domain::{
    error::{err_msg, Error},
    products::{Product, ProductId, ProductStore},
    Resolver,
};

pub type Result = ::std::result::Result<(), Error>;

/** Input for a `CreateProductCommand`. */
#[derive(Clone, Deserialize)]
pub struct CreateProduct {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
}

/** Create a new product. */
#[auto_impl(FnMut)]
pub trait CreateProductCommand {
    fn create_product(&mut self, command: CreateProduct) -> Result;
}

/** Default implementation for a `CreateProductCommand`. */
pub(in crate::domain) fn create_product_command(
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

        store.set_product(product)?;

        info!("created product `{}`", command.id);

        Ok(())
    }
}

impl Resolver {
    pub fn create_product_command(&self) -> impl CreateProductCommand {
        let store = self.products().product_store();

        create_product_command(store)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::domain::products::{model::store::in_memory_store, *};

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let create = CreateProduct {
            id: ProductId::new(),
            title: "Test Product".into(),
            price: 1f32,
        };

        let mut cmd = create_product_command(&store);

        cmd.create_product(create.clone()).unwrap();

        assert!(cmd.create_product(create).is_err());
    }
}
