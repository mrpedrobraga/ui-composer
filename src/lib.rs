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
* use ui_composer::standard::prelude::*;
*
* fn main() {
*     UIComposer::run(Window(()));
* }
* ```

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

use standard::backends;
use standard::backends::{tui, winitwgpu};

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

/// Module for optional builtin standard.Might move this to a sub-crate, too.
/// `ui-composer-standard` has a nice ring to it, no?
pub mod standard;
