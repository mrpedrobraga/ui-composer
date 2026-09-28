use crate::Tui;
use crate::render::present_canvas_to_terminal;
use crate::runner::{TerminalBlueprintResources, TerminalEnvironment};
use core::pin::Pin;
use core::task::{Context, Poll};
use ::std::hint::black_box;
use futures_signals::signal::Mutable;
use pin_project::pin_project;
use ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::algebra::Bubble;
use ui_composer_core::app::composition::elements::{Blueprint, Element};
use ui_composer_core::app::composition::layout::Ui;
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_core::app::composition::visit::DriveThru;
use ui_composer_input::event::{CursorEvent, Event};
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_math::glamour::Rect;
use ui_composer_math::palette::Srgba;
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

impl<U: Ui<TerminalEnvironment>> Blueprint<TerminalEnvironment>
    for TerminalBlueprint<U>
{
    type Output = TerminalElement<U>;

    fn make(self, _: &TerminalBlueprintResources) -> Self::Output {
        TerminalElement {
            state: self.state,
            ui: self.ui,
            first_time: true,
        }
    }
}

#[pin_project(project = TerminalElementProj)]
pub struct TerminalElement<U> {
    pub state: TerminalState,
    #[pin]
    pub ui: U,
    first_time: bool,
}

impl<U> TerminalElement<U> where U: Ui<TerminalEnvironment> {
    pub fn update_within(&mut self, resources: &TerminalBlueprintResources) {
        /* TODO: Move this somewhere else. */
        let parent_hints = ParentHints {
            rect: Rect::new(Point2::ZERO, self.state.size.get()),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction:
                    CartesianFlow::TopToBottom,
            },
        };

        self.ui.place(parent_hints, resources);
        self.redraw();
    }

    pub fn redraw(&mut self) {
        let ui_effects = self.ui.effect();
        self.state.render_target.clear();
        let mut vis = TerminalEffectVisitor {
            canvas: &mut self.state.render_target,
        };
        ui_effects.drive_thru(&mut vis);

        /* Draws a cute little mouse cursor... useful for troubleshooting certain interactions. */
        if let Some(mouse_position) = self.state.mouse_position.get() {
            vis.canvas.put_pixel(
                Point2::new(mouse_position.x as u32, mouse_position.y as u32 - 1),
                TextModePixel {
                    bg_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
                    fg_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
                    character: '\u{f01bf}',
                },
            )
        } else {
            black_box(())
        }

        present_canvas_to_terminal(vis.canvas)
            .expect("Failed to present canvas to terminal?");

    }
}

impl<U> Bubble<Event, bool> for TerminalElement<U> where U: Ui<TerminalEnvironment> {
    async fn bubble(&mut self, cx: &mut Event) -> bool {
        if let Event::Resized(new_size) = cx {
            self.state.render_target.resize(new_size.as_());
            self.state.size.set(*new_size);
            /* Needs redrawing! */

            // TODO: Move this somewhere else?
            self.update_within(&TerminalBlueprintResources {});
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
        resources: &TerminalBlueprintResources,
    ) -> Poll<Option<()>> {
        
        let TerminalElementProj {
            state,
            mut ui,
            first_time,
        } = self.project();

        /* TODO: Move this somewhere else. */
        let parent_hints = ParentHints {
            rect: Rect::new(Point2::ZERO, state.size.get()),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction:
                    CartesianFlow::TopToBottom,
            },
        };

        let ui_poll = ui.as_mut().poll_change(cx, resources, parent_hints);

        if ui_poll.is_pending() {
            return Poll::Pending;
        };
        if let Poll::Ready(None) = ui_poll
            && !*first_time
        {
            return Poll::Ready(None);
        }

        *first_time = false;

        /*
            Instead of rendering immediately, mark the terminal as dirty
            and re-rener as a result of a "redraw requested" event!
        */
        let ui_effects = ui.effect();
        state.render_target.clear();
        let mut vis = TerminalEffectVisitor {
            canvas: &mut state.render_target,
        };
        ui_effects.drive_thru(&mut vis);

        /* Draws a cute little mouse cursor... useful for troubleshooting certain interactions. */
        if let Some(mouse_position) = state.mouse_position.get() {
            vis.canvas.put_pixel(
                mouse_position.as_(),
                TextModePixel {
                    bg_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
                    fg_color: Srgba::new(0.0, 0.0, 0.0, 1.0),
                    character: '\u{f01bf}',
                },
            )
        }

        present_canvas_to_terminal(vis.canvas)
            .expect("Failed to present canvas to terminal?");
        

        Poll::Ready(Some(()))
    }

    fn update(
        &mut self,
        _: TerminalBlueprint<U>,
        resources: &TerminalBlueprintResources,
    ) {
        self.update_within(resources);
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
    //render_target.set_draw_transform(Vector2::new(1.0/2.0, 1.0/4.0));

    let state = TerminalState {
        size: Mutable::new(size.as_()),
        mouse_position: Mutable::new(None),
        render_target,
    };

    TerminalBlueprint { ui, state }
}
