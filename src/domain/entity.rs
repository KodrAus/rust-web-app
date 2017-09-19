/// Defines the constrains that all entities must satisfy.
///
/// This trait is really just a marker for ensuring all entities follow a basic structure.
/// It's a checklist: the first thing to do when creating a new entity is to implement this trait and fill in the blanks.
/// Any changes to entities that should be consistent can be added here.
pub(in domain) trait Entity {
    type Id;
    type Version;
    type Data;
    type Error;
}
