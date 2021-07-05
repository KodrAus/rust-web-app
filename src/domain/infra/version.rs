/*! Contains the shared `Version` type. */

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

use crate::store;

/**
A version.

The version provides optimistic concurrency.
Versions have a phantom generic type so you can't compare `Version<T>` to `Version<U>`.
*/
pub struct Version<T>(Uuid, PhantomData<T>);

impl<T> From<Version<T>> for store::Version {
    fn from(id: Version<T>) -> store::Version {
        store::Version::from_raw(id.0)
    }
}

impl<T> From<store::Version> for Version<T> {
    fn from(id: store::Version) -> Version<T> {
        Version(id.into_raw(), PhantomData)
    }
}

impl<T> fmt::Debug for Version<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> fmt::Display for Version<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Clone for Version<T> {
    fn clone(&self) -> Self {
        Version(self.0, PhantomData)
    }
}

impl<T> Copy for Version<T> {}

impl<T> Default for Version<T> {
    fn default() -> Self {
        Version(Uuid::default(), PhantomData)
    }
}

impl<T> PartialEq for Version<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<T> Eq for Version<T> {}

impl<T> PartialOrd for Version<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Version<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for Version<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<T> Serialize for Version<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Version<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = Uuid::deserialize(deserializer)?;
        Ok(Version(id, PhantomData))
    }
}

impl<T> Version<T> {
    pub fn new() -> Self {
        Version(Uuid::new_v4(), PhantomData)
    }
}

impl<T> Version<T> {
    pub(in crate::domain) fn next(&mut self) -> Version<T> {
        *self = Version(Uuid::new_v4(), PhantomData);
        *self
    }
}
