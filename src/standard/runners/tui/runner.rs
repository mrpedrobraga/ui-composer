use crate::app::composition::elements::{Blueprint, Element};
use crate::app::runner::Runner;
use crate::runners::tui::render::shaders;
use crate::standard::runners::tui::render::canvas::Canvas;
use crossterm::cursor::{Hide, RestorePosition, SavePosition, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture,
};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
    LeaveAlternateScreen,
};
use crossterm::QueueableCommand;
use std::io::{stdout, Write};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use vek::{Rect, Rgba, Vec2};

pub struct TerminalEnvironment;
pub type Own<T> = Arc<Mutex<T>>;

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

        let app = blueprint.make(&TerminalEnvironment);
        let app = Arc::new(Mutex::new(app));

        Self::release_terminal(&mut stdout()).unwrap();
    }

    // fn event_stream(&mut self) -> impl Stream<Item = Event> + Send + Sync + 'static {
    //     let event_stream = EventStream::new();
    //
    //     event_stream.filter_map(|event| async {
    //         let event = event.ok()?;
    //
    //         match event {
    //             event::Event::Key(key_event) => {
    //                 if let KeyCode::Char('q') = key_event.code {
    //                     let _ = Self::release_terminal(&mut stdout());
    //                     std::process::exit(1);
    //                 }
    //                 Some(Event::Keyboard {
    //                     id: DeviceId(0),
    //                     event: KeyboardEvent::Key(KeyEvent {
    //                         is_implicit: false,
    //                         text_repr: Some(key_event.code.to_smolstr()),
    //                         button_state: if key_event.is_press() {
    //                             ButtonState::Pressed
    //                         } else {
    //                             ButtonState::Released
    //                         },
    //                     }),
    //                 })
    //             }
    //             event::Event::Resize(width, height) => {
    //                 /*self.redraw(
    //                     &mut stdout(),
    //                     Rect::new(0.0, 0.0, width as f32, height as f32),
    //                     Vec2::new(0.0, 0.0),
    //                 );*/
    //
    //                 None
    //             }
    //             _ => None,
    //         }
    //     })
    // }
    //
    // fn on_update(&mut self) -> impl FnMut() + 'static {
    //     let app = self.app.clone();
    //
    //     move || {
    //         Self::redraw(
    //             app.clone(),
    //             &mut stdout(),
    //             Rect::new(0.0, 0.0, 32.0, 16.0),
    //             Vec2::new(1.0, 1.0),
    //         );
    //     }
    // }
}

impl<AppBlueprint> TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TerminalEnvironment>,
{
    pub fn redraw<C: Canvas<Pixel = Rgba<u8>>>(
        app: Own<AppBlueprint::Element>,
        canvas: &mut C,
        rect: Rect<f32, f32>,
        mouse: Vec2<f32>,
    ) {
        let app = app.lock().unwrap();
        dbg!(app.effect());
        canvas.quad(rect, shaders::image);
        canvas.put_pixel(mouse.as_(), Rgba::new(0, 255, 255, 0));
    }

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
