//! # SignalExt
//!
//! Extension traits and impls to make dealing with states more ergonomic.

use crate::app::composition::algebra::{Empty, Semigroup};
use crate::state::process::SignalReactItem;
use futures_signals::signal::{Map, SignalExt};
use futures_signals::signal::{Mutable, MutableSignal};
use std::task::Poll;

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

impl Semigroup for Poll<Option<()>> {
    /// If any of the values are Poll::Ready(Some(())), the result will be Poll::Ready(Some(()));
    /// If not and any of the values are Poll::Pending, the result will be Poll::Pending;
    /// Otherwise (if both values are Poll::Ready(None)), the result will be Poll::Ready(None);
    fn combine(self, other: Self) -> Self {
        use std::task::Poll::*;

        match (self, other) {
            (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
            (Pending, _) | (_, Pending) => Pending,
            _ => Ready(None),
        }
    }
}

impl Empty for Poll<Option<()>> {
    fn empty() -> Self {
        Poll::Ready(None)
    }
}
