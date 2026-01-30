use crate::app::composition::algebra::Bubble;
use crate::app::composition::reify::Reify;
use crate::geometry::layout::flow::CartesianFlow;
use crate::geometry::layout::hints::ParentHints;
use crate::geometry::layout::LayoutItem;
use crate::standard::runners::tui::render::Canvas;
use crate::standard::runners::tui::{Node, NodeRe};
use crate::state::process::{Pollable, SignalReactItem};
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect, Rgba};
use {crate::app::input::Event, vek::Vec2};
use crate::runners::tui::render::RenderTui;

/// An auto trait that simplifies description of a type that can be a building block in TUI.
pub trait TUI: LayoutItem<Content: Reify<(), Output: RenderTui + Pollable<()> + Bubble<Event, bool>>> {}
impl<T> TUI for T where T: LayoutItem<Content: Reify<(), Output: RenderTui + Pollable<()> + Bubble<Event, bool>>> {}

#[allow(non_snake_case)]
pub fn Terminal<UI>(mut ui: UI) -> TerminalNode<SignalReactItem<impl Signal<Item = UI::Content>>>
where
    UI: LayoutItem,
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

impl<UI> Node for TerminalNode<UI>
where
    UI: Reify<(), Output: Sized + RenderTui + Bubble<Event, bool> + Pollable<()>>
        + Send,
{
    type Output = TerminalNodeRe<UI::Output>;

    fn reify(self) -> Self::Output {
        TerminalNodeRe { ui: self.ui.reify(&mut ()) }
    }
}

pub struct TerminalNodeRe<UI> {
    pub(crate) ui: UI,
}

impl<UI> Bubble<Event, bool> for TerminalNodeRe<UI>
where
    UI: Bubble<Event, bool>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
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

impl<UI> NodeRe for TerminalNodeRe<UI>
where
    UI: RenderTui + Pollable<()> + Bubble<Event, bool>,
{
    fn setup(&mut self) {}

    fn draw<C>(&self, canvas: &mut C, rect: vek::Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        // Clear canvas

        let item_size = Extent2::new(100.0, 100.0);
        let texture_size = Into::<Vec2<f32>>::into(rect.extent().as_());
        let item_position = (texture_size - Into::<Vec2<f32>>::into(item_size)) / 2.0;

        let item_rect = vek::Rect::from((item_position, item_size)).as_();
        let item_aabr: vek::Aabr<u16> = item_rect.into();

        self.ui.draw(canvas, item_rect);

        let color: vek::Rgba<u8> = vek::Rgba {
            r: 0x77,
            g: 0x3a,
            b: 0xf4,
            a: 0xff,
        };
        for y in item_aabr.min.y..item_aabr.max.y {
            for x in item_aabr.min.x..item_aabr.max.x {
                canvas.put_pixel(Vec2::new(x as u32, y as u32), color);
            }
        }

        // Flush canvas to framebuffer, possibly!
    }
}
