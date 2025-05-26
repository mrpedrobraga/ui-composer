use futures_signals::signal::{Mutable, Signal, SignalFuture};
use pin_project::pin_project;
use std::future::Future;
use std::{pin::Pin, task::Poll};
use wgpu::{RenderPass, Texture};

use crate::gpu::backend::GPUResources;
use crate::gpu::pipeline::graphics::{RenderGraphic, RenderGraphicDescriptor};
use crate::gpu::pipeline::text::RenderText;
use crate::gpu::pipeline::Renderers;
use crate::gpu::render_target::Render;
use crate::prelude::UIItemDescriptor;
use crate::ui::node::UIEvent;
use crate::ui::node::UIItem;

/// UI Item that processes a signal and updates part of the UI tree whenever it changes.
#[pin_project(project = SignalProcessorProj)]
#[must_use = "Processes are Signals, and therefore do nothing unless polled"]
pub struct SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: UIItem + Send,
{
    #[pin]
    signal: HoldSignal<S, T>,
}

impl<S: Signal<Item = T> + Send, T: UIItem> Signal for SignalProcessor<S, T> {
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
    B: UIItem,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let HoldSignalProj { signal, held_item } = self.project();

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
                held_item.poll_processors(cx)
            }
            None => poll,
        }
    }
}

impl<S, T> RenderGraphicDescriptor for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Render + RenderGraphicDescriptor + Send,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        match &self.signal.held_item {
            Some(item) => item.get_render_rect(),
            None => panic!("Reactor was asked for its render rect before being polled!"),
        }
    }
}
impl<S, T> RenderGraphic for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Render + RenderGraphicDescriptor + Send,
{
    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {
        match &self.signal.held_item {
            Some(item) => item.write_quads(quad_buffer),
            None => panic!("Reactor was drawn (graphics) without being polled first!"),
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
impl<S, T> RenderText for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: Render + Send,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: glyphon::TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        match &self.signal.held_item {
            Some(item) => item.push_text(buffer, bounds, container),
            None => panic!("Reactor was drawn (text) without being polled first!"),
        }
    }
}
impl<S: Send + Sync, T> UIItem for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: UIItem + RenderGraphicDescriptor + Send,
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
        Self::Item: UIItemDescriptor + Send,
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
    T: UIItem,
{
    #[pin]
    signal: HoldFuture<F, T>,
}

impl<F: Future<Output = T>, T: UIItem> Signal for FutureProcessor<F, T> {
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
    B: UIItem,
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
                Poll::Ready(mut v) => {
                    held_item.replace(v);
                    Poll::Ready(Some(()))
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

impl<F, T> RenderGraphicDescriptor for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: Render + RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        match &self.signal.held_item {
            Some(item) => item.get_render_rect(),
            None => panic!("Reactor was asked for its render rect before being polled!"),
        }
    }
}
impl<F, T> RenderGraphic for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: Render + UIItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [crate::prelude::Graphic]) {
        match &self.signal.held_item {
            Some(item) => item.write_quads(quad_buffer),
            None => (),
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
impl<F, T> RenderText for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: Render,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: glyphon::TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        match &self.signal.held_item {
            Some(item) => item.push_text(buffer, bounds, container),
            None => (),
        }
    }
}
impl<F, T> UIItem for FutureProcessor<F, T>
where
    F: Future<Output = T> + Send,
    T: UIItem + RenderGraphicDescriptor,
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
        Self::Output: UIItem,
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
