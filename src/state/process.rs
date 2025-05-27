use crate::{
    app::node::{AppItem, AppItemDescriptor, UIEvent},
    winitwgpu::pipeline::graphics::RenderGraphicDescriptor,
};
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::future::Future;
use std::{pin::Pin, task::Poll};

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: AppItem + Send,
{
    #[pin]
    pub(crate) signal: HoldSignal<S, T>,
}

impl<S: Signal<Item = T> + Send, T: AppItem> Signal for SignalProcessor<S, T> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<Self::Item>> {
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
    B: AppItem,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
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
                held_item.poll_processors(cx)
            }
            None => poll,
        }
    }
}

impl<S: Send + Sync, T> AppItem for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: AppItem + RenderGraphicDescriptor + Send,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        match &mut self.signal.held_item {
            Some(item) => item.handle_ui_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<()>> {
        self.poll_change(cx)
    }
}

pub trait UISignalExt: Signal {
    /// Transforms this signal into a processable part of the UI tree.
    fn process(self) -> SignalProcessor<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: AppItemDescriptor + Send,
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
    T: AppItem,
{
    #[pin]
    pub(crate) signal: HoldFuture<F, T>,
}

impl<F: Future<Output = T>, T: AppItem> Signal for FutureProcessor<F, T> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<Self::Item>> {
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
    B: AppItem,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let HoldFutureProj { future, held_item } = self.project();

        match held_item {
            Some(held_item) => {
                let held_item = unsafe { Pin::new_unchecked(held_item) };
                held_item.poll_processors(cx)
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

impl<F, T> AppItem for FutureProcessor<F, T>
where
    F: Future<Output = T> + Send,
    T: AppItem + RenderGraphicDescriptor,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        match &mut self.signal.held_item {
            Some(item) => item.handle_ui_event(event),
            None => false, //panic!("Reactor was asked to handle event without being polled first."),
        }
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut std::task::Context) -> Poll<Option<()>> {
        self.poll_change(cx)
    }
}

pub trait UIFutureExt: Future {
    /// Transforms this future into a processable part of the UI tree.
    fn process(self) -> FutureProcessor<Self, Self::Output>
    where
        Self: Sized,
        Self::Output: AppItem,
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
