use crate::domain::{
    products::*,
    infra::*,
};

pub fn default_title() -> String {
    "A test product".to_owned()
}

pub fn default_price() -> Currency {
    Currency::usd(100)
}

pub fn default_product() -> Product {
    Product::new(NextProductId::new(), default_title(), default_price()).unwrap()
}

pub struct ProductBuilder {
    product: Product,
}

impl Default for ProductBuilder {
    fn default() -> Self {
        ProductBuilder {
            product: default_product(),
        }
    }
}

impl ProductBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: ProductId) -> Self {
        self.product.data.id = id;
        self
    }

    pub fn build(self) -> Product {
        self.product
    }
}
