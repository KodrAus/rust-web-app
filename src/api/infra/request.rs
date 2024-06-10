use std::future::Future;

use rocket::{
    http::Status,
    request::{
        FromRequest,
        Outcome,
    },
    Request,
};

use crate::domain::{
    infra::Resolver,
    App,
};

use super::{
    Error,
    RequestSpan,
};

pub struct AppRequest<'r> {
    span: RequestSpan,
    app: &'r App,
}

impl<'r> AppRequest<'r> {
    pub async fn transaction<T, O>(self, f: impl FnOnce(Resolver) -> O) -> Result<T, Error>
    where
        O: Future<Output = Result<T, Error>> + Send,
    {
        self.span
            .trace(async {
                let r = self.app.transaction(f).await?;

                Ok(r)
            })
            .await
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AppRequest<'r> {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let Outcome::Success(span) = RequestSpan::from_request(req).await else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        let Some(app) = req.rocket().state::<App>() else {
            return Outcome::Error((Status::InternalServerError, ()));
        };

        Outcome::Success(AppRequest { span, app })
    }
}
