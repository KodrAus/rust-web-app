use rocket;
use domain;

mod products;

pub fn start() {
    rocket::ignite()
        .manage(domain::products::Resolver::default())
        .mount("/products", routes![
            products::get,
            products::post
        ]).launch();
}