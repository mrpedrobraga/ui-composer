// use glyphon::{Cache, FontSystem, SwashCache, TextRenderer};
// use vek::Rect;
// use wgpu::{ColorTargetState, MultisampleState, TextureFormat};

// use crate::gpu::render_target::GPURenderTarget;

// use super::GPURenderPipeline;

// /// The pipeline for rendering text.
// pub struct TextRenderPipeline {
//     pipeline: wgpu::RenderPipeline,
// }

// impl GPURenderPipeline for TextRenderPipeline {
//     fn install_on_render_pass<'pass>(&'pass self, render_pass: &mut wgpu::RenderPass<'pass>) {
//         todo!()
//     }
// }

// impl TextRenderPipeline {
//     pub fn singleton<'a, Target>(
//         adapter: &'a wgpu::Adapter,
//         device: &'a wgpu::Device,
//         queue: &'a wgpu::Queue,
//         render_target_formats: &'a [Option<ColorTargetState>],
//     ) where
//         Target: GPURenderTarget,
//     {
//         let mut font_system = FontSystem::new();
//         font_system.db_mut().load_font_file("./TestFont.ttf");
//         let swash_cashe = SwashCache::new();
//         let cache = Cache::new(device);
//         let viewport = glyphon::Viewport::new(device, &cache);
//         let mut atlas =
//             glyphon::TextAtlas::new(device, adapter, queue, cache, TextureFormat::Bgra8UnormSrgb);
//         let text_renderer =
//             TextRenderer::new(&mut atlas, device, MultisampleState::default(), None);
//     }
// }

// /// A single rectangle of text that can be rendered to the screen.
// pub struct TextArea {
//     buffer: glyphon::Buffer,
//     rect: Rect<f32, f32>,
// }
