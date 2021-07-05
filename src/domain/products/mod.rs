/*! Domain module for products. */

pub mod commands;
pub mod model;
pub mod queries;
pub(in crate::domain) mod resolver;

pub(self) use self::model::store::{
    ProductStore,
    ProductStoreFilter,
};
pub use self::{
    commands::*,
    model::*,
    queries::*,
};
