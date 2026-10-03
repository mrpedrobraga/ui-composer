use ::std::{
    pin::Pin,
    task::{Context, Poll},
};
use ::ui_composer_input::event::Event;
use ::ui_composer_math::{
    flow::{CartesianFlow, CurrentFlow},
    glamour::{Point2, Rect, Size2},
};

use super::{
    elements::Environment,
    layout::{
        Ui,
        hints::{ChildHints, ParentHints},
    },
};

#[pin_project::pin_project]
pub struct RenderModule<Env: Environment, U: Ui<Env>> {
    #[pin]
    pub ui: U,
    pub state: RenderModuleState,
    pub render_resources: Env::RenderResources,
}

pub trait RenderModuleResources {
    fn resize(&mut self, new_size: Size2);
}

pub struct NoopRenderResources;

impl RenderModuleResources for NoopRenderResources {
    fn resize(&mut self, _: Size2) {}
}

pub struct RenderModuleState {
    is_dirty: bool,
    size: Size2,
    minimum_size: Size2,
    current_flow: CurrentFlow,
}

impl<Env: Environment, U: Ui<Env>> RenderModule<Env, U> {
    pub fn new(mut ui: U, initial_size: Size2, render_resources: Env::RenderResources) -> Self {
        let initial_parent_hints =
            RenderModuleState::default_parent_hints(initial_size);
        let initial_child_hints = ui.prepare(initial_parent_hints);

        Self {
            ui,
            state: RenderModuleState::new(
                initial_size.max(initial_child_hints.minimum_size),
                initial_child_hints.minimum_size,
            ),
            render_resources
        }
    }

    pub fn resize(
        &mut self,
        new_size: Size2,
        resources: &Env::BlueprintResources<'_>,
    ) {
        self.state.resize(new_size);
        let parent_hints = self.state.parent_hints();
        let ui_child_hints = self.ui.prepare(parent_hints);
        self.state.update_with_child_hints(ui_child_hints);
        self.ui.place(parent_hints, resources);
        self.render_resources.resize(new_size);
    }

    pub async fn propagate_event(&mut self, event: &mut Event) -> bool {
        self.ui.propagate(event).await
    }

    pub fn poll_ui_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env::BlueprintResources<'_>,
    ) -> Poll<Option<()>> {
        let mut this = self.project();

        let parent_hints = this.state.parent_hints();
        let ui_poll: Poll<Option<_>> =
            this.ui.as_mut().poll_change(cx, env, parent_hints);

        if let Poll::Ready(Some(())) = ui_poll {
            this.state.set_dirty(true)
        }

        ui_poll
    }
}

impl RenderModuleState {
    pub fn new(initial_size: Size2, initial_minimum_size: Size2) -> Self {
        let default_parent_hints = Self::default_parent_hints(initial_size);

        Self {
            is_dirty: true,
            size: initial_size,
            minimum_size: initial_minimum_size,
            current_flow: default_parent_hints.current_flow,
        }
    }

    pub fn default_parent_hints(size: Size2) -> ParentHints {
        ParentHints {
            rect: Rect::new(Point2::ZERO, size),
            current_flow: CurrentFlow {
                current_flow_direction: CartesianFlow::LeftToRight,
                current_cross_flow_direction: CartesianFlow::TopToBottom,
                current_writing_flow_direction: CartesianFlow::LeftToRight,
                current_writing_cross_flow_direction:
                    CartesianFlow::TopToBottom,
            },
        }
    }

    pub fn set_dirty(&mut self, is_dirty: bool) {
        self.is_dirty = is_dirty;
    }

    pub fn resize(&mut self, new_size: Size2) {
        self.size = new_size;
    }

    pub fn update_with_child_hints(&mut self, child_hints: ChildHints) {
        self.minimum_size = child_hints.minimum_size;
    }

    pub fn parent_hints(&self) -> ParentHints {
        ParentHints {
            rect: Rect::new(Point2::ZERO, self.size),
            current_flow: self.current_flow,
        }
    }

    pub fn child_hints(&self) -> ChildHints {
        ChildHints {
            minimum_size: self.minimum_size,
        }
    }
}
