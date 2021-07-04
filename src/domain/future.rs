use std::{
    future,
    pin::Pin,
};

pub use futures::FutureExt;

pub type Future<T> = Pin<Box<dyn future::Future<Output = T> + Send>>;
