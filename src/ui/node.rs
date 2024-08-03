use vek::Rect;

use crate::ui::graphics::Quad;

pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction. In practice, it should handle as little as possible.
/// The entire user interface interaction and render pipelines are created with UI Nodes.
pub trait LiveUINode {
    /// Handles an UI Event (or not). Returns whether the event was handled.
    fn handle_ui_event(&mut self, event: UIEvent) -> bool;

    /// Pushes quads to a quad buffer slice.
    fn push_quads(&self, quad_buffer: &mut [Quad]);
}

/// Trait to get the info about an UI node statically.
pub trait UINode: LiveUINode {
    /// The amount of primitives this UI Node has.
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
}

impl UINode for () {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl<T> LiveUINode for Option<T>
where
    T: LiveUINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_ui_event(event))
            .unwrap_or(false)
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        if let Some(inner) = self {
            self.push_quads(quad_buffer)
        }
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

impl<T> LiveUINode for Box<T>
where
    T: LiveUINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        self.as_mut().handle_ui_event(event)
    }

    fn push_quads(&self, quad_buffer: &mut [Quad]) {
        self.as_ref().push_quads(quad_buffer)
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
