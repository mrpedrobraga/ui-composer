use crate::app::building_blocks::Reifiable;
use {
    super::{input::Event, building_blocks::BuildingBlock},
    crate::state::signal_ext::coalesce_polls,
    core::{
        pin::Pin,
        task::{Context as TaskContext, Poll},
    },
};
use crate::state::process::Pollable;

impl<Cx> Reifiable<Cx> for () {
    type Reified = ();

    #[expect(unused)]
    fn reify(self, context: &mut Cx) -> Self::Reified {
        ()
    }
}

impl<Cx> BuildingBlock<Cx> for () {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Cx> Pollable<Cx> for () {}

impl<Cx, T> Reifiable<Cx> for Option<T>
where
    T: Reifiable<Cx>,
{
    type Reified = Option<T::Reified>;

    fn reify(self, context: &mut Cx) -> Self::Reified {
        self.map(|v| v.reify(context))
    }
}

impl<Cx, A: Send + BuildingBlock<Cx>> BuildingBlock<Cx> for Option<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_event(event))
            .unwrap_or(false)
    }
}

impl<Cx, A: Send + Pollable<Cx>> Pollable<Cx> for Option<A> {
    fn poll(self: Pin<&mut Self>, task_context: &mut TaskContext, context: &mut Cx) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll(task_context, context))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<Cx, T, E> Reifiable<Cx> for Result<T, E>
where
    T: Reifiable<Cx>,
    E: Reifiable<Cx>,
{
    type Reified = Result<T::Reified, E::Reified>;

    fn reify(self, context: &mut Cx) -> Self::Reified {
        match self {
            Ok(v) => Ok(Reifiable::reify(v, context)),
            Err(e) => Err(Reifiable::reify(e, context)),
        }
    }
}

impl<Cx, T: Send + BuildingBlock<Cx>, E: Send + BuildingBlock<Cx>> BuildingBlock<Cx> for Result<T, E> {
    fn handle_event(&mut self, event: Event) -> bool {
        match self {
            Ok(v) => v.handle_event(event),
            Err(e) => e.handle_event(event),
        }
    }
}

impl<Cx, T, E> Pollable<Cx> for Result<T, E>
where
    T: Send + Pollable<Cx>,
    E: Send + Pollable<Cx>,
{
    fn poll(self: Pin<&mut Self>, task_context: &mut TaskContext, context: &mut Cx) -> Poll<Option<()>> {
        // TODO: I don't understand pin, is this safe chat??
        unsafe {
            match self.get_unchecked_mut() {
                Ok(t) => Pin::new_unchecked(t).poll(task_context, context),
                Err(e) => Pin::new_unchecked(e).poll(task_context, context),
            }
        }
    }
}

#[cfg(feature = "std")]
impl<Cx, A> Reifiable<Cx> for Box<A>
where
    A: Reifiable<Cx>,
{
    type Reified = A::Reified;

    fn reify(self, context: &mut Cx) -> Self::Reified {
        (*self).reify(context)
    }
}

#[cfg(feature = "std")]
impl<Cx, A: Send + BuildingBlock<Cx>> BuildingBlock<Cx> for Box<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut().handle_event(event)
    }
}

#[cfg(feature = "std")]
impl<Cx, A: Send + Pollable<Cx>> Pollable<Cx> for Box<A> {
    fn poll(mut self: Pin<&mut Self>, task_context: &mut TaskContext, context: &mut Cx) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll(task_context, context)
    }
}

impl<Cx, A, B> Reifiable<Cx> for (A, B)
where
    A: Reifiable<Cx>,
    B: Reifiable<Cx>,
{
    type Reified = (A::Reified, B::Reified);

    fn reify(self, context: &mut Cx) -> Self::Reified {
        (self.0.reify(context), self.1.reify(context))
    }
}

impl<Cx, A: Send + BuildingBlock<Cx>, B: Send + BuildingBlock<Cx>> BuildingBlock<Cx> for (A, B) {
    fn handle_event(&mut self, event: Event) -> bool {
        let a_handled = self.0.handle_event(event.clone());
        let b_handled = self.1.handle_event(event);

        a_handled || b_handled
    }
}

impl<Cx, A: Send + BuildingBlock<Cx>, B: Send + BuildingBlock<Cx>> Pollable<Cx> for (A, B) {
    fn poll(self: Pin<&mut Self>, task_context: &mut TaskContext, context: &mut Cx) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll(task_context, context);
        let poll_b = pinned_b.poll(task_context, context);

        coalesce_polls(poll_a, poll_b)
    }
}

impl<Cx, A, const N: usize> Reifiable<Cx> for [A; N]
where
    A: Reifiable<Cx>,
{
    type Reified = [A::Reified; N];

    fn reify(self, _resources: &mut Cx) -> Self::Reified {
        todo!()
    }
}

impl<Cx, A: Send + BuildingBlock<Cx>, const N: usize> BuildingBlock<Cx> for [A; N] {
    fn handle_event(&mut self, _event: Event) -> bool {
        todo!("Handle the event, broadcasting to all children...")
    }
}

impl<Cx, A: Send + BuildingBlock<Cx>, const N: usize> Pollable<Cx> for [A; N] {
    fn poll(self: Pin<&mut Self>, _cx: &mut TaskContext, _resources: &mut Cx) -> Poll<Option<()>> {
        todo!("Poll all children and coalesce their polls, too.")
    }
}
