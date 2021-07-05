/*!
API client support for using and testing the app's API.
*/

use std::error;

use serde::de::DeserializeOwned;
use serde_json::Value;

use rocket::{
    http::Header,
    local::{
        LocalRequest,
        LocalResponse,
    },
};

pub type Error = Box<dyn error::Error + Send + Sync>;

/**
Extensions for a local request for some JSON data.
*/
pub trait LocalRequestExt {
    /** Add a JSON body to the request. */
    fn body_json(self, json: impl Into<Value>) -> Self;
}

impl<'c> LocalRequestExt for LocalRequest<'c> {
    fn body_json(self, json: impl Into<Value>) -> Self {
        self.header(Header::new("Content-Type", "application/json"))
            .body(json.into().to_string())
    }
}

/**
Extensions for a local response for some JSON data.
*/
pub trait LocalResponseExt {
    /** Read a JSON body from the response. */
    fn body_json(&mut self) -> Result<Value, Error>;

    /** Read a JSON body of a specific type from the resppnse. */
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
