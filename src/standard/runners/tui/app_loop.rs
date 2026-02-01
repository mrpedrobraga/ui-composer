use crate::app::composition::algebra::Bubble;
use crate::app::composition::reify::Reify;
use crate::app::input::{CursorEvent, DeviceId, Event};
use crate::runners::tui::render::Canvas;
use crate::runners::tui::{Element, TUIRunner};
use crossterm::cursor::{Hide, MoveTo, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode,
};
use crossterm::style::{
    style, Color, Print, PrintStyledContent, ResetColor, StyledContent, Stylize,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, Clear, ClearType, DisableLineWrap, EnableLineWrap,
    EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::{event, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::ops::DerefMut;
use std::sync::{Arc, Mutex};
use vek::{Extent2, Rect, Rgba, Vec2};

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

                    n.bubble(&mut Event::Resized(Extent2::new(w, h).as_()));

                    redraw_requested = true;
                }
                event::Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Moved {
                        mouse = Vec2::new(mouse_event.column, mouse_event.row).as_::<f32>();

                        n.bubble(&mut Event::Cursor {
                            id: DeviceId(0),
                            event: CursorEvent::Moved { position: mouse },
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

    pub fn redraw<C: Canvas<Pixel = vek::Rgba<u8>>>(
        element: &mut impl Element,
        canvas: &mut C,
        rect: Rect<f32, f32>,
        mouse: Vec2<f32>,
    ) {
        canvas.clear();
        element.draw(canvas, rect.as_());

        //canvas.quad(rect, shaders::image);
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

    fn rect(&mut self, rect: Rect<f32, f32>, color: Self::Pixel)
    where
        Self::Pixel: Clone,
    {
        for y in (rect.y as u16..(rect.y + rect.h) as u16) {
            let _ = self.queue(MoveTo(rect.x as u16, y));

            for x in (rect.x as u16..(rect.x + rect.w) as u16) {
                let color = Color::Rgb {
                    r: color.r,
                    g: color.g,
                    b: color.b,
                };
                let styled_pixel = style('▀').with(color).on(color);
                let _ = self.queue(PrintStyledContent(styled_pixel));
            }
        }

        let _ = self.queue(ResetColor);
    }

    fn clear(&mut self) {
        let _ = self.queue(Clear(ClearType::All));
    }

    fn quad(&mut self, rect: Rect<f32, f32>, shader: impl Fn(Vec2<f32>, f32) -> Rgba<f32>) {
        for screen_y_offset in 0..rect.h as u32 {

            let screen_y = (rect.y as u32 + screen_y_offset) as u16;
            let _ = self.queue(MoveTo(rect.x as u16, screen_y));

            for x in 0..rect.w as u32 {
                let source_y_top = screen_y_offset * 2;
                let source_y_bottom = source_y_top + 1;

                let uv_top = Vec2::new(x as f32, source_y_top as f32) / Vec2::new(rect.w, rect.h * 2.0);
                let uv_bottom = Vec2::new(x as f32, source_y_bottom as f32) / Vec2::new(rect.w, rect.h * 2.0);

                let p_top: Self::Pixel = (shader(uv_top, 0.0) * 255.0).as_();
                let p_bottom: Self::Pixel = (shader(uv_bottom, 0.0) * 255.0).as_();

                let styled_pixel = style('▀')
                    .with(Color::Rgb { r: p_top.r, g: p_top.g, b: p_top.b })
                    .on(Color::Rgb { r: p_bottom.r, g: p_bottom.g, b: p_bottom.b });

                let _ = self.queue(PrintStyledContent(styled_pixel));
            }
        }
        let _ = self.queue(ResetColor);
    }
}
