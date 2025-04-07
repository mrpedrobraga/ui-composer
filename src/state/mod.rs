pub mod animation;
pub mod process;
pub mod signal_ext;

pub use futures_signals::signal::Mutable;
pub use futures_signals::signal::SignalExt;
pub use futures_signals::signal_map::MutableBTreeMap;
pub use futures_signals::signal_map::SignalMapExt;
pub use futures_signals::signal_vec::MutableVec;
pub use futures_signals::signal_vec::SignalVecExt;

pub trait Slot {
    type Item;
    fn put(&self, value: Self::Item);
    fn take(&self) -> Self::Item
    where
        Self::Item: Copy;
}
