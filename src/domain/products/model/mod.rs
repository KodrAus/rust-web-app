/*! Contains the `Product` entity. */

use std::convert::{
    TryFrom,
    TryInto,
};

pub mod store;

#[cfg(test)]
pub mod test_data;

use crate::domain::{
    error,
    infra::*,
    Error,
};

pub type ProductId = Id<ProductData>;
pub type NextProductId = NextId<ProductData>;
pub type ProductVersion = Version<ProductData>;

/**
A product title.

The title must not be empty.
*/
pub struct Title(String);

impl TryFrom<String> for Title {
    type Error = Error;

    fn try_from(title: String) -> Result<Self, Self::Error> {
        if title.is_empty() {
            return Err(error::msg("title must not be empty"));
        }

        Ok(Title(title))
    }
}

impl<'a> TryFrom<&'a str> for Title {
    type Error = Error;

    fn try_from(title: &'a str) -> Result<Self, Self::Error> {
        Self::try_from(title.to_owned())
    }
}

/**
A produce price.

The price must be greater than zero.
*/
pub struct Price(Currency);

impl TryFrom<Currency> for Price {
    type Error = Error;

    fn try_from(price: Currency) -> Result<Self, Self::Error> {
        Ok(Price(price))
    }
}

/** Data for a product. */
#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: ProductId,
    pub version: ProductVersion,
    pub title: String,
    pub price: Currency,
    _private: (),
}

/** A product with some simple metadata. */
pub struct Product {
    data: ProductData,
}

impl Product {
    pub(self) fn from_data(data: ProductData) -> Self {
        Product { data }
    }

    pub fn into_data(self) -> ProductData {
        self.data
    }

    pub fn to_data(&self) -> &ProductData {
        &self.data
    }

    pub fn new(
        id: impl IdProvider<ProductData>,
        title: impl TryInto<Title, Error = Error>,
        price: impl TryInto<Price, Error = Error>,
    ) -> Result<Self, Error> {
        let id = id.get()?;

        Ok(Product::from_data(ProductData {
            id,
            version: ProductVersion::default(),
            title: title.try_into()?.0,
            price: price.try_into()?.0,
            _private: (),
        }))
    }

    pub fn set_title(
        &mut self,
        title: impl TryInto<Title, Error = Error>,
    ) -> Result<(), Error> {
        self.data.title = title.try_into()?.0;

        Ok(())
    }
}

impl Entity for Product {
    type Id = ProductId;
    type Version = ProductVersion;
    type Data = ProductData;
    type Error = Error;
}

impl Resolver {
    pub fn product_id(&self) -> impl IdProvider<ProductData> {
        NextId::<ProductData>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_must_be_non_empty() {
        assert!(Product::new(ProductId::new(), "", Currency::usd(100)).is_err());

        let mut product = Product::new(ProductId::new(), "A title", Currency::usd(100)).unwrap();

        assert!(product.set_title("").is_err());
    }
}
