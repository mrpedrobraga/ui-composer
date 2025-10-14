use crate::app::primitives::{PrimitiveDescriptor, Processor};
use crate::layout::{LayoutItem, ParentHints};
use crate::state::signal_ext::coalesce_polls;
use core::task::Context;
use vek::Extent2;
use {
    crate::app::{input::Event, primitives::Primitive},
    core::{future::Future, pin::Pin, task::Poll},
    futures_signals::signal::Signal,
    pin_project::pin_project,
};

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalProcessor<Sig, Resources>
where
    Sig: Signal,
    Sig::Item: PrimitiveDescriptor<Resources>,
{
    #[pin]
    signal: Sig,
    pub held_item: Option<<Sig::Item as PrimitiveDescriptor<Resources>>::Primitive>,
}

impl<Sig, Res> Primitive<Res> for SignalProcessor<Sig, Res>
where
    Sig: Signal + Send,
    Sig::Item: PrimitiveDescriptor<Res>,
    <Sig::Item as PrimitiveDescriptor<Res>>::Primitive: Processor<Res>,
{
    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.held_item {
            Some(item) => item.handle_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }
}

impl<Sig, Res> Processor<Res> for SignalProcessor<Sig, Res>
where
    Sig: Signal + Send,
    Sig::Item: PrimitiveDescriptor<Res>,
    <Sig::Item as PrimitiveDescriptor<Res>>::Primitive: Processor<Res>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        let SignalProcessorProj { signal, held_item } = self.project();

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
pub struct React<Sig>(pub Sig)
where
    Sig: Signal;

impl<Res, Sig> PrimitiveDescriptor<Res> for React<Sig>
where
    Sig: Signal + Send,
    Sig::Item: PrimitiveDescriptor<Res>,
{
    type Primitive = SignalProcessor<Sig, Res>;

    fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
        SignalProcessor {
            signal: self.0,
            // This is initially `None` and it should reify its content when
            // `poll` is called!
            held_item: None,
        }
    }
}

/// A wrapper for [Future] that allows it to interact with UI Composer.
///
/// This wrapper is necessary as a technical limitation.
pub struct Await<Fut>(pub Fut)
where
    Fut: Future;

impl<Fut, Res> PrimitiveDescriptor<Res> for Await<Fut>
where
    Fut: Future + Send,
    Fut::Output: PrimitiveDescriptor<Res>,
{
    type Primitive = FutureProcessor<Fut, Res>;

    fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
        FutureProcessor {
            future: self.0,
            // Held item is empty up to the first poll...
            held_item: None,
        }
    }
}

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = FutureProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct FutureProcessor<Fut, Resources>
where
    Fut: Future,
    Fut::Output: PrimitiveDescriptor<Resources>,
{
    #[pin]
    future: Fut,
    pub held_item: Option<<Fut::Output as PrimitiveDescriptor<Resources>>::Primitive>,
}

impl<Fut, Res> Primitive<Res> for FutureProcessor<Fut, Res>
where
    Fut: Future + Send,
    Fut::Output: PrimitiveDescriptor<Res>,
{
    fn handle_event(&mut self, event: Event) -> bool {
        self.held_item
            .as_mut()
            .map(|item| item.handle_event(event))
            .unwrap_or(false)
    }
}

impl<Fut, Res> Processor<Res> for FutureProcessor<Fut, Res>
where
    Fut: Future + Send,
    Fut::Output: PrimitiveDescriptor<Res>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        let FutureProcessorProj { future, held_item } = self.project();

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

impl<Fut, Res> LayoutItem for FutureProcessor<Fut, Res>
where
    Fut: Future + Send,
    Fut::Output: PrimitiveDescriptor<Res>,
    <Fut::Output as PrimitiveDescriptor<Res>>::Primitive: LayoutItem,
{
    type Content =
        Option<<<Fut::Output as PrimitiveDescriptor<Res>>::Primitive as LayoutItem>::Content>;

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
