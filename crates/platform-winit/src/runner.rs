use futures::executor::block_on;
use futures_signals::signal::{Mutable, SignalExt};
use std::marker::PhantomData;
use std::sync::Arc;
use ui_composer_core::app::composition::algebra::Propagate;
use ui_composer_core::app::composition::elements::{Blueprint, Environment};
use ui_composer_core::app::runner::futures::AsyncExecutor;
use ui_composer_input::event::Event;
use ui_composer_math::glamour::Size2;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes};

use crate::{
    window::{WindowBlueprint, WindowElement},
    winit_uic_conversion, DesktopUi,
};

use crate::gpu::Gpu;
use crate::window::effect_handling::WindowEffectVisitor;

pub struct DesktopPlatform<Ui> {
    _ui: PhantomData<Ui>,
}

pub struct WinitAppHandler<'app, Ui: DesktopUi> {
    pub app_making_resources: DesktopResources<'app>,
    pub blueprint: Option<WindowBlueprint<Ui>>,
    pub element: Option<Arc<futures::lock::Mutex<WindowElement<Ui>>>>,
    pub element_sender:
        Option<futures::channel::oneshot::Sender<Arc<futures::lock::Mutex<WindowElement<Ui>>>>>,
}

// TODO: Add things to this Environment that elements might want to use.
// In mind I have a GPU allocator for allocating images and textures.
// This is probably how one requests a window, too.
pub struct DesktopEnvironment;

impl Environment for DesktopEnvironment {
    type BlueprintResources<'make> = DesktopResources<'make>;
    type EffectVisitor<'fx> = WindowEffectVisitor<'fx>;
    const TILE_SIZE: Size2 = Size2::new(16.0, 16.0);
}

#[derive(Clone)]
pub struct DesktopResources<'make> {
    pub(crate) winit_requester: &'make WinitRequester,
    pub(crate) gpu: Gpu,
    pub(crate) window_size_mutable: Mutable<Size2>,
    pub(crate) window: Option<Arc<Window>>,
}

#[allow(unused)]
pub(crate) struct WinitRequester {
    pub proxy: EventLoopProxy<DesktopUicRequest>,
}

#[allow(unused)]
pub(crate) enum DesktopUicRequest {
    // CreateWindow {
    //     attributes: WindowAttributes,
    //     tx: oneshot::Sender<Arc<Window>>,
    // },
}

impl WinitRequester {}

impl<Ui> DesktopPlatform<Ui>
where
    Ui: DesktopUi,
{
    pub fn run(window_blueprint: WindowBlueprint<Ui>) {
        println!("[Winit Runner] Starting.");

        let e_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = e_loop.create_proxy();
        let gpu = futures::executor::block_on(Gpu::new());

        let winit_requester = WinitRequester { proxy };

        std::thread::scope(|scope| {
            let app_making_resources = DesktopResources {
                winit_requester: &winit_requester,
                gpu,
                window_size_mutable: Mutable::new(Size2::ZERO),
                window: None,
            };

            let (tx, rx) = futures::channel::oneshot::channel();

            let mut winit_app_handler: WinitAppHandler<Ui> = WinitAppHandler {
                app_making_resources: app_making_resources.clone(),
                blueprint: Some(window_blueprint),
                element: None,
                element_sender: Some(tx),
            };

            scope.spawn(|| {
                let element = block_on(rx).unwrap();

                let async_executor: AsyncExecutor<'_, DesktopEnvironment, _, _> =
                    AsyncExecutor::new(element, app_making_resources, || {});
                block_on(async_executor.to_future())
            });

            e_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            println!("[Winit Runner] Transferring control to winit.");
            e_loop.run_app(&mut winit_app_handler).unwrap();
            println!("[Winit Runner] All done.")
        });
    }
}

impl<'app, Ui> ApplicationHandler<DesktopUicRequest> for WinitAppHandler<'app, Ui>
where
    Ui: DesktopUi,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[Winit] Resumed.");

        if let Some(mut blueprint) = self.blueprint.take() {
            let initial_child_hints = blueprint.initial_child_hints();

            let minimum_window_size = initial_child_hints.minimum_size;

            // TODO: Allow changing the attributes!
            let initial_title = "Ui Composer Window!";
            let initial_window_size: Size2<f32> = Size2::new(640.0, 360.0).max(minimum_window_size);

            let window_attributes = WindowAttributes::default()
                .with_title(initial_title)
                .with_inner_size(PhysicalSize {
                    width: initial_window_size.width,
                    height: initial_window_size.height,
                });
            let window = event_loop.create_window(window_attributes).unwrap();
            let window = Arc::new(window);
            let element = blueprint.make(&DesktopResources {
                window: Some(window),
                gpu: self.app_making_resources.gpu.clone(),
                window_size_mutable: self.app_making_resources.window_size_mutable.clone(),
                winit_requester: self.app_making_resources.winit_requester,
            });
            // Set the element's window so Window is kept alive?

            let element = Arc::new(futures::lock::Mutex::new(element));
            self.element_sender
                .take()
                .expect("[Winit] But there was no sender anymore?")
                .send(element.clone())
                .expect("[Winit] Other side was closed.");
            self.element = Some(element);
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        // TODO: Use this to know to which window to send these.
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Ok(mut uic_event) = winit_uic_conversion::into_event(event.clone()) {
            if let Some(element) = &self.element {
                let mut lock = block_on(element.lock());

                if let Event::Resized(new_size) = &uic_event {
                    lock.prepare_to_resize(*new_size, &self.app_making_resources);
                }

                let _effect_was_handled = block_on(lock.propagate(&mut uic_event));
            }
        } else {
            //println!("Unrecognized event: {:?}", event);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: DesktopUicRequest) {
        /* Maybe will go unused? */
    }
}
