use crate::domain::{
    customers::model::test_data::default_customer,
    orders::*,
    products::*,
};

pub fn default_order() -> Order {
    Order::new(NextOrderId::new(), &default_customer()).unwrap()
}

pub struct OrderBuilder {
    order: Order,
    line_items: Vec<(
        Product,
        Box<Fn(OrderLineItemBuilder) -> OrderLineItemBuilder>,
    )>,
}

impl Default for OrderBuilder {
    fn default() -> Self {
        OrderBuilder {
            order: default_order(),
            line_items: vec![],
        }
    }
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: OrderId) -> Self {
        self.order.order.id = id;
        self
    }

    pub fn add_product<F>(mut self, product: Product, builder: F) -> Self
    where
        F: Fn(OrderLineItemBuilder) -> OrderLineItemBuilder + 'static,
    {
        self.line_items.push((product, Box::new(builder)));
        self
    }

    pub fn build(mut self) -> Order {
        for (product, builder) in self.line_items {
            self.order
                .add_product(NextLineItemId::new(), &product, 1)
                .unwrap();
            let line_item = self.order.line_items.pop().unwrap();

            let line_item = builder(OrderLineItemBuilder {
                line_item: OrderLineItem::from_data(self.order.order.clone(), line_item),
            });

            self.order.line_items.push(line_item.build());
        }
        self.order
    }
}

pub struct OrderLineItemBuilder {
    line_item: OrderLineItem,
}

impl OrderLineItemBuilder {
    pub fn id(mut self, id: LineItemId) -> Self {
        self.line_item.line_item.id = id;
        self
    }

    pub fn quantity(mut self, quantity: u32) -> Self {
        self.line_item.set_quantity(quantity).unwrap();
        self
    }

    fn build(self) -> LineItemData {
        self.line_item.into_data().1
    }
}
