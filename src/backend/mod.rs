//! Right now the module doesn't do a lot, but the intention is to generalize over targeting the GPU
//! v.s. targeting embedded, the terminal or canvases.
//!
//! I still don't know all that you need from a backend, so I'm leaving it here.

pub trait Backend {
    /// The type used for UI Events.
    type Event;
}
