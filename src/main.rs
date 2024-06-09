/*!
An example Rust web application.

This is the main entrypoint for the app. It depends on the library version
of the same app and hosts its API on the default `localhost:8000`.
*/

#[rocket::main]
async fn main() {
    shop::logger::init();

    emit::info!("starting up");

    if let Err(err) = shop::api::init().launch().await {
        emit::error!("rocket failed with {err}");
    }

    emit::info!("shutting down");

    shop::logger::finish();
}
