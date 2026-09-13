use futures::executor::block_on;
use futures_signals::signal::Mutable;
use std::marker::PhantomData;
use std::sync::Arc;
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Environment};
use ui_composer_input::event::Event;
use ui_composer_math::glamour::Size2;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Window, WindowAttributes};

use crate::{
    window::{WindowBlueprint, WindowElement},
    winit_uic_conversion, WinitUi,
};

use crate::gpu::Gpu;
use crate::window::effect_handling::WindowEffectVisitor;

pub struct WinitRunner<Ui> {
    _ui: PhantomData<Ui>,
}

pub struct WinitAppHandler<'app, Ui: WinitUi> {
    pub app_making_resources: WinitBlueprintResources<'app>,
    pub blueprint: Option<WindowBlueprint<Ui>>,
    pub element: Option<WindowElement<Ui>>,
}

// TODO: Add things to this Environment that elements might want to use.
// In mind I have a GPU allocator for allocating images and textures.
// This is probably how one requests a window, too.
pub struct WinitEnvironment;

impl Environment for WinitEnvironment {
    type BlueprintResources<'make> = WinitBlueprintResources<'make>;
    type EffectVisitor<'fx> = WindowEffectVisitor<'fx>;
}

#[derive(Clone)]
pub struct WinitBlueprintResources<'make> {
    pub(crate) winit_requester: &'make WinitRequester,
    pub(crate) gpu: Gpu,
    pub(crate) window_size_mutable: Mutable<Size2>,
    pub(crate) window: Option<Arc<Window>>,
}

#[allow(unused)]
pub(crate) struct WinitRequester {
    pub proxy: EventLoopProxy<WinitUicRequest>,
}

#[allow(unused)]
pub(crate) enum WinitUicRequest {
    // CreateWindow {
    //     attributes: WindowAttributes,
    //     tx: oneshot::Sender<Arc<Window>>,
    // },
}

impl WinitRequester {}

impl<Ui> WinitRunner<Ui>
where
    Ui: WinitUi,
{
    pub fn run(window_blueprint: WindowBlueprint<Ui>) {
        println!("[Winit Runner] Starting.");

        let e_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = e_loop.create_proxy();
        let gpu = futures::executor::block_on(Gpu::new());

        let app_making_resources = WinitBlueprintResources {
            winit_requester: &WinitRequester { proxy },
            gpu,
            window_size_mutable: Mutable::new(Size2::ZERO),
            window: None,
        };

        let mut winit_app_handler: WinitAppHandler<Ui> = WinitAppHandler {
            app_making_resources,
            blueprint: Some(window_blueprint),
            element: None,
        };
        e_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

        println!("[Winit Runner] Transferring control to winit.");
        e_loop.run_app(&mut winit_app_handler).unwrap();
        println!("[Winit Runner] All done.")
    }
}

impl<'app, Ui> ApplicationHandler<WinitUicRequest> for WinitAppHandler<'app, Ui>
where
    Ui: WinitUi,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[Winit] Resumed.");

        if let Some(blueprint) = self.blueprint.take() {
            let window_attributes = WindowAttributes::default()
                .with_title("UI Composer Window")
                .with_inner_size(PhysicalSize {
                    width: 640,
                    height: 360,
                });
            let window = event_loop.create_window(window_attributes).unwrap();
            let window = Arc::new(window);
            let element = blueprint.make(&WinitBlueprintResources {
                window: Some(window),
                gpu: self.app_making_resources.gpu.clone(),
                window_size_mutable: self.app_making_resources.window_size_mutable.clone(),
                winit_requester: self.app_making_resources.winit_requester,
            });
            // Set the element's window so Window is kept alive?
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
            if let Some(element) = &mut self.element {
                //println!("Bubbling event: {:?}", uic_event);

                if let Event::Resized(new_size) = &uic_event {
                    element.prepare_to_resize(*new_size, &self.app_making_resources);
                }

                let _effect_was_handled = block_on(element.bubble(&mut uic_event));

                //println!("Handled? {}", _effect_was_handled);
            }
        } else {
            //println!("Unrecognized event: {:?}", event);
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, _event: WinitUicRequest) {
        /* Maybe will go unused? */
    }
}
