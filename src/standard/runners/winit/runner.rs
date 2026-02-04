use crate::app::composition::elements::Blueprint;
use crate::app::input::Event;
use crate::app::runner::Runner;
use async_std::prelude::StreamExt;
use async_std::task::block_on;
use futures::channel::mpsc;
use futures::SinkExt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalSize, Size};
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::platform::wayland::EventLoopBuilderExtWayland;
use winit::window::{Window, WindowId};

pub struct WinitEnvironment;

pub type Share<T> = Arc<Mutex<T>>;

pub struct WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<WinitEnvironment>,
{
    pub app: Share<AppBlueprint::Element>,
    __thread_fixed: PhantomData<*mut ()>,
}

impl<AppBlueprint> Runner for WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<WinitEnvironment>,
{
    type AppBlueprint = AppBlueprint;

    fn run(_: Self::AppBlueprint) {
        println!("Running!");
        let (tx, rx) = mpsc::channel(10);
        let mut winit_app_handler = WinitAppHandler {
            sink: tx,
            window: None,
        };

        // For cross-platform compatibility, the EventLoop must be created on the main thread, and only once per application.
        let event_loop = EventLoop::builder()
            .with_any_thread(true)
            .build()
            .expect("Failed to build winit event loop.");
        event_loop.set_control_flow(ControlFlow::Wait);

        std::thread::spawn(move || {
            block_on(async move {
                println!("[Event Loop] Listening for winit events.");
                rx.for_each(|event| {
                    dbg!(event);
                })
                .await;
                println!("[Event Loop] No longer listening for winit events.")
            })
        });

        // TODO: Implements this Runner so it works for every platform `winit` supports.
        event_loop
            .run_app(&mut winit_app_handler)
            .expect("Failed to run winit event loop.");
    }

    async fn event_loop(&self) {
        todo!()
    }

    async fn react_loop(&self) {
        todo!()
    }
}

pub struct WinitAppHandler {
    sink: mpsc::Sender<Event>,
    window: Option<Window>,
}
impl ApplicationHandler for WinitAppHandler {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("[Winit App] Resumed!");

        let attributes = Window::default_attributes()
            .with_title("My App!".to_string())
            .with_inner_size(Size::Physical(PhysicalSize {
                width: 100,
                height: 100,
            }))
            .with_visible(true);

        let window = event_loop
            .create_window(attributes)
            .expect("Failed to create window!");

        self.window = Some(window);
    }

    fn window_event(&mut self, _: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        let uic_event = Event::try_from(event).expect("Unrecognized event.");
        //TODO: Restructure how the event loop sends events.
        block_on(self.sink.send(uic_event)).expect("Receiver is gone!");
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        println!("[Winit App] Goodbye!")
    }
}
