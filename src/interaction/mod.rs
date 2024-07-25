//! ## Interaction
//!
//! Interaction in ui-composer is achieved by creating interaction trees.
//!
//! When an event is emitted by the winit event loop, the root interactor node will be given the event.
//!

use wgpu::util::RenderEncoder;

use crate::{
    reaction::Reactor,
    render_module::RenderModule,
    standard::render::{
        tuple_render_module::TupleRenderModule, AllocationInfo, AllocationOffset, UIFragment,
        UIFragmentLive,
    },
};
pub mod hover;
pub mod keyboard;
pub mod tap;

/// An interactor node can receive an event and handle it, possibly flaring up reactors.
pub trait InteractorNode: Send {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool;
}

pub trait InteractorNodeContainer: Send {
    fn push(&mut self, child: Box<dyn InteractorNode>);
}

impl<I> UIFragment for I
where
    I: InteractorNode + UIFragmentLive + 'static,
{
    fn get_allocation_info() -> crate::standard::render::AllocationInfo {
        AllocationInfo {
            buffer_size: 0,
            primitive_count: 0,
            reactor_count: 0,
        }
    }
}

impl<I> UIFragmentLive for I
where
    I: InteractorNode + Clone + 'static,
{
    fn splat_allocation(
        &mut self,
        allocation_offset: AllocationOffset,
        render_module: &mut dyn RenderModule,
        initial: bool,
    ) {
        render_module.interactors().push(Box::new(self.clone()));
    }
}

pub struct VecNode {
    inner: Vec<Box<dyn InteractorNode>>,
}

impl VecNode {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
}

impl InteractorNode for VecNode {
    fn handle_event(&mut self, event: winit::event::WindowEvent) -> bool {
        self.inner
            .iter_mut()
            .map(|inner| inner.handle_event(event.clone()))
            .fold(true, |acc, handled| acc && handled)
    }
}

impl InteractorNodeContainer for VecNode {
    fn push(&mut self, child: Box<dyn InteractorNode>) {
        self.inner.push(child);
    }
}

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
