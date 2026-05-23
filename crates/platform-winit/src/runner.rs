use futures::channel::mpsc::{self, Sender};
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::{SinkExt, StreamExt, join};
use futures_signals::signal::SignalExt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{
    Blueprint, Element, Environment,
};
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
pub struct WinitEnvironment;

impl Environment for WinitEnvironment {
    type BlueprintResources<'make> = WinitBlueprintResources<'make>;
    type EffectVisitor<'fx> = ();
}

pub struct WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<WinitEnvironment>,
{
    _app: PhantomData<AppBlueprint>,
}

impl<AppBlueprint> Runner for WinitRunner<AppBlueprint>
where
    AppBlueprint: Blueprint<
            WinitEnvironment,
            Element: Element<WinitEnvironment> + Send + 'static,
        > + Send,
{
    type AppBlueprint = AppBlueprint;

    fn run(app_blueprint: Self::AppBlueprint) {
        println!("[Winit Runner] Initializing.");

        std::thread::scope(move |scope| {
            // TODO: Decide how wide to make the throat of this channel.
            // This decision should probably come from benchmarking?
            let (event_tx, event_rx) = mpsc::channel::<Event>(32);

            /*
                Initialize thread that will receive events from winit.
            */
            let e_loop = EventLoop::with_user_event()
                .build()
                .expect("[Winit Runner] Failed to create event loop");
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
                    let mut event_rx = event_rx;
                    let app2 = app2;

                    while let Some(mut event) = event_rx.next().await {
                        let span = tracing::debug_span!("event handler");
                        span.in_scope(|| {
                            let mut _lock = app2.lock().expect(
                                "[Event Handler] Failed to lock app to send event.",
                            );

                            /* Push event down app! */
                            tracing::debug!(
                                "[Event Handler] New event `{event:?}`. Broadcasting."
                            );
                            // TODO: Use something with a little more data than a bool.
                            let event_was_handled = _lock.bubble(&mut event);
                            tracing::debug!(
                                "[Event Handler] The event was {}.",
                                if event_was_handled {
                                    "handled"
                                } else {
                                    "not handled"
                                }
                            );
                        });
                    }
                };

                let res = WinitBlueprintResources {
                    winit_requester: &winit_requester,
                };
                let async_handler =
                    AsyncExecutor::new(app, res, || {}).to_future();

                // TODO: Think very well about how these two tasks will coordinate,
                // such that one doesn't hog all the resources when running on a single-threaded
                // environment.
                let processes = async { join!(async_handler, event_handler) };

                block_on(processes);
            });

            /*
                Create a handler in the format winit requires.
                It must run on the main thread, and it IS blocking...
                And thus we _must_ create a new thread if we want any futures/signals to be polled.
            */
            let mut winit_app_handler = WinitAppHandler { event_tx };

            /*
                Create event loop and run the handler.
            */
            e_loop.set_control_flow(ControlFlow::Wait);
            e_loop
                .run_app(&mut winit_app_handler)
                .expect("[Winit Runner] Failed to run event loop...");
        });

        println!("[Winit Runner] All processes finished. Shutting down.");
    }
}

pub struct WinitAppHandler {
    event_tx: Sender<Event>,
}
impl ApplicationHandler<WinitUicRequest> for WinitAppHandler {
    fn resumed(&mut self, _: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        _: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        tracing::debug!("[Winit App Handler] Window Event {event:?}.");
        let uic_event = crate::winit_uic_conversion::into_event(event)
            .expect("Unrecognized event.");
        //TODO: Restructure how the event loop sends events.
        block_on(self.event_tx.send(uic_event))
            .expect("[Winit App Handler] Failed to send event though channel.");
    }

    fn exiting(&mut self, _: &ActiveEventLoop) {
        tracing::debug!("[Winit App Handler] Exiting.")
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
                    .expect("[Winit App Handler] Failed to create window");
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
            panic!("[Winit Channel] event loop isn't running")
        }

        block_on(rx).expect("[Winit Channel] to receive a window")
    }
}
