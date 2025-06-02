use crossterm::{
    ExecutableCommand, QueueableCommand, cursor,
    style::{self, Stylize},
    terminal,
};
use std::io::{self, Stdout, Write};
use vek::{Rect, Rgb};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    draw_rect(
        &mut stdout,
        Rect::new(8, 16, 64, 16),
        Rgb::new(0.5, 0.4, 0.7),
    )?;
    draw_rect(
        &mut stdout,
        Rect::new(32, 8, 16, 16),
        Rgb::new(0.6, 0.5, 0.3),
    )?;
    draw_text(&mut stdout, Rect::new(8, 16, 10, 10), "Hello world!")?;

    stdout.flush()?;

    stdin.read_line(&mut String::new())?;

    Ok(())
}

fn draw_rect(stdout: &mut Stdout, rect: Rect<u16, u16>, color: vek::Rgb<f32>) -> io::Result<()> {
    for (v, y) in ((rect.y)..(rect.y + rect.h)).enumerate() {
        for (u, x) in ((rect.x)..(rect.x + rect.w)).enumerate() {
            let u = (u as u16 * 255) / rect.w;
            let v = (v as u16 * 255) / rect.h;

            let rgb = color * 255.0;
            //let rgb = rgb.as_();
            let color = style::Color::Rgb {
                r: u as u8,
                g: v as u8,
                b: 0,
            };

            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent(" ".on(color)))?;
        }
    }

    Ok(())
}

fn draw_text<S>(stdout: &mut Stdout, rect: Rect<u16, u16>, text: S) -> io::Result<()>
where
    S: AsRef<str>,
{
    stdout
        .queue(cursor::MoveTo(rect.x, rect.y))?
        .queue(style::PrintStyledContent(
            text.as_ref().black().on(style::Color::Reset),
        ))?;

    Ok(())
}
