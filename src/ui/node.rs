use vek::Rect;

pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction. In practice, it should handle as little as possible.
/// The entire user interface interaction and render pipelines are created with UI Nodes.
pub trait LiveUINode {
    /// Handles an UI Event (or not). Returns whether the event was handled.
    fn handle_ui_event(&mut self, event: UIEvent) -> bool;
}

/// Trait to get the info about an UI node statically.
pub trait UINode: LiveUINode {
    /// The amount of primitives this UI Node has.
    const PRIMITIVE_COUNT: usize;

    /// Gets the rectangle this primitive occupies, for rendering purposes.
    #[inline(always)]
    fn get_render_rect(&self) -> Option<Rect<f32, f32>>;
}

impl LiveUINode for () {
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        false
    }
}

impl UINode for () {
    const PRIMITIVE_COUNT: usize = 0;

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
}

impl<T> UINode for Option<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;

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
}

impl<T> UINode for Box<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

impl<A, B> LiveUINode for (A, B)
where
    A: LiveUINode,
    B: LiveUINode,
{
    fn handle_ui_event(&mut self, event: UIEvent) -> bool {
        let a_handled = self.0.handle_ui_event(event.clone());
        let b_handled = self.1.handle_ui_event(event);

        a_handled || b_handled
    }
}

impl<A, B> UINode for (A, B)
where
    A: UINode,
    B: UINode,
{
    const PRIMITIVE_COUNT: usize = A::PRIMITIVE_COUNT + B::PRIMITIVE_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match (self.0.get_render_rect(), self.1.get_render_rect()) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.union(b)),
        }
    }
}
