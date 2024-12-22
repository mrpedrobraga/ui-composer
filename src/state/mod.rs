pub mod signal_ext;
pub mod animation;

pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal::Mutable as Editable;
pub use futures_signals::signal_vec::SignalVecExt;
pub use futures_signals::signal_vec::MutableVec as EditableVec;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_map::MutableBTreeMap as EditableBTreeMap;