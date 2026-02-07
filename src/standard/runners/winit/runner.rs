use crate::app::composition::elements::Blueprint;
use crate::app::input::Event;
use crate::app::runner::Runner;
use async_std::task::block_on;
use futures::channel::mpsc::Sender;
use futures::SinkExt;
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

pub struct WinitEnvironment;

pub type Share<T> = Arc<Mutex<T>>;

pub struct WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<WinitEnvironment>,
{
    _app: PhantomData<AppBlueprint>,
}

impl<AppBlueprint> Runner for WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<WinitEnvironment, Element: Send + 'static>,
{
    type AppBlueprint = AppBlueprint;

    fn run(app: Self::AppBlueprint) {
        println!("[Winit] Initializing.");

        let env = WinitEnvironment;
        let app = app.make(&env);
        let app = Arc::new(Mutex::new(app));

        let (sink, tap) = futures::channel::mpsc::channel(0);

        /*
            Create a handler in the format winit requires.
            It must run on the main thread, and it IS blocking...
            And thus we _must_ create a new thread if we want any futures/signals to be polled.
         */
        let mut winit_app_handler = WinitAppHandler { sink, window: None };

        /*
            Create event loop and run the handler.
        */
        let e_loop = EventLoop::builder().build().expect("[Winit] Failed to create event loop");
        e_loop.set_control_flow(ControlFlow::Wait);
        e_loop.run_app(&mut winit_app_handler).expect("[Winit] Failed to run event loop...");
    }
}

pub struct WinitAppHandler {
    sink: Sender<Event>,
    window: Option<Window>,
}
impl ApplicationHandler for WinitAppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let attributes = Window::default_attributes()
            .with_title("My App!".to_string())
            .with_inner_size(Size::Physical(PhysicalSize {
                width: 100,
                height: 100,
            }))
            .with_visible(true);

        let window = event_loop
            .create_window(attributes)
            .expect("[Winit] Failed to create window!");

        self.window = Some(window);
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let uic_event = Event::try_from(event).expect("Unrecognized event.");
        //TODO: Restructure how the event loop sends events.
        block_on(self.sink.send(uic_event)).expect("[Winit] Failed to send event though channel.");
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        println!("[Winit] Goodbye!")
    }
}
