pub type UIEvent = winit::event::WindowEvent;

/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction. In practice, it should handle as little as possible.
/// The entire user interface interaction and render pipelines are created with UI Nodes.
pub trait LiveUINode {
    /// Handles an UI Event (or not). Returns whether the event was handled.
    fn handle_event(&mut self, event: UIEvent) -> bool;
}

/// Trait to get the info about an UI node statically.
pub trait UINode: LiveUINode {
    /// The amount of primitives this UI Node has.
    const PRIMITIVE_COUNT: usize;
}

impl LiveUINode for () {
    fn handle_event(&mut self, event: UIEvent) -> bool {
        false
    }
}

impl UINode for () {
    const PRIMITIVE_COUNT: usize = 0;
}

impl<T> LiveUINode for Option<T>
where
    T: LiveUINode,
{
    fn handle_event(&mut self, event: UIEvent) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_event(event))
            .unwrap_or(false)
    }
}

impl<T> UINode for Option<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;
}

impl<T> LiveUINode for Box<T>
where
    T: LiveUINode,
{
    fn handle_event(&mut self, event: UIEvent) -> bool {
        self.as_mut().handle_event(event)
    }
}

impl<T> UINode for Box<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;
}

impl<A, B> LiveUINode for (A, B)
where
    A: LiveUINode,
    B: LiveUINode,
{
    fn handle_event(&mut self, event: UIEvent) -> bool {
        let a_handled = self.0.handle_event(event.clone());
        let b_handled = self.1.handle_event(event);

        a_handled || b_handled
    }
}

impl<A, B> UINode for (A, B)
where
    A: UINode,
    B: UINode,
{
    const PRIMITIVE_COUNT: usize = A::PRIMITIVE_COUNT + B::PRIMITIVE_COUNT;
}
