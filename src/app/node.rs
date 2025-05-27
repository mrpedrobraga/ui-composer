use crate::{
    gpu::{backend::GPUResources, pipeline::graphics::RenderGraphicDescriptor},
    state::signal_ext::coalesce_polls,
    ui::graphics::Graphic,
};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use vek::Rect;

pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction.
/// In practice, any single UI node should handle as little as possible.
/// The entire user interface is made of UI Nodes arranged in a graph.
pub trait AppItem: Send {
    /// Handles a UI Event (or not). Returns whether the event was handled.
    #[inline(always)]
    fn handle_ui_event(&mut self, event: UIEvent) -> bool;

    /// Polls this node's processors: `Future`s and `Signal`s.
    #[inline(always)]
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

pub trait AppItemDescriptor: AppItem + RenderGraphicDescriptor {}
impl<A> AppItemDescriptor for A where A: AppItem + RenderGraphicDescriptor {}

impl AppItem for () {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        false
    }

    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}

impl<A: Send + AppItem> AppItem for Option<A> {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_ui_event(event))
            .unwrap_or(false)
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll_processors(cx))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<T: Send + AppItem, E: Send + AppItem> AppItem for Result<T, E> {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        match self {
            Ok(v) => v.handle_ui_event(event),
            Err(e) => e.handle_ui_event(event),
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
}

impl<A: Send + AppItem> AppItem for Box<A> {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut().handle_ui_event(event)
    }

    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll_processors(cx)
    }
}

impl<A: Send + AppItem, B: Send + AppItem> AppItem for (A, B) {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let a_handled = self.0.handle_ui_event(event.clone());
        let b_handled = self.1.handle_ui_event(event);

        a_handled || b_handled
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
