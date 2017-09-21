/*!
Rocket app configuration.
*/

use rocket;
use domain::Resolver;

pub mod products;
pub mod orders;
pub mod customers;

pub fn start() {
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
        .launch();
}
