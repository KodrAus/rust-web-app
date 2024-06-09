/*!
Rocket app configuration.
*/

use rocket::Build;

use crate::domain::App;

mod infra;

pub mod customers;
pub mod orders;
pub mod products;

/**
Create a `Rocket` that will host the app.

The rocket can either be launched or passed to a local client for testing.
*/
pub fn init() -> rocket::Rocket<Build> {
    rocket::build()
        .manage(App::default())
        .mount(
            "/products",
            rocket::routes![products::get, products::create, products::set_title],
        )
        .mount(
            "/orders",
            rocket::routes![orders::get, orders::create, orders::add_or_update_product],
        )
        .mount(
            "/customers",
            rocket::routes![customers::get, customers::create],
        )
        .attach(infra::span::SpanFairing)
        .register(
            "/",
            rocket::catchers![infra::error::not_found, infra::error::internal_error],
        )
}
