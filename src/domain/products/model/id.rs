use std::str::FromStr;

use uuid::Uuid;

pub type ProductIdError = String;

/// A builder for a new product id.
pub trait ProductIdProvider {
    fn product_id(self) -> Result<ProductId, ProductIdError>;
}

/// A product id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new() -> Self {
        ProductId(Uuid::new_v4())
    }
}

impl FromStr for ProductId {
    type Err = ProductIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|_| "invalid id")?;

        Ok(ProductId(uuid))
    }
}

impl ProductIdProvider for ProductId {
    fn product_id(self) -> Result<ProductId, ProductIdError> {
        Ok(self)
    }
}
