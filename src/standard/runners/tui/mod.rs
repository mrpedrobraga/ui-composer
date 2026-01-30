//! # TUI
//!
//! This module contains a [`TUIRunner`] that can run applications in a terminal.

pub mod app_loop;
pub mod nodes;
pub mod render;

pub use nodes::Terminal;
pub use render::Graphic;

use crate::app::backend::{AppContext, Runner};
use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::runners::tui::render::Canvas;
use crate::state::process::Pollable;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use futures::FutureExt;
use pin_project::pin_project;
use spin::Mutex;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use vek::{Rect, Rgba};

#[pin_project(project=TUIBackendProj)]
pub struct TUIRunner<N: Node> {
    #[pin]
    pub app: Arc<Mutex<N::Output>>,
}

impl<App> Runner for TUIRunner<App>
where
    App: Node,
{
    type App = App;

    fn run(app: Self::App) {
        enable_raw_mode().expect("Couldn't enable raw mode");

        let node_tree = app.reify();
        // TODO: Reflect on whether this should be blocking. That's most likely the case.
        async_std::task::block_on(Self::app_loop(node_tree).map(|r| r.unwrap()));

        disable_raw_mode().expect("Couldn't disable raw mode.")
    }

    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        _resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        let TUIBackendProj { app, .. } = self.project();

        let mut app = app.lock();
        let app = DerefMut::deref_mut(&mut app);
        let app = unsafe { Pin::new_unchecked(app) };

        app.poll(cx, &mut ())
    }
}

pub trait Node: Send {
    type Output: NodeRe;
    fn reify(self) -> Self::Output;
}

pub trait NodeRe: Send + Bubble<Event, bool> + Pollable<()> {
    fn setup(&mut self);
    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>;
}
