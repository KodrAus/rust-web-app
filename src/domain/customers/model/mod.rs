#[derive(Clone, Serialize, Deserialize)]
pub struct CustomerData {
    pub id: i32,
    _private: (),
}

pub struct Customer {
    data: CustomerData,
}

impl Customer {
    fn from_data(data: CustomerData) -> Self {
        Customer {
            data: data
        }
    }

    pub fn to_data(&self) -> &CustomerData {
        &self.data
    }

    pub fn into_data(self) -> CustomerData {
        self.data
    }

    pub fn new(id: i32) -> Self {
        Customer::from_data(CustomerData {
            id: id,
            _private: (),
        })
    }
}
