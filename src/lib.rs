#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::needless_doctest_main)]
/*!
# UI Composer

Rust-based library for modern, native user interface rendering.

It makes extensive use of signals, such that even things like layout changes
make use of them.

## Getting started

After adding the library, you should be able to create a simple Window like this:

```rust
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(()));
}
```

This library does support several backends, depending on what features you have enabled.
At the moment it supports:

- PC, Android, Web
    - [winitwgpu]
    - [backends::wgpu]
- Terminal
    - [tui]

## No-std

No-std is planned but not yet available.
*/

use backends::{tui, winitwgpu};

#[doc(hidden)]
#[rust_analyzer::completions(ignore_flyimport)]
pub mod prelude;

/// Module for the app "orchestration" functionality of UI composer,
/// as it pertains to generic user interface.
///
/// Apps, Graphical Backends, input and traits for things that can be
/// "composed" together to create apps.
pub mod app;

/// Module for graphical mathematics utilities.
pub mod geometry;

/// Module for layout functions and utilities.
pub mod layout;

/// Module for state management.
pub mod state;

/// Module for optional builtin backends. Might move each backend to a sub-crate.
/// `ui-composer-winit` and `ui-composer-tui` and `ui-composer-embedded`.
pub mod backends;
/// Module for optional builtin components.Might move this to a sub-crate, too.
/// `ui-composer-standard` has a nice ring to it, no?
pub mod components;
