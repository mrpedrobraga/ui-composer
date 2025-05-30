use crate::app::primitives::Processor;
use crate::layout::{LayoutItem, ParentHints};
use core::task::Context;
use vek::Extent2;
use {
    crate::app::{
        input::Event,
        primitives::{Primitive, PrimitiveDescriptor},
    },
    core::{future::Future, pin::Pin, task::Poll},
    futures_signals::signal::Signal,
    pin_project::pin_project,
};

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Processor,
{
    #[pin]
    pub(crate) signal: HoldSignal<S, T>,
}

impl<S: Signal<Item = T> + Send, T: Processor> Signal for SignalProcessor<S, T> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let SignalProcessorProj { signal } = self.project();
        signal.poll_change(cx)
    }
}

#[pin_project(project = HoldSignalProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct HoldSignal<A, B>
where
    A: Signal<Item = B>,
{
    #[pin]
    signal: A,
    pub held_item: Option<B>,
}

impl<A, B> Signal for HoldSignal<A, B>
where
    A: Signal<Item = B>,
    B: Processor,
{
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let HoldSignalProj { signal, held_item } = self.project();

        let poll = match signal.poll_change(cx) {
            Poll::Ready(Some(v)) => {
                held_item.replace(v);
                Poll::Ready(Some(()))
            }
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        };

        match held_item {
            Some(held_item) => {
                let held_item = unsafe { Pin::new_unchecked(held_item) };
                held_item.poll(cx)
            }
            None => poll,
        }
    }
}

impl<S: Send + Sync, T> Primitive for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Primitive + Send,
{
    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.signal.held_item {
            Some(item) => item.handle_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }
}

impl<S: Send + Sync, T> Processor for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Primitive + Send,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        self.poll_change(cx)
    }
}

pub trait UISignalExt: Signal {
    /// Transforms this signal into a processable part of the UI tree.
    fn process(self) -> SignalProcessor<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: PrimitiveDescriptor + Send,
    {
        SignalProcessor {
            signal: HoldSignal {
                signal: self,
                held_item: None,
            },
        }
    }
}
impl<T> UISignalExt for T where T: Signal {}

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = FutureProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct FutureProcessor<F, T>
where
    F: Future<Output = T>,
{
    #[pin]
    pub(crate) signal: HoldFuture<F, T>,
}

impl<T, F> Signal for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: Processor,
{
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let FutureProcessorProj { signal } = self.project();
        signal.poll_change(cx)
    }
}

#[pin_project(project = HoldFutureProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct HoldFuture<A, B>
where
    A: Future<Output = B>,
{
    #[pin]
    future: A,
    pub held_item: Option<B>,
}

impl<A, B> Signal for HoldFuture<A, B>
where
    A: Future<Output = B>,
    B: Processor,
{
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let HoldFutureProj { future, held_item } = self.project();

        match held_item {
            Some(held_item) => {
                let held_item = unsafe { Pin::new_unchecked(held_item) };
                held_item.poll(cx)
            }
            None => match future.poll(cx) {
                Poll::Ready(v) => {
                    held_item.replace(v);
                    Poll::Ready(Some(()))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

impl<F, T> Primitive for FutureProcessor<F, T>
where
    F: Future<Output = T> + Send,
    T: Primitive,
{
    fn handle_event(&mut self, event: Event) -> bool {
        match &mut self.signal.held_item {
            Some(item) => item.handle_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }
}

impl<F, T> Processor for FutureProcessor<F, T>
where
    F: Future<Output = T> + Send,
    T: Primitive,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        self.poll_change(cx)
    }
}

impl<F, T> LayoutItem for FutureProcessor<F, T>
where
    F: Future<Output = T> + Send,
    T: LayoutItem,
{
    type Content = Option<T::Content>;

    fn get_natural_size(&self) -> Extent2<f32> {
        match &self.signal.held_item {
            None => Extent2::zero(),
            Some(item) => item.get_natural_size(),
        }
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        match &self.signal.held_item {
            None => Extent2::zero(),
            Some(item) => item.get_natural_size(),
        }
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        self.signal
            .held_item
            .as_mut()
            .map(|held_item| held_item.lay(parent_hints))
    }
}
pub trait UIFutureExt: Future {
    /// Transforms this future into a processable part of the UI tree.
    fn process(self) -> FutureProcessor<Self, Self::Output>
    where
        Self: Sized,
        Self::Output: Primitive,
    {
        FutureProcessor {
            signal: HoldFuture {
                future: self,
                held_item: None,
            },
        }
    }
}
impl<T> UIFutureExt for T where T: Future + Send {}
