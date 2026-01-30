use crate::state::process::SignalReactItem;
use core::task::Poll;
use std::cmp::Ordering;
use futures_signals::signal::{Map, SignalExt};
use futures_signals::signal::{Mutable, MutableSignal};

pub trait MutableExt {
    type Item;
    type Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;

    fn derive<F, U>(&self, predicate: F) -> Self::Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;
}

impl<Item> MutableExt for Mutable<Item> {
    type Item = Item;
    type Output<F, U>
        = SignalReactItem<Map<MutableSignal<Item>, F>>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy;

    fn derive<F, U>(&self, predicate: F) -> Self::Output<F, U>
    where
        F: FnMut(Self::Item) -> U,
        Self::Item: Copy,
    {
        SignalReactItem(self.signal().map(predicate))
    }
}
