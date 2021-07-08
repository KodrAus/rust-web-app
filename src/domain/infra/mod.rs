/*!
Supporting shared infrastructure for the domain.

This module contains shared types like `Currency` and services like `Resolver` that other
domain modules can use.
*/

pub(in crate::domain) mod currency;
pub(in crate::domain) mod entity;
pub(in crate::domain) mod future;
pub(in crate::domain) mod id;
pub(in crate::domain) mod resolver;
pub(in crate::domain) mod transactions;
pub(in crate::domain) mod version;

pub use self::{
    currency::*,
    id::*,
    resolver::*,
    transactions::*,
    version::*,
};

pub(in crate::domain) use self::{
    entity::*,
    future::*,
};
