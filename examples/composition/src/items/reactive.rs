use std::task::{Context, Poll};

use futures_signals::signal::Signal;

use crate::{
    element::{Blueprint, Element, ParentHints, Ui},
    runner::InitializationResources,
};

#[pin_project::pin_project]
pub struct Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    #[pin]
    future: Fut,
    #[pin]
    ui: Option<U>,
    map: Option<Map>,
}

impl<U, Fut, Map> Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    pub fn new(future: Fut, map: Map) -> Self {
        Self {
            future,
            ui: None,
            map: Some(map),
        }
    }
}

impl<U, Fut, Map> Ui for Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    type Blueprint = Option<U::Blueprint>;

    fn plan(&mut self, parent_hints: ParentHints, resources: &InitializationResources) {
        if let Some(inner) = &mut self.ui {
            inner.plan(parent_hints, resources);
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect {
        self.ui.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context,
        resources: &InitializationResources,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let mut this = self.project();

        /* The future has not yet yielded! */
        // To satisfy FnOnce, we `take` the map here.
        if let Some(map) = this.map.take() {
            let fut_poll = this.future.poll(cx);

            match fut_poll {
                Poll::Ready(value) => {
                    let mut new_ui = map(value);
                    new_ui.plan(parent_hints.clone(), resources);
                    this.ui.set(Some(new_ui));
                    // Safe to unwrap because we just set it, duh.
                    return this
                        .ui
                        .as_pin_mut()
                        .unwrap()
                        .poll_change(cx, resources, parent_hints);
                }
                Poll::Pending => {
                    // Put the mapper back because it wasn't used hehe
                    *this.map = Some(map);
                }
            }
        }
        /* The future has yielded! */
        else {
            if let Some(element) = this.ui.as_pin_mut() {
                let inner_poll = element.poll_change(cx, resources, parent_hints);

                return inner_poll;
            } else {
                return Poll::Ready(None);
            }
        }

        Poll::Pending
    }
}

#[pin_project::pin_project]
pub struct React<U, Sig, Map>
where
    U: Ui,
    Sig: Signal,
    Map: FnMut(Sig::Item) -> U,
{
    #[pin]
    signal: Sig,
    signal_is_done: bool,
    #[pin]
    ui: Option<U>,
    map: Map,
}

pub trait SignalExt: Signal {
    fn react<U, Map>(self, map: Map) -> React<U, Self, Map>
    where
        U: Ui,
        Map: FnMut(Self::Item) -> U,
        Self: std::marker::Sized,
    {
        React::new(self, map)
    }
}
impl<Sig> SignalExt for Sig where Sig: Signal {}

impl<U, Sig, Map> React<U, Sig, Map>
where
    U: Ui,
    Sig: Signal,
    Map: FnMut(Sig::Item) -> U,
{
    pub fn new(signal: Sig, map: Map) -> Self {
        Self {
            signal,
            signal_is_done: false,
            ui: None,
            map,
        }
    }
}

impl<U, Sig, Map> Ui for React<U, Sig, Map>
where
    U: Ui,
    Sig: Signal,
    Map: FnMut(Sig::Item) -> U,
{
    type Blueprint = Option<<U as Ui>::Blueprint>;

    fn plan(&mut self, parent_hints: ParentHints, resources: &InitializationResources) {
        if let Some(inner) = &mut self.ui {
            inner.plan(parent_hints, resources);
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect {
        self.ui.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context,
        resources: &InitializationResources,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let mut this = self.project();

        let signal_poll = if *this.signal_is_done {
            Poll::Ready(None)
        } else {
            match this.signal.poll_change(cx) {
                Poll::Ready(Some(value)) => {
                    let mut new_ui = (this.map)(value);
                    new_ui.plan(parent_hints.clone(), resources);
                    this.ui.set(Some(new_ui));
                    println!("Signal yields");
                    Poll::Ready(Some(()))
                }
                Poll::Ready(None) => {
                    println!("Signal is done");
                    *this.signal_is_done = true;
                    Poll::Ready(None)
                }
                Poll::Pending => {
                    println!("Signal pends");
                    Poll::Pending
                }
            }
        };

        let ui_poll = if let Some(ui) = this.ui.as_pin_mut() {
            match ui.poll_change(cx, resources, parent_hints) {
                Poll::Ready(Some(())) => {
                    println!("Ui yields");
                    Poll::Ready(Some(()))
                }
                Poll::Ready(None) => {
                    println!("Ui elements finished");
                    Poll::Ready(None)
                }
                Poll::Pending => {
                    println!("Ui pends!");
                    Poll::Pending
                }
            }
        } else {
            Poll::Ready(None)
        };

        max(signal_poll, ui_poll)
    }
}

/// Returns the maximum between two polls, according to the ordering `Ready(None) < Pending < Ready(Some)` and using `max(a, b)`;
///
/// This is useful if you must create a "composite" signal that:
/// - Yields when ANY of its dependencies yield;
/// - Is kept alive if ANY of its dependencies are alive;
/// - Finishes when ALL its dependencies finish;
fn max(a: Poll<Option<()>>, b: Poll<Option<()>>) -> Poll<Option<()>> {
    use std::task::Poll::*;

    /* Those are better understood as `Done`, `Pending`, `Yield` */

    match (a, b) {
        (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
        (Pending, _) | (_, Pending) => Pending,
        _ => Ready(None),
    }
}
