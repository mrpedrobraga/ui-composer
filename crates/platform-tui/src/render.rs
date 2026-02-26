use ui_composer_canvas::{PixelCanvas, TextModePixel};

use crossterm::QueueableCommand as _;
use crossterm::cursor::MoveTo;
use crossterm::style::{
    Print, ResetColor, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{
    BeginSynchronizedUpdate, Clear, ClearType, EndSynchronizedUpdate,
};
use std::io::{BufWriter, Stdout, Write, stdout};

pub fn present_canvas_to_terminal(
    canvas: &mut PixelCanvas<TextModePixel>,
) -> std::io::Result<()> {
    // Using a BufWriter so we don't write all at once.
    let mut screen_buffer = BufWriter::with_capacity(
        canvas.size.width as usize * canvas.size.height as usize * 32,
        stdout(),
    );

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

    for y in 0..canvas.size.height {
        let mut cursor_x: Option<usize> = None;
        for x in 0..canvas.size.width {
            let index = y * canvas.size.width + x;
            let back_pixel = &canvas.back_buffer[index as usize];
            let front_pixel = &mut canvas.front_buffer[index as usize];

            if back_pixel == front_pixel && !canvas.needs_full_redraw {
                continue;
            }
            *front_pixel = *back_pixel;

            // If the cursor isn't in the correct place, queue moving it.
            if cursor_x != Some(x as usize) {
                screen_buffer.queue(MoveTo(x as u16, y as u16))?;
            }
            cursor_x = Some((x + 1) as usize);

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
    let r_fg = (back_pixel.fg_color.red.clamp(0.0, 1.0) * 255.0) as u8;
    let g_fg = (back_pixel.fg_color.green.clamp(0.0, 1.0) * 255.0) as u8;
    let b_fg = (back_pixel.fg_color.blue.clamp(0.0, 1.0) * 255.0) as u8;

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

    let r_bg = (back_pixel.bg_color.red.clamp(0.0, 1.0) * 255.0) as u8;
    let g_bg = (back_pixel.bg_color.green.clamp(0.0, 1.0) * 255.0) as u8;
    let b_bg = (back_pixel.bg_color.blue.clamp(0.0, 1.0) * 255.0) as u8;

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
