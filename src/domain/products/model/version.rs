use domain::version::Version;

/// A product version.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProductVersion(pub Version);
