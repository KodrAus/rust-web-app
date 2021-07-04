/*! Contains the shared `Id` type. */

use serde::{
    de::{
        Deserialize,
        Deserializer,
    },
    ser::{
        Serialize,
        Serializer,
    },
};
use std::{
    cmp::Ordering,
    convert::TryFrom,
    fmt::{
        self,
        Formatter,
        Result as FmtResult,
    },
    hash::{
        Hash,
        Hasher,
    },
    marker::PhantomData,
};
use uuid::Uuid;

use crate::domain::error::Error;

/**
An id.

Ids have a phantom generic parameter so you can't compare an `Id<T>` to an `Id<U>`.
It means you also can't use an `Id<T>` in place of an `Id<U>`.
*/
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0, PhantomData)
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Id<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Id(Uuid::new_v4(), PhantomData)
    }
}

impl<'a, T> TryFrom<&'a str> for Id<T> {
    type Error = Error;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        Ok(Id(Uuid::parse_str(id)?, PhantomData))
    }
}

impl<T> Serialize for Id<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = Uuid::deserialize(deserializer)?;
        Ok(Id(id, PhantomData))
    }
}

/**
A builder for a new id.

Items that need to generate an id should depend on an `IdProvider` rather than taking an `Id` directly.
*/
#[auto_impl(&, Arc)]
pub trait IdProvider<T> {
    fn id(&self) -> Result<Id<T>, Error>;
}

impl<T> IdProvider<T> for Id<T> {
    fn id(&self) -> Result<Id<T>, Error> {
        Ok(*self)
    }
}

/** Generate a new `Id` randomly. */
pub struct NextId<T>(PhantomData<T>);

impl<T> Default for NextId<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> NextId<T> {
    pub fn new() -> Self {
        NextId(PhantomData)
    }

    pub fn next(&self) -> Id<T> {
        Id::new()
    }
}

impl<T> IdProvider<T> for NextId<T> {
    fn id(&self) -> Result<Id<T>, Error> {
        Ok(self.next())
    }
}
