use futures_signals::signal::{Map, Signal};
use pin_project::pin_project;
use std::{mem::MaybeUninit, task::Poll};

use super::node::UINode;

/// UI Node that reacts to a signal and updates part of the UI tree.
pub struct React<S, T>
where
    S: Signal<Item = T>,
    T: UINode,
{
    signal: Hold<S, T>,
}

impl<S, T> UINode for React<S, T>
where
    S: Signal<Item = T>,
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;
}

pub trait UISignalExt: Signal {
    fn into_ui(self) -> React<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: UINode,
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
