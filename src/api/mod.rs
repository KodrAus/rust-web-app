/*!
Rocket app configuration.
*/

use rocket::Build;

use crate::domain::Resolver;

mod error;
mod id;

pub mod customers;
pub mod orders;
pub mod products;

/**
Create a `Rocket` that will host the app.

The rocket can either be launched or passed to a local client for testing.
*/
pub fn init() -> rocket::Rocket<Build> {
    info!("starting up");

    rocket::build()
        .manage(Resolver::default())
        .mount(
            "/products",
            routes![products::get, products::create, products::set_title],
        )
        .mount(
            "/orders",
            routes![orders::get, orders::create, orders::add_or_update_product],
        )
        .mount("/customers", routes![customers::get, customers::create])
        .register("/", catchers![error::not_found, error::internal_error])
}
