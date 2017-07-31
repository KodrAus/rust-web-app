pub(in domain) mod store;

pub type ProductError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: i32,
    pub title: String,
    pub price: f32,
    _private: (),
}

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

    pub fn new<TTitle>(id: i32, title: TTitle, price: f32) -> Result<Self, ProductError> 
        where TTitle: Into<String>
    {
        Ok(Product::from_data(ProductData {
            id: id,
            title: title.into(),
            price: price,
            _private: (),
        }))
    }

    pub fn set_title(&mut self, title: String) -> Result<(), ProductError> {
        self.data.title = title;

        Ok(())
    }
}
