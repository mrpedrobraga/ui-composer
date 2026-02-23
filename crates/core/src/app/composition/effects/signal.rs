use crate::app::composition::algebra::{Bubble, Semigroup as _};
use crate::app::composition::elements::{Blueprint, Element, Environment};
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use ui_composer_input::event::Event;

#[pin_project]
#[must_use = "React does nothing unless polled"]
pub struct React<Sig, Env: Environment>
where
    Sig: Signal,
    Sig::Item: Blueprint<Env>,
{
    #[pin]
    signal: Sig,
    element: Option<<Sig::Item as Blueprint<Env>>::Element>,
}

impl<Sig, Env: Environment> Blueprint<Env> for React<Sig, Env>
where
    Sig: Signal<Item: Blueprint<Env>>,
{
    type Element = Self;

    fn make(self, _: &Env) -> Self::Element {
        self
    }
}

impl<Sig, Env: Environment> Bubble<Event, bool> for React<Sig, Env>
where
    Sig: Signal<Item: Blueprint<Env>>,
{
    fn bubble(&mut self, cx: &mut Event) -> bool {
        self.element
            .as_mut()
            .map(|e| e.bubble(cx))
            .unwrap_or_default()
    }
}

impl<Sig, Env: Environment> Element<Env> for React<Sig, Env>
where
    Sig: Signal<Item: Blueprint<Env>>,
{
    type Effect<'a>
        =
        Option<
            <<<Sig as Signal>::Item as Blueprint<Env>>::Element as Element<
                Env,
            >>::Effect<'a>,
        >
    where
        Sig: 'a,
        Env: 'a;

    fn effect(&self) -> Self::Effect<'_> {
        self.element.as_ref().map(|e| e.effect())
    }

    fn poll(
        self: Pin<&mut Self>,
        cx: &mut Context,
        env: &Env,
    ) -> Poll<Option<()>> {
        let this = self.project();

        // SAFETY: Because the signal is pinned in this struct, its captures are stable.
        let signal_poll = match this.signal.poll_change(cx) {
            Poll::Ready(Some(blueprint)) => {
                let mut element = blueprint.make(env);

                // Wake up the element.
                let _ =
                    unsafe { Pin::new_unchecked(&mut element) }.poll(cx, env);
                *this.element = Some(element);

                Poll::Ready(Some(()))
            }
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
        };

        let element_poll = this
            .element
            .as_mut()
            .map(|element| unsafe { Pin::new_unchecked(element) }.poll(cx, env))
            .unwrap_or(Poll::Pending);

        signal_poll.combine(element_poll)
    }
}

/*
    This is necessary while we don't have `min_specialization`.

    We can't implement `Blueprint` for all futures without problems,
    so we need to a type this crate owns.
*/

pub trait IntoBlueprint<Env: Environment> {
    type Output: Blueprint<Env>;

    fn into_blueprint(self) -> Self::Output;
}

impl<Sig, Env> IntoBlueprint<Env> for Sig
where
    Sig: Signal,
    Env: Environment,
    Sig::Item: Blueprint<Env>,
{
    type Output = React<Sig, Env>;

    fn into_blueprint(self) -> Self::Output {
        React {
            signal: self,
            element: None,
        }
    }
}

/*
// This is not possible without specialization.
// Because upstream (`futures-signals`) could add an implemntation of `Signal`
// for other types in this crate that `Blueprint<Env>` is implemented for
// and that would resut in conflicting implementations.

impl<Sig, Env: Environment> Blueprint<Env> for Sig
where
    Sig: Signal<Item: Blueprint<Env>>,
{
    type Element = React<Sig, Env>;

    fn make(self, _: &Env) -> Self::Element {
        React {
            signal: self,
            element: None,
        }
    }
}*/
