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

    let put = app.put("/customers").json(&json!({})).dispatch().await;

    assert_eq!(Status::Created, put.status());
    let id: String = serde_json::from_str(&put.into_string().await.expect("missing body"))
        .expect("invalid value");

    let get = app.get(format!("/customers/{}", id)).dispatch().await;
    assert_eq!(Status::Ok, get.status());
}
