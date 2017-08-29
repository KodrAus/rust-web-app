use std::error::Error;
use std::convert::TryFrom;
use uuid::Uuid;

pub type OrderIdError = String;

/// An order id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new() -> Self {
        OrderId(Uuid::new_v4())
    }
}

impl<'a> TryFrom<&'a str> for OrderId {
    type Error = OrderIdError;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        Ok(OrderId(Uuid::parse_str(id).map_err(|e| format!("{}", e))?))
    }
}

/// A builder for a new order id.
pub trait OrderIdProvider {
    fn order_id(self) -> Result<OrderId, OrderIdError>;
}

impl OrderIdProvider for OrderId {
    fn order_id(self) -> Result<OrderId, OrderIdError> {
        Ok(self)
    }
}

pub struct NextOrderId;

impl NextOrderId {
    pub fn next(&self) -> OrderId {
        OrderId::new()
    }
}

impl OrderIdProvider for NextOrderId {
    fn order_id(self) -> Result<OrderId, OrderIdError> {
        Ok(self.next())
    }
}

pub type LineItemIdError = String;

/// An order line item id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineItemId(Uuid);

impl LineItemId {
    pub fn new() -> Self {
        LineItemId(Uuid::new_v4())
    }
}

impl<'a> TryFrom<&'a str> for LineItemId {
    type Error = LineItemIdError;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        Ok(LineItemId(Uuid::parse_str(id).map_err(|e| format!("{}", e))?))
    }
}

/// A builder for a new line item id.
pub trait LineItemIdProvider {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError>;
}

impl LineItemIdProvider for LineItemId {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError> {
        Ok(self)
    }
}

pub struct NextLineItemId;

impl NextLineItemId {
    pub fn next(&self) -> LineItemId {
        LineItemId::new()
    }
}

impl LineItemIdProvider for NextLineItemId {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError> {
        Ok(self.next())
    }
}
