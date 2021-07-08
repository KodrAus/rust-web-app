#[macro_use]
extern crate serde_json;

use rocket::{
    http::Status,
    local::blocking::Client,
};

#[test]
fn set_get() {
    let app = Client::untracked(shop::api::init()).expect("invalid app");

    let put = app
        .put("/products")
        .json(&json!({
            "title": "A new product",
            "price": {
                "usd": {
                    "cents": 123
                }
            }
        }))
        .dispatch();

    assert_eq!(Status::Created, put.status());
    let id: String = put.into_json().expect("invalid id");

    let get = app.get(format!("/products/{}", id)).dispatch();

    assert_eq!(Status::Ok, get.status());
    let product: serde_json::Value = get.into_json().expect("invalid product");

    assert_eq!(
        "A new product",
        product.as_object().expect("invalid product")["title"]
    );
}
