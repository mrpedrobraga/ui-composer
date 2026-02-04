use crate::app::composition::elements::Blueprint;
use crate::app::input::{ButtonState, DeviceId, Event, KeyEvent};
use crate::app::runner::Runner;
use crate::runners::tui::render::shaders;
use crate::standard::prelude::KeyboardEvent;
use crate::standard::runners::tui::render::canvas::Canvas;
use async_std::prelude::Stream;
use crossterm::cursor::{Hide, RestorePosition, SavePosition, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
    EventStream, KeyCode,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::{event, QueueableCommand};
use futures::StreamExt;
use pin_project::pin_project;
use smol_str::ToSmolStr;
use std::cell::RefCell;
use std::io::{stdout, Stdout, Write};
use std::rc::Rc;
use vek::{Rect, Rgba, Vec2};

pub struct TerminalEnvironment;
pub type Own<T> = Rc<RefCell<T>>;

#[pin_project(project=TUIBackendProj)]
pub struct TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    #[pin]
    pub app: Own<AppBlueprint::Element>,
}

impl<AppBlueprint> Runner for TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    type AppBlueprint = AppBlueprint;

    fn run(blueprint: Self::AppBlueprint) -> Self {
        Self::grab_terminal(&mut stdout()).unwrap();

        let app = blueprint.make(&TerminalEnvironment);
        let runner = TUIRunner::<AppBlueprint> {
            app: Rc::new(RefCell::new(app)),
        };

        runner
    }

    fn event_stream(&mut self) -> impl Stream<Item = Event> {
        let event_stream = EventStream::new();

        event_stream.filter_map(|event| async {
            let event = event.ok()?;

            match event {
                event::Event::Key(key_event) => {
                    if let KeyCode::Char('q') = key_event.code {
                        let _ = Self::release_terminal(&mut stdout());
                        std::process::exit(1);
                    }
                    Some(Event::Keyboard {
                        id: DeviceId(0),
                        event: KeyboardEvent::Key(KeyEvent {
                            is_implicit: false,
                            text_repr: Some(key_event.code.to_smolstr()),
                            button_state: if key_event.is_press() {
                                ButtonState::Pressed
                            } else {
                                ButtonState::Released
                            },
                        }),
                    })
                }
                event::Event::Resize(width, height) => {
                    self.redraw(
                        &mut stdout(),
                        Rect::new(0.0, 0.0, width as f32, height as f32),
                        Vec2::new(0.0, 0.0),
                    );

                    None
                }
                _ => None,
            }
        })
    }

    fn on_update(&mut self) {
        // TODO: Drawing should be granular.
        self.redraw(
            &mut stdout(),
            Rect::new(0.0, 0.0, 32.0, 16.0),
            Vec2::new(1.0, 1.0),
        );
    }
}

impl<AppBlueprint> TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    pub fn redraw<C: Canvas<Pixel = Rgba<u8>>>(
        &self,
        canvas: &mut C,
        rect: Rect<f32, f32>,
        mouse: Vec2<f32>,
    ) {
        let app = self.app.borrow_mut();
        //app.draw(canvas, rect.as_());
        canvas.quad(rect, shaders::image);
        canvas.put_pixel(mouse.as_(), Rgba::new(0, 255, 255, 0));
    }

    pub fn grab_terminal(terminal: &mut (impl QueueableCommand + Write)) -> Result<(), std::io::Error> {
        enable_raw_mode().expect("Couldn't enable raw mode");
        terminal
            .queue(SavePosition)?
            .queue(EnterAlternateScreen)?
            .queue(EnableMouseCapture)?
            .queue(DisableLineWrap)?
            .queue(SetCursorStyle::BlinkingUnderScore)?
            .queue(EnableBracketedPaste)?
            .queue(Hide)?
            .flush()?;

        Ok(())
    }

    pub fn release_terminal(terminal: &mut (impl QueueableCommand + Write)) -> Result<(), std::io::Error> {
        terminal
            .queue(Show)?
            .queue(DisableBracketedPaste)?
            .queue(SetCursorStyle::DefaultUserShape)?
            .queue(EnableLineWrap)?
            .queue(DisableMouseCapture)?
            .queue(LeaveAlternateScreen)?
            .queue(RestorePosition)?
            .flush()?;

        disable_raw_mode().expect("Couldn't disable raw mode.");
        Ok(())
    }
}
