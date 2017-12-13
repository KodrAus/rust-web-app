/*!
An example Rust web application.

The project is split into two main parts:

- `app`: the rocket web application where the app is configured and hosted
- `domain`: the business domain where the app logic is defined

Most of the `domain` module is `pub(restricted)`, so these docs only show the items that can be consumed by the application.
Refer to the source for a fuller picture of what's in there.
*/

#![feature(plugin, proc_macro, conservative_impl_trait, universal_impl_trait, try_from)]
#![plugin(rocket_codegen)]

extern crate auto_impl;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate failure;
#[macro_use]
extern crate failure_derive;

pub mod domain;
pub mod app;

fn main() {
    app::start();
}
