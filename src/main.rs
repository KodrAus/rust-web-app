/*!
An example Rust web application.

The project is split into two main parts:

- `app`: the rocket web application where the app is configured and hosted
- `domain`: the business domain where the app logic is defined

Most of the `domain` module is `pub(restricted)`, so these docs only show the items that can be consumed by the application.
Refer to the source for a fuller picture of what's in there.
*/

#![feature(decl_macro)]
#![allow(clippy::manual_non_exhaustive, clippy::type_complexity)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate log;

#[macro_use]
extern crate auto_impl;

pub mod app;
pub mod domain;
pub mod logger;
pub mod store;

fn main() {
    logger::init();
    app::start();
}
