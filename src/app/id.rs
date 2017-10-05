use std::convert::TryFrom;
use rocket::request::FromParam;
use rocket::http::RawStr;
use domain::id::Id;

impl<'r, T> FromParam<'r> for Id<T> {
    type Error = String;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        Id::try_from(param.as_str())
    }
}
