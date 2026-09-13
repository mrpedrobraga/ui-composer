use futures::channel::mpsc::{self, Sender};
use futures::channel::oneshot;
use futures::executor::block_on;
use futures::{StreamExt, join};
use futures_signals::signal::{Mutable, SignalExt};
use ui_composer_math::glamour::Size2;
use std::marker::PhantomData;
use std::sync::{Arc};
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

use crate::gpu::Gpu;
use crate::window::effect_handling::WindowEffectVisitor;

// TODO: Add things to this Environment that elements might want to use.
// In mind I have a GPU allocator for allocating images and textures.
// This is probably how one requests a window, too.
pub struct WinitEnvironment;

impl Environment for WinitEnvironment {
    type BlueprintResources<'make> = WinitBlueprintResources<'make>;
    type EffectVisitor<'fx> = WindowEffectVisitor<'fx>;
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
            e_loop.set_control_flow(ControlFlow::Wait);
            let proxy = e_loop.create_proxy();

            let gpu = block_on(Gpu::new());
            let gpu2 = gpu.clone();

            let window_size_mutable = Mutable::new(Size2 { width: 640_f32, height: 360_f32 });
            let window_size_mutable2 = window_size_mutable.clone();

            scope.spawn(move || {
                // NOTE: Because of winit's very model where it monopolises the main thread,
                // the app blueprint is sent to the ApplicationHandler to be made,
                // like a 15 year old to a board school.
                let winit_requester = WinitRequester { proxy };
                let app_making_resources = WinitBlueprintResources {
                        winit_requester: &winit_requester,
                        gpu: gpu2,
                        window_size_mutable: window_size_mutable2.clone()
                    };
                let app = {
                    let res = app_making_resources.clone();
                    app_blueprint.make(&res)
                };
                let app = Arc::new(futures::lock::Mutex::new(app));
                let app2 = app.clone();

                let event_handler = async move {
                    let mut event_rx = event_rx;
                    let app2 = app2;
                    let mut event_no = 0;

                    while let Some(mut event) = event_rx.next().await {
                        let span = tracing::debug_span!("event handler");
                        span.in_scope(async || {                            
                            let mut _lock = app2.lock().await;

                            tracing::debug!(
                                "[Event Handler] Received event no {event_no} `{event:?}`. Broadcasting."
                            );
                            // TODO: Use something with a little more data than a bool.
                            let event_was_handled = _lock.bubble(&mut event).await;
                           
                            /* Push event down app! */
                            event_no += 1;

                            tracing::debug!(
                                "[Event Handler] The event was {}.",
                                if event_was_handled {
                                    "handled"
                                } else {
                                    "not handled"
                                }
                            );
                        }).await;
                    }
                };

                let mut s = 0;
                let async_handler =
                    AsyncExecutor::new(app, app_making_resources, || {
                        println!("S = {}!", s);
                        s += 1;
                    }).to_future();

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
            let mut winit_app_handler = WinitAppHandler { state: (0,), event_tx, window_size_mutable };

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
    state: (i32,),
    event_tx: Sender<Event>,
    window_size_mutable: Mutable<Size2>
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
        //TODO: Restructure how the event loop sends events.
        if let Ok(uic_event) = crate::winit_uic_conversion::into_event(event) {
            tracing::debug!("[Winit App Handler] Sending event no {} `{:?}`.", self.state.0, uic_event);
            self.state.0 += 1;
            
            if let Event::Resized(new_size) = uic_event {
                self.window_size_mutable.set(new_size);
                return;
            }
            
            // block_on(self.event_tx.send(uic_event))
            //     .expect("[Winit App Handler] Failed to send event though channel.");
            let _ = self.event_tx.try_send(uic_event);
        } else {
            tracing::warn!("Unrecognized event.");
        }
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

#[derive(Clone)]
pub struct WinitBlueprintResources<'make> {
    pub(crate) winit_requester: &'make WinitRequester,
    pub(crate) gpu: Gpu,
    pub(crate) window_size_mutable: Mutable<Size2>
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
