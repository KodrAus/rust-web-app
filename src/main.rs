/*!
An example Rust web application.

This is the main entrypoint for the app. It depends on the library version
of the same app and hosts its API on the default `localhost:8000`.
*/

use std::process::ExitCode;

#[rocket::main]
async fn main() -> ExitCode {
    shop::logger::init();

    emit::info!("starting up");

    let exit = match shop::api::init().ignite().await {
        Ok(rocket) => {
            let listen = format!("{}:{}", rocket.config().address, rocket.config().port);

            emit::info!("listening at {listen}", #[emit::as_serde] config: rocket.config());

            match rocket.launch().await {
                Ok(_) => ExitCode::SUCCESS,
                Err(err) => {
                    emit::error!("rocket launch failed with {err}");
                    ExitCode::FAILURE
                }
            }
        }
        Err(err) => {
            emit::error!("rocket ignite failed with {err}");
            ExitCode::FAILURE
        }
    };

    emit::info!("shutting down");

    shop::logger::finish();

    exit
}
