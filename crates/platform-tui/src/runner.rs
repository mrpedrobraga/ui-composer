use crossterm::QueueableCommand;
use crossterm::cursor::{
    Hide, RestorePosition, SavePosition, SetCursorStyle, Show,
};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste,
    EnableMouseCapture, Event as CrosstermEvent, EventStream, KeyCode,
};
use crossterm::terminal::{
    DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
    LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use futures::executor::block_on;
use futures::{StreamExt, join};
use futures_signals::signal::SignalExt as _;
use smol_str::ToSmolStr as _;
use std::io::{Write, stdout};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use ui_composer_core::app::composition::algebra::Bubble as _;
use ui_composer_core::app::composition::elements::{Blueprint, Environment};
use ui_composer_core::app::runner::Runner;
use ui_composer_core::app::runner::futures::AsyncExecutor;
use ui_composer_input::event::{
    ButtonState, CursorEvent, DeviceId, Event, KeyEvent, KeyboardEvent,
    TouchStage,
};
use vek::{Extent2, Vec2};

use crate::nodes::TerminalEffectVisitor;

pub struct TerminalEnvironment;
pub struct TerminalBlueprintResources;

impl Environment for TerminalEnvironment {
    type BlueprintResources<'make> = TerminalBlueprintResources;
    type EffectVisitor<'fx> = TerminalEffectVisitor<'fx>;
}

pub struct TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    _app: PhantomData<AppBlueprint>,
}

impl<AppBlueprint> Runner for TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
    AppBlueprint::Element: Send + 'static,
{
    type AppBlueprint = AppBlueprint;

    fn run(blueprint: Self::AppBlueprint) {
        Self::grab_terminal(&mut stdout()).unwrap();

        #[allow(unused)]
        let env = TerminalEnvironment;
        let res = TerminalBlueprintResources;
        let app = blueprint.make(&res);
        let app = Arc::new(Mutex::new(app));
        let app_e = app.clone();

        // Correction for the terminal's way of indexing.
        let top_left_correction = Vec2::new(1.0, 1.0);

        let event_handler = async {
            let e_stream = EventStream::new();

            e_stream
                .filter_map(|e| async { e.ok() })
                .for_each(move |event| {
                    let app_e = app_e.clone();
                    async move {
                        if let CrosstermEvent::Key(e) = event
                            && let KeyCode::Char('q') = e.code
                        {
                            let _ = Self::release_terminal(&mut stdout());
                            std::process::exit(1);
                        }

                        if let CrosstermEvent::Resize(new_width, new_height) =
                            event
                        {
                            let mut l = app_e.lock().unwrap();
                            l.bubble(&mut Event::Resized(Extent2::new(
                                new_width as f32,
                                new_height as f32,
                            )));
                        }

                        if let CrosstermEvent::Key(k) = event {
                            let mut l = app_e.lock().unwrap();
                            l.bubble(&mut Event::Keyboard {
                                id: DeviceId(0),
                                event: KeyboardEvent::Key(KeyEvent {
                                    is_implicit: false,
                                    text_repr: k
                                        .code
                                        .as_char()
                                        .map(|x| x.to_smolstr()),
                                    button_state: if k.is_press() {
                                        ButtonState::Pressed
                                    } else {
                                        ButtonState::Released
                                    },
                                }),
                            });
                        }

                        if let CrosstermEvent::Mouse(m) = event {
                            let mut l = app_e.lock().unwrap();

                            if m.kind.is_moved() {
                                l.bubble(&mut Event::Cursor {
                                    id: DeviceId(0),
                                    event: CursorEvent::Moved {
                                        position: (Vec2::new(m.column, m.row)
                                            .as_()
                                            + top_left_correction),
                                    },
                                });
                            }

                            if m.kind.is_drag() {
                                l.bubble(&mut Event::Cursor {
                                    id: DeviceId(0),
                                    event: CursorEvent::Moved {
                                        position: (Vec2::new(m.column, m.row)
                                            .as_()
                                            + top_left_correction),
                                    },
                                });
                            }

                            if m.kind.is_down() {
                                l.bubble(&mut Event::Cursor {
                                    id: DeviceId(0),
                                    event: CursorEvent::Touched {
                                        finger_id: 0,
                                        stage: TouchStage::Started,
                                    },
                                });
                            }
                        }
                    }
                })
                .await;
        };
        let async_handler = AsyncExecutor::new(app, res, || {}).to_future();
        let processes = async { join!(event_handler, async_handler) };
        block_on(processes);

        Self::release_terminal(&mut stdout()).unwrap();
    }
}

impl<AppBlueprint> TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    pub fn grab_terminal(
        terminal: &mut (impl QueueableCommand + Write),
    ) -> Result<(), std::io::Error> {
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

    pub fn release_terminal(
        terminal: &mut (impl QueueableCommand + Write),
    ) -> Result<(), std::io::Error> {
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
