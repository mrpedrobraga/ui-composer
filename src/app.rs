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
pub struct AppBuilder<'app, N: LiveNode, W: Node<LiveType = N>> {
    root_item: Option<W>,
    event_loop: Option<winit::event_loop::EventLoop<()>>,
    running_app: Option<RunningApp<N>>,
    _marker: PhantomData<&'app ()>,
}

/// An app in execution (the ui fragment has been transformed into a [`RenderModule`]).
pub struct RunningApp<N: LiveNode> {
    engine: Arc<Mutex<Box<dyn UIEngineInterface<RootNodeType = N>>>>,
}

/// TODO: PRovide methods to bind to an existing Event Loop or window.
impl<'app, N: LiveNode + 'static, W: Node<LiveType = N> + 'static> AppBuilder<'app, N, W> {
    // Creates a new app.
    // For cross-platform compatibility, this should be called in the main thread,
    // and only once in your program.
    pub fn new(root_fragment: W) -> Self {
        Self {
            root_item: Some(root_fragment),
            event_loop: None,
            running_app: None,
            _marker: PhantomData,
        }
    }

    pub fn run(mut self) {
        let event_loop = self.event_loop.take().unwrap_or_else(|| {
            let event_loop = winit::event_loop::EventLoop::builder().build().unwrap();
            event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            event_loop
        });
        event_loop.run_app(&mut self).unwrap();
    }

    fn build(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(root_item) = self.root_item.take() {
            let engine = UIEngine::new(event_loop, root_item);
            self.running_app = Some(RunningApp { engine });
        }
    }
}

impl<'app, N: LiveNode + 'static, W: Node<LiveType = N> + 'static> ApplicationHandler
    for AppBuilder<'app, N, W>
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.build(event_loop);
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
