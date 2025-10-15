use crate::app::building_blocks::Reifiable;
use crate::layout::LayoutItem;
use crate::layout::hints::ParentHints;
use crate::state::signal_ext::coalesce_polls;
use core::task::Context;
use vek::Extent2;
use {
    crate::app::{building_blocks::BuildingBlock, input::Event},
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
        Poll::Ready(Some(()))
    }
}

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalReactItemReProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalReactItemRe<Sig, Cx>
where
    Sig: Signal,
    Sig::Item: Reifiable<Cx>,
{
    #[pin]
    signal: Sig,
    pub held_item: Option<<Sig::Item as Reifiable<Cx>>::Reified>,
}

impl<Sig, Cx> BuildingBlock<Cx> for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reifiable<Cx>,
    <Sig::Item as Reifiable<Cx>>::Reified: Pollable<Cx>,
{
    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.held_item {
            Some(item) => item.handle_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }
}

impl<Sig, Cx> Pollable<Cx> for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reifiable<Cx>,
    <Sig::Item as Reifiable<Cx>>::Reified: Pollable<Cx>,
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

        coalesce_polls(signal_poll, inner_poll)
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

impl<Cx, Sig> Reifiable<Cx> for SignalReactItem<Sig>
where
    Sig: Signal + Send,
    Sig::Item: Reifiable<Cx>,
{
    type Reified = SignalReactItemRe<Sig, Cx>;

    fn reify(self, #[expect(unused)] context: &mut Cx) -> Self::Reified {
        SignalReactItemRe {
            signal: self.0,
            held_item: None,
        }
    }
}

impl<Sig, Cx> LayoutItem for SignalReactItemRe<Sig, Cx>
where
    Sig: Signal + Send,
    Sig::Item: Reifiable<Cx>,
    <Sig::Item as Reifiable<Cx>>::Reified: LayoutItem,
{
    type Content = Option<<<Sig::Item as Reifiable<Cx>>::Reified as LayoutItem>::Content>;

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

impl<Fut, Res> Reifiable<Res> for FutureAwaitItem<Fut>
where
    Fut: Future + Send,
    Fut::Output: Reifiable<Res>,
{
    type Reified = FutureAwaitItemRe<Fut, Res>;

    fn reify(self, #[expect(unused)] context: &mut Res) -> Self::Reified {
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
    Fut::Output: Reifiable<Cx>,
{
    #[pin]
    future: Fut,
    pub held_item: Option<<Fut::Output as Reifiable<Cx>>::Reified>,
}

impl<Fut, Cx> BuildingBlock<Cx> for FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future + Send,
    Fut::Output: Reifiable<Cx>,
{
    fn handle_event(&mut self, event: Event) -> bool {
        self.held_item
            .as_mut()
            .map(|item| item.handle_event(event))
            .unwrap_or(false)
    }
}

impl<Fut, Cx> Pollable<Cx> for FutureAwaitItemRe<Fut, Cx>
where
    Fut: Future + Send,
    Fut::Output: Reifiable<Cx>,
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
    Fut::Output: Reifiable<Cx>,
    <Fut::Output as Reifiable<Cx>>::Reified: LayoutItem,
{
    type Content = Option<<<Fut::Output as Reifiable<Cx>>::Reified as LayoutItem>::Content>;

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
