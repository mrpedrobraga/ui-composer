use crate::nodes::TerminalEffectVisitor;
use crate::runner::TerminalEnvironment;
use ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::algebra::{Bubble, Empty};
use ui_composer_core::app::composition::effects::ElementEffect;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::visit::{Apply, DriveThru};
use ui_composer_core::app::input::Event;
use vek::Rect;
use vek::Rgba;

/// An effect that describes rendering of a quad in the terminal.
#[derive(Debug)]
pub struct RenderQuad(pub Rect<f32, f32>, pub Rgba<f32>);

//impl ElementEffect<WinitEnvironment> for RenderQuad {}

impl ElementEffect<TerminalEnvironment> for RenderQuad {}
impl<'fx> Apply<RenderQuad> for TerminalEffectVisitor<'fx> {
    fn visit(&mut self, RenderQuad(rect, color): &RenderQuad) {
        self.canvas.rect(
            rect.as_(),
            TextModePixel {
                bg_color: *color,
                fg_color: Rgba::new_transparent(0.0, 0.0, 0.0),
                character: ' ',
            },
        );
    }
}
impl Apply<RenderQuad> for () {
    fn visit(&mut self, _: &RenderQuad) {
        /* Do nothing for now */
    }
}
impl<V> DriveThru<V> for RenderQuad
where
    V: Apply<Self>,
{
    fn drive_thru(&self, visitor: &mut V) {
        visitor.visit(self);
    }
}

#[allow(non_snake_case)]
pub fn Graphic() -> Graphic {
    Graphic {
        rect: Rect::default(),
        color: Rgba::default(),
    }
}

/// A simple coloured graphic.
#[derive(Default, Clone, Copy, PartialEq)]
pub struct Graphic {
    pub rect: Rect<f32, f32>,
    pub color: Rgba<f32>,
}

impl From<Rect<f32, f32>> for Graphic {
    fn from(value: Rect<f32, f32>) -> Self {
        Graphic {
            rect: value,
            ..Default::default()
        }
    }
}

impl Graphic {
    pub fn new(rect: Rect<f32, f32>, color: Rgba<f32>) -> Self {
        Self { rect, color }
    }

    /// Adapts this graphic with a new colour!
    pub fn with_color(self, color: Rgba<f32>) -> Self {
        Self { color, ..self }
    }

    /// Adapts this graphic with a new rect!
    pub fn with_rect(self, rect: Rect<f32, f32>) -> Self {
        Self { rect, ..self }
    }
}

impl Bubble<Event, bool> for Graphic {
    fn bubble(&mut self, _: &mut Event) -> bool {
        Empty::empty()
    }
}

impl Blueprint<TerminalEnvironment> for Graphic {
    type Element = Self;

    fn make(self, _: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Graphic {
    type Effect<'fx> = RenderQuad;

    fn effect(&self) -> Self::Effect<'_> {
        RenderQuad(self.rect, self.color)
    }
}

// impl Lerp for Graphic {
//     fn linear_interpolate(self, other: Self, t: f32) -> Self {
//         Graphic {
//             rect: Rect {
//                 x: self.rect.x.linear_interpolate(other.rect.x, t),
//                 y: self.rect.y.linear_interpolate(other.rect.y, t),
//                 w: self.rect.w.linear_interpolate(other.rect.w, t),
//                 h: self.rect.h.linear_interpolate(other.rect.h, t),
//             },
//             color: Rgba {
//                 r: self.color.r.linear_interpolate(other.color.r, t),
//                 g: self.color.g.linear_interpolate(other.color.g, t),
//                 b: self.color.b.linear_interpolate(other.color.b, t),
//                 a: self.color.a.linear_interpolate(other.color.a, t),
//             },
//         }
//     }
// }

use crossterm::cursor::MoveTo;
use crossterm::style::{
    Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate,
};
use crossterm::QueueableCommand as _;
use std::io::{stdout, BufWriter, Stdout, Write};

pub fn present_canvas_to_terminal(
    canvas: &mut PixelCanvas<TextModePixel>,
) -> std::io::Result<()> {
    // Using a BufWriter so we don't write all at once.
    let mut screen_buffer =
        BufWriter::with_capacity(canvas.size.w * canvas.size.h * 32, stdout());

    // Notify the terminal to start a "transaction" and not update the screen until we're done drawing.
    screen_buffer.queue(BeginSynchronizedUpdate)?;

    // If the screen has been recently resized, we need to fully clear it.
    if canvas.needs_full_redraw {
        screen_buffer.queue(Clear(ClearType::All))?;
    }

    // Send drawing commands to the buffer, but only of what changed.
    draw_diff(canvas, &mut screen_buffer)?;

    // Finish the transaction and send the commands.
    screen_buffer.queue(ResetColor)?;
    screen_buffer.queue(EndSynchronizedUpdate)?;
    screen_buffer.flush()?;

    Ok(())
}

fn draw_diff(
    canvas: &mut PixelCanvas<TextModePixel>,
    screen_buffer: &mut BufWriter<Stdout>,
) -> std::io::Result<()> {
    let mut current_fg: Option<(u8, u8, u8)> = None;
    let mut current_bg: Option<(u8, u8, u8)> = None;

    for y in 0..canvas.size.h {
        let mut cursor_x: Option<usize> = None;
        for x in 0..canvas.size.w {
            let index = y * canvas.size.w + x;
            let back_pixel = &canvas.back_buffer[index];
            let front_pixel = &mut canvas.front_buffer[index];

            if back_pixel == front_pixel && !canvas.needs_full_redraw {
                continue;
            }
            *front_pixel = *back_pixel;

            // If the cursor isn't in the correct place, queue moving it.
            if cursor_x != Some(x) {
                screen_buffer.queue(MoveTo(x as u16, y as u16))?;
            }
            cursor_x = Some(x + 1);

            // Queues sending the pixel to the terminal.
            queue_pixel(
                screen_buffer,
                &mut current_fg,
                &mut current_bg,
                back_pixel,
            )?;
        }
    }

    canvas.needs_full_redraw = false;

    Ok(())
}

fn queue_pixel(
    screen_buffer: &mut BufWriter<Stdout>,
    current_fg: &mut Option<(u8, u8, u8)>,
    current_bg: &mut Option<(u8, u8, u8)>,
    back_pixel: &TextModePixel,
) -> std::io::Result<()> {
    let r_fg = (back_pixel.fg_color.r.clamp(0.0, 1.0) * 255.0) as u8;
    let g_fg = (back_pixel.fg_color.g.clamp(0.0, 1.0) * 255.0) as u8;
    let b_fg = (back_pixel.fg_color.b.clamp(0.0, 1.0) * 255.0) as u8;

    if *current_fg != Some((r_fg, g_fg, b_fg)) {
        screen_buffer.queue(SetForegroundColor(
            crossterm::style::Color::Rgb {
                r: r_fg,
                g: g_fg,
                b: b_fg,
            },
        ))?;
        *current_fg = Some((r_fg, g_fg, b_fg));
    }

    let r_bg = (back_pixel.bg_color.r.clamp(0.0, 1.0) * 255.0) as u8;
    let g_bg = (back_pixel.bg_color.g.clamp(0.0, 1.0) * 255.0) as u8;
    let b_bg = (back_pixel.bg_color.b.clamp(0.0, 1.0) * 255.0) as u8;

    if *current_bg != Some((r_bg, g_bg, b_bg)) {
        screen_buffer.queue(SetBackgroundColor(
            crossterm::style::Color::Rgb {
                r: r_bg,
                g: g_bg,
                b: b_bg,
            },
        ))?;
        *current_bg = Some((r_bg, g_bg, b_bg));
    }

    screen_buffer.queue(Print(back_pixel.character))?;

    Ok(())
}
