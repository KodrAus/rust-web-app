/*! Contains the `CreateProductCommand` type. */

use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{Product, ProductId, ProductStore};

pub type Error = String;
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
pub fn create_product_command<TStore>(store: TStore) -> impl CreateProductCommand
where
    TStore: ProductStore,
{
    move |command: CreateProduct| {
        let product = {
            if store.get_product(command.id)?.is_some() {
                Err("already exists")?
            } else {
                Product::new(command.id, command.title, command.price)?
            }
        };

        store.set_product(product)?;

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
    use domain::products::model::store::in_memory_store;
    use domain::products::*;
    use super::*;

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
