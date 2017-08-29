use std::cmp::Ordering;
use uuid::Uuid;

/// An order version.
/// 
/// The version provides optimistic concurrency.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Version(Uuid);

impl Version {
    pub (in domain::orders) fn next(&mut self) {
        self.0 = Uuid::new_v4();
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Version) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for Version {
    fn cmp(&self, other: &Version) -> Ordering {
        self.0.cmp(&other.0)
    }
}
