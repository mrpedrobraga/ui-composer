#![cfg_attr(not(feature = "std"), no_std)]
/*!
# UI Composer

Rust-based library for modern, native UI Rendering. It makes use of signals
and the Editor pattern to make interface creation a breeze. The reactivity it uses
are so efficient, that the app _itself_ uses signals for broadcasting layout changes.

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
    - [wgpu]
- Terminal
    - [tui]

## No-std

No-std is planned but not yet available.
*/

#[doc(hidden)]
#[rust_analyzer::completions(ignore_flyimport)]
pub mod prelude;

pub mod app;
pub mod geometry;
pub mod layout;
pub mod state;

#[cfg(feature = "winit")]
pub mod winit;

pub mod components;

#[cfg(feature = "wgpu")]
pub mod wgpu;

#[cfg(all(feature = "winit", feature = "wgpu"))]
pub mod winitwgpu;

#[cfg(feature = "tui")]
pub mod tui;
