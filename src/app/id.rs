use std::convert::TryFrom;
use rocket::request::FromParam;
use rocket::http::RawStr;
use domain::id::Id;
use domain::error::Error;

impl<'r, T> FromParam<'r> for Id<T> {
    type Error = Error;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        Id::try_from(param.as_str())
    }
}
