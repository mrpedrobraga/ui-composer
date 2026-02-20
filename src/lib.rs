#![cfg_attr(feature = "specialization", feature(specialization))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_doctest_main)]
/*!
# UI Composer

A batteries-included user interface library.

```rust
* use ui_composer::prelude::UIComposer;
UIComposer::run_winit(Window(()))
```

*/

/// Module for app definition, blueprints and composition.
pub mod app;
/// Module for geometry and visual mathematics.
pub mod geometry;
/// Module for state management, tasks, animations and reactivity.
pub mod state;

#[doc(hidden)]
#[rust_analyzer::completions(ignore_flyimport)]
pub mod prelude;

pub mod standard;

pub use standard::runners;
