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
        .dispatch()
        .await;

    assert_eq!(Status::Created, put.status());
    let id: String = serde_json::from_str(&put.into_string().await.expect("missing body"))
        .expect("invalid value");

    let get = app.get(format!("/products/{}", id)).dispatch().await;

    assert_eq!(Status::Ok, get.status());
    let product: serde_json::Value =
        serde_json::from_str(&get.into_string().await.expect("missing body"))
            .expect("invalid value");

    assert_eq!(
        "A new product",
        product.as_object().expect("invalid product")["title"]
    );
}
