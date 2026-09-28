use std::{
    marker::PhantomData,
    task::{Context, Poll},
};

use crate::app::composition::{
    elements::{Blueprint, Element, Environment},
    layout::{
        hints::{ChildHints, ParentHints},
        Ui,
    },
};

use super::adapters::WithEmptyState;

#[pin_project::pin_project]
pub struct Await<Env, U, Fut, Map>
where
    Env: Environment,
    U: Ui<Env>,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    #[pin]
    pub(crate) future: Fut,
    #[pin]
    pub(crate) ui: Option<U>,
    pub(crate) map: Option<Map>,
    _marker: PhantomData<Env>,
}

pub trait FutureExt: Future {
    /// Returns a branch of reactive UI that polls the future and updates when the future resolves.
    fn react<Env, U, Map>(self, map: Map) -> Await<Env, U, Self, Map>
    where
        Env: Environment,
        U: Ui<Env>,
        Map: FnMut(Self::Output) -> U,
        Self: std::marker::Sized,
    {
        Await::new(self, map)
    }

    /// Returns a branch of reactive UI that polls the future but doesn't render anything.
    fn into_ui_process<Env>(self) -> Await<Env, (), Self, impl FnMut(Self::Output)>
    where
        Env: Environment,
        Self: std::marker::Sized,
    {
        self.react(|_| {})
    }

    /// Returns a branch of reactive UI that polls the future and updates when the future resolves.
    fn for_of<Env, U, Map>(self, map: Map) -> Await<Env, U, Self, Map>
    where
        Env: Environment,
        U: Ui<Env>,
        Map: FnMut(Self::Output) -> U,
        Self: std::marker::Sized,
    {
        self.react(map)
    }
}
impl<Fut> FutureExt for Fut where Fut: Future {}

impl<Env, U, Fut, Map> Await<Env, U, Fut, Map>
where
    Env: Environment,
    U: Ui<Env>,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    pub fn new(future: Fut, map: Map) -> Self {
        Self {
            future,
            ui: None,
            map: Some(map),
            _marker: PhantomData,
        }
    }

    pub fn with_empty_state<E>(self, empty_state: E) -> WithEmptyState<Self, E> {
        WithEmptyState(self, empty_state)
    }
}

impl<Env, U, Fut, Map> Ui<Env> for Await<Env, U, Fut, Map>
where
    Env: Environment + Send,
    U: Ui<Env>,
    Fut: Future + Send,
    Map: FnOnce(Fut::Output) -> U + Send,
{
    type Blueprint = Option<U::Blueprint>;

    fn prepare(&mut self, _: ParentHints) -> ChildHints {
        /* TODO: Not sure what to do here? */
        ChildHints::default()
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        if let Some(inner) = &mut self.ui {
            inner.place(parent_hints, resources);
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.ui.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
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
                    new_ui.place(parent_hints, resources);
                    this.ui.set(Some(new_ui));
                    return Poll::Ready(Some(()));
                }
                Poll::Pending => {
                    // Put the mapper back because it wasn't used hehe
                    *this.map = Some(map);
                }
            }
        }
        /* The future has yielded, so we just pass forth the value of the inner poll! */
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
