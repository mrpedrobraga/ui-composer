//! # TUIBackend
//!
//! This module is for rendering UI to the terminal using [`crossterm`].

use crate::standard::backends::tui::pipeline::Canvas;
use core::ops::DerefMut;
use core::pin::Pin;
use core::task::{Context, Poll};
use crossterm::cursor::{Hide, SetCursorStyle, Show};
use crossterm::event;
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode,
};
use crossterm::terminal::{Clear, ClearType, DisableLineWrap, EnableLineWrap};
use spin::Mutex;
use vek::{Rgba, Vec2};
use {
    crate::app::{backend::Backend, input::Event},
    crossterm::terminal::{disable_raw_mode, enable_raw_mode},
    futures::FutureExt,
    pin_project::pin_project,
    vek::Rect,
};

use crate::app::backend::AppContext;
use crate::app::input::DeviceId;
use crate::app::input::CursorEvent;

#[cfg(feature = "std")]
use {
    crossterm::ExecutableCommand,
    std::{
        io::{Write as _, stdout},
        sync::Arc,
    },
};

#[pin_project(project=TUIBackendProj)]
pub struct TUIBackend<N: Node> {
    #[pin]
    pub node_tree: Arc<Mutex<N::Output>>,
}

impl<N> Backend for TUIBackend<N>
where
    N: Node,
{
    type Tree = N;

    fn run(node_tree: Self::Tree) {
        enable_raw_mode().expect("Couldn't enable raw mode");

        let node_tree = node_tree.reify();
        async_std::task::block_on(Self::app_loop(node_tree).map(|r| r.unwrap()));

        disable_raw_mode().expect("Couldn't disable raw mode.")
    }

    fn poll_processors(
        self: Pin<&mut Self>,
        cx: &mut Context,
        _resources: &mut AppContext,
    ) -> Poll<Option<()>> {
        let TUIBackendProj { node_tree, .. } = self.project();

        let mut engine_tree = node_tree.lock();
        let engine_tree = DerefMut::deref_mut(&mut engine_tree);
        let engine_tree_pin = unsafe { Pin::new_unchecked(engine_tree) };

        engine_tree_pin.poll_processors(cx)
    }
}

#[cfg(feature = "tui")]
impl<N> TUIBackend<N>
where
    N: Node,
{
    async fn app_loop(mut node_tree: N::Output) -> std::io::Result<()> {
        let mut stdout = stdout();
        stdout
            .execute(EnableMouseCapture)?
            .execute(DisableLineWrap)?
            .execute(SetCursorStyle::BlinkingUnderScore)?
            .execute(EnableBracketedPaste)?
            .execute(Hide)?
            .execute(Clear(ClearType::All))?
            .flush()?;

        //Self::redraw(&node_tree, &mut stdout, Rect::new(0, 0, 16, 16));
        loop {
            let event = event::read()?;

            match event {
                event::Event::Key(key_event) => {
                    if let KeyCode::Char('q') = key_event.code {
                        break;
                    }
                }
                #[allow(unused)]
                event::Event::Resize(w, h) => {
                    //Self::redraw(&node_tree, &mut stdout, Rect::new(0, 0, w, h));
                }
                event::Event::Mouse(mouse_event) => {
                    if mouse_event.kind == event::MouseEventKind::Moved {
                        node_tree.handle_event(Event::Cursor {
                            id: DeviceId(0),
                            event: CursorEvent::Moved {
                                position: Vec2::new(
                                    mouse_event.column as f32,
                                    mouse_event.row as f32,
                                ),
                            },
                        });
                    }
                }
                _ => (),
            }
        }

        stdout
            .execute(Show)?
            .execute(DisableBracketedPaste)?
            .execute(EnableLineWrap)?
            .execute(DisableMouseCapture)?
            .execute(Clear(ClearType::All))?
            .flush()?;

        Ok(())
    }

    #[allow(unused)]
    fn redraw<C>(tree: &N::Output, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>,
    {
        tree.draw(canvas, rect);
    }
}

/// Trait that represents a descriptor main node of the app tree.
/// These nodes are used for creating windows and processes and rendering contexts.
///
/// In this module there is only one node: "Terminal".
pub trait Node: Send {
    /// The type this node descriptor generates when Output.
    type Output: NodeRe;
    fn reify(self) -> Self::Output;
}

/// A main node in the app tree.
pub trait NodeRe: Send {
    fn setup(&mut self);

    /// Handles an event and cascades it down its children.
    fn handle_event(&mut self, event: Event);

    /// Draws to the standard output.
    fn draw<C>(&self, canvas: &mut C, rect: Rect<u16, u16>)
    where
        C: Canvas<Pixel = Rgba<u8>>;

    /// Polls underlying processors: `Future`s and `Signal`s within the app.
    /// This should advance animations, async processes and reactivity.
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>>;
}
