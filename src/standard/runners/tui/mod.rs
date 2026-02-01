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
use futures::FutureExt;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use vek::{Rect, Rgba};
use crate::app::backend::futures::AsyncExecutor;
use crate::app::composition::reify::Reify;

#[pin_project(project=TUIBackendProj)]
pub struct TUIRunner<N> where N: Send + Reify<(), Output: Element> {
    #[pin]
    pub app: Arc<Mutex<N::Output>>,
}

impl<App> Runner for TUIRunner<App>
where
    App: Send + Reify<(), Output: Element>,
{
    type App = App;

    fn run(app: Self::App) {
        let runtime_app = app.reify(&mut ());
        //let runtime_app = Arc::new(Mutex::new(runtime_app));

        /* Spawn a new thread that polls the signals... Not sure how ideal this is... */
        /*let executor = AsyncExecutor::new(runtime_app.clone());
        let _join_handle = std::thread::spawn(|| {
            futures::executor::block_on(executor);
        });*/

        async_std::task::block_on(Self::app_loop(runtime_app).map(|r| r.unwrap()));
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

pub trait Element: Send + Bubble<Event, bool> + Pollable<()> {
    fn setup(&mut self);
    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>;
}
