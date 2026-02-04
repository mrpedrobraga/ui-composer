#[cfg(feature = "std")]
use std::{ops::Range, path::PathBuf};
use {
    smol_str::SmolStr,
    vek::{Extent2, Vec2},
};

pub mod items;

/// Trait that describes an `Input` element, something that handles user input and mutates state.
pub trait InputItem {}

pub type EvNum = f32;

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Event {
    /// The user requested that the application closes.
    CloseRequested,
    /// The user requested that the app is fully or partially redrawn.
    /// Something that might trigger this event. For example, resizing.
    RedrawRequested,
    /// The app became (or ceased to be) completely hidden from view.
    OcclusionStateChanged(bool),
    /// The app became (or ceased to be) focused.
    FocusStateChanged(bool),
    /// The app's render target has been resized.
    Resized(Extent2<EvNum>),
    /// The app's scale factor changed due to OS settings or being moved between monitors.
    ScaleFactorChanged(EvNum),
    /// The app's theme changed
    ThemeTypeChanged(ThemeType),

    /// An event relating to a "mouse" cursor.
    Cursor { id: DeviceId, event: CursorEvent },
    /// A keyboard key event
    Keyboard { id: DeviceId, event: KeyboardEvent },

    /// An Input Method Editor event, for inserting text.
    #[cfg(feature = "std")]
    Ime(ImeEvent),
    /// A [FileDragAndDropEvent].
    #[cfg(feature = "std")]
    File(FileDragAndDropEvent),
}

/// Tag struct that identifies a cursor device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceId(pub i32);

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CursorEvent {
    Moved {
        position: Vec2<EvNum>,
    },
    Entered,
    Exited,
    Button(MouseButton, ButtonState),
    Scroll(ScrollOffset, TouchStage),
    Touched {
        /// The unique identifier of continuous touches emitted
        /// by the same physical actor. What makes two touches be from the same "finger"
        /// is decided by the host operating system.
        finger_id: u32,
        stage: TouchStage,
    },
    DoubleTapped,
    Pinched {
        /// The magnification caused by this pinch — positive values are zooming in.
        scaling: EvNum,
        stage: TouchStage,
    },
    Panned {
        /// The vector by which the app should translate its content, in pixels.
        ///
        /// If a user pans with their fingers to the right,
        /// the content should follow in that direction.
        translation: Vec2<EvNum>,
        stage: TouchStage,
    },
    Rotated {
        /// In radians, clockwise.
        angle: EvNum,
        stage: TouchStage,
    },
    PressureApplied {
        pressure: f32,
        click_level: i32,
    },
}

/// The mouse button that an event was about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum MouseButton {
    /// The left mouse button, usually associated with activation.
    Left,
    /// The right mouse button, usually associated with "options".
    Right,
    /// The scroll wheel (yeah, it's also a button you can click).
    /// Usually associated with panning.
    Middle,
    /// The "forward" button, usually associated with browser history.
    Forward,
    /// The "back" button, usually associated with browser history.
    Back,
    /// Some other button was pressed.
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Released,
    Pressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TouchStage {
    Started,
    Moved,
    Ended,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScrollOffset {
    // Scroll offset in lines.
    Lines(Vec2<EvNum>),
    // Scroll offset in pixels.
    Pixels(Vec2<EvNum>),
}
/// Events for dragging files into your application.
#[cfg(feature = "std")]
#[derive(Debug, Clone, PartialEq)]
pub enum FileDragAndDropEvent {
    /// The user entered the app holding a file.
    Hovered(PathBuf),
    /// The user dropped the file onto the app.
    Dropped(PathBuf),
    /// The user previously emitted a `Hovered` event,
    /// but did not drop the file.
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeType {
    Dark,
    Light,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg(feature = "std")]
pub enum ImeEvent {
    /// IME was enabled, and you should get ready to handle IME events such as
    /// [Self::Preedit] or [Self::Commit].
    Enabled,
    /// IME is no longer enabled.
    Disabled,

    /// User composed text at some range. The range here is byte-indexed.
    /// When the range is `None`, the cursor should be hidden.
    ///
    /// When the `String` is empty, this indicates the pre-edit was cleared.
    /// This usually happens before [Self::Commit] is emitted.
    Preedit(Option<Range<usize>>, String),
    Commit(String),
}

// MARK: Keyboard

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyboardEvent {
    Key(KeyEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEvent {
    /// Implicit events (also called synthetic) are emitted in some platforms
    /// for when a window gains or loses focus while a key is down.
    /// It's a way of knowing what keys are down.
    pub is_implicit: bool,

    /// The text representation of this event.
    pub text_repr: Option<SmolStr>,

    /// The state of this key event.
    pub button_state: ButtonState,
}
