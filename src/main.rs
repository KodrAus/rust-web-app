/*!
An example Rust web application.

The project is split into two main parts:

- The rocket web application where the app is configured and hosted
- The _domain_ where the app logic is defined
*/

#![feature(plugin, proc_macro, conservative_impl_trait, try_from)]
#![plugin(rocket_codegen)]

extern crate auto_impl;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

pub mod domain;
pub mod app;

fn main() {
    app::start();
}
