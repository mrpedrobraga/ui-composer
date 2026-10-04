use ::futures::executor::block_on;
use ::futures::join;
use ::futures_signals::signal::SignalExt as _;
use ::ui_composer_canvas::{Canvas, PixelCanvas, TextModePixel};
use ::ui_composer_core::app::composition::layout::Ui;
use ::ui_composer_core::app::composition::modules::{
    NoopRenderResources, RenderModule,
};
use ::ui_composer_core::app::composition::visit::DriveThru as _;
use ::ui_composer_core::app::runner::futures::RenderModulePoller;
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
use futures::StreamExt;
use smol_str::ToSmolStr as _;
use std::io::{Write, stdout};
use std::marker::PhantomData;
use std::sync::Arc;
use ui_composer_core::app::composition::elements::{Blueprint, Environment};
use ui_composer_input::event::{
    ButtonState, CursorEvent, DeviceId, Event, KeyEvent, KeyboardEvent,
    TouchStage,
};
use ui_composer_math::prelude::{Point2, Size2, Vector2};

use crate::Tui;
use crate::items::{TerminalBlueprint, TerminalEffectVisitor};
use crate::render::present_canvas_to_terminal;

pub struct TerminalEnvironment;

#[derive(Clone)]
pub struct TerminalBlueprintResources;

impl Environment for TerminalEnvironment {
    type BlueprintResources<'make> = TerminalBlueprintResources;
    type RenderResources = NoopRenderResources;
    type EffectVisitor<'fx> = TerminalEffectVisitor<'fx>;
    const TILE_SIZE: Size2 = Size2::new(1.0, 1.0);
}

pub struct TuiPlatform<U>
where
    U: Tui,
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
        let terminal_initial_size = terminal_blueprint.state.size.get();
        let mut terminal_element = terminal_blueprint.make(&resources);
        terminal_element.update_within(&resources);
        // TODO: Make the canvas a `RenderResource` of the render module in the tui platform?
        let terminal_state = terminal_element.state;
        let terminal_state = Arc::new(::futures::lock::Mutex::new(terminal_state));
        let terminal_state_2 = terminal_state.clone();
        let render_module = RenderModule::new(
            terminal_element.ui,
            terminal_initial_size,
            NoopRenderResources,
        );
        let render_module =
            Arc::new(::futures::lock::Mutex::new(render_module));
        let render_module_2 = render_module.clone();

        // Correction for the terminal's way of indexing.
        let top_left_correction = Vector2::new(1.0, 1.0);

        let res2 = resources.clone();
        let event_handler = async {
            let e_stream = EventStream::new();
            let resources = res2.clone();
            e_stream
                .filter_map(|e| async { e.ok() })
                .for_each(move |event| {
                    let app_e = render_module_2.clone();
                    let terminal_state = terminal_state_2.clone();
                    {
                        let resources = resources.clone();
                        let mut needs_redrawing = false;
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
                                let mut terminal_state = terminal_state.lock().await;
                                let new_size = Size2::new(
                                    new_width as f32,
                                    new_height as f32,
                                );
                                l.propagate_event(&mut Event::Resized(
                                    new_size,
                                ))
                                .await;
                                l.resize(new_size, &resources);
                                terminal_state.render_target.resize(new_size.as_());
                                needs_redrawing = true;
                            }

                            if let CrosstermEvent::Key(k) = event {
                                let mut l = app_e.lock().await;
                                l.propagate_event(&mut Event::Keyboard {
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
                                    l.propagate_event(&mut Event::Cursor {
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
                                    needs_redrawing = true;
                                }

                                if m.kind.is_drag() {
                                    l.propagate_event(&mut Event::Cursor {
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
                                    needs_redrawing = true;
                                }

                                if m.kind.is_down() {
                                    l.propagate_event(&mut Event::Cursor {
                                        id: DeviceId(0),
                                        event: CursorEvent::Touched {
                                            finger_id: 0,
                                            stage: TouchStage::Started,
                                        },
                                    })
                                    .await;
                                }
                            }

                            if needs_redrawing {
                                let render_module = app_e.lock().await;
                                let mut terminal_state = terminal_state.lock().await;

                                draw_render_module_onto_terminal(&*render_module, &mut terminal_state.render_target);
                            }
                        }
                    }
                })
                .await;
        };

        // let render_module_3 = render_module.clone();

        let async_handler =
            RenderModulePoller::new(render_module, resources, || {
                /* TODO: Find a way to draw the screen when the ui changes by itself. */
                // let render_module = block_on(render_module_3.lock());
                // let mut canvas = block_on(canvas.lock());
                // draw_render_module_onto_terminal(&*render_module, &mut canvas);
            })
                .to_future();
        let processes = async { join!(event_handler, async_handler) };
        block_on(processes);
        Self::release_terminal(&mut stdout()).unwrap();
    }
}

pub fn draw_render_module_onto_terminal<U: Ui<TerminalEnvironment>>(render_module: &RenderModule<TerminalEnvironment, U>, canvas: &mut PixelCanvas<TextModePixel>) {
    let ui_effects = render_module.ui.effect();
        canvas.clear();
        let mut vis = TerminalEffectVisitor {
            canvas,
        };
        ui_effects.drive_thru(&mut vis);

        /* Draws a cute little mouse cursor... useful for troubleshooting certain interactions. */
        // if let Some(mouse_position) = self.state.mouse_position.get() {
        //     vis.canvas.put_pixel(
        //         Point2::new(mouse_position.x as u32, mouse_position.y as u32 - 1),
        //         TextModePixel {
        //             bg_color: Srgba::new(0.0, 0.0, 0.0, 0.0),
        //             fg_color: Srgba::new(1.0, 1.0, 1.0, 1.0),
        //             character: '\u{f01bf}',
        //         },
        //     )
        // } else {
        //     black_box(())
        // }

        present_canvas_to_terminal(vis.canvas)
            .expect("Failed to present canvas to terminal?");
}

impl<U> TuiPlatform<U>
where
    U: Tui,
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
