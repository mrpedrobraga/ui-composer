//! # TUI
//!
//! This module contains a [`TUIRunner`] that can run applications in a terminal.

use futures_signals::signal::SignalExt;
pub mod app_loop;
pub mod nodes;
pub mod render;
pub mod signals;

pub use nodes::Terminal;
pub use render::Graphic;

use crate::app::backend::{AppContext, Runner};
use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::runners::tui::render::Canvas;
use crate::state::process::Pollable;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};
use vek::{Rect, Rgba};
use crate::app::composition::reify::Reify;
use futures::FutureExt;
use crate::runners::tui::signals::AsyncExecutor;

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
        Self::grab_terminal().unwrap();

         std::thread::scope(|scope| {
            let runtime_app = app.reify(&mut ());
            let runtime_app = Arc::new(Mutex::new(runtime_app));
            
            /* Handle Async and Reactivity */
            let r_app = runtime_app.clone();
            let _executor_thread = scope.spawn(|| {
                futures::executor::block_on(AsyncExecutor::new(r_app).to_future());
            });

            /* Handle Events */
            let _event_thread = scope.spawn(|| {
                async_std::task::block_on(Self::process_events(runtime_app).map(|r| r.unwrap()));
            });
        });

        Self::release_terminal().unwrap();
        dbg!("Threads returned!");
    }

    fn process(
        self: Pin<&mut Self>,
        cx: &mut Context,
        _resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        let TUIBackendProj { app, .. } = self.project();

        let mut app = app.lock().unwrap();
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