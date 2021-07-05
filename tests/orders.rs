#[macro_use]
extern crate serde_json;

use shop::api::client::*;

use rocket::{
    http::{
        Header,
        Status,
    },
    local::Client,
};

#[test]
fn set_get() {
    let app = Client::new(shop::api::init()).expect("invalid app");

    let product_id: String = app
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
        .dispatch()
        .body_value()
        .expect("invalid id");

    let customer_id: String = app
        .put("/customers")
        .header(Header::new("Content-Type", "application/json"))
        .body_json(json!({}))
        .dispatch()
        .body_value()
        .expect("invalid id");

    let mut put = app
        .put("/orders")
        .header(Header::new("Content-Type", "application/json"))
        .body_json(json!({ "customer": customer_id }))
        .dispatch();

    assert_eq!(Status::Created, put.status());
    let order_id: String = put.body_value().expect("invalid id");

    app.post(format!("/orders/{}/products/{}", order_id, product_id))
        .header(Header::new("Content-Type", "application/json"))
        .body_json(json!({
            "quantity": 4
        }))
        .dispatch();

    let mut get = app.get(format!("/orders/{}", order_id)).dispatch();

    assert_eq!(Status::Ok, get.status());
    let order = get.body_json().expect("invalid order");

    assert_eq!(
        1,
        order.as_object().expect("invalid order")["line_items"]
            .as_array()
            .expect("invalid order")
            .len()
    );
}
