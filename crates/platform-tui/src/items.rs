use crate::Tui;
use crate::runner::{TerminalBlueprintResources, TerminalEnvironment};
use futures_signals::signal::Mutable;
use pin_project::pin_project;
use ui_composer_canvas::{PixelCanvas, TextModePixel};
use ui_composer_core::app::composition::layout::Ui;
use ui_composer_core::app::composition::layout::hints::ParentHints;
use ui_composer_math::flow::{CartesianFlow, CurrentFlow};
use ui_composer_math::glamour::Rect;
use ui_composer_math::prelude::{Point2, Size2};

pub struct TerminalBlueprint<U> {
    pub(crate) state: TerminalState,
    pub(crate) ui: U,
}

pub struct TerminalState {
    pub size: Mutable<Size2>,
    pub mouse_position: Mutable<Option<Point2>>,
    pub render_target: PixelCanvas<TextModePixel>,
}

#[pin_project(project = TerminalElementProj)]
pub struct TerminalElement<U> {
    pub state: TerminalState,
    #[pin]
    pub ui: U,
    first_time: bool,
}

impl<U> TerminalElement<U>
where
    U: Ui<TerminalEnvironment>,
{
    pub fn new_from_blueprint(
        blueprint: TerminalBlueprint<U>,
        blueprint_resources: &TerminalBlueprintResources,
    ) -> Self {
        let mut element = Self {
            state: blueprint.state,
            ui: blueprint.ui,
            first_time: true,
        };
        element.update_within(blueprint_resources);
        element
    }

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
