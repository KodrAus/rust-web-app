/**
API client support for using and testing the app's API.
*/
use std::error;

use serde::de::DeserializeOwned;
use serde_json::Value;

use rocket::local::{
    LocalRequest,
    LocalResponse,
};

pub type Error = Box<dyn error::Error + Send + Sync>;

pub trait LocalRequestExt {
    fn body_json(self, json: impl Into<Value>) -> Self;
}

impl<'c> LocalRequestExt for LocalRequest<'c> {
    fn body_json(self, json: impl Into<Value>) -> Self {
        self.body(json.into().to_string())
    }
}

pub trait LocalResponseExt {
    fn body_json(&mut self) -> Result<Value, Error>;

    fn body_value<T>(&mut self) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let value = serde_json::from_value(self.body_json()?)?;

        Ok(value)
    }
}

impl<'c> LocalResponseExt for LocalResponse<'c> {
    fn body_json(&mut self) -> Result<Value, Error> {
        let body = self
            .body_string()
            .ok_or_else(|| Error::from("missing response"))?;
        let value = serde_json::from_str(&body)?;

        Ok(value)
    }
}
