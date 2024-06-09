/*!
An example Rust web application.

The project is split into a few main parts:

- `api`: the rocket web application where the app is configured and hosted
- `domain`: the core app logic
- `store`: a little transactional datastore implementation

Most of the `domain` module is `pub(restricted)`, so these docs only show the items
that can be consumed by the application. Refer to the source for a fuller picture of what's in there.
You can also build the docs with private items visible to explore everything:

```text
cargo doc --document-private-items
```
 */

#![allow(
    clippy::manual_non_exhaustive,
    clippy::type_complexity,
    clippy::nonstandard_macro_braces
)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate thiserror;

#[macro_use]
extern crate auto_impl;

pub mod api;
pub mod domain;
pub mod logger;
pub mod store;
