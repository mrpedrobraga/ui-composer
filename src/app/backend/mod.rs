use crate::winitwgpu::backend::Node;
use std::pin::Pin;
use std::task::{Context, Poll};

/// The layer of the application that stands between the app and the outside world.
pub trait Backend {
    /// The type used for UI Events.
    type EventType;

    type NodeTreeType: Node + 'static;

    /// Blocking function that runs the application.
    fn run(node_tree: Self::NodeTreeType);

    /// Polls the `Futures` and `Signals` from the node tree.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}
