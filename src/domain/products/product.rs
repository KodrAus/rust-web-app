pub type ProductError = String;

#[derive(Serialize, Deserialize)]
pub struct Product {
    id: i32,
    title: String
}

impl Product {
    pub(in domain::products) fn new(id: i32, title: String) -> Product {
        Product {
            id: id,
            title: title
        }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: String) -> Result<(), ProductError> {
        self.title = title;

        Ok(())
    }
}