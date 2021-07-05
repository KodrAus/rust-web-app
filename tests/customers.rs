#[macro_use]
extern crate serde_json;

use shop::api::client::*;

use rocket::{
    http::Status,
    local::Client,
};

#[test]
fn set_get() {
    let app = Client::new(shop::api::init()).expect("invalid app");

    let mut put = app.put("/customers").body_json(json!({})).dispatch();

    assert_eq!(Status::Created, put.status());
    let id: String = put.body_value().expect("invalid id");

    let get = app.get(format!("/customers/{}", id)).dispatch();
    assert_eq!(Status::Ok, get.status());
}
