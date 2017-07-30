use auto_impl::auto_impl;

use domain::Resolver;
use domain::products::{Product, ProductStore};

pub type CreateProductError = String;

#[derive(Deserialize)]
pub struct CreateProduct {
    pub id: i32,
    pub title: String,
    pub price: f32,
}

#[auto_impl(FnMut)]
pub trait CreateProductCommand {
    fn create_product(&mut self, command: CreateProduct) -> Result<(), CreateProductError>;
}

pub fn create_product_command<TStore>(store: TStore) -> impl CreateProductCommand 
    where TStore: ProductStore
{
    move |command: CreateProduct| {
        if let Some(_) = store.get(command.id)? {
            Err("already exists")?
        }
        else {
            let product = Product::new(command.id, command.title, command.price)?;

            store.set(product)?;

            Ok(())
        }
    }
}

impl Resolver {
    pub fn create_product_command(&self) -> impl CreateProductCommand {
        let store = self.product_store();

        create_product_command(store)
    }
}
