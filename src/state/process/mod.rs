use crate::app::composition::algebra::{Bubble, Semigroup};
use crate::app::composition::reify::Reify;
use crate::geometry::layout::hints::ParentHints;
use crate::geometry::layout::LayoutItem;
use core::task::Context;
use std::cell::RefCell;
use std::rc::Rc;
use vek::Extent2;
use {
    crate::app::input::Event,
    core::{future::Future, pin::Pin, task::Poll},
    futures_signals::signal::Signal,
    pin_project::pin_project,
};

/// A trait representing a [BuildingBlock] or [Node] that _might_
/// process internal [Future]s or [Signal]s.
#[must_use = "processors are lazy and do nothing unless polled"]
pub trait Pollable<Resources>: Send {
    /// Recursively polls this primitive's inner processes (`Future`s and `Signal`s).
    fn poll(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] resources: &mut Resources,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

pub trait PollableExt<R>: Pollable<R> {
    fn once<'a>(&'a mut self, resources: &'a mut R) -> PollOnce<'a, Self, R>
    where Self: Sized
    {
        PollOnce { pollable: self, resources }
    }
}

impl<T: Pollable<R>, R> PollableExt<R> for T {}

pub struct PollOnce<'a, P, R> {
    pollable: &'a mut P,
    resources: &'a mut R,
}

impl<'a, P, R> Future for PollOnce<'a, P, R>
where P: Pollable<R>
{
    type Output = Option<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let pin = unsafe { Pin::new_unchecked(&mut *this.pollable) };
        pin.poll(cx, this.resources)
    }
}

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalReactItemReProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalReactItemRe<Sig, Cx>
where
    Sig: Signal,
    Sig::Item: Reify<Cx>,
{
    #[pin]
    signal: Sig,
    pub held_item: Option<<Sig::Item as Reify<Cx>>::Output>,
}

impl<Sig, Cx> Bubble<Event, bool> for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reify<Cx>,
    <Sig::Item as Reify<Cx>>::Output: Bubble<Event, bool>,
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        match &mut self.held_item {
            Some(item) => item.bubble(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }
}

impl<Sig, Cx> Pollable<Cx> for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reify<Cx>,
    <Sig::Item as Reify<Cx>>::Output: Pollable<Cx>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Cx) -> Poll<Option<()>> {
        let SignalReactItemReProj { signal, held_item } = self.project();

        let signal_poll = signal.poll_change(cx);

        let signal_poll = match signal_poll {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(item_descriptor)) => {
                *held_item = Some(item_descriptor.reify(resources));
                Poll::Ready(Some(()))
            }
        };

        let inner_poll = if let Some(held_item) = held_item {
            let held_item = unsafe { Pin::new_unchecked(held_item) };
            held_item.poll(cx, resources)
        } else {
            Poll::Pending
        };

        signal_poll.combine(inner_poll)
    }
}

/// A wrapper for [Signal] that allows it to interact with UI Composer.
///
/// This wrapper is necessary as a technical limitation.
#[allow(non_snake_case)]
pub fn React<Sig>(signal: Sig) -> SignalReactItem<Sig>
where
    Sig: Signal + Send,
{
    SignalReactItem(signal)
}

pub struct SignalReactItem<Sig>(pub Sig)
where
    Sig: Signal;

impl<Cx, Sig> Reify<Cx> for SignalReactItem<Sig>
where
    Sig: Signal + Send,
    Sig::Item: Reify<Cx>,
{
    type Output = SignalReactItemRe<Sig, Cx>;

    fn reify(self, #[expect(unused)] context: &mut Cx) -> Self::Output {
        SignalReactItemRe {
            signal: self.0,
            held_item: None,
        }
    }
}

impl<Sig, Cx> LayoutItem for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reify<Cx>,
    <Sig::Item as Reify<Cx>>::Output: LayoutItem,
{
    type Content = Option<<<Sig::Item as Reify<Cx>>::Output as LayoutItem>::Content>;

    #[allow(deprecated)]
    fn get_natural_size(&self) -> Extent2<f32> {
        self.held_item
            .as_ref()
            .map(|item| item.get_natural_size())
            .unwrap_or(Extent2::zero())
    }

    #[allow(deprecated)]
    fn get_minimum_size(&self) -> Extent2<f32> {
        self.held_item
            .as_ref()
            .map(|item| item.get_minimum_size())
            .unwrap_or(Extent2::zero())
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        self.held_item
            .as_mut()
            .map(|held_item| held_item.lay(parent_hints))
    }
}

/// A wrapper for [Future] that allows it to interact with UI Composer.
///
/// This wrapper is necessary as a technical limitation.
#[allow(non_snake_case)]
pub fn Await<Fut>(future: Fut) -> FutureAwaitItem<Fut>
where
    Fut: Future,
{
    FutureAwaitItem(future)
}

pub struct FutureAwaitItem<Fut>(pub Fut)
where
    Fut: Future;

impl<Fut, Res> Reify<Res> for FutureAwaitItem<Fut>
where
    Fut: Future + Send,
    Fut::Output: Reify<Res>,
{
    type Output = FutureAwaitItemRe<Fut, Res>;

    fn reify(self, #[expect(unused)] context: &mut Res) -> Self::Output {
        FutureAwaitItemRe {
            future: self.0,
            // Held item is empty up to the first poll...
            held_item: None,
        }
    }
}

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = FutureAwaitItemReProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future,
    Fut::Output: Reify<Cx>,
{
    #[pin]
    future: Fut,
    pub held_item: Option<<Fut::Output as Reify<Cx>>::Output>,
}

impl<Fut, Cx> Bubble<Event, bool> for FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future + Send,
    Fut::Output: Reify<Cx, Output: Bubble<Event, bool>>,
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        self.held_item
            .as_mut()
            .map(|item| item.bubble(event))
            .unwrap_or(false)
    }
}

impl<Fut, Cx> Pollable<Cx> for FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future + Send,
    Fut::Output: Reify<Cx, Output: Pollable<Cx>>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Cx) -> Poll<Option<()>> {
        let FutureAwaitItemReProj { future, held_item } = self.project();

        match held_item {
            Some(held_item) => {
                let held_item = unsafe { Pin::new_unchecked(held_item) };
                held_item.poll(cx, resources)
            }
            None => match future.poll(cx) {
                Poll::Ready(descriptor) => {
                    let item = descriptor.reify(resources);
                    held_item.replace(item);
                    Poll::Ready(Some(()))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

impl<Fut, Cx> LayoutItem for FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future + Send,
    Fut::Output: Reify<Cx>,
    <Fut::Output as Reify<Cx>>::Output: LayoutItem,
{
    type Content = Option<<<Fut::Output as Reify<Cx>>::Output as LayoutItem>::Content>;

    #[allow(deprecated)]
    fn get_natural_size(&self) -> Extent2<f32> {
        self.held_item
            .as_ref()
            .map(|item| item.get_natural_size())
            .unwrap_or(Extent2::zero())
    }

    #[allow(deprecated)]
    fn get_minimum_size(&self) -> Extent2<f32> {
        self.held_item
            .as_ref()
            .map(|item| item.get_minimum_size())
            .unwrap_or(Extent2::zero())
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        self.held_item
            .as_mut()
            .map(|held_item| held_item.lay(parent_hints))
    }
}
