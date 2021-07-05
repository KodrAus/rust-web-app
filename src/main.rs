fn main() -> Result<(), Box<dyn std::error::Error>> {
    shop::logger::init();
    let err = shop::api::init().launch();

    Err(err.into())
}
