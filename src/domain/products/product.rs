pub type ProductError = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct ProductData {
    pub id: i32,
    pub title: String,
}

pub struct Product {
    data: ProductData
}

impl Product {
    pub(in domain::products) fn from_data(data: ProductData) -> Self {
        Product {
            data: data
        }
    }

    pub fn into_data(self) -> ProductData {
        self.data
    }

    pub fn new(id: i32, title: String) -> Result<Self, ProductError> {
        Ok(Product::from_data(ProductData {
            id: id,
            title: title
        }))
    }

    pub fn set_title(&mut self, title: String) -> Result<(), ProductError> {
        self.data.title = title;

        Ok(())
    }
}
