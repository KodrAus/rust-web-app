use std::convert::TryFrom;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use uuid::Uuid;

pub type IdError = String;

/// An id.
#[derive(Serialize, Deserialize)]
pub struct Id<T>(Uuid, PhantomData<T>);

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Id(self.0.clone(), PhantomData)
    }
}

impl<T> Copy for Id<T> { }

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl<T> Eq for Id<T> { }

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
    pub fn new() -> Self {
        Id(Uuid::new_v4(), PhantomData)
    }
}

impl<'a , T> TryFrom<&'a str> for Id<T> {
    type Error = IdError;

    fn try_from(id: &'a str) -> Result<Self, Self::Error> {
        Ok(Id(Uuid::parse_str(id).map_err(|e| format!("{}", e))?, PhantomData))
    }
}

/// A builder for a new id.
pub trait IdProvider<T> {
    fn id(self) -> Result<Id<T>, IdError>;
}

impl<T> IdProvider<T> for Id<T> {
    fn id(self) -> Result<Id<T>, IdError> {
        Ok(self)
    }
}

pub struct NextId<T>(PhantomData<T>);

impl<T> NextId<T> {
    pub fn new() -> Self {
        NextId(PhantomData)
    }

    pub fn next(&self) -> Id<T> {
        Id::new()
    }
}

impl<T> IdProvider<T> for NextId<T> {
    fn id(self) -> Result<Id<T>, IdError> {
        Ok(self.next())
    }
}
