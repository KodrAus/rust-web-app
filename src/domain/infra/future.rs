use std::{
    future,
    pin::Pin,
};

pub(in crate::domain) use futures::FutureExt;

pub(in crate::domain) type Future<T> = Pin<Box<dyn future::Future<Output = T> + Send>>;
