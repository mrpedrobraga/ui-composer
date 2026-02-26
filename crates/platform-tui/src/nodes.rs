use crate::Tui;
use crate::render::present_canvas_to_terminal;
use crate::runner::{TerminalBlueprintResources, TerminalEnvironment};
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use pin_project::pin_project;
use ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::effects::signal::{
    IntoBlueprint, React,
};
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::visit::DriveThru;
use ui_composer_input::event::{CursorEvent, Event};
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_state::Slot;
use vek::{Extent2, Rect, Vec2};

pub struct TerminalBlueprint<UiBlueprint> {
    pub(crate) state: TerminalState,
    pub(crate) ui: UiBlueprint,
}

pub struct TerminalState {
    pub size: Mutable<Extent2<f32>>,
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
    pub render_target: PixelCanvas<TextModePixel>,
}

impl<UiBlueprint> Blueprint<TerminalEnvironment>
    for TerminalBlueprint<UiBlueprint>
where
    UiBlueprint: Blueprint<TerminalEnvironment> + Send,
{
    type Element = TerminalElement<UiBlueprint::Element>;

    fn make(self, env: &TerminalBlueprintResources) -> Self::Element {
        TerminalElement {
            state: self.state,
            ui: self.ui.make(env),
        }
    }
}

#[pin_project(project = TerminalElementProj)]
pub struct TerminalElement<UiElement> {
    pub state: TerminalState,
    #[pin]
    pub ui: UiElement,
}

impl<UiElement> Bubble<Event, bool> for TerminalElement<UiElement>
where
    UiElement: Bubble<Event, bool>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
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

        self.ui.bubble(cx)
    }
}

impl<UiElement> Element<TerminalEnvironment> for TerminalElement<UiElement>
where
    UiElement: Element<TerminalEnvironment>,
{
    type Effect<'a>
        = ()
    where
        UiElement: 'a;

    fn effect(&self) -> Self::Effect<'_> {
        todo!()
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &TerminalBlueprintResources,
    ) -> Poll<Option<()>> {
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

                /* Draws a cute little mouse cursor... useful for troubleshooting certain interactions. */
                // if let Some(mouse_position) = state.mouse_position.get() {
                //     vis.canvas.put_pixel(
                //         mouse_position.as_(),
                //         TextModePixel {
                //             bg_color: Rgba::zero(),
                //             fg_color: Rgba::black(),
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
}

pub struct TerminalEffectVisitor<'fx> {
    pub canvas: &'fx mut PixelCanvas<TextModePixel>,
}

#[allow(non_snake_case)]
pub fn Terminal<UiBlueprint>(
    mut ui: UiBlueprint,
) -> TerminalBlueprint<
    React<impl Signal<Item = UiBlueprint::Blueprint>, TerminalEnvironment>,
>
where
    UiBlueprint: Tui,
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
            let parent_hints = ParentHints {
                rect: Rect::new(0.0, 0.0, terminal_size.w, terminal_size.h),
                // TODO: Turn these into signals, maybe?
                current_flow: CurrentFlow {
                    current_flow_direction: CartesianFlow::LeftToRight,
                    current_cross_flow_direction: CartesianFlow::TopToBottom,
                    current_writing_flow_direction: CartesianFlow::LeftToRight,
                    current_writing_cross_flow_direction:
                        CartesianFlow::TopToBottom,
                },
            };
            // TODO: Listen to and respect the child hints.
            #[allow(unused)]
            let child_hints = ui.prepare(parent_hints);
            let clamped_rect = Rect::new(
                0.0,
                0.0,
                parent_hints.rect.w.max(child_hints.minimum_size.w),
                parent_hints.rect.h.max(child_hints.minimum_size.h),
            );
            ui.place(ParentHints {
                rect: clamped_rect,
                ..parent_hints
            })
        })
        .into_blueprint();

    TerminalBlueprint { ui, state }
}
