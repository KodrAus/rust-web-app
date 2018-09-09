/*! Contains the `Product` entity. */

use std::convert::{TryFrom, TryInto};

pub mod store;

#[cfg(test)]
pub mod test_data;

use crate::domain::{
    entity::Entity,
    error::{err_msg, Error},
    id::{Id, IdProvider, NextId},
    version::Version,
    Resolver,
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
        if title.len() == 0 {
            Err(err_msg("title must not be empty"))?
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
pub struct Price(f32);

impl TryFrom<f32> for Price {
    type Error = Error;

    fn try_from(price: f32) -> Result<Self, Self::Error> {
        if !price.is_normal() || !price.is_sign_positive() {
            Err(err_msg("price must be greater than 0"))?
        }

        Ok(Price(price))
    }
}

/** Data for a product. */
#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: ProductId,
    pub version: ProductVersion,
    pub title: String,
    pub price: f32,
    _private: (),
}

/** A product with some simple metadata. */
pub struct Product {
    data: ProductData,
}

impl Product {
    pub(self) fn from_data(data: ProductData) -> Self {
        Product { data: data }
    }

    pub fn into_data(self) -> ProductData {
        self.data
    }

    pub fn to_data(&self) -> &ProductData {
        &self.data
    }

    pub fn new<TId, TTitle, TPrice>(
        id_provider: TId,
        title: TTitle,
        price: TPrice,
    ) -> Result<Self, Error>
    where
        TId: IdProvider<ProductData>,
        TTitle: TryInto<Title, Error = Error>,
        TPrice: TryInto<Price, Error = Error>,
    {
        let id = id_provider.id()?;

        Ok(Product::from_data(ProductData {
            id: id,
            version: ProductVersion::default(),
            title: title.try_into()?.0,
            price: price.try_into()?.0,
            _private: (),
        }))
    }

    pub fn set_title<TTitle>(&mut self, title: TTitle) -> Result<(), Error>
    where
        TTitle: TryInto<Title, Error = Error>,
    {
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
    pub fn product_id_provider(&self) -> impl IdProvider<ProductData> {
        NextId::<ProductData>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_must_be_non_empty() {
        assert!(Product::new(ProductId::new(), "", 1f32).is_err());

        let mut product = Product::new(ProductId::new(), "A title", 1f32).unwrap();

        assert!(product.set_title("").is_err());
    }

    #[test]
    fn price_must_be_greater_than_0() {
        assert!(Product::new(ProductId::new(), "A title", 0f32).is_err());
        assert!(Product::new(ProductId::new(), "A title", -1f32).is_err());
    }
}
