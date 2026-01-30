use crate::app::input::{CursorEvent, DeviceId, Event};
use crate::runners::tui::TUIRunner;
use crossterm::cursor::{Hide, SetCursorStyle, Show};
use crossterm::event::{
    DisableBracketedPaste, DisableMouseCapture, EnableBracketedPaste, EnableMouseCapture, KeyCode,
};
use crossterm::terminal::{Clear, ClearType, DisableLineWrap, EnableLineWrap};
use crossterm::{event, ExecutableCommand};
use std::io::{stdout, Write};
use vek::Vec2;
use crate::app::composition::algebra::Bubble;
use crate::standard::runners::tui::Node;

impl<N> TUIRunner<N>
where
    N: Node,
{
    pub(crate) async fn app_loop(mut node_tree: N::Output) -> std::io::Result<()> {
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
                        node_tree.bubble(&mut Event::Cursor {
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
}
