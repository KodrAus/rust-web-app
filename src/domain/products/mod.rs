/*! Domain module for products. */

pub mod resolver;
pub mod model;
pub mod commands;
pub mod queries;

pub use self::resolver::*;
pub use self::model::*;
pub(self) use self::model::store::{ProductStore, ProductStoreFilter};

pub use self::commands::*;
pub use self::queries::*;
