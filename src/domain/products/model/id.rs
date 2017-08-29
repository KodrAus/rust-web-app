use domain::id::Id;

pub type ProductIdError = String;

/// A builder for a new product id.
pub trait ProductIdProvider {
    fn product_id(self) -> Result<ProductId, ProductIdError>;
}

/// A product id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(pub Id);

impl ProductIdProvider for ProductId {
    fn product_id(self) -> Result<ProductId, ProductIdError> {
        Ok(self)
    }
}

pub struct NextProductId;

impl NextProductId {
    pub fn next(&self) -> ProductId {
        ProductId(Id::new())
    }
}

impl ProductIdProvider for NextProductId {
    fn product_id(self) -> Result<ProductId, ProductIdError> {
        Ok(self.next())
    }
}
