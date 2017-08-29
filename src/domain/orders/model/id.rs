use domain::id::Id;

pub type OrderIdError = String;

/// A builder for a new order id.
pub trait OrderIdProvider {
    fn order_id(self) -> Result<OrderId, OrderIdError>;
}

/// An order id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(pub Id);

impl OrderIdProvider for OrderId {
    fn order_id(self) -> Result<OrderId, OrderIdError> {
        Ok(self)
    }
}

pub struct NextOrderId;

impl NextOrderId {
    pub fn next(&self) -> OrderId {
        OrderId(Id::new())
    }
}

impl OrderIdProvider for NextOrderId {
    fn order_id(self) -> Result<OrderId, OrderIdError> {
        Ok(self.next())
    }
}

pub type LineItemIdError = String;

/// A builder for a new line item id.
pub trait LineItemIdProvider {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError>;
}

/// An order line item id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineItemId(pub Id);

impl LineItemIdProvider for LineItemId {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError> {
        Ok(self)
    }
}

pub struct NextLineItemId;

impl NextLineItemId {
    pub fn next(&self) -> LineItemId {
        LineItemId(Id::new())
    }
}

impl LineItemIdProvider for NextLineItemId {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError> {
        Ok(self.next())
    }
}
