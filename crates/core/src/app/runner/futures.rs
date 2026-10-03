//! # Async
//!
//! An application is defined at compile-time, statically. But your program might contain values which
//! are made available only at run-time ([`Future`] or IO) or that repeatedly change along
//! the duration of the program ([`Signal`]).
//!
//! In either case, the UI might want to "React" to it.
//!
//! In this crate, the description of how reactive UI reacts to signal changes is done with
//! code from [`crate::state`]. But, like `Future`s need executors, this crate offers "App executors",
//! which can poll the app's futures and signals.

use crate::app::composition::elements::Environment;
use crate::app::composition::layout::Ui;
use crate::app::composition::modules::RenderModule;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::ops::DerefMut;
use std::pin::Pin;
use std::task::{Context, Poll};

type Own<A> = std::sync::Arc<futures::lock::Mutex<A>>;

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=UiPollSignalProj)]
pub struct RenderModulePoller<'exec, Env: Environment, U: Ui<Env>, Callback: FnMut()> {
    #[pin]
    render_module: Own<RenderModule<Env, U>>,
    blueprint_resources: Env::BlueprintResources<'exec>,
    has_yet_to_yield: bool,
    callback: Callback,
}

impl<'exec, Env: Environment, U: Ui<Env>, Callback: FnMut()>
    RenderModulePoller<'exec, Env, U, Callback>
{
    pub fn new(
        render_module: Own<RenderModule<Env, U>>,
        environment: Env::BlueprintResources<'exec>,
        callback: Callback,
    ) -> Self {
        RenderModulePoller {
            render_module,
            blueprint_resources: environment,
            has_yet_to_yield: true,
            callback,
        }
    }
}

impl<'exec, Env: Environment, U: Ui<Env>, Callback: FnMut()> Signal
    for RenderModulePoller<'exec, Env, U, Callback>
{
    type Item = ();

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>> {
        let UiPollSignalProj {
            render_module: element,
            blueprint_resources,
            has_yet_to_yield,
            callback,
        } = self.project();

        let has_yet_to_yield2 = *has_yet_to_yield;
        
        if let Some(mut element_borrow) = element.try_lock() {
            let pinned_element =
                unsafe { Pin::new_unchecked(element_borrow.deref_mut()) };

            // Because of how signals work internally, we must yield at least once.
            let element_poll = pinned_element.poll_ui_change(cx, blueprint_resources);

            if element_poll.is_pending() { 
                return Poll::Pending;
            };
            if let Poll::Ready(None) = element_poll
                && !has_yet_to_yield2
            {
                return Poll::Ready(None);
            }

            if let Poll::Ready(Some(())) = element_poll {
                (callback)();
            }

            *has_yet_to_yield = false;
            Poll::Ready(Some(()))
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
