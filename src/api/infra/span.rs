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
    http::Status,
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
    ctxt: Option<emit::span::SpanCtxt>,
}

impl RequestSpan {
    pub async fn trace<T>(self, f: impl Future<Output = Result<T, Error>>) -> Result<T, Error> {
        let rt = emit::runtime::shared();

        emit::Frame::push(rt.ctxt(), self.ctxt)
            .in_future(async move {
                match f.await {
                    Ok(r) => Ok(r),
                    Err(err) => {
                        if err.status().code == Status::InternalServerError.code {
                            emit::error!("request failed with {err}",);
                        } else {
                            emit::warn!("request failed with {err}",);
                        }

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

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let span = mem::replace(
            &mut *req
                .local_cache(|| CachedSpan(Mutex::new(Span::disabled())))
                .0
                .lock()
                .unwrap(),
            Span::disabled(),
        );

        span.complete_with(|event| {
            let status = res.status().code;

            emit::emit!(
                event,
                "HTTP {method: req.method()} {uri: req.uri().to_string()} {status}",
                lvl: match status {
                    200..=399 => emit::Level::Debug,
                    400..=499 => emit::Level::Warn,
                    _ => emit::Level::Error,
                },
            )
        });
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestSpan {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let ctxt = req
            .local_cache(|| CachedSpan(Mutex::new(Span::disabled())))
            .0
            .lock()
            .unwrap()
            .ctxt()
            .copied();

        Outcome::Success(RequestSpan { ctxt })
    }
}
