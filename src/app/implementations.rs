use crate::app::primitives::PollProcessors;
use {
    super::{input::Event, primitives::Primitive},
    crate::state::signal_ext::coalesce_polls,
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
};

impl Primitive for () {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl PollProcessors for () {
    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}

impl<A: Send + Primitive> Primitive for Option<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_event(event))
            .unwrap_or(false)
    }
}

impl<A: Send + Primitive> PollProcessors for Option<A> {
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll_processors(cx))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<T: Send + Primitive, E: Send + Primitive> Primitive for Result<T, E> {
    fn handle_event(&mut self, event: Event) -> bool {
        match self {
            Ok(v) => v.handle_event(event),
            Err(e) => e.handle_event(event),
        }
    }
}

impl<T: Send + Primitive, E: Send + Primitive> PollProcessors for Result<T, E> {
    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        // let this: &mut Self = self.deref_mut();
        // match this {
        //     Ok(v) => todo!(),
        //     Err(e) => todo!(),
        // }
        unimplemented!()
    }
}

#[cfg(feature = "std")]
impl<A: Send + Primitive> Primitive for Box<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut().handle_event(event)
    }
}

#[cfg(feature = "std")]
impl<A: Send + Primitive> PollProcessors for Box<A> {
    fn poll_processors(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll_processors(cx)
    }
}

impl<A: Send + Primitive, B: Send + Primitive> Primitive for (A, B) {
    fn handle_event(&mut self, event: Event) -> bool {
        let a_handled = self.0.handle_event(event.clone());
        let b_handled = self.1.handle_event(event);

        a_handled || b_handled
    }
}

impl<A: Send + Primitive, B: Send + Primitive> PollProcessors for (A, B) {
    fn poll_processors(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (ref mut a, ref mut b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_processors(cx);
        let poll_b = pinned_b.poll_processors(cx);

        coalesce_polls(poll_a, poll_b)
    }
}
