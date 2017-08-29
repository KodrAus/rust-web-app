use domain::version::Version;

/// An order version.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrderVersion(pub Version);

/// An order version.
#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LineItemVersion(pub Version);
