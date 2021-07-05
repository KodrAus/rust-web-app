/*! Domain module for customers. */

pub mod commands;
pub mod model;
pub mod queries;
pub(in crate::domain) mod resolver;

pub use self::{
    commands::*,
    model::*,
    queries::*,
};

pub(self) use self::model::store::CustomerStore;
