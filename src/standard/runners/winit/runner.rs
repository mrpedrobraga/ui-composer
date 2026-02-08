use crate::app::composition::elements::Blueprint;
use crate::app::input::Event;
use crate::app::runner::Runner;
use async_std::task::block_on;
use futures::channel::mpsc::Sender;
use futures::{join, SinkExt, StreamExt};
use std::marker::PhantomData;
use std::sync::{mpsc, Arc, Mutex};
use futures_signals::signal::SignalExt;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::app::runner::futures::AsyncExecutor;

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
    AppBlueprint: Blueprint<WinitEnvironment, Element: Send + 'static> + Send,
{
    type AppBlueprint = AppBlueprint;

    fn run(app: Self::AppBlueprint) {
        println!("[Winit] Initializing.");

        std::thread::scope(move |scope| {
            let (sink, tap) = futures::channel::mpsc::channel::<Event>(0);

            /*
                Initialize thread that will receive events from winit.
            */

            scope.spawn(move || {
                let env = WinitEnvironment;
                let app = app.make(&env);
                let app = Arc::new(Mutex::new(app));
                let app2 = app.clone();

                let event_handler = async move {
                    let mut tap = tap;
                    let app2 = app2;

                    while let Some(event) = tap.next().await {
                        let _lock = app2.lock().expect("[Event] Failed to lock app to send event.");
                        /* Push event down app! */
                        println!("A new event arrived! {:?}", event);
                    }
                };

                let async_handler = AsyncExecutor::new(app, env).to_future();

                let processes = async { join!(async_handler, event_handler) };

                block_on(processes);
            });

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
        });

        println!("[Winit] All processes finished. Shutting down.");
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
