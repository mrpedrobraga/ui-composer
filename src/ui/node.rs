use crate::ui::graphics::Quad;
use std::{
    ops::{Deref, DerefMut},
    pin::{pin, Pin},
    task::{Context, Poll},
};
use vek::Rect;

pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction.
/// In practice, any single UI node should handle as little as possible.
/// The entire user interface is made of UI Nodes arranged in a graph.
pub trait LiveUINode: Send {
    /// Handles an UI Event (or not). Returns whether the event was handled.
    #[inline(always)]
    fn handle_ui_event(&mut self, event: UIEvent) -> bool;

    /// Pushes quads to a quad buffer slice.
    #[inline(always)]
    fn push_quads(&self, quad_buffer: &mut [Quad]);

    /// TODO: Remove this when using generics on the engine?
    fn get_quad_count(&self) -> usize;

    /// Polls this node's processors: `Future`s and `Signal`s.
    #[inline(always)]
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

/// Trait to get compile-time information about an UI Node.
pub trait UINode: LiveUINode {
    /// The amount of primitives this UI Node will have when drawing.
    const QUAD_COUNT: usize;

    /// Gets the rectangle this primitive occupies, for rendering purposes.
    #[inline(always)]
    fn get_render_rect(&self) -> Option<Rect<f32, f32>>;
}

impl LiveUINode for () {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        false
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        /* No quads to push! */
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl UINode for () {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl<T> LiveUINode for Option<T>
where
    T: UINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_ui_event(event))
            .unwrap_or(false)
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        match self {
            Some(inner) => inner.push_quads(quad_buffer),
            None => {
                for idx in 0..Self::QUAD_COUNT {
                    quad_buffer[0] = Quad::default()
                }
            }
        }
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll_processors(cx))
            .unwrap_or(Poll::Ready(Some(())))
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl<T> UINode for Option<T>
where
    T: UINode,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().and_then(UINode::get_render_rect)
    }
}

impl<T, E> LiveUINode for Result<T, E>
where
    T: UINode,
    E: UINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        match self {
            Ok(v) => v.handle_ui_event(event),
            Err(e) => e.handle_ui_event(event),
        }
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        match self {
            Ok(v) => v.push_quads(quad_buffer),
            Err(e) => e.push_quads(quad_buffer),
        }
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // let this: &mut Self = self.deref_mut();
        // match this {
        //     Ok(v) => todo!(),
        //     Err(e) => todo!(),
        // }
        unimplemented!()
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl<T, E> UINode for Result<T, E>
where
    T: UINode,
    E: UINode,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match self {
            Ok(v) => v.get_render_rect(),
            Err(e) => e.get_render_rect(),
        }
    }
}

impl<T> LiveUINode for Box<T>
where
    T: UINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut().handle_ui_event(event)
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        self.as_ref().push_quads(quad_buffer)
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll_processors(cx)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl<T> UINode for Box<T>
where
    T: UINode,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

impl<A, B> LiveUINode for (A, B)
where
    A: UINode,
    B: UINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let a_handled = self.0.handle_ui_event(event.clone());
        let b_handled = self.1.handle_ui_event(event);

        a_handled || b_handled
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        let (slice_a, slice_b) = quad_buffer.split_at_mut(A::QUAD_COUNT);
        self.0.push_quads(slice_a);
        self.1.push_quads(slice_b);
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

        crate::prelude::coalesce_polls(poll_a, poll_b)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl<A, B> UINode for (A, B)
where
    A: UINode,
    B: UINode,
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
impl<T, const N: usize> SizedVec<T, N> {
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner }
    }
}
impl<A: UINode, const N: usize> LiveUINode for SizedVec<A, N> {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let mut any_handled = false;
        for item in self.inner.iter_mut() {
            any_handled = item.handle_ui_event(event.clone()) || any_handled;
        }
        any_handled
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        if self.inner.len() == 0 {
            return;
        }

        for idx in 0..N {
            self.inner[idx]
                .push_quads(&mut quad_buffer[(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)])
        }
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
            poll_acc = crate::prelude::coalesce_polls(poll_acc, item_poll)
        }
        poll_acc
    }
}
impl<A: UINode, const N: usize> UINode for SizedVec<A, N> {
    const QUAD_COUNT: usize = N * A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        let mut iterator = self.inner.iter();
        let first = iterator.next()?.get_render_rect();
        iterator.fold(first, |acc, item| Some(acc?.union(item.get_render_rect()?)))
    }
}
