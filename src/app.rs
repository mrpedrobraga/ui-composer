use crate::{
    gpu::{
        engine::{LiveNode, Node, UIEngine, UIEngineInterface},
        window::WindowNode,
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
pub struct App<N: LiveNode, W: Node<LiveType = N>> {
    root_item: Option<W>,
    running_app: Option<RunningApp<N>>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp<N: LiveNode> {
    engine: Arc<Mutex<Box<dyn UIEngineInterface<RootNodeType = N>>>>,
}

/// TODO: PRovide methods to bind to an existing Event Loop or window.
impl<N: LiveNode + Send + 'static, W: Node<LiveType = N> + 'static> App<N, W> {
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

impl<N: LiveNode + Send + 'static, W: Node<LiveType = N> + 'static> ApplicationHandler
    for App<N, W>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let engine = UIEngine::new(event_loop, unsafe {
            self.root_item.take().unwrap_unchecked()
        });
        self.running_app = Some(RunningApp { engine });
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

impl<N: LiveNode> ApplicationHandler for RunningApp<N> {
    fn resumed(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {}

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
