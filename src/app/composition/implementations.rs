use crate::app::composition::algebra::Semigroup;
use crate::state::process::Pollable;
use {
    core::{
        pin::Pin,
        task::{Context as TaskContext, Poll},
    },
};

impl<Cx> Pollable<Cx> for () {}

impl<Cx, A: Send + Pollable<Cx>> Pollable<Cx> for Option<A> {
    fn poll(
        self: Pin<&mut Self>,
        task_context: &mut TaskContext,
        context: &mut Cx,
    ) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll(task_context, context))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<Cx, T, E> Pollable<Cx> for Result<T, E>
where
    T: Send + Pollable<Cx>,
    E: Send + Pollable<Cx>,
{
    fn poll(
        self: Pin<&mut Self>,
        task_context: &mut TaskContext,
        context: &mut Cx,
    ) -> Poll<Option<()>> {
        // TODO: I don't understand pin, is this safe chat??
        unsafe {
            match self.get_unchecked_mut() {
                Ok(t) => Pin::new_unchecked(t).poll(task_context, context),
                Err(e) => Pin::new_unchecked(e).poll(task_context, context),
            }
        }
    }
}

impl<Cx, A: Send + Pollable<Cx>, B: Send + Pollable<Cx>> Pollable<Cx> for (A, B) {
    fn poll(
        self: Pin<&mut Self>,
        task_context: &mut TaskContext,
        context: &mut Cx,
    ) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll(task_context, context);
        let poll_b = pinned_b.poll(task_context, context);

        Semigroup::combine(poll_a, poll_b)
    }
}

impl<Cx, A: Send + Pollable<Cx>, const N: usize> Pollable<Cx> for [A; N] {
    fn poll(self: Pin<&mut Self>, _cx: &mut TaskContext, _resources: &mut Cx) -> Poll<Option<()>> {
        todo!("Poll all children and coalesce their polls, too.")
    }
}
