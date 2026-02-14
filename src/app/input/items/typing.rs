use crate::app::composition::algebra::Bubble;
use crate::app::input::Event;
use crate::app::input::{ButtonState, InputItem, KeyEvent, KeyboardEvent};
use futures_signals::signal::Mutable;
use crate::app::composition::elements::{Blueprint, Element};
use crate::runners::tui::runner::TerminalEnvironment;

/// Input item that receives key events...
#[derive(Clone)]
pub struct Typing {
    state: Mutable<String>,
}

impl Typing {
    pub fn new(state: Mutable<String>) -> Self {
        Self { state }
    }
}

impl InputItem for Typing {}

impl Bubble<Event, bool> for Typing {
    fn bubble(&mut self, event: &mut Event) -> bool {
        if let Event::Keyboard {
            event:
                KeyboardEvent::Key(KeyEvent {
                    text_repr: Some(text),
                    button_state: ButtonState::Pressed,
                    ..
                }),
            ..
        } = event
        {
            if text == "\u{08}" {
                self.state.lock_mut().pop();
            } else {
                self.state.lock_mut().push_str(text.as_str());
            }
            return true;
        }

        false
    }
}

impl Blueprint<TerminalEnvironment> for Typing {
    type Element = Self;

    fn make(self, env: &TerminalEnvironment) -> Self::Element {
        self
    }
}

impl Element<TerminalEnvironment> for Typing {
    type Effect<'fx> = ();

    fn effect(&self) -> Self::Effect<'_> {}
}
