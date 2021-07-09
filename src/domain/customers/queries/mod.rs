/*! Queries for fetching customer state. */

mod get_customer;
mod get_customer_with_orders;

pub use self::{
    get_customer::*,
    get_customer_with_orders::*,
};
