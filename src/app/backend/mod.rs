use std::{
    pin::Pin,
    task::{Context, Poll},
};

/// The layer of the application that stands between the app and the outside world.
pub trait Backend {
    /// The type used for UI Events.
    type Event;

    /// The type of the Node tree this Backend executes.
    type Tree;

    /// Blocking function that runs the application.
    fn run(node_tree: Self::Tree);

    /// Polls the `Futures` and `Signals` from the node tree.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}
