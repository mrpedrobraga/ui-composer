use crate::app::composition::algebra::Combine;
use crate::app::composition::elements::{Blueprint, Element, Environment};
use crate::app::composition::layout::hints::{ChildHints, ParentHints};
use crate::app::composition::layout::Ui;
use futures_signals::signal::Signal;
use ::ui_composer_input::event::Event;
use std::marker::PhantomData;
use std::task::{Context, Poll};

#[pin_project::pin_project]
#[must_use = "React does nothing unless polled"]
pub struct React<Env, U, Sig, Map>
where
    Env: Environment,
    U: Ui<Env>,
    Sig: Signal,
    Map: FnMut(Sig::Item) -> U,
{
    #[pin]
    signal: Sig,
    signal_is_done: bool,
    #[pin]
    ui: Option<U>,
    map: Map,
    _marker: PhantomData<Env>,
}

pub trait SignalExt: Signal {
    fn react<Env, U, Map>(self, map: Map) -> React<Env, U, Self, Map>
    where
        Env: Environment,
        U: Ui<Env>,
        Map: FnMut(Self::Item) -> U,
        Self: std::marker::Sized,
    {
        React::new(self, map)
    }

    fn for_of<Env, U, Map>(self, map: Map) -> React<Env, U, Self, Map>
    where
        Env: Environment,
        U: Ui<Env>,
        Map: FnMut(Self::Item) -> U,
        Self: std::marker::Sized,
    {
        self.react(map)
    }
}
impl<Sig> SignalExt for Sig where Sig: Signal {}

impl<Env, U, Sig, Map> React<Env, U, Sig, Map>
where
    Env: Environment,
    U: Ui<Env>,
    Sig: Signal,
    Map: FnMut(Sig::Item) -> U,
{
    pub fn new(signal: Sig, map: Map) -> Self {
        Self {
            signal,
            signal_is_done: false,
            ui: None,
            map,
            _marker: PhantomData,
        }
    }
}

impl<Env, U, Sig, Map> Ui<Env> for React<Env, U, Sig, Map>
where
    Env: Environment + Send,
    U: Ui<Env>,
    Sig: Signal + Send,
    Map: FnMut(Sig::Item) -> U + Send,
{
    type Blueprint = Option<U::Blueprint>;

    fn prepare(&mut self, expected_parent_hints: ParentHints) -> ChildHints {
        self.ui
            .as_mut()
            .map(|u| u.prepare(expected_parent_hints))
            .unwrap_or_default()
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        if let Some(inner) = &mut self.ui {
            inner.place(parent_hints, resources);
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.ui.as_ref().map(|inner| inner.effect())
    }

    async fn propagate(&mut self, event: &mut Event) -> bool {
        if let Some(ui) = &mut self.ui {
            ui.propagate(event).await
        } else {
            false
        }
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let mut this = self.project();

        let signal_poll = if *this.signal_is_done {
            Poll::Ready(None)
        } else {
            match this.signal.poll_change(cx) {
                Poll::Ready(Some(value)) => {
                    let mut new_ui = (this.map)(value);
                    new_ui.prepare(parent_hints);
                    new_ui.place(parent_hints, resources);
                    this.ui.set(Some(new_ui));
                    Poll::Ready(Some(()))
                }
                Poll::Ready(None) => {
                    *this.signal_is_done = true;
                    Poll::Ready(None)
                }
                Poll::Pending => Poll::Pending,
            }
        };

        let ui_poll = if let Some(ui) = this.ui.as_pin_mut() {
            match ui.poll_change(cx, resources, parent_hints) {
                Poll::Ready(Some(())) => Poll::Ready(Some(())),
                Poll::Ready(None) => Poll::Ready(None),
                Poll::Pending => Poll::Pending,
            }
        } else {
            Poll::Ready(None)
        };

        Combine::combine(signal_poll, ui_poll)
    }
}
