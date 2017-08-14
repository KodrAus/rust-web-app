use std::str::FromStr;

use uuid::Uuid;

pub type OrderIdError = String;

/// A builder for a new order id.
pub trait OrderIdProvider {
    fn order_id(self) -> Result<OrderId, OrderIdError>;
}

/// An order id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OrderId(Uuid);

impl OrderId {
    pub fn new() -> Self {
        OrderId(Uuid::new_v4())
    }
}

impl FromStr for OrderId {
    type Err = OrderIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|_| "invalid id")?;

        Ok(OrderId(uuid))
    }
}

impl OrderIdProvider for OrderId {
    fn order_id(self) -> Result<OrderId, OrderIdError> {
        Ok(self)
    }
}

pub type LineItemIdError = String;

/// A builder for a new line item id.
pub trait LineItemIdProvider {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError>;
}

/// An order line item id.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LineItemId(Uuid);

impl LineItemId {
    pub fn new() -> Self {
        LineItemId(Uuid::new_v4())
    }
}

impl FromStr for LineItemId {
    type Err = LineItemIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|_| "invalid id")?;

        Ok(LineItemId(uuid))
    }
}

impl LineItemIdProvider for LineItemId {
    fn line_item_id(self) -> Result<LineItemId, LineItemIdError> {
        Ok(self)
    }
}
