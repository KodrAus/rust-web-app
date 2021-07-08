/*!
An example Rust web application.

This is the main entrypoint for the app. It depends on the library version
of the same app and hosts its API on the default `localhost:8000`.
*/

#[macro_use]
extern crate rocket;

#[launch]
async fn launch() -> _ {
    shop::logger::init();
    shop::api::init()
}
