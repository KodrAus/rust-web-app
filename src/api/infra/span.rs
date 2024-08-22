use std::future::Future;

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

pub struct RequestSpan {
    ctxt: emit::SpanCtxt,
}

impl RequestSpan {
    pub async fn trace<T>(self, f: impl Future<Output = Result<T, Error>>) -> Result<T, Error> {
        self.ctxt
            .push(emit::ctxt())
            .in_future(async move {
                match f.await {
                    Ok(r) => Ok(r),
                    Err(err) => {
                        if err.status().code == Status::InternalServerError.code {
                            emit::error!("request failed with {err}");
                        } else {
                            emit::warn!("request failed with {err}");
                        }

                        Err(err)
                    }
                }
            })
            .await
    }
}

// An internal type used to store a span on the request
struct CachedSpan {
    timer: emit::Timer<emit::runtime::AmbientClock<'static>>,
    ctxt: emit::SpanCtxt,
}

impl CachedSpan {
    fn start() -> Self {
        // Construct a span that represents the request
        // and store it in the request's local metadata
        let timer = emit::Timer::start(emit::clock());
        let ctxt = emit::SpanCtxt::new_root(emit::rng());

        CachedSpan { timer, ctxt }
    }
}

#[rocket::async_trait]
impl Fairing for SpanFairing {
    fn info(&self) -> Info {
        Info {
            name: "Span Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _: &mut Data<'_>) {
        req.local_cache(CachedSpan::start);
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, res: &mut Response<'r>) {
        let span = req.local_cache(CachedSpan::start);

        let status = res.status().code;

        emit::emit!(
            evt: emit::Span::new(
                emit::mdl!(),
                "HTTP request",
                span.timer,
                span.ctxt,
            ),
            "HTTP {method: req.method()} {uri: req.uri().to_string()} {status}",
            lvl: match status {
                200..=399 => emit::Level::Debug,
                400..=499 => emit::Level::Warn,
                _ => emit::Level::Error,
            },
        );
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestSpan {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let ctxt = req.local_cache(CachedSpan::start).ctxt;

        Outcome::Success(RequestSpan { ctxt })
    }
}
