use futures_signals::signal::{Map, Signal, SignalExt};
use pin_project::pin_project;
use std::{mem::MaybeUninit, pin::Pin, task::Poll};

use super::{
    layout::LayoutItem,
    node::{LiveUINode, UINode},
};

/// UI Node that reacts to a signal and updates part of the UI tree.
#[pin_project(project = ReactProj)]
#[must_use = "Reactors are Signals, and therefore do nothing unless polled"]
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
    T: UINode,
{
    fn handle_ui_event(&mut self, event: super::node::UIEvent) -> bool {
        match &mut self.signal.held_item {
            Some(item) => item.handle_ui_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }

    fn push_quads(&self, quad_buffer: &mut [crate::prelude::Quad]) {
        match &self.signal.held_item {
            Some(item) => item.push_quads(quad_buffer),
            None => panic!("Reactor was drawn without being polled first!"),
        }
    }

    fn poll_reactivity_change(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<()>> {
        self.poll_change(cx)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl<S: Send, T> UINode for React<S, T>
where
    S: Signal<Item = T>,
    T: UINode,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        match &self.signal.held_item {
            Some(item) => item.get_render_rect(),
            None => panic!("Reactor was asked for its render rect before being polled!"),
        }
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
    B: LiveUINode,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let HoldProj { signal, held_item } = self.project();

        let poll = match signal.poll_change(cx) {
            Poll::Ready(Some(mut v)) => {
                held_item.replace(v);
                Poll::Ready(Some(()))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        };

        match held_item {
            Some(held_item) => {
                let held_item = unsafe { Pin::new_unchecked(held_item) };
                held_item.poll_reactivity_change(cx)
            }
            None => poll,
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
