# An Example Rust Application

# What's this all about?

This repository contains a sample Rust application for an online store. The goal is to demonstrate some design patterns that leverage the Rust language to build scalable and maintainable applications.

It's a playground for different ideas, some of them might not pan out in practice. If you have any feedback on anything here please feel free to open up an issue. For anyone reading this code, I'd encourage you to scrutinise it based on those design decisions, think of the constraints you face in your own environment and how those might inform the software.

# What's this not about?

It's not about specific Rust frameworks or libraries.

# How it works

The following sections describe some of the pieces that make up the application and why they're put together the way they are.

## Domain folders

The project layout is focused mostly on privacy. By limiting the scope of certain items, you also limit the scope of potential breakage. In Rust, items that are private in a module _are visible to all of that module's children_. That might sound like a bad thing, but we leverage it to prevent domain APIs from leaking implementation details for the sake of outside concerns, like serialisation and storage.

We split each core concept in the application into its own (mostly) self-contained folder, like `products` or `customers`. Each folder encapsulates everything there is to know about a particular set of entities:

- What data those entities manage
- How that data is stored (`/store`)
- How that data can be queried (`/queries`)
- How that data can be changed (`/commands`)

Entities can depend on entities from another folder, like an `Order` depending on a `Product` when adding it.

These folders are _sort of_ heavy-weight, but in a proper application adding new domain folders could be simplified using macros. I haven't used macros in this application so the code remains easy to follow.

One problem with a perfectly crafted module hierarchy is that it can all fall apart when you end up with a concept that simply doesn't fit in the current layout. The more frequently this happens, the more difficult it becomes to conform to the layout that existed before because nobody can tell what it should be.

We want these folders to manage their own destiny, but we don't want them to be self-contained to the point where they could be split into separate services. This is to keep things simple. If you did want to do this then I'd suggest using separate crates instead of just separate modules.

## Dependency injection

This application doesn't use an _Inversion of Control_ container like you might be used to if you write .NET applications. This is mostly because there aren't really any for Rust. It's a hard problem.

Dependency injection does have benefits as a practice to lean on when designing applications though. It lets you separate the concerns of dependency resolution from app logic. It also gives you an obvious way to scale an application. This application adopts a simple pattern that gives us these benefits without a lot of infrastructure.

### What you need to worry about when doing dependency injection

- _Resolution:_ what dependencies do I need to build this thing?
- _Injection:_ how do I get these dependencies into this thing?
- _Storage:_ where do I store shared dependencies and remain abstract over it?

### How it works

Injectable components live in their own module. That module contains:

- _Resolution:_ an impl block for a shared `Resolver` type that contains a method that returns the default implementation without requiring its dependencies.
- _Injection:_ a function that provides a default implementation, requiring its dependencies as generics and returning an `impl Trait`. You never know what concrete type this default implementation uses.
- _Storage:_ A trait that describes the component that is blanket implemented for a few smart pointers, like `Arc`, `Box`.

The shared `Resolver` sounds a bit service-locator-y, and it is, but because the dependency resolution is wholly contained in impl blocks on the `Resolver` itself we avoid the issue of depending on magic global state in our app logic.

To reduce boilerplate, for components with only a single method we also blanket implement them for `Fn` traits. This lets you avoid declaring a structure for them that's generic over all of their dependencies. The Rust compiler will take care of that for you.

This pattern is difficult to describe in prose, you need to see it. Have a look at the `domain/products/commands/create_product` module, or the `domain/products/model/store` modules for examples of this dependency injection pattern at work.

## Models

The entities are the heart of the application. Despite the lack of a real business, I've made an effort to keep the domain model rich. Entities aren't just bags of CRUDdy state. They are:

- A collection of data types with behaviour implemented on top of them
- _Write-only_. You can get a _read-only_ view of an entity by calling `.to_data()`. While viewing an entity you can't call modifying behaviour on it. This is guaranteed by Rust's borrowing system. An entity can move ownership into its read-only data with `.into_data()`. This is a one-way operation, so any changes made to state can't be persisted back to the store.

The goal of an entity is to encapsulate the invariants of some key domain concept. The entities here are easy to use with either a mock in-memory store or an external database. We should be careful not to rely on state changes with one entity being reflected in another because they happen to point to the same source.

Entities also need to be careful not to depend on the datatypes of another entity because there's no guarantee that data is actually valid. Instead they depend on an entity and convert it into data as needed, so they always know that state is valid.

### Stores

We use the following Rust features to protect our entity state:

- Stores are child modules of entities, which means they can depend on internal details to hydrate an entity, but the rest of the world can't
- Moving an entity into its data means you can freely modify that state, but can't pretend it's an entity anymore. This means we don't lose Rust's ergonomics around structures, but don't also have the burden of maintaining invariants
- Invariants are captured in newtypes that are as thin as possible
- Types with invariants don't implement `Serialize` or `Deserialize`. This may be changed down the track, but I find it easier to keep serialisable state fast-and-loose for backwards compatibility.

### Data

Entities encapsulate some state, or data and ensure any changes made to that data don't break any invariants that data expects to hold. Rather than implementing getters, we expose a read-only view of the data as a structure. The benefit is that you don't have to give up Rust's nice features for working with datastructures, like you would with getter methods. This view is _read-only_, so changes can't be written directly back to the structure. The entity still provides setter methods for that.

You could argue that exposing state in this way leaks implementation details, like the `version` that have no value being public. This is probably true. To work around it, you could move the lifetime of the read-only view onto the fields, and compose a potentially different borrowed view of the state, and keep the data structure managed by the entity private.

You could also argue that holding invariants on a structure that isn't storing them is brittle. This makes sense when the privacy boundary for some field is at the object-level, like it is in C#. Rust is a bit different though. The tightest privacy boundary is _at the module and its childen_. So the burden of maintaining the invariants of a given field falls on all items in the module it's defined in, plus all of that module's children.

This may sound like an awful leak but this application exploits that to build nicely abstracted storage. Instead of having to expose holes in our API to support an ORM, maintaining the state of invariants simply extends into the model store, without leaking back out to the public.

### Ids and versions

The `Id` and `Version` types both have a phantom generic parameter. This parameter exists purely to let you express ids with incompatible types, like `Id<ProductData>` and `Id<OrderData>`, but still share other implementation details.

It's a pattern that's easier to follow than using a macro to reduce boilerplate because there's always a difinition in source you can go back to.

### Optimistic concurrency

Each persistable entity has a `version` field. This field is a non-sequential identifier that corresponds to the state of the entity at a given point in time. When an entity is fetched from the store we hydrate its version, this is then checked just before updating and if they don't match we balk. 

The version check works fine for the in-memory store because we have an exclusive lock on the data (only 1 caller can modify state at a time), but will need a different approach for a proper db. We can probably update where the id and version match, select the number of updated records and balk if it's 0 (means the version didn't match, or it doesn't exist).

## Commands and queries

The application follows a command-query-responsibility-separation design. The commands capture some domain interaction and work directly on entities whereas queries are totally arbitrary. This application doesn't use any special infrastructure for realising CQRS, they're just simple traits implemented using the dependency injection pattern described earlier. Essentially:

- Commands return a `Result<()>`
- Queries return a `Result<T>`
- Commands require a `&mut self` receiver
- Queries require a `&self` receiver

The difference in mutability means commands can call queries but queries can't call commands.
