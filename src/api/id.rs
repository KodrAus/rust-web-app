use rocket::{
    request::FromParam,
};
use std::convert::TryFrom;

use crate::domain::{
    infra::*,
    Error,
};

impl<'r, T> FromParam<'r> for Id<T> {
    type Error = Error;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        Id::try_from(param)
    }
}
