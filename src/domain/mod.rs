/*!
Domain modules.

The domain contains modules for entities like products and customers as well as some shared infrastructure.
Entity submodules are organized around a single entity, or group of closely related entities, and their storage.
The public API contains entities, queries and commands that can depend on private storage.

Organizing entities this way means we don't need a leaky public API for the sake of storage.
Cross-cutting concerns should either live in the most specific entity submodule, or go in a new one.
*/

#[macro_use]
mod error;
pub mod infra;

pub mod customers;
pub mod orders;
pub mod products;

pub use self::{
    error::{
        Error,
        ErrorKind,
    },
    infra::App,
};
