use std::sync::Arc;

use domain::customers::model::store as customer_store;
use domain::products::model::store as product_store;
use domain::orders::model::store as order_store;

pub struct Resolver {
    product_store: Arc<product_store::InMemoryStore>,
    order_store: Arc<order_store::InMemoryStore>,
    customer_store: Arc<customer_store::InMemoryStore>,
}

impl Default for Resolver {
    fn default() -> Self {
        Resolver {
            product_store: Arc::new(product_store::in_memory_store()),
            order_store: Arc::new(order_store::in_memory_store()),
            customer_store: Arc::new(customer_store::in_memory_store()),
        }
    }
}

impl Resolver {
    pub fn product_store(&self) -> impl product_store::ProductStore {
        self.product_store.clone()
    }

    pub fn order_store(&self) -> impl order_store::OrderStore {
        self.order_store.clone()
    }

    pub fn order_line_item_store(&self) -> impl order_store::OrderLineItemStore {
        self.order_store.clone()
    }

    pub fn customer_store(&self) -> impl customer_store::CustomerStore {
        self.customer_store.clone()
    }
}
