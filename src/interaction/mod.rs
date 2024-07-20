//! ## Interaction
//!
//! Interaction in ui-composer is achieved by creating interaction trees.
//!
//! When an event is emitted by the winit event loop, the root interactor node will be given the event.
//!

/// An interactor node can receive an event and handle it, possibly flaring up reactors.
pub trait InteractorNode {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool;
}

pub mod hover;
pub mod keyboard;

pub struct ToggleNode<N: InteractorNode> {
    enabled: bool,
    inner: N,
}

impl<N> InteractorNode for ToggleNode<N>
where
    N: InteractorNode,
{
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.inner.handle_event(event)
    }
}

pub struct SwapNode<N: InteractorNode> {
    inner: N,
}

impl<N> InteractorNode for SwapNode<N>
where
    N: InteractorNode,
{
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.inner.handle_event(event)
    }
}
