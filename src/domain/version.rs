use uuid::Uuid;

/// A version.
/// 
/// The version provides optimistic concurrency.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version(Uuid);

impl Version {
    pub fn next(&mut self) {
        self.0 = Uuid::new_v4();
    }
}
