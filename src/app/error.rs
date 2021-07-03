use std::{
    error,
    fmt,
    io::Cursor,
};

use serde::Serializer;

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

use crate::domain;

/** The main application error. */
#[derive(Error, Debug)]
pub enum Error {
    #[error("an entity wasn't found")]
    NotFound(#[source] Box<dyn error::Error + Send + Sync>),
    #[error("the user input was invalid")]
    BadRequest(#[source] Box<dyn error::Error + Send + Sync>),
    #[error("an unexpected error occurred")]
    Other(#[source] Box<dyn error::Error + Send + Sync>),
}

pub fn msg(err: impl fmt::Display) -> Box<dyn error::Error + Send + Sync> {
    err.to_string().into()
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

impl From<Box<dyn error::Error + Send + Sync>> for Error {
    fn from(err: Box<dyn error::Error + Send + Sync>) -> Self {
        Error::Other(err)
    }
}

#[derive(Serialize)]
struct SerializeError<'a> {
    #[serde(serialize_with = "serialize_msg")]
    msg: &'a dyn fmt::Display,
}

fn serialize_msg<S>(msg: &&dyn fmt::Display, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.collect_str(msg)
}

#[catch(500)]
pub(super) fn internal_error(_: &Request) -> content::Json<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
    .unwrap_or_else(|_| Vec::new());

    content::Json(err)
}

#[catch(404)]
pub(super) fn not_found(_: &Request) -> content::Json<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
    .unwrap_or_else(|_| Vec::new());

    content::Json(err)
}
