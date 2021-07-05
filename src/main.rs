fn main() -> Result<(), Box<dyn std::error::Error>> {
    shop::logger::init();
    let err = shop::app::init().launch();

    Err(err.into())
}
