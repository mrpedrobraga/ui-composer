//! # Pipeline
//!
//! The TUI rendering pipeline consists of rendering everything to a 'framebuffer' texture
//! and rendering that to the screen. This is used instead of directly rendering pixels,
//! for compatibility reasons (and to avoid writing to stdout).
//!
//! ## Partial Renders
//!
//! The render can update the stdout partially, by rendering only an AABB. This is useful for huge screens,
//! but, really, terminals don't really have a lot of pixels.

use crate::app::primitives::Processor;
use vek::Vec2;
use {
    crate::app::{input::Event, primitives::Primitive},
    ndarray::Array2,
    vek::{Rgb, Rgba},
};

/// A trait that marks a trait as renderable with this pipeline.
pub trait Render {
    fn draw<C: Canvas>(&self, canvas: &mut C, rect: vek::Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>;
}

/// A simple coloured graphic.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Graphic {
    pub rect: vek::Rect<f32, f32>,
    pub color: vek::Rgba<f32>,
}

impl From<vek::Rect<f32, f32>> for Graphic {
    fn from(value: vek::Rect<f32, f32>) -> Self {
        Graphic {
            rect: value,
            ..Default::default()
        }
    }
}

impl Graphic {
    /// Adapts this graphic with a new colour!
    pub fn with_color(self, color: vek::Rgba<f32>) -> Self {
        Self { color, ..self }
    }
}

impl<Res> Primitive<Res> for Graphic {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Res> Processor<Res> for Graphic {}

impl Render for Graphic {
    fn draw<C>(&self, canvas: &mut C, _rect: vek::Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        let my_rect: vek::Aabr<u32> = self.rect.as_().into_aabr();

        let color: Rgba<u8> = (self.color * 255.0).as_();
        for y in my_rect.min.y..my_rect.max.y {
            for x in my_rect.min.x..my_rect.max.x {
                canvas.put_pixel(Vec2::new(x, y), color);
            }
        }
    }
}

pub trait Canvas {
    type Pixel;

    // Places a single pixel within the frame buffer.
    fn put_pixel(&mut self, position: Vec2<u32>, pixel: Self::Pixel);
}

struct Framebuffer<P> {
    pixels: Array2<P>,
}
impl<P> Canvas for Framebuffer<P> {
    type Pixel = P;

    fn put_pixel(&mut self, position: Vec2<u32>, pixel: P) {
        self.pixels[(position.x as usize, position.y as usize)] = pixel;
    }
}

trait Pixel {
    /// Blends `self` on top of `other`.
    ///
    /// If `self` is opaque, the result equals `self`.
    /// If `self` is transparent, the result equals `other`.
    /// If `self` is translucent, it lerp `self` and `other`
    /// according to `self`'s opacity.
    ///
    /// This operation is non-commutative.
    fn blend_normal(&self, other: Self) -> Self;
}

/// A unit of the beautiful frame buffer.
struct TextModePixel {
    bg_color: Rgba<f32>,
    fg_color: Rgb<f32>,
    character: char,
}

impl Pixel for TextModePixel {
    fn blend_normal(&self, other: Self) -> Self {
        todo!()
    }
}
