use crate::gpu::pipeline::Renderers;
use crate::state::signal_ext::coalesce_polls;
use crate::{gpu::backend::GPUResources, ui::graphics::Graphic};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::Rect;
use wgpu::{RenderPass, Texture};

pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction.
/// In practice, any single UI node should handle as little as possible.
/// The entire user interface is made of UI Nodes arranged in a graph.
pub trait UIItem: Send {
    /// Handles a UI Event (or not). Returns whether the event was handled.
    #[inline(always)]
    fn handle_ui_event(&mut self, event: UIEvent) -> bool;

    /// Pushes quads to a quad buffer slice.
    #[inline(always)]
    fn write_quads(&self, quad_buffer: &mut [Graphic]);

    #[inline(always)]
    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,
        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
    }

    /// TODO: Remove this when using generics on the engine?
    fn get_quad_count(&self) -> usize;

    /// Polls this node's processors: `Future`s and `Signal`s.
    #[inline(always)]
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

/// Trait to get compile-time information about a UI Item.
pub trait ItemDescriptor: UIItem {
    /// The amount of primitives this UI Item will have when drawing.
    const QUAD_COUNT: usize;

    /// Gets the rectangle this primitive occupies, for rendering purposes.
    #[inline(always)]
    fn get_render_rect(&self) -> Option<Rect<f32, f32>>;
}

impl UIItem for () {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        false
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        /* No quads to write */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}

impl ItemDescriptor for () {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl<T> UIItem for Option<T>
where
    T: ItemDescriptor,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_ui_event(event))
            .unwrap_or(false)
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Some(inner) => inner.write_quads(quad_buffer),
            None => {
                for idx in 0..Self::QUAD_COUNT {
                    quad_buffer[idx] = Graphic::default()
                }
            }
        }
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,

        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        match self {
            Some(inner) => inner.prepare(gpu_resources, pipelines, render_pass, texture),
            None => {}
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll_processors(cx))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<T> ItemDescriptor for Option<T>
where
    T: ItemDescriptor,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().and_then(ItemDescriptor::get_render_rect)
    }
}

impl<T, E> UIItem for Result<T, E>
where
    T: ItemDescriptor,
    E: ItemDescriptor,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        match self {
            Ok(v) => v.handle_ui_event(event),
            Err(e) => e.handle_ui_event(event),
        }
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Ok(v) => v.write_quads(quad_buffer),
            Err(e) => e.write_quads(quad_buffer),
        }
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,

        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        match self {
            Ok(v) => v.prepare(gpu_resources, pipelines, render_pass, texture),
            Err(e) => e.prepare(gpu_resources, pipelines, render_pass, texture),
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // let this: &mut Self = self.deref_mut();
        // match this {
        //     Ok(v) => todo!(),
        //     Err(e) => todo!(),
        // }
        unimplemented!()
    }
}

impl<T, E> ItemDescriptor for Result<T, E>
where
    T: ItemDescriptor,
    E: ItemDescriptor,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match self {
            Ok(v) => v.get_render_rect(),
            Err(e) => e.get_render_rect(),
        }
    }
}

impl<T> UIItem for Box<T>
where
    T: ItemDescriptor,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut().handle_ui_event(event)
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        self.as_ref().write_quads(quad_buffer)
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,
        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        self.as_mut()
            .prepare(gpu_resources, pipelines, render_pass, texture)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll_processors(cx)
    }
}

impl<T> ItemDescriptor for Box<T>
where
    T: ItemDescriptor,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

impl<A, B> UIItem for (A, B)
where
    A: ItemDescriptor,
    B: ItemDescriptor,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let a_handled = self.0.handle_ui_event(event.clone());
        let b_handled = self.1.handle_ui_event(event);

        a_handled || b_handled
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        let (slice_a, slice_b) = quad_buffer.split_at_mut(A::QUAD_COUNT);
        self.0.write_quads(slice_a);
        self.1.write_quads(slice_b);
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,

        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        self.0
            .prepare(gpu_resources, pipelines, render_pass, texture);
        self.1
            .prepare(gpu_resources, pipelines, render_pass, texture);
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut mut_ref = unsafe { self.get_unchecked_mut() };
            let (ref mut a, ref mut b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_processors(cx);
        let poll_b = pinned_b.poll_processors(cx);

        coalesce_polls(poll_a, poll_b)
    }
}

impl<A, B> ItemDescriptor for (A, B)
where
    A: ItemDescriptor,
    B: ItemDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT + B::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match (self.0.get_render_rect(), self.1.get_render_rect()) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.union(b)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SizedVec<T, const N: usize> {
    inner: Vec<T>,
}
impl<A, const N: usize> FromIterator<A> for SizedVec<A, N> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        SizedVec {
            inner: iter.into_iter().take(N).collect(),
        }
    }
}
impl<A: ItemDescriptor, const N: usize> UIItem for SizedVec<A, N> {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let mut any_handled = false;
        for item in self.inner.iter_mut() {
            any_handled = item.handle_ui_event(event.clone()) || any_handled;
        }
        any_handled
    }

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        if self.inner.len() == 0 {
            return;
        }

        for idx in 0..N {
            self.inner[idx]
                .write_quads(&mut quad_buffer[(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)])
        }
    }

    fn prepare<'pass>(
        &'pass mut self,
        gpu_resources: &'pass GPUResources,
        pipelines: &'pass Renderers,

        render_pass: &mut RenderPass<'pass>,
        texture: &Texture,
    ) {
        self.inner
            .iter_mut()
            .for_each(|item| item.prepare(gpu_resources, pipelines, render_pass, texture));
    }

    fn get_quad_count(&self) -> usize {
        N * A::QUAD_COUNT
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let mut poll_acc = Poll::Pending;
        let this = unsafe { self.get_unchecked_mut() };
        for idx in 0..N {
            let item = unsafe { Pin::new_unchecked(&mut this.inner[idx]) };
            let item_poll = item.poll_processors(cx);
            poll_acc = coalesce_polls(poll_acc, item_poll)
        }
        poll_acc
    }
}
impl<A: ItemDescriptor, const N: usize> ItemDescriptor for SizedVec<A, N> {
    const QUAD_COUNT: usize = N * A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        let mut iterator = self.inner.iter();
        let first = iterator.next()?.get_render_rect();
        iterator.fold(first, |acc, item| Some(acc?.union(item.get_render_rect()?)))
    }
}
