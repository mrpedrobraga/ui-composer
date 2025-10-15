use winit::event::ElementState;
use {
    crate::{
        app::input::{
            ButtonState, DeviceId, FileDragAndDropEvent, ImeEvent, KeyEvent, KeyboardEvent,
            MouseButton, ScrollOffset, ThemeType, TouchStage,
        },
        prelude::{CursorEvent, EvNum, Event},
    },
    smol_str::SmolStr,
    vek::{Extent2, Vec2},
    winit::event::{MouseScrollDelta, TouchPhase, WindowEvent},
};

impl TryFrom<WindowEvent> for Event {
    type Error = ();

    fn try_from(value: WindowEvent) -> Result<Self, Self::Error> {
        match value {
            // MARK: App
            WindowEvent::CloseRequested => Ok(Event::CloseRequested),
            WindowEvent::Resized(physical_size) => Ok(Event::Resized(Extent2 {
                w: physical_size.width as EvNum,
                h: physical_size.height as EvNum,
            })),
            WindowEvent::ThemeChanged(theme) => Ok(Event::ThemeTypeChanged(match theme {
                winit::window::Theme::Light => ThemeType::Light,
                winit::window::Theme::Dark => ThemeType::Light,
            })),
            WindowEvent::RedrawRequested => Ok(Event::RedrawRequested),
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer: _,
            } => Ok(Event::ScaleFactorChanged(scale_factor as EvNum)),
            WindowEvent::Focused(is_focused) => Ok(Event::FocusStateChanged(is_focused)),

            // MARK: Cursor and Gestures
            WindowEvent::CursorMoved {
                device_id: _,
                position,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Moved {
                    position: Vec2 {
                        x: position.x as EvNum,
                        y: position.y as EvNum,
                    },
                },
            }),
            WindowEvent::CursorEntered { device_id: _ } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Entered,
            }),
            WindowEvent::CursorLeft { device_id: _ } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Exited,
            }),
            WindowEvent::MouseWheel {
                device_id: _,
                delta,
                phase,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Scroll(
                    match delta {
                        MouseScrollDelta::LineDelta(column_delta, row_delta) => {
                            ScrollOffset::Lines(Vec2::new(column_delta, row_delta))
                        }
                        MouseScrollDelta::PixelDelta(physical_position) => ScrollOffset::Pixels(
                            Vec2::new(physical_position.x as f32, physical_position.y as f32),
                        ),
                    },
                    phase.into(),
                ),
            }),
            WindowEvent::MouseInput {
                device_id: _,
                state,
                button,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Button(
                    match button {
                        winit::event::MouseButton::Left => MouseButton::Left,
                        winit::event::MouseButton::Right => MouseButton::Right,
                        winit::event::MouseButton::Middle => MouseButton::Middle,
                        winit::event::MouseButton::Back => MouseButton::Back,
                        winit::event::MouseButton::Forward => MouseButton::Forward,
                        winit::event::MouseButton::Other(id) => MouseButton::Other(id),
                    },
                    match state {
                        winit::event::ElementState::Pressed => ButtonState::Pressed,
                        winit::event::ElementState::Released => ButtonState::Released,
                    },
                ),
            }),
            WindowEvent::Touch(touch) => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Touched {
                    finger_id: touch.id as u32,
                    stage: touch.phase.into(),
                },
            }),
            WindowEvent::TouchpadPressure {
                device_id: _,
                pressure,
                stage,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::PressureApplied {
                    pressure,
                    click_level: stage as i32,
                },
            }),
            WindowEvent::PinchGesture {
                device_id: _,
                delta,
                phase,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Pinched {
                    scaling: delta as EvNum,
                    stage: phase.into(),
                },
            }),
            WindowEvent::PanGesture {
                device_id: _,
                delta,
                phase,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Panned {
                    translation: Vec2 {
                        x: delta.x,
                        y: delta.y,
                    },
                    stage: phase.into(),
                },
            }),
            WindowEvent::DoubleTapGesture { device_id: _ } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::DoubleTapped,
            }),
            WindowEvent::RotationGesture {
                device_id: _,
                delta,
                phase,
            } => Ok(Event::Cursor {
                id: DeviceId(0),
                event: CursorEvent::Rotated {
                    angle: delta,
                    stage: phase.into(),
                },
            }),

            // MARK: Drag'n'Drop
            WindowEvent::HoveredFile(path_buf) => {
                Ok(Event::File(FileDragAndDropEvent::Hovered(path_buf)))
            }
            WindowEvent::HoveredFileCancelled => Ok(Event::File(FileDragAndDropEvent::Cancelled)),
            WindowEvent::DroppedFile(path_buf) => {
                Ok(Event::File(FileDragAndDropEvent::Dropped(path_buf)))
            }

            // MARK: Keyboard
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic,
            } => Ok(Event::Keyboard {
                id: DeviceId(0),
                event: KeyboardEvent::Key(KeyEvent {
                    is_implicit: is_synthetic,
                    text_repr: event.text.map(|s| SmolStr::from(s.as_str())),
                    button_state: event.state.into(),
                }),
            }),

            // MARK: Text (IME)
            WindowEvent::Ime(ime) => Ok(Event::Ime(match ime {
                winit::event::Ime::Enabled => ImeEvent::Enabled,
                winit::event::Ime::Preedit(text, range) => {
                    ImeEvent::Preedit(range.map(|(a, b)| a..b), text)
                }
                winit::event::Ime::Commit(text) => ImeEvent::Commit(text),
                winit::event::Ime::Disabled => ImeEvent::Disabled,
            })),

            WindowEvent::AxisMotion {
                device_id: _,
                axis: _,
                value: _,
            } => unimplemented!(),
            WindowEvent::Occluded(is_occluded) => Ok(Event::OcclusionStateChanged(is_occluded)),

            // MARK: Unsupported
            WindowEvent::ModifiersChanged(_modifiers) => Err(()),
            WindowEvent::ActivationTokenDone {
                serial: _,
                token: _,
            } => Err(()),
            WindowEvent::Moved(_physical_position) => Err(()),
            WindowEvent::Destroyed => Err(()),
        }
    }
}

impl From<TouchPhase> for TouchStage {
    fn from(value: TouchPhase) -> Self {
        match value {
            TouchPhase::Started => TouchStage::Started,
            TouchPhase::Moved => TouchStage::Moved,
            TouchPhase::Ended => TouchStage::Ended,
            TouchPhase::Cancelled => TouchStage::Cancelled,
        }
    }
}

impl From<ElementState> for ButtonState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => ButtonState::Pressed,
            ElementState::Released => ButtonState::Released,
        }
    }
}
