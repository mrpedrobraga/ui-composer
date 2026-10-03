use ::ui_composer_core::app::composition::modules::RenderModule;
use ::ui_composer_core::app::composition::visit::DriveThru;
use futures::executor::block_on;
use futures_signals::signal::SignalExt;
use std::marker::PhantomData;
use std::sync::Arc;
use ui_composer_core::app::composition::elements::Environment;
use ui_composer_core::app::runner::futures::RenderModulePoller;
use ui_composer_input::event::Event;
use ui_composer_math::glamour::Size2;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::WindowAttributes;

use crate::render::{
    DesktopRenderResources, draw_render_module_onto_render_target,
};
use crate::window::{WindowState, WindowStateDescriptor};
use crate::{DesktopUi, window::WindowDescriptor, winit_uic_conversion};

use crate::gpu::Gpu;
use crate::window::effect_handling::RenderModuleEffectVisitor;

pub struct DesktopPlatform<Ui> {
    _ui: PhantomData<Ui>,
}

pub struct WinitAppHandler<'app, Ui: DesktopUi> {
    pub app_making_resources: DesktopResources<'app>,
    // TODO: Allow an app to have more than one window.
    pub blueprint: Option<WindowStateDescriptor>,
    pub window_state: Option<WindowState>,
    pub root_module:
        Arc<futures::lock::Mutex<RenderModule<DesktopEnvironment, Ui>>>,
}

// TODO: Add things to this Environment that elements might want to use.
// In mind I have a GPU allocator for allocating images and textures.
// This is probably how one requests a window, too.
pub struct DesktopEnvironment;

impl Environment for DesktopEnvironment {
    type BlueprintResources<'make> = DesktopResources<'make>;
    type RenderResources = DesktopRenderResources;
    type EffectVisitor<'fx> = RenderModuleEffectVisitor<'fx>;
    const TILE_SIZE: Size2 = Size2::new(16.0, 16.0);
}

#[derive(Clone)]
pub struct DesktopResources<'make> {
    pub(crate) gpu: Gpu,
    __marker: PhantomData<&'make ()>,
}

#[allow(unused)]
pub(crate) struct WinitRequester {
    pub proxy: EventLoopProxy<DesktopUicRequest>,
}

#[allow(unused)]
pub(crate) enum DesktopUicRequest {
    AppUpdate,
}

impl WinitRequester {}

impl<Ui> DesktopPlatform<Ui>
where
    Ui: DesktopUi,
{
    pub fn run(win_descriptor: WindowDescriptor<Ui>) {
        println!("[Winit Runner] Starting.");

        let e_loop = EventLoop::with_user_event().build().unwrap();
        let proxy = e_loop.create_proxy();
        let gpu = futures::executor::block_on(Gpu::new());

        std::thread::scope(|scope| {
            let app_making_resources = DesktopResources {
                gpu: gpu.clone(),
                __marker: PhantomData,
            };
            let root_module_initial_size = win_descriptor.initial_size();
            let render_resources = DesktopRenderResources::new(gpu);
            let root_module = RenderModule::new(
                win_descriptor.ui,
                root_module_initial_size,
                render_resources,
            );
            let root_module =
                Arc::new(::futures::lock::Mutex::new(root_module));

            let mut winit_app_handler: WinitAppHandler<Ui> = WinitAppHandler {
                app_making_resources: app_making_resources.clone(),
                blueprint: Some(win_descriptor.state),
                window_state: None,
                root_module: root_module.clone(),
            };

            // Only start the winit thread once the app yields at least once,
            // which it is guaranteed to do.
            let (tx, rx) = ::futures::channel::oneshot::channel();

            scope.spawn(move || {
                let mut tx = Some(tx);

                let async_executor: RenderModulePoller<
                    '_,
                    DesktopEnvironment,
                    _,
                    _,
                > = RenderModulePoller::new(
                    root_module,
                    app_making_resources,
                    || {
                        if let Some(tx) = tx.take() {
                            let _ = tx.send(());
                        }

                        let _ = proxy.send_event(DesktopUicRequest::AppUpdate);
                    },
                );
                block_on(async_executor.to_future())
            });

            let _ = block_on(rx);

            e_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);
            println!("[Winit Runner] Transferring control to winit.");
            e_loop.run_app(&mut winit_app_handler).unwrap();
            println!("[Winit Runner] All done.")
        });
    }
}

impl<'app, Ui> ApplicationHandler<DesktopUicRequest>
    for WinitAppHandler<'app, Ui>
where
    Ui: DesktopUi,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        println!("[Winit] Resumed.");

        if let Some(blueprint) = self.blueprint.take() {
            // TODO: Allow configuring initial title in the blueprint.
            let initial_title = "Ui Composer Window!";
            let child_hints;
            {
                let mut root_module = block_on(self.root_module.lock());
                root_module
                    .resize(blueprint.app_size, &self.app_making_resources);
                child_hints = root_module.state.child_hints();
            }
            let window_attributes = WindowAttributes::default()
                .with_title(initial_title)
                .with_inner_size(PhysicalSize {
                    width: blueprint.app_size.width.max(child_hints.minimum_size.width),
                    height: blueprint.app_size.height.max(child_hints.minimum_size.height),
                });
            let window = event_loop.create_window(window_attributes).unwrap();
            self.window_state = Some(WindowState::from_blueprint(
                blueprint,
                self.app_making_resources.gpu.clone(),
                Arc::new(window),
                child_hints,
            ));
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        // TODO: Use this to know to which window to send these.
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // TODO: Keep many root modules, stored by `_window_id` and use that
        // to get the right one to send the event to.

        if let Ok(mut uic_event) =
            winit_uic_conversion::into_event(event.clone())
        {
            let mut root_module = block_on(self.root_module.lock());

            let Some(window_state) = &mut self.window_state else {
                return;
            };

            match &uic_event {
                Event::CloseRequested => {
                    std::process::exit(1);
                }
                Event::RedrawRequested => {
                    // TODO: Move this to `RenderModule`.
                    // TODO: Use a fixed-size buffer instead of a Vec.
                    let mut quads = Vec::new();
                    let ui_effects = root_module.ui.effect();
                    let mut effect_visitor =
                        RenderModuleEffectVisitor { quads: &mut quads };
                    ui_effects.drive_thru(&mut effect_visitor);
                    drop(ui_effects);
                    root_module.render_resources.quads = quads;
                    root_module.render_resources.sync();
                    draw_render_module_onto_render_target(
                        window_state.render_target(),
                        &root_module.render_resources,
                    );
                }
                Event::Resized(new_size) => {
                    root_module.resize(*new_size, &self.app_making_resources);
                    let inner_child_hints = root_module.state.child_hints();
                    window_state.update_with_child_hints(inner_child_hints);
                    window_state
                        .resize(&root_module.render_resources.gpu, *new_size);
                }
                _ => {}
            }

            let _effect_was_handled =
                block_on(root_module.propagate_event(&mut uic_event));
        } else {
            //println!("Unrecognized event: {:?}", event);
        }
    }

    fn user_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _event: DesktopUicRequest,
    ) {
        match _event {
            DesktopUicRequest::AppUpdate => {
                if let Some(window_state) = &self.window_state {
                    window_state.window.request_redraw();
                }
            }
        }
        /* Maybe will go unused? */
    }
}
