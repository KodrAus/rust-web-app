use std::{
    error,
    fmt,
};

/** The main error type */
#[derive(Debug)]
pub struct Error {
    kind: Kind,
    inner: Box<dyn error::Error + Send + Sync>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

#[derive(Debug)]
pub enum Kind {
    BadInput,
    Other,
}

pub fn msg(err: impl fmt::Display) -> Error {
    Error {
        kind: Kind::Other,
        inner: err.to_string().into(),
    }
}

pub fn bad_input(msg: impl fmt::Display) -> Error {
    Error {
        kind: Kind::BadInput,
        inner: msg.to_string().into(),
    }
}

impl Error {
    pub(crate) fn split(self) -> (Kind, Box<dyn error::Error + Send + Sync>) {
        (self.kind, self.inner)
    }
}

impl<E> From<E> for Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    fn from(err: E) -> Error {
        Error {
            kind: Kind::Other,
            inner: err.into(),
        }
    }
}

macro_rules! err {
    ($($err:tt)*) => {{
        error!($($err)*);
        Err(crate::domain::error::msg(format!($($err)*)))
    }};
}
