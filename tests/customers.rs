#[macro_use]
extern crate serde_json;

use rocket::{
    http::Status,
    local::blocking::Client,
};

#[test]
fn set_get() {
    let app = Client::untracked(shop::api::init()).expect("invalid app");

    let put = app.put("/customers").json(&json!({})).dispatch();

    assert_eq!(Status::Created, put.status());
    let id: String = put.into_json().expect("invalid id");

    let get = app.get(format!("/customers/{}", id)).dispatch();
    assert_eq!(Status::Ok, get.status());
}
