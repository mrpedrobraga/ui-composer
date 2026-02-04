#![cfg_attr(feature = "specialization", feature(specialization))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_doctest_main)]
/*!
# UI Composer

Rust-based library for modern, native user interface rendering.

Define your application by composing building blocks, then run them in different environments.

## Getting started

Add the library to your crate with `cargo add ui-composer`.

After adding the library, you should be able to create a simple Window like this:

```rust
* use ui_composer::standard::prelude::*;
*
* fn main() {
*   UIComposer::run(
*       Window(())
*   );
* }
```

> [!INFO] Not on crates.io
> While this library isn't on crates.io yet, you can add it with
> `cargo add --git https://github.com/mrpedrobraga/ui-composer.git ui-composer`.

## No-std

No-std is not yet available.
*/

/// Module for composition and execution of declarative programs.
pub mod app;

/// Module for mathematics related to visual arrangements of things.
pub mod geometry;

/// Module for state definition and management.
pub mod state;

/// Module for optional builtin standard. Might move this to a sub-crate, too.
/// `ui-composer-standard` has a nice ring to it, no?
pub mod standard;

pub use standard::runners;