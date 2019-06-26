use std::error;

use failure;

use failure_derive::*;

/** The main error type */
#[derive(Fail, Debug)]
#[fail(display = "application error ({:?})", kind)]
pub struct Error {
    kind: Kind,
    #[cause]
    inner: failure::Error,
}

#[derive(Debug)]
pub enum Kind {
    BadInput,
    Other,
}

pub fn err_msg(msg: impl Into<String>) -> Error {
    Error {
        kind: Kind::Other,
        inner: failure::err_msg(msg.into()),
    }
}

pub fn bad_input(msg: impl Into<String>) -> Error {
    Error {
        kind: Kind::BadInput,
        inner: failure::err_msg(msg.into()),
    }
}

impl Error {
    pub(crate) fn split(self) -> (Kind, failure::Error) {
        (self.kind, self.inner)
    }
}

impl<E> From<E> for Error
where
    E: error::Error + Send + Sync + 'static,
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
        Err(err_msg(format!($($err)*)))
    }};
}
