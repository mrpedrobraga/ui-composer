use core::{
    ops::DerefMut as _,
    pin::Pin,
    task::{Context, Poll},
};
use futures_signals::signal::Signal;
use pin_project::pin_project;

/// The layer of the application that stands between the app and the outside world.
pub trait Backend {
    /// The type of the Node tree this Backend executes.
    type Tree;

    /// Blocking function that runs the application.
    fn run(node_tree: Self::Tree);

    /// Polls the `Futures` and `Signals` from the node tree.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}

#[cfg(feature = "std")]
type Own<A> = std::sync::Arc<std::sync::Mutex<A>>;
#[cfg(not(feature = "std"))]
type Own<A> = spin::Mutex<A>;

/// A futures-based construct that polls the engine's processes.
#[pin_project(project=BackendProcessExecutorProj)]
pub struct BackendProcessExecutor<B: Backend> {
    #[pin]
    backend: Own<B>,
}

impl<E: Backend> BackendProcessExecutor<E> {
    pub fn new(backend: Own<E>) -> Self {
        BackendProcessExecutor { backend }
    }
}

impl<B: Backend> Signal for BackendProcessExecutor<B> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let BackendProcessExecutorProj { backend } = self.project();

        let mut backend = backend.lock().unwrap();
        let backend = backend.deref_mut();
        let backend = unsafe { Pin::new_unchecked(backend) };

        backend.poll_processors(cx)
    }
}
