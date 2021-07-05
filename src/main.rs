/*!
An example Rust web application.

This is the main entrypoint for the app. It depends on the library version
of the same app and hosts its API on the default `localhost:8000`.
*/

fn main() -> Result<(), Box<dyn std::error::Error>> {
    shop::logger::init();
    let err = shop::api::init().launch();

    Err(err.into())
}
