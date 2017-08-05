# An Example Rust Application

## Design

This application has a few key design decisions:

- Aggregates are a collection of data types with behaviour implemented on top of them
- Aggregates are _write-only_. You can get a _read-only_ view of an aggregate by calling `.to_data()`. While viewing an aggregate you can't call modifying behaviour on it.
- An aggregate can move ownership into its read-only data with `.into_data()`. This is a one-way operation, so any changes made to state can't be persisted back to the store.

The goal of an aggregate is to encapsulate the invariants of some system state. The aggregates here are easy to use with either a mock in-memory store or an external database. We should be careful not to rely on state changes with one aggregate being reflected in another because they happen to point to the same source.

We use the following Rust features to protect our aggregate state:

- Stores are child modules of aggregates, which means they can depend on internal details to hydrate an aggregate, but the rest of the world can't
- Moving an aggregate into its data means you can freely modify that state, but can't pretend it's an aggregate. This means we don't lose Rust's ergonomics around structures, but don't also have the burden of maintaining invariants
- Invariants are captured in newtypes that are as thin as possible
- Types with invariants don't implement `Serialize` or `Deserialize`. This may be changed down the track, but I find it easier to keep serialisable state fast-and-loose.