use std::{
    error,
    fmt,
};

/**
The main error type.

The error type is a simple collector that surfaces enough detail to the API so it
can pick the right status code to return. It doesn't know anything about HTTP or
status codes itself.

As a collector, the error type doesn't implement Rust's `Error` trait itself,
otherwise it wouldn't be able to collect any other kind of error.
*/
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    inner: Box<dyn error::Error + Send + Sync>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

/**
The kind of an error captured.
*/
#[derive(Debug)]
pub enum ErrorKind {
    /** A command or query was given bad input. */
    BadInput,
    /** Some other kind of error. */
    Other,
}

/**
Create an error from a message.

This message may make its way to end-users so it should be friendly.
*/
pub fn msg(err: impl fmt::Display) -> Error {
    Error {
        kind: ErrorKind::Other,
        inner: err.to_string().into(),
    }
}

/**
Create an error from a diagnostic event.

The event will be emitted.
*/
pub fn emit(event: impl emit::event::ToEvent) -> Error {
    let event = event.to_event();

    emit::error!(evt: &event);

    msg(event.msg())
}

/**
Create an error for some bad input.

This message may make its way to end-users so it should be friendly.
*/
pub fn bad_input(msg: impl fmt::Display) -> Error {
    Error {
        kind: ErrorKind::BadInput,
        inner: msg.to_string().into(),
    }
}

impl Error {
    /**
    Split an error into its kind and value.
    */
    pub(crate) fn split(self) -> (ErrorKind, Box<dyn error::Error + Send + Sync>) {
        (self.kind, self.inner)
    }
}

impl<E> From<E> for Error
where
    E: Into<Box<dyn error::Error + Send + Sync>>,
{
    fn from(err: E) -> Error {
        Error {
            kind: ErrorKind::Other,
            inner: err.into(),
        }
    }
}
