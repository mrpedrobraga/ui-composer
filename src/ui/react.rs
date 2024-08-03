use futures_signals::signal::{Map, Signal};
use pin_project::pin_project;
use std::{mem::MaybeUninit, task::Poll};

use super::node::{LiveUINode, UINode};

/// UI Node that reacts to a signal and updates part of the UI tree.
pub struct React<S, T>
where
    S: Signal<Item = T>,
    T: LiveUINode,
{
    signal: Hold<S, T>,
}

impl<S, T> LiveUINode for React<S, T>
where
    S: Signal<Item = T>,
    T: LiveUINode,
{
    fn handle_ui_event(&mut self, event: super::node::UIEvent) -> bool {
        // This should be safe if you process the React before sending any events...
        // But maaaaybe I should change this MaybeUninit to an Option?
        let inner = unsafe { self.signal.held_item.assume_init_mut() };
        inner.handle_ui_event(event)
    }
}

impl<S, T> UINode for React<S, T>
where
    S: Signal<Item = T>,
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        // This should be safe if you only ever do this after the React is polled once (yielding a value).
        // If you need to run this before the React receives a value, then, well, change the MaybeUninit to an Option.
        unsafe { self.signal.held_item.assume_init_ref() }.get_render_rect()
    }
}

pub trait UISignalExt: Signal {
    fn into_ui(self) -> React<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: LiveUINode,
    {
        React {
            signal: Hold {
                signal: self,
                held_item: MaybeUninit::uninit(),
            },
        }
    }
}
impl<T> UISignalExt for T where T: Signal {}

#[pin_project(project = HoldProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct Hold<A, B>
where
    A: Signal<Item = B>,
{
    #[pin]
    signal: A,
    pub held_item: MaybeUninit<B>,
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
                held_item.write(v);
                Poll::Ready(Some(()))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}
