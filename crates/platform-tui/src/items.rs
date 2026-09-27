use crate::render::present_canvas_to_terminal;
use crate::runner::{TerminalBlueprintResources, TerminalEnvironment};
use crate::Tui;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use pin_project::pin_project;
use ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::layout::Ui;
use ui_composer_core::app::composition::visit::DriveThru;
use ui_composer_input::event::{CursorEvent, Event};
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_math::glamour::Rect;
use ui_composer_math::prelude::{Point2, Size2};
use ui_composer_state::Slot;

pub struct TerminalBlueprint<U> {
    pub(crate) state: TerminalState,
    pub(crate) ui: U,
}

pub struct TerminalState {
    pub size: Mutable<Size2>,
    pub mouse_position: Mutable<Option<Point2>>,
    pub render_target: PixelCanvas<TextModePixel>,
}

impl<U: Ui<TerminalEnvironment>> Blueprint<TerminalEnvironment> for TerminalBlueprint<U> {
    type Output = TerminalElement<U>;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        TerminalElement {
            state: self.state,
            ui: self.ui,
        }
    }
}

#[pin_project(project = TerminalElementProj)]
pub struct TerminalElement<U> {
    pub state: TerminalState,
    #[pin]
    pub ui: U,
}

impl<Ui> Bubble<Event, bool> for TerminalElement<Ui> {
    async fn bubble(&mut self, cx: &mut Event) -> bool {
        if let Event::Resized(new_size) = cx {
            self.state.render_target.resize(new_size.as_());
            self.state.size.set(*new_size);
        };

        if let Event::Cursor {
            id: _,
            event: CursorEvent::Moved { position },
        } = cx
        {
            self.state.mouse_position.put(Some(*position));
        }

        if let Event::Cursor {
            id: _,
            event: CursorEvent::Exited,
        } = cx
        {
            self.state.mouse_position.put(None);
        }

        // TODO: Bubble the event down the UI!
        //self.ui.bubble(cx).await
        false
    }
}

impl<U> Element<TerminalEnvironment> for TerminalElement<U>
where
    U: Ui<TerminalEnvironment>,
{
    type Effect = ();
    type Blueprint = TerminalBlueprint<U>;

    fn effect(&self) -> Self::Effect {}

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &TerminalBlueprintResources,
    ) -> Poll<Option<()>> {
        let TerminalElementProj { state, mut ui } = self.project();

        /* TODO: Get these from somewhere... */
        let parent_hints = ParentHints {
            rect: Rect::new(Point2::ZERO, state.size.get()),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction: CartesianFlow::TopToBottom,
            },
        };

        let inner = ui.as_mut().poll_change(cx, env, parent_hints);

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

                /* Draws a cute little mouse cursor... useful for troubleshooting certain interactions. */
                // if let Some(mouse_position) = state.mouse_position.get() {
                //     vis.canvas.put_pixel(
                //         mouse_position.as_(),
                //         TextModePixel {
                //             bg_color: Srgba::zero(),
                //             fg_color: Srgba::black(),
                //             character: '\u{f01bf}',
                //         },
                //     )
                // }

                present_canvas_to_terminal(vis.canvas)
                    .expect("Failed to present canvas to terminal?");

                Poll::Ready(Some(()))
            }
        }
    }

    fn update(&mut self, _: TerminalBlueprint<U>, _: &TerminalBlueprintResources) {
        unimplemented!()
    }
}

pub struct TerminalEffectVisitor<'fx> {
    pub canvas: &'fx mut PixelCanvas<TextModePixel>,
}

#[allow(non_snake_case)]
pub fn Terminal<U>(ui: U) -> TerminalBlueprint<U>
where
    U: Tui,
{
    let size = crossterm::terminal::size()
        .map(|(x, y)| Size2::<u16>::new(x, y))
        .unwrap_or(Size2::new(8, 8));

    let render_target = PixelCanvas::new(size.as_());

    let state = TerminalState {
        size: Mutable::new(size.as_()),
        mouse_position: Mutable::new(None),
        render_target,
    };

    TerminalBlueprint { ui, state }
}
