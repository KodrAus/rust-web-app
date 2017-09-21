/*! Queries for fetching customer state. */

pub mod get_customer;
pub use self::get_customer::{GetCustomer, GetCustomerQuery};

pub mod get_customer_with_orders;
pub use self::get_customer_with_orders::{CustomerWithOrders, GetCustomerWithOrders, GetCustomerWithOrdersQuery};
