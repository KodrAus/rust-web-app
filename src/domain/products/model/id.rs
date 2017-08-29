use std::error::Error;
use std::convert::TryFrom;
use uuid::Uuid;

pub type ProductIdError = String;

/// A product id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new() -> Self {
        ProductId(Uuid::new_v4())
    }
}

impl<'a> TryFrom<&'a str> for ProductId {
    type Error = ProductIdError;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        Ok(ProductId(Uuid::parse_str(id).map_err(|e| format!("{}", e))?))
    }
}

/// A builder for a new product id.
pub trait ProductIdProvider {
    fn product_id(self) -> Result<ProductId, ProductIdError>;
}

impl ProductIdProvider for ProductId {
    fn product_id(self) -> Result<ProductId, ProductIdError> {
        Ok(self)
    }
}

pub struct NextProductId;

impl NextProductId {
    pub fn next(&self) -> ProductId {
        ProductId::new()
    }
}

impl ProductIdProvider for NextProductId {
    fn product_id(self) -> Result<ProductId, ProductIdError> {
        Ok(self.next())
    }
}
