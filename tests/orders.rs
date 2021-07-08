#[macro_use]
extern crate serde_json;

use rocket::{
    http::Status,
    local::blocking::Client,
};

#[test]
fn set_get() {
    let app = Client::untracked(shop::api::init()).expect("invalid app");

    let product_id: String = app
        .put("/products")
        .json(&json!({
            "title": "A new product",
            "price": {
                "usd": {
                    "cents": 123
                }
            }
        }))
        .dispatch()
        .into_json()
        .expect("invalid id");

    let customer_id: String = app
        .put("/customers")
        .json(&json!({}))
        .dispatch()
        .into_json()
        .expect("invalid id");

    let put = app
        .put("/orders")
        .json(&json!({ "customer": customer_id }))
        .dispatch();

    assert_eq!(Status::Created, put.status());
    let order_id: String = put.into_json().expect("invalid id");

    app.post(format!("/orders/{}/products/{}", order_id, product_id))
        .json(&json!({
            "quantity": 4
        }))
        .dispatch();

    let get = app.get(format!("/orders/{}", order_id)).dispatch();

    assert_eq!(Status::Ok, get.status());
    let order: serde_json::Value = get.into_json().expect("invalid order");

    assert_eq!(
        1,
        order.as_object().expect("invalid order")["line_items"]
            .as_array()
            .expect("invalid order")
            .len()
    );
}
