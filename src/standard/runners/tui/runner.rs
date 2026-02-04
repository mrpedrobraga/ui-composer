use crate::app::backend::Runner;
use crate::app::composition::elements::Blueprint;
use crate::standard::runners::tui::render::canvas::Canvas;
use crate::runners::tui::signals::AsyncExecutor;
use async_std::task::yield_now;
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
use futures_signals::signal::SignalExt;
use pin_project::pin_project;
use std::cell::RefCell;
use std::io::{stdout, Write};
use std::rc::Rc;
use vek::{Extent2, Rect, Rgba, Vec2};

pub struct TUIEnvironment;
pub type Own<T> = Rc<RefCell<T>>;

#[pin_project(project=TUIBackendProj)]
pub struct TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TUIEnvironment>,
{
    #[pin]
    pub app: Own<AppBlueprint::Element>,
}

impl<AppBlueprint> Runner for TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TUIEnvironment>
{
    type AppBlueprint = AppBlueprint;

    fn run(blueprint: Self::AppBlueprint) {
        Self::grab_terminal().unwrap();

        let app = blueprint.make(&TUIEnvironment);
        let runner = TUIRunner::<AppBlueprint> {
            app: Rc::new(RefCell::new(app)),
        };

        // Combine both futures and run them on the current thread
        let _ = futures::executor::block_on(async {
            let executor_fut = AsyncExecutor::new(runner.app.clone()).to_future();
            let event_fut = runner.event_loop();

            futures::join!(executor_fut, event_fut)
        });

        Self::release_terminal().unwrap();
    }

    async fn event_loop(&self) {
        let _ = self.process_events().await;
    }

    async fn react_loop(&self) {
        todo!()
    }
}

impl<AppBlueprint> TUIRunner<AppBlueprint>
where
    AppBlueprint: Send + Blueprint<TUIEnvironment>,
{
    pub(crate) async fn process_events(&self) -> std::io::Result<()> {
        let mut stdout = stdout();

        let (cols, rows) = crossterm::terminal::size()?;
        let mut rect: Rect<f32, f32> = Rect::new(0.0, 0.0, cols as f32, rows as f32);
        let mut mouse: Vec2<f32> = Vec2::new(1.0, 1.0);
        let mut redraw_requested = false;

        {
            let mut app = self.app.borrow_mut();
            //app.bubble(&mut Event::Resized(rect.extent()));
        }
        // Calling `await` here yields execution back to the executor
        // allowing the other concurrent processes to do layouting.
        yield_now().await;
        self.redraw(&mut stdout, rect, mouse);
        let _ = stdout.flush()?;

        let mut event_stream = EventStream::new();

        while let Some(Ok(event)) = event_stream.next().await {
            let app = self.app.borrow_mut();

            match event {
                event::Event::Key(key_event) => {
                    if let KeyCode::Char('q') = key_event.code {
                        let _ = Self::release_terminal();
                        std::process::exit(1);
                    }
                }
                #[allow(unused)]
                event::Event::Resize(w, h) => {
                    rect.set_extent(Extent2::new(w, h).as_::<f32>());

                    //app.bubble(&mut Event::Resized(Extent2::new(w, h).as_()));

                    redraw_requested = true;
                }
                event::Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Moved {
                        mouse = Vec2::new(mouse_event.column, mouse_event.row).as_::<f32>();

                        /*app.bubble(&mut Event::Cursor {
                            id: DeviceId(0),
                            event: CursorEvent::Moved { position: mouse },
                        });*/

                        redraw_requested = true;
                    }
                }
                _ => (),
            }

            drop(app);

            // TODO: Another concurrent process should be responsible for drawing when dirty.
            if redraw_requested {
                self.redraw(&mut stdout, rect, mouse);
                let _ = stdout.flush()?;
            }

            yield_now().await;
        }

        Ok(())
    }

    pub fn redraw<C: Canvas<Pixel = Rgba<u8>>>(
        &self,
        canvas: &mut C,
        rect: Rect<f32, f32>,
        mouse: Vec2<f32>,
    ) {
        let app = self.app.borrow_mut();
        //app.draw(canvas, rect.as_());
        //canvas.quad(rect, shaders::image);
        canvas.put_pixel(mouse.as_(), Rgba::new(255, 255, 255, 0));
    }

    pub fn grab_terminal() -> Result<(), std::io::Error> {
        enable_raw_mode().expect("Couldn't enable raw mode");

        let mut stdout = stdout();
        stdout
            .queue(SavePosition)?
            .queue(EnterAlternateScreen)?
            .queue(EnableMouseCapture)?
            .queue(DisableLineWrap)?
            .queue(SetCursorStyle::BlinkingUnderScore)?
            .queue(EnableBracketedPaste)?
            .queue(Hide)?
            .flush()?;

        // TODO: Just think and wonder about how to handle panics, you know?
        let original_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            let _ = Self::release_terminal();
            original_hook(panic_info);
        }));

        Ok(())
    }

    pub fn release_terminal() -> Result<(), std::io::Error> {
        let mut stdout = stdout();

        stdout
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

