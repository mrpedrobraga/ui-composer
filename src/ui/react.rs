use futures_signals::signal::{Map, Signal, SignalExt};
use pin_project::pin_project;
use std::{mem::MaybeUninit, pin::Pin, task::Poll};

use super::{
    layout::LayoutItem,
    node::{LiveUINode, UINode},
};

/// UI Node that reacts to a signal and updates part of the UI tree.
#[pin_project(project = ReactProj)]
pub struct React<S: Send, T>
where
    S: Signal<Item = T>,
    T: LiveUINode,
{
    #[pin]
    signal: Hold<S, T>,
}

impl<S: Send, T> LiveUINode for React<S, T>
where
    S: Signal<Item = T>,
    T: LiveUINode,
{
    fn handle_ui_event(&mut self, event: super::node::UIEvent) -> bool {
        self.signal
            .held_item
            .as_mut()
            .expect("Process the React before trying to handle an event.")
            .handle_ui_event(event)
    }

    fn push_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        self.signal
            .held_item
            .as_ref()
            .expect("Process the React before trying to render it.")
            .push_quads(quad_buffer)
    }

    fn poll_reactivity_change(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<()>> {
        self.poll_change(cx)
    }
}

impl<S: Send, T> UINode for React<S, T>
where
    S: Signal<Item = T>,
    T: UINode,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        self.signal
            .held_item
            .as_ref()
            .expect("Process the React before trying to get its render rect.")
            .get_render_rect()
    }
}

pub trait UISignalExt: Signal + Send {
    fn into_ui(self) -> React<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: LiveUINode,
    {
        React {
            signal: Hold {
                signal: self,
                held_item: None,
            },
        }
    }
}
impl<T: Send> UISignalExt for T where T: Signal {}

#[pin_project(project = HoldProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct Hold<A, B>
where
    A: Signal<Item = B>,
{
    #[pin]
    signal: A,
    pub held_item: Option<B>,
}

impl<A, B> Signal for Hold<A, B>
where
    A: Signal<Item = B>,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let HoldProj { signal, held_item } = self.project();

        match signal.poll_change(cx) {
            Poll::Ready(Some(v)) => {
                *held_item = Some(v);
                Poll::Ready(Some(()))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<S: Signal<Item = T> + Send, T: LiveUINode> Signal for React<S, T> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<Self::Item>> {
        let ReactProj { signal } = self.project();
        signal.poll_change(cx)
    }
}
