use auto_impl::auto_impl;

use domain::products::{Product, ProductId, ProductStore, Resolver};

pub type CreateProductError = String;

#[derive(Clone, Deserialize)]
pub struct CreateProduct {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
}

#[auto_impl(FnMut)]
pub trait CreateProductCommand {
    fn create_product(&mut self, command: CreateProduct) -> Result<(), CreateProductError>;
}

pub fn create_product_command<TStore>(store: TStore) -> impl CreateProductCommand
where
    TStore: ProductStore,
{
    move |command: CreateProduct| {
        let product = {
            if store.get(command.id)?.is_some() {
                Err("already exists")?
            } else {
                Product::new(command.id, command.title, command.price)?
            }
        };

        store.set(product)?;

        Ok(())
    }
}

impl Resolver {
    pub fn create_product_command(&self) -> impl CreateProductCommand {
        let store = self.product_store();

        create_product_command(store)
    }
}

#[cfg(test)]
mod tests {
    use domain::products::model::store::in_memory_store;
    use super::*;

    #[test]
    fn err_if_already_exists() {
        let store = in_memory_store();

        let product_id = ProductId::new();

        let create = CreateProduct {
            id: product_id,
            title: "Test Product".into(),
            price: 1f32,
        };

        let mut cmd = create_product_command(&store);

        cmd.create_product(create.clone()).unwrap();

        assert!(cmd.create_product(create).is_err());
    }
}
