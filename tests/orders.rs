#[macro_use]
extern crate rocket;

#[macro_use]
extern crate serde_json;

use rocket::{
    http::Status,
    local::asynchronous::Client,
};

#[async_test]
async fn set_get() {
    let app = Client::untracked(shop::api::init())
        .await
        .expect("invalid app");

    let product_id: String = {
        let get = app
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
            .await;

        serde_json::from_str(&get.into_string().await.expect("missing body"))
            .expect("invalid value")
    };

    let customer_id: String = {
        let get = app.put("/customers").json(&json!({})).dispatch().await;

        serde_json::from_str(&get.into_string().await.expect("missing body"))
            .expect("invalid value")
    };

    let put = app
        .put("/orders")
        .json(&json!({ "customer": customer_id }))
        .dispatch()
        .await;

    assert_eq!(Status::Created, put.status());
    let order_id: String = serde_json::from_str(&put.into_string().await.expect("missing body"))
        .expect("invalid value");

    app.post(format!("/orders/{}/products/{}", order_id, product_id))
        .json(&json!({
            "quantity": 4
        }))
        .dispatch()
        .await;

    let get = app.get(format!("/orders/{}", order_id)).dispatch().await;

    assert_eq!(Status::Ok, get.status());
    let order: serde_json::Value =
        serde_json::from_str(&get.into_string().await.expect("missing body"))
            .expect("invalid value");

    assert_eq!(
        1,
        order.as_object().expect("invalid order")["line_items"]
            .as_array()
            .expect("invalid order")
            .len()
    );
}
