use crate::app::composition::algebra::Bubble;
use crate::app::composition::effects::signal::{React, SignalReactExt};
use crate::app::composition::elements::{Blueprint, Element};
use crate::geometry::flow::CartesianFlow;
use crate::geometry::layout::hints::ParentHints;
use crate::geometry::layout::LayoutItem;
use crate::runners::tui::runner::TerminalEnvironment;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_signals::signal::Mutable;
use futures_signals::signal::{Signal, SignalExt};
use vek::{Extent2, Rect};
use {crate::app::input::Event, vek::Vec2};
use crate::runners::tui::TUI;

#[allow(non_snake_case)]
pub fn Terminal<UI>(
    mut ui: UI,
) -> TerminalBlueprint<
    React<
        impl Signal<Item = UI::Content>,
        TerminalEnvironment,
    >,
>
where
    UI: TUI,
{
    let state = TerminalState {
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
    }).react();

    TerminalBlueprint { ui, state }
}

pub struct TerminalBlueprint<UI> {
    pub(crate) state: TerminalState,
    pub(crate) ui: UI,
}

pub struct TerminalState {
    pub size: Mutable<Extent2<f32>>,
    pub mouse_position: Mutable<Option<Vec2<f32>>>,
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

    fn poll(self: Pin<&mut Self>, cx: &mut Context, env: &TerminalEnvironment) -> Poll<Option<()>> {
        let ui = unsafe { self.map_unchecked_mut(|this| &mut this.ui) };
        ui.poll(cx, env)
    }
}
