pub(in crate::domain) type Future<T> =
    ::std::pin::Pin<Box<dyn ::std::future::Future<Output = T> + Send>>;

pub(in crate::domain) use futures::FutureExt;
