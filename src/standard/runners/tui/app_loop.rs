use crate::app::composition::algebra::Bubble;
use crate::app::composition::reify::Reify;
use crate::app::input::{CursorEvent, DeviceId, Event};
use crate::runners::tui::render::Canvas;
use crate::runners::tui::{Element, TUIRunner};
use crossterm::cursor::{Hide, MoveTo, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode,
};
use crossterm::style::{style, Color, PrintStyledContent, ResetColor, StyledContent, Stylize};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::ops::{DerefMut};
use std::sync::{Arc, Mutex};
use vek::{Extent2, Rect, Rgba, Vec2, Vec3};

pub mod shaders;

impl<N> TUIRunner<N>
where
    N: Send + Reify<(), Output: Element + Sized>,
{
    pub(crate) async fn process_events(node_tree: Arc<Mutex<N::Output>>) -> std::io::Result<()> {
        let mut stdout = stdout();

        let (cols, rows) = crossterm::terminal::size()?;
        let mut rect: Rect<f32, f32> = Rect::new(0.0, 0.0, cols as f32, rows as f32);
        let mut mouse: Vec2<f32> = Vec2::new(1.0, 1.0);
        let mut redraw_requested = false;

        {
            let mut lock = node_tree.lock().unwrap();
            let n = lock.deref_mut();
            Self::redraw(n, &mut stdout, rect, mouse);
            let _ = stdout.flush()?;
        }

        loop {
            let event = event::read()?;

            let mut lock = node_tree.lock().unwrap();
            let n = lock.deref_mut();

            match event {
                event::Event::Key(key_event) => {
                    if let KeyCode::Char('q') = key_event.code {
                        break;
                    }
                }
                #[allow(unused)]
                event::Event::Resize(w, h) => {
                    rect.set_extent(Extent2::new(w, h).as_::<f32>());
                    redraw_requested = true;
                }
                event::Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Moved {
                        mouse = Vec2::new(mouse_event.column, mouse_event.row).as_::<f32>();

                        n.bubble(&mut Event::Cursor {
                            id: DeviceId(0),
                            event: CursorEvent::Moved {
                                position: mouse,
                            },
                        });

                        redraw_requested = true;
                    }
                }
                _ => (),
            }

            if (redraw_requested) {
                Self::redraw(n, &mut stdout, rect, mouse);
                let _ = stdout.flush()?;
            }
        }

        Ok(())
    }

    pub fn redraw<C: Canvas<Pixel = vek::Rgba<u8>>>(element: &mut impl Element, canvas: &mut C, rect: Rect<f32, f32>, mouse: Vec2<f32>) {
        // element.draw(canvas, rect.as_());

        canvas.full_screen(rect, shaders::image);

        canvas.put_pixel(mouse.as_(), Rgba::new(255, 255, 255, 0));
    }

    pub fn grab_terminal() -> Result<(), std::io::Error> {
        enable_raw_mode().expect("Couldn't enable raw mode");

        let mut stdout = stdout();
        stdout
            .queue(EnterAlternateScreen)?
            .queue(EnableMouseCapture)?
            .queue(DisableLineWrap)?
            .queue(SetCursorStyle::BlinkingUnderScore)?
            .queue(EnableBracketedPaste)?
            .queue(Hide)?
            .queue(Clear(ClearType::All))?
            .flush()?;

        Ok(())
    }

    pub fn release_terminal() -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        stdout
            .queue(Show)?
            .queue(DisableBracketedPaste)?
            .queue(EnableLineWrap)?
            .queue(DisableMouseCapture)?
            .queue(LeaveAlternateScreen)?
            .flush()?;

        disable_raw_mode().expect("Couldn't disable raw mode.");
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

        let styled_pixel: StyledContent<char> = style('█').with(color).on(color);

        let _ = self
            .queue(MoveTo(position.x as u16, position.y as u16))
            .and_then(|stdout| stdout.queue(PrintStyledContent(styled_pixel)))
            .and_then(|stdout| stdout.queue(ResetColor));
    }

    fn full_screen(&mut self, rect: Rect<f32, f32>, shader: impl Fn(Vec2<f32>, f32) -> Rgba<f32>) {
        for y in (0..(rect.h * 2.0) as u32).step_by(2) {
            let _ = self
                .queue(MoveTo(0u16, (y/2) as u16));

            for x in 0..rect.w as u32 {
                let uv_top = Vec2::new(x as f32, y as f32 * 0.5) / rect.extent();
                let top: Self::Pixel = (shader(uv_top, 0.0) * 255.0).as_();

                let uv_bottom = Vec2::new(x as f32, (y + 1) as f32 * 0.5) / rect.extent();
                let bottom: Self::Pixel = (shader(uv_bottom, 0.0) * 255.0).as_();

                let fg = Color::Rgb { r: top.r, g: top.g, b: top.b };
                let bg = Color::Rgb { r: bottom.r, g: bottom.g, b: bottom.b };

                let styled_pixel = style('▀').with(fg).on(bg);

                let _ = self.queue(PrintStyledContent(styled_pixel));
            }
        }

        let _ = self.queue(ResetColor);
    }
}
