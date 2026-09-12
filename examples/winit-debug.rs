use std::sync::Arc;
use ui_composer_basic_ui::primitives::graphic::Graphic;
use ui_composer_core::app::composition::elements::Element;
use ui_composer_math::palette::Srgba;
use ui_composer_platform_winit::runner::WinitEnvironment;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent::{CloseRequested, RedrawRequested, Resized};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};

use ui_composer_math::glamour::{Point2, Rect};
use ui_composer_math::prelude::Size2;
use ui_composer_platform_winit::gpu::Gpu;
use ui_composer_platform_winit::render::{RenderPipeline, RenderResources, RenderTarget, render};
use ui_composer_platform_winit::window::WindowRenderTarget;
use ui_composer_platform_winit::{wgpu, winit};

fn App2(rect: Rect) -> (Graphic, Graphic) {
    (
        Graphic {
            rect,
            color: Srgba::new(0.0, 0.0, 1.0, 1.0),
        },
        Graphic {
            rect: Rect::new(Point2::new(10.0, 10.0), Size2::new(20.0, 20.0)),
            color: Srgba::new(1.0, 1.0, 0.0, 1.0),
        },
    )
}

pub struct DirectWindowApp {
    window: Option<Arc<Window>>,
    render_target: Option<WindowRenderTarget>,
    render_pipeline: Option<RenderPipeline>,
    render_resources: Option<RenderResources>,
}

impl DirectWindowApp {
    pub fn new() -> Self {
        Self {
            window: None,
            render_target: None,
            render_pipeline: None,
            render_resources: None,
        }
    }

    fn update_graphics(&mut self, width: f32, height: f32) {
        if let Some(res) = &mut self.render_resources {
            let rect = Rect::new(Point2::ZERO, Size2::new(width, height));
            let (g1, g2) = App2(rect);

            res.quads.clear();
            // Push graphics/quads into res.quads depending on your quad struct definition
            res.quads.push(Element::<WinitEnvironment>::effect(&g1).as_quad_instance());
            res.quads.push(Element::<WinitEnvironment>::effect(&g2).as_quad_instance());
        }
    }
}

impl ApplicationHandler for DirectWindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    WindowAttributes::default()
                        .with_title("Direct Window Test - Two Quads")
                        .with_inner_size(PhysicalSize::new(640, 360)),
                )
                .expect("Failed to create window"),
        );

        let gpu = futures::executor::block_on(Gpu::new());
        let render_target = WindowRenderTarget::new(&gpu, window.clone());
        let render_pipeline = RenderPipeline::new(&gpu, wgpu::TextureFormat::Bgra8UnormSrgb);
        let render_resources = RenderResources::new(gpu, &render_pipeline);

        self.window = Some(window);
        self.render_target = Some(render_target);
        self.render_pipeline = Some(render_pipeline);
        self.render_resources = Some(render_resources);

        self.update_graphics(640.0, 360.0);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            Resized(physical_size) => {
                if physical_size.width == 0 || physical_size.height == 0 {
                    return;
                }
                let size_f32 = Size2::new(physical_size.width as f32, physical_size.height as f32);
                let size_u32 = ui_composer_math::glamour::Size2::new(physical_size.width, physical_size.height);

                if let Some(res) = &mut self.render_resources {
                    res.uniforms.resize(size_f32);
                    if let Some(target) = &mut self.render_target {
                        target.resize(&res.gpu, size_u32);
                    }
                }

                self.update_graphics(size_f32.width, size_f32.height);

                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            RedrawRequested => {
                if let (Some(target), Some(pipeline), Some(resources)) = (
                    &mut self.render_target,
                    &mut self.render_pipeline,
                    &mut self.render_resources,
                ) {
                    resources.sync(&resources.gpu);
                    render(target, pipeline, resources);
                }
            }
            CloseRequested => {
                event_loop.exit();
            }
            _ => {}
        }
    }
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .without_time()
        .init();

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = DirectWindowApp::new();
    let _ = event_loop.run_app(&mut app);
}