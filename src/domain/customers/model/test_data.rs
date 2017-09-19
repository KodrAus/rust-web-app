use domain::customers::*;

pub fn default_customer() -> Customer {
    Customer::new(NextCustomerId::new()).unwrap()
}

pub struct CustomerBuilder {
    customer: Customer,
}

impl Default for CustomerBuilder {
    fn default() -> Self {
        CustomerBuilder {
            customer: default_customer(),
        }
    }
}

impl CustomerBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: CustomerId) -> Self {
        self.customer.data.id = id;
        self
    }

    pub fn build(self) -> Customer {
        self.customer
    }
}
