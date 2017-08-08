#![feature(plugin, proc_macro, conservative_impl_trait, try_from)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate auto_impl;
extern crate uuid;

pub mod domain;
pub mod app;

fn main() {
    app::start();
}
