/// A node of a UI tree.
///
/// A UINode receives an order to render, to update child nodes or to handle an interaction. In practice, it should handle as little as possible.
/// The entire user interface interaction and render pipelines are created with UI Nodes.
pub trait UINode {
    /// The amount of primitives this UI Node has.
    const PRIMITIVE_COUNT: usize;
}

impl UINode for () {
    const PRIMITIVE_COUNT: usize = 0;
}

impl<T> UINode for Option<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;
}

impl<T> UINode for Box<T>
where
    T: UINode,
{
    const PRIMITIVE_COUNT: usize = T::PRIMITIVE_COUNT;
}

impl<A, B> UINode for (A, B)
where
    A: UINode,
    B: UINode,
{
    const PRIMITIVE_COUNT: usize = A::PRIMITIVE_COUNT + B::PRIMITIVE_COUNT;
}
