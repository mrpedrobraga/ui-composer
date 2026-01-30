//! # Applications
//!
//! This module contains two complementary parts: UI and runners.
//!
//! ## UI
//!
//! An application is defined by composing building blocks using functions. For example:
//!
//! ```rust
//! # use ui_composer::backends::wgpu::components::{Button, Label};
//! # use ui_composer::standard::Column;
//! Column(
//!     Label("Click The Button"),
//!     Button("The Button", || println!("I was clicked!"))
//! )
//! ```
//!
//! The components here are functional in the [functional programming](https://en.wikipedia.org/wiki/Functional_programming) sense.
//! They are _pure_ which gives you a few superpowers:
//!
//! 1. **Referential Transparency** - the compiler can inline and optimize your app without worrying whether anything will break. Additionally, the same application can run in multiple runners, like in the GPU or in the terminal.
//! 2. **Tidiness** - your application layout can not change based on outside influence and, thus, you won't worry about glitches of the sort.
//!
//! This does leave a question in the air: if the application is immutable, how can it react to
//! user events, operating system events, window resizing, asynchronous callbacks, etc?
//!
//! ## Runners
//!
//! Whereas your application is immutable and can not do side effects, it can still _express_ effectful ideas.
//! Those are [Algebraic Effects](https://en.wikipedia.org/wiki/Effect_system) which compose naturally as you compose your UI.
//!
//! ```rust
//! # use ui_composer::backends::wgpu::components::{Button, Label};
//! # use ui_composer::standard::Column;
//! # use ui_composer::prelude::*;
//! UIComposer::run( // a runner
//!     Column(
//!         Label("Click The Button"),
//!         Button("The Button", || println!("I was clicked!"))
//!     )
//! )
//! ```
//!
//! In the example above, whenever a user event, for example a mouse click, happens,
//! it will be shown to `Column`, which will show it to `Label` and `Button`.
//!
//! > It's interesting to note that the runner has _no idea_ of the existence of `Label` and `Button`.
//!
//! Button will handle that by triggering the effect described in its
//! second parameter and print `"I was clicked"`.
//!
//! Something similar happens for drawing, where the runner bubbles down a draw request
//! and the components bubble up responses.
//!
//! ### Cross-platform
//!
//! Decoupling the app from rendering and side effects allows the same app to be run by different runners,
//! including runners for different platforms.
//!
//! This means you could run your app in a cross-platform runner like `WinitWgpuRunner`...
//! Or on a web-specific `HTMLCanvasRunner`. The sky is the limit.

pub mod backend;
pub mod building_blocks;

/// User events are one of the things that can be bubbled down an application.
pub mod input;

