use std::{
    future::Future,
    mem,
    sync::Mutex,
};

use rocket::{
    fairing::{
        Fairing,
        Info,
        Kind,
    },
    http::{
        Method,
        Status,
    },
    request::{
        FromRequest,
        Outcome,
    },
    Data,
    Request,
    Response,
};

use super::Error;

/**
A fairing that creates diagnostic spans for incoming requests.

Endpoints need to accept a [`RequestSpan`] argument to use the generated span.
*/
pub struct SpanFairing;

type Span = emit::Span<
    'static,
    emit::runtime::AmbientClock<'static>,
    emit::Empty,
    fn(emit::span::SpanEvent<'static, emit::Empty>),
>;

// TODO: Come up with a generic `RequestExecutor` type that wraps up `app`
// and this span and gives us a generic `.invoke(|app: &App| { .. })` method
// Then any new per-request things will automatically be available on all routes
pub struct RequestSpan {
    span: Span,
    method: Method,
    uri: String,
}

impl RequestSpan {
    pub async fn trace<T>(mut self, f: impl Future<Output = Result<T, Error>>) -> Result<T, Error> {
        let rt = emit::runtime::shared();

        // TODO: Don't actually complete the span here; just use it to populate trace ids
        // Then we can attach response data in the response callback
        self.span
            .push_ctxt(rt.ctxt(), emit::Empty)
            .in_future(async move {
                match f.await {
                    Ok(r) => {
                        self.span.complete_with(|event| {
                            emit::debug!(event, "HTTP {method: self.method} {uri: self.uri}")
                        });

                        Ok(r)
                    }
                    Err(err) => {
                        self.span.complete_with(|event| {
                            let status = err.status();

                            if status.code == Status::InternalServerError.code {
                                emit::error!(
                                    event,
                                    "HTTP {method: self.method} {uri: self.uri}",
                                    status,
                                    err
                                );
                            } else {
                                emit::warn!(
                                    event,
                                    "HTTP {method: self.method} {uri: self.uri}",
                                    status,
                                    err
                                );
                            }
                        });

                        Err(err)
                    }
                }
            })
            .await
    }
}

// An internal type used to store a span on the request
struct CachedSpan(Mutex<Span>);

#[rocket::async_trait]
impl Fairing for SpanFairing {
    fn info(&self) -> Info {
        Info {
            name: "Span Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let rt = emit::runtime::shared();

        // Construct a span that represents the request
        // and store it in the request's local metadata
        let span = Span::new(
            emit::Timer::start(*rt.clock()),
            emit::module!(),
            emit::format!("HTTP {method: req.method()} {uri: req.uri().to_string()}"),
            emit::span::SpanCtxt::empty().new_child(rt.rng()),
            emit::Empty,
            |event| emit::emit!(event),
        );

        req.local_cache(|| CachedSpan(Mutex::new(span)));
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, _: &mut Response<'r>) {
        // If the caller didn't explicitly complete the span then do it here
        if let Outcome::Success(span) = RequestSpan::from_request(req).await {
            span.span.complete_with(|event| {
                emit::debug!(event, "HTTP {method: span.method} {uri: span.uri}")
            });
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestSpan {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let span = mem::replace(
            &mut *req
                .local_cache(|| CachedSpan(Mutex::new(Span::disabled())))
                .0
                .lock()
                .unwrap(),
            Span::disabled(),
        );

        Outcome::Success(RequestSpan {
            span,
            method: req.method(),
            uri: req.uri().to_string(),
        })
    }
}
