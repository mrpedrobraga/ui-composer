use crate::app::input::{ButtonState, InputItem, KeyEvent, KeyboardEvent};
use crate::app::primitives::{Primitive, Processor};
use crate::prelude::Event;
use futures_signals::signal::Mutable;

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

impl<Res> Primitive<Res> for Typing {
    fn handle_event(&mut self, event: Event) -> bool {
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

impl<Res> Processor<Res> for Typing {}
