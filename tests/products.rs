#[macro_use]
extern crate serde_json;

use shop::app::client::*;

use rocket::{
    http::{
        Header,
        Status,
    },
    local::Client,
};

#[test]
fn set_get() {
    let app = Client::new(shop::app::init()).expect("invalid app");

    let mut put = app
        .put("/products")
        .header(Header::new("Content-Type", "application/json"))
        .body_json(json!({
            "title": "A new product",
            "price": {
                "usd": {
                    "cents": 123
                }
            }
        }))
        .dispatch();

    assert_eq!(Status::Created, put.status());
    let id: String = put.body_value().expect("invalid id");

    let mut get = app.get(format!("/products/{}", id)).dispatch();

    assert_eq!(Status::Ok, get.status());
    let product = get.body_json().expect("invalid product");

    assert_eq!(
        "A new product",
        product.as_object().expect("invalid product")["title"]
    );
}
