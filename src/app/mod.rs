use rocket;
use domain::Resolver;

mod products;

pub fn start() {
    rocket::ignite()
        .manage(Resolver::default())
        .mount("/products", routes![
            products::get,
            products::create,
            products::set_title
        ]).launch();
}
