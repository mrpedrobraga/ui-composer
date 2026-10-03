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
use std::sync::Arc;
use ui_composer_core::app::composition::algebra::Propagate as _;
use ui_composer_core::app::composition::elements::{
    Blueprint, Environment,
};
use ui_composer_core::app::runner::futures::AsyncExecutor;
use ui_composer_input::event::{
    ButtonState, CursorEvent, DeviceId, Event, KeyEvent, KeyboardEvent,
    TouchStage,
};
use ui_composer_math::prelude::{Point2, Size2, Vector2};

use crate::Tui;
use crate::items::{TerminalBlueprint, TerminalEffectVisitor};

pub struct TerminalEnvironment;

#[derive(Clone)]
pub struct TerminalBlueprintResources;

impl Environment for TerminalEnvironment {
    type BlueprintResources<'make> = TerminalBlueprintResources;
    type EffectVisitor<'fx> = TerminalEffectVisitor<'fx>;
    const TILE_SIZE: Size2 = Size2::new(1.0, 1.0);
}

pub struct TuiPlatform<U>
where
    U: Tui
{
    _app: PhantomData<U>,
}

impl<U> TuiPlatform<U>
where
    U: Tui,
{
    pub fn run(terminal_blueprint: TerminalBlueprint<U>) {
        Self::grab_terminal(&mut stdout()).unwrap();

        #[allow(unused)]
        let environment = TerminalEnvironment;
        let resources = TerminalBlueprintResources;
        let mut terminal_element = terminal_blueprint.make(&resources);
        terminal_element.update_within(&resources);
        let terminal_element = Arc::new(futures::lock::Mutex::new(terminal_element));
        let terminal_element_2 = terminal_element.clone();

        // Correction for the terminal's way of indexing.
        let top_left_correction = Vector2::new(1.0, 1.0);


        let res2 = resources.clone();
        let event_handler = async {
            let e_stream = EventStream::new();
            let resources = res2.clone();
            e_stream
                .filter_map(|e| async { e.ok() })
                .for_each(move |event| {
                    let app_e = terminal_element_2.clone();
                    {
                        let resources = resources.clone();
                        async move {
                            if let CrosstermEvent::Key(e) = event
                                && let KeyCode::Char('q') = e.code
                            {
                                let _ = Self::release_terminal(&mut stdout());
                                std::process::exit(1);
                            }

                            if let CrosstermEvent::Resize(
                                new_width,
                                new_height,
                            ) = event
                            {
                                let mut l = app_e.lock().await;
                                l.propagate(&mut Event::Resized(Size2::new(
                                    new_width as f32,
                                    new_height as f32,
                                )))
                                .await;
                                l.update_within(&resources);
                            }

                            if let CrosstermEvent::Key(k) = event {
                                let mut l = app_e.lock().await;
                                l.propagate(&mut Event::Keyboard {
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
                                })
                                .await;
                            }

                            if let CrosstermEvent::Mouse(m) = event {
                                let mut l = app_e.lock().await;

                                if m.kind.is_moved() {
                                    l.propagate(&mut Event::Cursor {
                                        id: DeviceId(0),
                                        event: CursorEvent::Moved {
                                            position: (Point2::<u16>::new(
                                                m.column, m.row,
                                            )
                                            .as_()
                                                + top_left_correction),
                                        },
                                    })
                                    .await;
                                    l.redraw();
                                }

                                if m.kind.is_drag() {
                                    l.propagate(&mut Event::Cursor {
                                        id: DeviceId(0),
                                        event: CursorEvent::Moved {
                                            position: (Point2::<u16>::new(
                                                m.column, m.row,
                                            )
                                            .as_()
                                                + top_left_correction),
                                        },
                                    })
                                    .await;
                                    l.redraw();
                                }

                                if m.kind.is_down() {
                                    l.propagate(&mut Event::Cursor {
                                        id: DeviceId(0),
                                        event: CursorEvent::Touched {
                                            finger_id: 0,
                                            stage: TouchStage::Started,
                                        },
                                    })
                                    .await;
                                }
                            }
                        }
                    }
                })
                .await;
        };
        let async_handler = AsyncExecutor::new(terminal_element, resources, || {}).to_future();
        let processes = async { join!(event_handler, async_handler) };
        block_on(processes);

        Self::release_terminal(&mut stdout()).unwrap();
    }
}

impl<U> TuiPlatform<U>
where
    U: Tui
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
