use crate::{
    gpu::{
        engine::{Node, NodeDescriptor, UIEngine, UIEngineExt},
        window::WindowNodeDescriptor,
    },
    ui::{layout::LayoutItem, node::UINode, react::UISignalExt as _},
};
use futures_signals::signal::{Mutable, SignalExt};
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
pub struct App<N: Node, W: NodeDescriptor<RuntimeType = N>> {
    root_item: Option<W>,
    running_app: Option<RunningApp<N>>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp<N: Node> {
    engine: Arc<Mutex<Box<dyn UIEngineExt<RootNodeType = N>>>>,
}

/// TODO: PRovide methods to bind to an existing Event Loop or window.
impl<N: Node + Send + 'static, W: NodeDescriptor<RuntimeType = N> + 'static> App<N, W> {
    // Creates and runs a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn run(root_fragment: W) {
        let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
        event_loop
            .run_app(&mut App {
                root_item: Some(root_fragment),
                running_app: None,
            })
            .unwrap();
    }
}

impl<N: Node + Send + 'static, W: NodeDescriptor<RuntimeType = N> + 'static> ApplicationHandler
    for App<N, W>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // If there's no running app, create a new one using a new UIEngine.
        if (self.running_app.is_none()) {
            self.running_app = self.root_item.take().map(move |root_item| RunningApp {
                engine: UIEngine::new(event_loop, root_item),
            });
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

impl<N: Node> ApplicationHandler for RunningApp<N> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Nothing happens yet, but in the future the app should be able to respond to this!
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let mut engine = self
            .engine
            .lock()
            .expect("Could not lock Render Engine to pump window event");
        engine.handle_window_event(window_id, event);
    }
}
