use crate::runners::tui::render::shaders::PixelShaderInput;
use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;
use crossterm::style::{Color, PrintStyledContent, ResetColor, StyledContent, Stylize, style};
use crossterm::terminal::{Clear, ClearType};
use ndarray::Array2;
use std::io::Stdout;
use std::sync::OnceLock;
use std::time::Instant;
use vek::{Rect, Rgba, Vec2};

static START: OnceLock<Instant> = OnceLock::new();

fn start() -> &'static Instant {
    START.get_or_init(Instant::now)
}

impl Canvas for Stdout {
    type Pixel = Rgba<u8>;

    fn put_pixel(&mut self, position: Vec2<u32>, pixel: Self::Pixel) {
        let color = Color::Rgb {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        };

        let styled_pixel: StyledContent<char> = style('█').with(color).on(color);

        let _ = self
            .queue(MoveTo(position.x as u16, position.y as u16))
            .and_then(|stdout| stdout.queue(PrintStyledContent(styled_pixel)))
            .and_then(|stdout| stdout.queue(ResetColor));
    }

    fn rect(&mut self, rect: Rect<f32, f32>, color: Self::Pixel)
    where
        Self::Pixel: Clone,
    {
        let x0 = rect.x as u16;
        let y0 = rect.y as u16;
        let x1 = (rect.x + rect.w) as u16 - 1;
        let y1 = (rect.y + rect.h) as u16 - 1;

        for y in y0..=y1 {
            for x in x0..=x1 {
                if x != x0 && x != x1 && y != y0 && y != y1 {
                    //continue;
                }

                let color = Color::Rgb {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                };

                let styled_pixel = style('▀').with(color).on(color);
                let _ = self.queue(MoveTo(x, y));
                let _ = self.queue(PrintStyledContent(styled_pixel));
            }
        }

        let _ = self.queue(ResetColor);
    }

    fn clear(&mut self) {
        let _ = self.queue(Clear(ClearType::All));
    }

    fn quad(&mut self, rect: Rect<f32, f32>, shader: impl Fn(PixelShaderInput) -> Rgba<f32>) {
        let t0 = start();
        let time = (Instant::now() - *t0).as_secs_f32();

        for y_offset in 0..rect.h as u32 {
            let screen_y = (rect.y as u32 + y_offset) as u16;
            let _ = self.queue(MoveTo(rect.x as u16, screen_y));

            for x_offset in 0..rect.w as u32 {
                let source_y_top = y_offset * 2;
                let source_y_bottom = source_y_top + 1;

                let uv_top = Vec2::new(x_offset as f32, source_y_top as f32)
                    / Vec2::new(rect.w, rect.h * 2.0);
                let uv_bottom = Vec2::new(x_offset as f32, source_y_bottom as f32)
                    / Vec2::new(rect.w, rect.h * 2.0);

                let p_top: Self::Pixel = (shader(PixelShaderInput {
                    uv: uv_top,
                    pixelCoord: Vec2::new(x_offset, y_offset),
                    time,
                }) * 255.0)
                    .as_();
                let p_bottom: Self::Pixel = (shader(PixelShaderInput {
                    uv: uv_bottom,
                    pixelCoord: Vec2::new(x_offset, y_offset),
                    time,
                }) * 255.0)
                    .as_();

                let styled_pixel = style('▀')
                    .with(Color::Rgb {
                        r: p_top.r,
                        g: p_top.g,
                        b: p_top.b,
                    })
                    .on(Color::Rgb {
                        r: p_bottom.r,
                        g: p_bottom.g,
                        b: p_bottom.b,
                    });

                let _ = self.queue(PrintStyledContent(styled_pixel));
            }
        }
        let _ = self.queue(ResetColor);
    }
}

/// Trait that defines a pixel of a canvas.
pub trait Pixel {
    /// Blends `self` on top of `other`.
    ///
    /// If `self` is opaque, the result equals `self`.
    /// If `self` is transparent, the result equals `other`.
    /// If `self` is translucent, it interpolates opaque `self` and `other`
    /// according to `self`'s opacity.
    ///
    /// This operation is non-commutative.
    fn blend_normal(&self, other: Self) -> Self;
}

/// A unit of the beautiful frame buffer.
#[derive(Copy, Clone, PartialEq)]
pub struct TextModePixel {
    bg_color: Rgba<f32>,
    fg_color: Rgba<f32>,
    character: char,
}

impl Pixel for TextModePixel {
    fn blend_normal(&self, other: Self) -> Self {
        let self_bg_color_opaque =
            Rgba::new(self.bg_color.r, self.bg_color.g, self.bg_color.b, 1.0);

        // The background colour is just the NORMAL blend of the background colours.
        let bg_color = lerp(other.bg_color, self_bg_color_opaque, self.bg_color.a);

        let (fg_color, character) = if self.character == ' ' {
            (
                // The bottom pixel will have its character partially "occluded" by
                // the top pixel's background.
                lerp(other.fg_color, self_bg_color_opaque, self.bg_color.a),
                other.character,
            )
        } else {
            // The top pixel's character can not be occluded.
            // If it has any alpha, that won't be resolved right now,
            // it will be resolved when drawing to the canvas...
            (self.fg_color, self.character)
        };

        TextModePixel {
            bg_color,
            fg_color,
            character,
        }
    }
}

/// Interpolates linearly between A and B.
fn lerp(a: Rgba<f32>, b: Rgba<f32>, factor: f32) -> Rgba<f32> {
    a * (factor) + b * (1.0 - factor)
}

pub trait Canvas {
    type Pixel;

    /// Places a single pixel within the frame buffer.
    fn put_pixel(&mut self, position: Vec2<u32>, pixel: Self::Pixel);

    /// Draws a rectangle with a single colour.
    fn rect(&mut self, rect: Rect<f32, f32>, color: Self::Pixel)
    where
        Self::Pixel: Clone;

    /// Clears the canvas.
    fn clear(&mut self);

    /// Draws a quad with an image generated by a 'shader'.
    fn quad(&mut self, rect: Rect<f32, f32>, shader: impl Fn(PixelShaderInput) -> Rgba<f32>);
}

#[allow(unused)]
struct Framebuffer<P> {
    pixels: Array2<P>,
}

impl<P> Canvas for Framebuffer<P>
where
    P: Default,
{
    type Pixel = P;

    fn put_pixel(&mut self, position: Vec2<u32>, pixel: P) {
        self.pixels[(position.x as usize, position.y as usize)] = pixel;
    }

    fn rect(&mut self, rect: Rect<f32, f32>, color: Self::Pixel)
    where
        Self::Pixel: Clone,
    {
        for y in 0..rect.h as usize {
            for x in 0..rect.w as usize {
                self.put_pixel(Vec2::new(x as u32, y as u32), color.clone());
            }
        }
    }

    fn clear(&mut self) {
        for pixel in &mut self.pixels {
            *pixel = P::default();
        }
    }

    fn quad(&mut self, _rect: Rect<f32, f32>, _shader: impl Fn(PixelShaderInput) -> Rgba<f32>) {
        todo!("Handle this.")
    }
}
