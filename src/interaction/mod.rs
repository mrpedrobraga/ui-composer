//! ## Interaction
//!
//! Interaction in ui-composer is achieved by creating interaction trees.
//!
//! When an event is emitted by the winit event loop, the root interactor node will be given the event.
//!

/// An interactor node can receive an event and handle it, possibly flaring up reactors.
pub trait InteractorNode {}

pub struct Aabb(pub f32, pub f32, pub f32, pub f32);

pub struct Hover {
    pub aabb: Aabb,
}

impl InteractorNode for Hover {}

pub struct Keyboard {}

impl InteractorNode for Keyboard {}
