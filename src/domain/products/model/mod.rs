use std::convert::{TryInto, TryFrom};
use std::str::FromStr;

use uuid::Uuid;

pub mod store;

/// A product id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ProductId(Uuid);

impl ProductId {
    pub fn new() -> Self {
        ProductId(Uuid::new_v4())
    }
}

impl FromStr for ProductId {
    type Err = ProductError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|_| "invalid id")?;

        Ok(ProductId(uuid))
    }
}

/// A product title.
pub struct Title(String);

impl TryFrom<String> for Title {
    type Error = ProductError;

    fn try_from(title: String) -> Result<Self, Self::Error> {
        if title.len() == 0 {
            Err("title must not be empty")?
        }

        Ok(Title(title))
    }
}

impl<'a> TryFrom<&'a str> for Title {
    type Error = ProductError;

    fn try_from(title: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(title.to_owned())
    }
}

/// A produce price.
pub struct Price(f32);

impl TryFrom<f32> for Price {
    type Error = ProductError;

    fn try_from(price: f32) -> Result<Self, Self::Error> {
        if !price.is_normal() || !price.is_sign_positive() {
            Err("price must be greater than 0")?
        }

        Ok(Price(price))
    }
}

pub type ProductError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: ProductId,
    pub title: String,
    pub price: f32,
    _private: (),
}

/// A product with metadata.
pub struct Product {
    data: ProductData
}

impl Product {
    fn from_data(data: ProductData) -> Self {
        Product {
            data: data
        }
    }

    pub fn into_data(self) -> ProductData {
        self.data
    }

    pub fn to_data(&self) -> &ProductData {
        &self.data
    }

    pub fn new<TTitle, TPrice>(id: ProductId, title: TTitle, price: TPrice) -> Result<Self, ProductError> 
        where TTitle: TryInto<Title, Error = ProductError>,
              TPrice: TryInto<Price, Error = ProductError>
    {
        Ok(Product::from_data(ProductData {
            id: id,
            title: title.try_into()?.0,
            price: price.try_into()?.0,
            _private: (),
        }))
    }

    pub fn set_title<TTitle>(&mut self, title: TTitle) -> Result<(), ProductError> 
        where TTitle: TryInto<Title, Error = ProductError>
    {
        self.data.title = title.try_into()?.0;

        Ok(())
    }
}
