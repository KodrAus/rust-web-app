/*!
Defines the constraints that all entities must satisfy.

This trait is really just a marker for ensuring all entities follow a basic structure.
It's a checklist: the first thing to do when creating a new entity is to implement this trait and fill in the blanks.
Any changes to entities that should be consistent can be added here.
*/

#[allow(dead_code)]
pub(in crate::domain) trait Entity {
    /** Should be `Id<Self::Data>`. */
    type Id;
    /** Should be `Version<Self::Data>`. */
    type Version;
    /** Should be the result of calling `self.into_data()`. */
    type Data;
    /** Should be the `Err` variant for any `Result` returning methods on `Self`. */
    type Error;
}
