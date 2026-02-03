use crate::app::composition::algebra::Bubble;
use crate::app::composition::reify::Emit;
use crate::geometry::layout::flow::CartesianFlow;
use crate::geometry::layout::hints::ParentHints;
use crate::geometry::layout::LayoutItem;
use crate::runners::tui::render::shaders::PixelShaderInput;
use crate::runners::tui::render::RenderTui;
use crate::standard::runners::tui::render::Canvas;
use crate::standard::runners::tui::Element;
use crate::state::process::{Pollable, SignalReactItem};
use core::pin::Pin;
use core::task::{Context, Poll};
use crossterm::cursor::MoveTo;
use crossterm::style::{style, Color, PrintStyledContent, ResetColor, StyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use std::io::Stdout;
use std::sync::OnceLock;
use std::time::Instant;
use vek::{Extent2, Rect, Rgba};
use {crate::app::input::Event, vek::Vec2};

/// Trait alias that represents anything that can be laid out to get UI.
pub trait TUI: LayoutItem<Content: Emit<(), Output: UIFragment>> {}
impl<T> TUI for T where T: LayoutItem<Content: Emit<(), Output: UIFragment>> {}

/// Trait alias that represents anything that can:
/// 1. Render;
/// 2. Be polled for reactivity;
/// 3. Handle events;
///
/// TODO: Maybe unify all of these???
pub trait UIFragment: RenderTui + Pollable<()> + Bubble<Event, bool> {}
impl<T> UIFragment for T where T: RenderTui + Pollable<()> + Bubble<Event, bool> {}

#[allow(non_snake_case)]
pub fn Terminal<TUI>(mut ui: TUI) -> TerminalNode<SignalReactItem<impl Signal<Item = TUI::Content>>>
where
    TUI: LayoutItem,
{
    let state = TerminalNodeState {
        size: Default::default(),
        mouse_position: Default::default(),
    };

    let ui = state.size.signal().map(move |window_size| {
        ui.lay(ParentHints {
            rect: Rect::new(0.0, 0.0, window_size.w, window_size.h),
            // TODO: Allow configuring this from the locale/user settings,
            // possibly turning them into signals!
            current_flow_direction: CartesianFlow::LeftToRight,
            current_cross_flow_direction: CartesianFlow::TopToBottom,
            current_writing_flow_direction: CartesianFlow::LeftToRight,
            current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
        })
    });

    TerminalNode {
        ui: SignalReactItem(ui),
        state,
    }
}

pub struct TerminalNode<UI> {
    pub(crate) state: TerminalNodeState,
    pub(crate) ui: UI,
}

pub struct TerminalNodeState {
    pub size: Mutable<Extent2<f32>>,
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
}

impl<UI> Emit<()> for TerminalNode<UI>
where
    UI: Emit<(), Output: Sized + RenderTui + Bubble<Event, bool> + Pollable<()>> + Send,
{
    type Output = TerminalNodeRe<UI::Output>;

    fn reify(self, _: &mut ()) -> Self::Output {
        TerminalNodeRe {
            state: self.state,
            ui: self.ui.reify(&mut ()),
        }
    }
}

pub struct TerminalNodeRe<UI> {
    pub state: TerminalNodeState,
    pub ui: UI,
}

impl<UI> Bubble<Event, bool> for TerminalNodeRe<UI>
where
    UI: Bubble<Event, bool>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
        match cx {
            Event::Resized(new_size) => {
                self.state.size.set(*new_size);
            }
            _ => {}
        };

        self.ui.bubble(cx)
    }
}

impl<UI> Pollable<()> for TerminalNodeRe<UI>
where
    UI: Pollable<()>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut ()) -> Poll<Option<()>> {
        let ui = unsafe { self.map_unchecked_mut(|this| &mut this.ui) };
        ui.poll(cx, resources)
    }
}

impl<UI> Element for TerminalNodeRe<UI>
where
    UI: UIFragment,
{
    fn setup(&mut self) {}

    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        canvas.clear();
        self.ui.draw(canvas, rect);
    }
}

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
