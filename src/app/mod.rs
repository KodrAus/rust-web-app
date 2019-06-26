/*!
Rocket app configuration.
*/

use crate::domain::Resolver;
use rocket;

mod error;
mod id;

pub mod customers;
pub mod orders;
pub mod products;

pub fn start() {
    info!("starting up");

    rocket::ignite()
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
        .register(catchers![error::not_found, error::internal_error])
        .launch();
}
