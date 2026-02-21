use crate::render::present_canvas_to_terminal;
use crate::runner::TerminalEnvironment;
use crate::TUI;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use pin_project::pin_project;
use ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::effects::signal::{React, SignalReactExt};
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::visit::DriveThru;
use ui_composer_geometry::flow::{CartesianFlow, CurrentFlow};
use ui_composer_input::event::Event;
use vek::{Extent2, Rect, Vec2};

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

#[pin_project(project = TerminalElementProj)]
pub struct TerminalElement<UI> {
    pub state: TerminalState,
    #[pin]
    pub ui: UI,
}

impl<UI> Bubble<Event, bool> for TerminalElement<UI>
where
    UI: Bubble<Event, bool>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
        if let Event::Resized(new_size) = cx {
            self.state.render_target.resize(new_size.as_());
            self.state.size.set(*new_size);
        };

        self.ui.bubble(cx)
    }
}

impl<UI> Element<TerminalEnvironment> for TerminalElement<UI>
where
    UI: Element<TerminalEnvironment>,
{
    type Effect<'a>
        = ()
    where
        UI: 'a;

    fn effect(&self) -> Self::Effect<'_> {
        todo!()
    }

    fn poll(self: Pin<&mut Self>, cx: &mut Context, env: &TerminalEnvironment) -> Poll<Option<()>> {
        let TerminalElementProj { state, mut ui } = self.project();

        let inner = ui.as_mut().poll(cx, env);

        match inner {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(_)) => {
                let ui_effects = ui.effect();
                state.render_target.clear();
                let mut vis = TerminalEffectVisitor {
                    canvas: &mut state.render_target,
                };
                ui_effects.drive_thru(&mut vis);
                present_canvas_to_terminal(vis.canvas)
                    .expect("Failed to present canvas to terminal?");

                Poll::Ready(Some(()))
            }
        }
    }
}

pub struct TerminalEffectVisitor<'fx> {
    pub canvas: &'fx mut PixelCanvas<TextModePixel>,
}

#[allow(non_snake_case)]
pub fn Terminal<UI>(
    mut ui: UI,
) -> TerminalBlueprint<React<impl Signal<Item = UI::Blueprint>, TerminalEnvironment>>
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
                current_flow: CurrentFlow {
                    current_flow_direction: CartesianFlow::LeftToRight,
                    current_cross_flow_direction: CartesianFlow::TopToBottom,
                    current_writing_flow_direction: CartesianFlow::LeftToRight,
                    current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
                },
            })
        })
        .react();

    TerminalBlueprint { ui, state }
}
