use std::{
    fmt,
    io::Cursor,
};

use serde_derive::*;

use serde::Serializer;

use failure;
use failure_derive::*;

use rocket::{
    http,
    request::Request,
    response::{
        self,
        content,
        Responder,
        Response,
    },
};

pub use failure::err_msg;

use crate::domain;

/** The main application error. */
#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "an entity wasn't found")]
    NotFound(#[cause] failure::Error),
    #[fail(display = "the user input was invalid")]
    BadRequest(#[cause] failure::Error),
    #[fail(display = "an unexpected error occurred")]
    Other(#[cause] failure::Error),
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        let (status, err) = match self {
            Error::NotFound(err) => {
                debug!("request failed with {:?}", err);

                (http::Status::NotFound, err)
            }
            Error::BadRequest(err) => {
                debug!("request failed with {:?}", err);

                (http::Status::BadRequest, err)
            }
            Error::Other(err) => {
                error!("request failed with {:?}", err);

                (http::Status::InternalServerError, err)
            }
        };

        let err = serde_json::to_vec(&SerializeError { msg: &err }).unwrap_or_else(|_| Vec::new());

        Response::build()
            .sized_body(Cursor::new(err))
            .header(http::ContentType::JSON)
            .status(status)
            .ok()
    }
}

impl From<domain::error::Error> for Error {
    fn from(err: domain::error::Error) -> Self {
        use crate::domain::error::Kind::*;

        match err.split() {
            (BadInput, err) => Error::BadRequest(err),
            (_, err) => Error::Other(err),
        }
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Self {
        Error::Other(err)
    }
}

#[derive(Serialize)]
struct SerializeError<'a> {
    #[serde(serialize_with = "msg")]
    msg: &'a dyn fmt::Display,
}

fn msg<S>(msg: &&dyn fmt::Display, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(msg)
}

#[catch(500)]
pub(super) fn internal_error<'r>(_: &Request) -> content::Json<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
    .unwrap_or_else(|_| Vec::new());

    content::Json(err)
}

#[catch(404)]
pub(super) fn not_found<'r>(_: &Request) -> content::Json<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
    .unwrap_or_else(|_| Vec::new());

    content::Json(err)
}
