/*! Domain module for products. */

pub mod commands;
pub mod model;
pub mod queries;
pub mod resolver;

pub(self) use self::model::store::{
    ProductStore,
    ProductStoreFilter,
};
pub use self::{
    model::*,
    resolver::*,
};

pub use self::{
    commands::*,
    queries::*,
};
