use crate::app::composition::algebra::Bubble;
use crate::app::composition::effects::signal::{React, SignalReactExt};
use crate::app::composition::elements::{Blueprint, Element};
use crate::app::composition::layout::hints::ParentHints;
use crate::app::composition::visit::DriveThru;
use crate::geometry::flow::CartesianFlow;
use crate::runners::tui::TUI;
use crate::runners::tui::render::canvas::{PixelCanvas, TextModePixel};
use crate::runners::tui::runner::TerminalEnvironment;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect};
use {crate::app::input::Event, vek::Vec2};

#[allow(non_snake_case)]
pub fn Terminal<UI>(
    mut ui: UI,
) -> TerminalBlueprint<
    React<impl Signal<Item = UI::Blueprint>, TerminalEnvironment>,
>
where
    UI: TUI,
{
    let size = crossterm::terminal::size()
        .map(|(x, y)| Extent2::new(x, y))
        .unwrap_or(Extent2::new(8, 8));

    let render_target = PixelCanvas::new(size.as_());

    let state = TerminalState {
        size: Mutable::new(size.as_()),
        mouse_position: Mutable::new(None),
        render_target,
    };

    let ui = state
        .size
        .signal()
        .map(move |terminal_size| {
            ui.lay(ParentHints {
                rect: Rect::new(0.0, 0.0, terminal_size.w, terminal_size.h),
                // TODO: Turn these into signals, maybe?
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction:
                    CartesianFlow::TopToBottom,
            })
        })
        .react();

    TerminalBlueprint { ui, state }
}

pub struct TerminalBlueprint<UI> {
    pub(crate) state: TerminalState,
    pub(crate) ui: UI,
}

pub struct TerminalState {
    pub size: Mutable<Extent2<f32>>,
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
    pub render_target: PixelCanvas<TextModePixel>,
}

impl<UI> Blueprint<TerminalEnvironment> for TerminalBlueprint<UI>
where
    UI: Blueprint<TerminalEnvironment> + Send,
{
    type Element = TerminalElement<UI::Element>;

    fn make(self, env: &TerminalEnvironment) -> Self::Element {
        TerminalElement {
            state: self.state,
            ui: self.ui.make(env),
        }
    }
}

pub struct TerminalElement<UI> {
    pub state: TerminalState,
    pub ui: UI,
}

impl<UI> Bubble<Event, bool> for TerminalElement<UI>
where
    UI: Bubble<Event, bool>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
        if let Event::Resized(new_size) = cx {
            self.state.size.set(*new_size);
        };

        self.ui.bubble(cx)
    }
}

impl<UI> Element<TerminalEnvironment> for TerminalElement<UI>
where
    UI: Element<TerminalEnvironment>,
{
    type Effect = ();

    fn effect(&self) -> Self::Effect {
        todo!()
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &TerminalEnvironment,
    ) -> Poll<Option<()>> {
        let mut ui = unsafe { self.map_unchecked_mut(|this| &mut this.ui) };

        let inner = ui.as_mut().poll(cx, env);

        match inner {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(_)) => {
                let ui_effects = ui.effect();
                let mut env = TerminalEnvironment;
                ui_effects.drive_thru(&mut env);

                Poll::Ready(Some(()))
            }
        }
    }
}
