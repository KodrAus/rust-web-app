use std::{
    error,
    fmt,
    io::Cursor,
};

use serde::Serializer;

use rocket::{
    http::{
        ContentType,
        Status,
    },
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

impl Error {
    pub(in crate::api) fn status(&self) -> Status {
        match self {
            Error::NotFound(_) => Status::NotFound,
            Error::BadRequest(_) => Status::BadRequest,
            Error::Other(_) => Status::InternalServerError,
        }
    }

    fn into_inner(self) -> Box<dyn error::Error + Send + Sync> {
        match self {
            Error::NotFound(err) => err,
            Error::BadRequest(err) => err,
            Error::Other(err) => err,
        }
    }
}

pub fn msg(err: impl fmt::Display) -> Box<dyn error::Error + Send + Sync> {
    err.to_string().into()
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'o> {
        let status = self.status();

        let err = self.into_inner();

        let err = serde_json::to_vec(&SerializeError { msg: &err }).unwrap_or_else(|_| Vec::new());

        Response::build()
            .sized_body(None::<usize>, Cursor::new(err))
            .header(ContentType::JSON)
            .status(status)
            .ok()
    }
}

impl From<domain::Error> for Error {
    fn from(err: domain::Error) -> Self {
        use crate::domain::ErrorKind::*;

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

#[rocket::catch(500)]
pub(in crate::api) fn internal_error(_: &Request) -> content::RawJson<Vec<u8>> {
    let err = serde_json::to_vec(&SerializeError {
        msg: &"an internal error occurred",
    })
    .unwrap_or_else(|_| Vec::new());

    content::RawJson(err)
}

#[rocket::catch(404)]
pub(in crate::api) fn not_found(_: &Request) -> content::RawJson<Vec<u8>> {
    let err =
        serde_json::to_vec(&SerializeError { msg: &"not found" }).unwrap_or_else(|_| Vec::new());

    content::RawJson(err)
}
