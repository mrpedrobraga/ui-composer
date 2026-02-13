use crate::app::composition::algebra::Bubble;
use crate::app::composition::effects::ElementEffect;
use crate::app::composition::elements::{Blueprint, Environment};
use crate::app::composition::visit::Apply;
use crate::app::runner::Runner;
use crate::app::runner::futures::AsyncExecutor;
use crate::prelude::Event;
use crate::runners::tui::nodes::TerminalEffectVisitor;
use async_std::task::block_on;
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
    LeaveAlternateScreen, enable_raw_mode,
};
use futures::{StreamExt, join};
use futures_signals::signal::SignalExt;
use std::io::{Write, stdout};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use vek::Extent2;

pub struct TerminalEnvironment;

impl Environment for TerminalEnvironment {
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

        let env = TerminalEnvironment;
        let app = blueprint.make(&env);
        let app = Arc::new(Mutex::new(app));
        let app_e = app.clone();

        let event_handler = async {
            let e_stream = EventStream::new();

            e_stream
                .filter_map(|e| async { e.ok() })
                .for_each(move |event| {
                    let value = app_e.clone();
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
                            let mut l = value.lock().unwrap();
                            l.bubble(&mut Event::Resized(Extent2::new(
                                new_width as f32,
                                new_height as f32,
                            )));
                        }
                    }
                })
                .await;
        };
        let async_handler = AsyncExecutor::new(app, env, || {}).to_future();
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

        enable_raw_mode().expect("Couldn't disable raw mode.");
        Ok(())
    }
}
