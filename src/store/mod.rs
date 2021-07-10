/*!
Transactional value storage.

This module sketches out a storage API that uses transactions to coordinate updates to
independent data stores. The design assumes data for a given transaction will be technically
observable (such as being written to disk or some external database) before the transaction
itself is committed. The transaction store keeps track of whether or not the data associated
with a given transaction should be surfaced to callers or not.
*/

mod transaction;
mod value;

pub use self::{
    transaction::*,
    value::*,
};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
