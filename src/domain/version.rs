use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::marker::PhantomData;
use uuid::Uuid;

/// A version.
/// 
/// The version provides optimistic concurrency.
#[derive(Serialize, Deserialize)]
pub struct Version<T>(Uuid, PhantomData<T>);

impl<T> Debug for Version<T> {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.0.fmt(f)
    }
}

impl<T> Clone for Version<T> {
    fn clone(&self) -> Self {
        Version(self.0.clone(), PhantomData)
    }
}

impl<T> Copy for Version<T> { }

impl<T> Default for Version<T> {
    fn default() -> Self {
        Version(Uuid::default(), PhantomData)
    }
}

impl<T> PartialEq for Version<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }

    fn ne(&self, other: &Self) -> bool {
        self.0.ne(&other.0)
    }
}

impl<T> Eq for Version<T> { }

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

impl<T> Version<T> {
    pub fn new() -> Self {
        Version(Uuid::new_v4(), PhantomData)
    }
}

impl<T> Version<T> {
    pub fn next(&mut self) {
        self.0 = Uuid::new_v4();
    }
}
