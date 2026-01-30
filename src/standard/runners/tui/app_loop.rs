use crate::app::composition::algebra::Bubble;
use crate::app::input::{CursorEvent, DeviceId, Event};
use crate::runners::tui::render::Canvas;
use crate::runners::tui::{RuntimeElement, TUIRunner};
use crate::standard::runners::tui::Element;
use crossterm::cursor::{Hide, MoveTo, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode,
};
use crossterm::style::{
    style, Color, PrintStyledContent, ResetColor,
    StyledContent, Stylize,
};
use crossterm::terminal::{Clear, ClearType, DisableLineWrap, EnableLineWrap};
use crossterm::{event, ExecutableCommand};
use std::io::{stdout, Stdout, Write};
use vek::{Rect, Rgba, Vec2};

impl<N> TUIRunner<N>
where
    N: Element,
{
    pub(crate) async fn app_loop(mut node_tree: N::Output) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout
            .execute(EnableMouseCapture)?
            .execute(DisableLineWrap)?
            .execute(SetCursorStyle::BlinkingUnderScore)?
            .execute(EnableBracketedPaste)?
            .execute(Hide)?
            .execute(Clear(ClearType::All))?
            .flush()?;

        node_tree.draw(&mut stdout, Rect::new(0, 0, 16, 16));
        loop {
            let event = event::read()?;

            match event {
                event::Event::Key(key_event) => {
                    if let KeyCode::Char('q') = key_event.code {
                        break;
                    }
                }
                #[allow(unused)]
                event::Event::Resize(w, h) => {
                    node_tree.draw(&mut stdout, Rect::new(0, 0, w, h));
                    stdout.put_pixel(Vec2::new(1, 1), Rgba::cyan());
                    let _ = stdout.flush();
                }
                event::Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Moved {
                        node_tree.bubble(&mut Event::Cursor {
                            id: DeviceId(0),
                            event: CursorEvent::Moved {
                                position: Vec2::new(
                                    mouse_event.column as f32,
                                    mouse_event.row as f32,
                                ),
                            },
                        });
                    }
                }
                _ => (),
            }
        }

        stdout
            .execute(Show)?
            .execute(DisableBracketedPaste)?
            .execute(EnableLineWrap)?
            .execute(DisableMouseCapture)?
            .execute(Clear(ClearType::All))?
            .flush()?;

        Ok(())
    }
}

impl Canvas for Stdout {
    type Pixel = Rgba<u8>;

    fn put_pixel(&mut self, position: Vec2<u32>, pixel: Self::Pixel) {
        let color = Color::Rgb {
            r: pixel.r,
            g: pixel.g,
            b: pixel.b,
        };

        let styled_pixel: StyledContent<char> = style('█')
            .with(color) // foreground
            .on(color); // background

        let _ = self
            .execute(MoveTo(position.x as u16, position.y as u16))
            .and_then(|stdout| stdout.execute(PrintStyledContent(styled_pixel)))
            .and_then(|stdout| stdout.execute(ResetColor));
    }
}
