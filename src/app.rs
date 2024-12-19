use crate::{
    gpu::{
        backend::{Backend, Node, NodeDescriptor, WinitBackend, WinitWGPUBackend},
        window::WindowNodeDescriptor,
    },
    ui::{layout::LayoutItem, node::UINodeDescriptor, react::UISignalExt as _},
};
use std::{
    marker::PhantomData,
    sync::{Arc, Mutex, RwLock},
};
use vek::Extent2;
use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent,
    platform::wayland::WindowAttributesExtWayland, window::WindowAttributes,
};

/// App builder, receives a layout item with the entirety of your app.
pub struct App<NodeDescriptorType: NodeDescriptor> {
    root_item: Option<NodeDescriptorType>,
    running_app: Option<RunningApp<NodeDescriptorType::ReifiedType>>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp<A: Node> {
    backend: Arc<Mutex<WinitWGPUBackend<A>>>,
}

/// TODO: PRovide methods to bind to an existing Event Loop or window.
impl<NodeDescriptorType: NodeDescriptor + 'static> App<NodeDescriptorType> {
    // Creates and runs a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn run(root_fragment: NodeDescriptorType) {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        event_loop
            .run_app(&mut Self {
                root_item: Some(root_fragment),
                running_app: None,
            })
            .unwrap();
    }
}

impl<NodeDescriptorType: NodeDescriptor + 'static> ApplicationHandler for App<NodeDescriptorType> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // If there's no running app, create a new one using a new UIEngine.
        if (self.running_app.is_none()) {
            self.running_app = self.root_item.take().map(move |root_item| RunningApp {
                backend: WinitWGPUBackend::create(event_loop, root_item),
            });
        }
        if let Some(running_app) = &mut self.running_app {
            running_app.resumed(event_loop)
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Some(running_app) = &mut self.running_app {
            running_app.window_event(event_loop, window_id, event)
        }
    }
}

impl<N: Node + 'static> ApplicationHandler for RunningApp<N> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Nothing happens yet, but in the future the app should be able to respond to this!
        let mut backend = self
            .backend
            .lock()
            .expect("Could not lock Render Engine to pump resumed event.");
        backend.handle_resumed();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let mut backend = self
            .backend
            .lock()
            .expect("Could not lock Render Engine to pump window event");
        backend.handle_window_event(window_id, event);
    }
}
