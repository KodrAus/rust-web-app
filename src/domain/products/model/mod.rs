use std::convert::TryInto;

pub mod store;

/// A product id.
#[derive(Clone, Serialize, Deserialize)]
pub struct Id(i32);

/// A product title.
pub struct Title(String);

/// A produce price.
pub struct Price(f32);

pub type ProductError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: Id,
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

    pub fn new<TTitle, TPrice>(id: Id, title: TTitle, price: TPrice) -> Result<Self, ProductError> 
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
