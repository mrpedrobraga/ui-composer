use futures::channel::mpsc::Sender;
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt, join};
use futures_signals::signal::SignalExt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use ui_composer_core::app::composition::elements::{Blueprint, Environment};
use ui_composer_core::app::runner::Runner;
use ui_composer_core::app::runner::futures::AsyncExecutor;
use ui_composer_input::event::Event;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{
    ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy,
};
use winit::window::{Window, WindowAttributes, WindowId};

// TODO: Add things to this Environment that elements might want to use.
// In mind I have a GPU allocator for allocating images and textures.
// This is probably how one requests a window, too.
pub struct WinitEnvironment {}

impl Environment for WinitEnvironment {
    type BlueprintResources<'make> = WinitBlueprintResources<'make>;
    type EffectVisitor<'fx> = ();
}

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

    fn run(app_blueprint: Self::AppBlueprint) {
        println!("[Winit] Initializing.");

        std::thread::scope(move |scope| {
            // TODO: Decide how wide to make the throat of this channel.
            // This decision should probably come from benchmarking?
            let (event_sink, event_source) =
                futures::channel::mpsc::channel::<Event>(32);

            /*
                Initialize thread that will receive events from winit.
            */
            let e_loop = EventLoop::with_user_event()
                .build()
                .expect("[Winit] Failed to create event loop");
            let proxy = e_loop.create_proxy();

            scope.spawn(move || {
                // NOTE: Because of winit's very model where it monopolises the main thread,
                // the app blueprint is sent to the ApplicationHandler to be made,
                // like a 15 year old to a board school.
                let winit_requester = WinitRequester { proxy };
                let app = {
                    let res = WinitBlueprintResources {
                        winit_requester: &winit_requester,
                    };
                    app_blueprint.make(&res)
                };
                let app = Arc::new(Mutex::new(app));
                let app2 = app.clone();

                let event_handler = async move {
                    let mut tap = event_source;
                    let app2 = app2;

                    while let Some(event) = tap.next().await {
                        let _lock = app2.lock().expect(
                            "[Event] Failed to lock app to send event.",
                        );
                        /* Push event down app! */
                        println!("A new event arrived! {:?}", event);
                    }
                };

                let res = WinitBlueprintResources {
                    winit_requester: &winit_requester,
                };
                let async_handler =
                    AsyncExecutor::new(app, res, || {}).to_future();

                let processes = async { join!(async_handler, event_handler) };

                block_on(processes);
            });

            /*
                Create a handler in the format winit requires.
                It must run on the main thread, and it IS blocking...
                And thus we _must_ create a new thread if we want any futures/signals to be polled.
            */
            let mut winit_app_handler = WinitAppHandler { event_sink };

            /*
                Create event loop and run the handler.
            */
            e_loop.set_control_flow(ControlFlow::Wait);
            e_loop
                .run_app(&mut winit_app_handler)
                .expect("[Winit] Failed to run event loop...");
        });

        println!("[Winit] All processes finished. Shutting down.");
    }
}

pub struct WinitAppHandler {
    event_sink: Sender<Event>,
}
impl ApplicationHandler<WinitUicRequest> for WinitAppHandler {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let uic_event = crate::winit_uic_conversion::into_event(event)
            .expect("Unrecognized event.");
        //TODO: Restructure how the event loop sends events.
        block_on(self.event_sink.send(uic_event))
            .expect("[Winit] Failed to send event though channel.");
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        println!("[Winit] Goodbye!")
    }

    fn user_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: WinitUicRequest,
    ) {
        match event {
            WinitUicRequest::CreateWindow {
                attributes,
                tx: response,
            } => {
                let window = event_loop
                    .create_window(attributes)
                    .expect("Failed to create window");
                let _ = response.send(Arc::new(window));
            }
        }
    }
}

pub struct WinitBlueprintResources<'make> {
    pub(crate) winit_requester: &'make WinitRequester,
}

pub(crate) struct WinitRequester {
    pub proxy: EventLoopProxy<WinitUicRequest>,
}

pub(crate) enum WinitUicRequest {
    CreateWindow {
        attributes: WindowAttributes,
        tx: oneshot::Sender<Arc<Window>>,
    },
}

impl WinitRequester {
    pub fn request_window(&self, attributes: WindowAttributes) -> Arc<Window> {
        let (tx, rx) = oneshot::channel();
        if self
            .proxy
            .send_event(WinitUicRequest::CreateWindow { attributes, tx })
            .is_err()
        {
            panic!("event loop isn't running")
        }

        block_on(rx).expect("to receive a window")
    }
}
