use crate::app::primitives::{PrimitiveDescriptor, Processor};
use {
    super::{input::Event, primitives::Primitive},
    crate::state::signal_ext::coalesce_polls,
    core::{
        pin::Pin,
        task::{Context, Poll},
    },
};

impl<Res> PrimitiveDescriptor<Res> for () {
    type Primitive = ();

    #[expect(unused)]
    fn reify(self, resources: &mut Res) -> Self::Primitive {
        /* return unit */
    }
}

impl<Res> Primitive<Res> for () {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Res> Processor<Res> for () {}

impl<Res, T> PrimitiveDescriptor<Res> for Option<T>
where
    T: PrimitiveDescriptor<Res>,
{
    type Primitive = Option<T::Primitive>;

    fn reify(self, resources: &mut Res) -> Self::Primitive {
        self.map(|v| v.reify(resources))
    }
}

impl<Res, A: Send + Primitive<Res>> Primitive<Res> for Option<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut()
            .map(|inner| inner.handle_event(event))
            .unwrap_or(false)
    }
}

impl<Res, A: Send + Processor<Res>> Processor<Res> for Option<A> {
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        // TODO: Maybe I shouldn't return Some(()) in the option by default?
        self.as_pin_mut()
            .map(|inner| inner.poll(cx, resources))
            .unwrap_or(Poll::Ready(Some(())))
    }
}

impl<Res, T, E> PrimitiveDescriptor<Res> for Result<T, E>
where
    T: PrimitiveDescriptor<Res>,
    E: PrimitiveDescriptor<Res>,
{
    type Primitive = Result<T::Primitive, E::Primitive>;

    fn reify(self, resources: &mut Res) -> Self::Primitive {
        match self {
            Ok(v) => Ok(PrimitiveDescriptor::reify(v, resources)),
            Err(e) => Err(PrimitiveDescriptor::reify(e, resources)),
        }
    }
}

impl<Res, T: Send + Primitive<Res>, E: Send + Primitive<Res>> Primitive<Res> for Result<T, E> {
    fn handle_event(&mut self, event: Event) -> bool {
        match self {
            Ok(v) => v.handle_event(event),
            Err(e) => e.handle_event(event),
        }
    }
}

impl<Res, T, E> Processor<Res> for Result<T, E>
where
    T: Send + Processor<Res>,
    E: Send + Processor<Res>,
{
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        // TODO: I don't understand pin, is this safe chat??
        unsafe {
            match self.get_unchecked_mut() {
                Ok(t) => Pin::new_unchecked(t).poll(cx, resources),
                Err(e) => Pin::new_unchecked(e).poll(cx, resources),
            }
        }
    }
}

#[cfg(feature = "std")]
impl<Res, A> PrimitiveDescriptor<Res> for Box<A>
where
    A: PrimitiveDescriptor<Res>,
{
    type Primitive = A::Primitive;

    fn reify(self, resources: &mut Res) -> Self::Primitive {
        (*self).reify(resources)
    }
}

#[cfg(feature = "std")]
impl<Res, A: Send + Primitive<Res>> Primitive<Res> for Box<A> {
    fn handle_event(&mut self, event: Event) -> bool {
        self.as_mut().handle_event(event)
    }
}

#[cfg(feature = "std")]
impl<Res, A: Send + Processor<Res>> Processor<Res> for Box<A> {
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        // TODO: Why is this unsafe?
        let inner = unsafe { self.as_mut().map_unchecked_mut(|v| &mut **v) };
        inner.poll(cx, resources)
    }
}

impl<Res, A, B> PrimitiveDescriptor<Res> for (A, B)
where
    A: PrimitiveDescriptor<Res>,
    B: PrimitiveDescriptor<Res>,
{
    type Primitive = (A::Primitive, B::Primitive);

    fn reify(self, resources: &mut Res) -> Self::Primitive {
        (self.0.reify(resources), self.1.reify(resources))
    }
}

impl<Res, A: Send + Primitive<Res>, B: Send + Primitive<Res>> Primitive<Res> for (A, B) {
    fn handle_event(&mut self, event: Event) -> bool {
        let a_handled = self.0.handle_event(event.clone());
        let b_handled = self.1.handle_event(event);

        a_handled || b_handled
    }
}

impl<Res, A: Send + Primitive<Res>, B: Send + Primitive<Res>> Processor<Res> for (A, B) {
    fn poll(self: Pin<&mut Self>, cx: &mut Context, resources: &mut Res) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll(cx, resources);
        let poll_b = pinned_b.poll(cx, resources);

        coalesce_polls(poll_a, poll_b)
    }
}

impl<Res, A, const N: usize> PrimitiveDescriptor<Res> for [A; N]
where
    A: PrimitiveDescriptor<Res>,
{
    type Primitive = [A::Primitive; N];

    fn reify(self, _resources: &mut Res) -> Self::Primitive {
        todo!()
    }
}

impl<Res, A: Send + Primitive<Res>, const N: usize> Primitive<Res> for [A; N] {
    fn handle_event(&mut self, _event: Event) -> bool {
        todo!("Handle the event, broadcasting to all children...")
    }
}

impl<Res, A: Send + Primitive<Res>, const N: usize> Processor<Res> for [A; N] {
    fn poll(self: Pin<&mut Self>, _cx: &mut Context, _resources: &mut Res) -> Poll<Option<()>> {
        todo!("Poll all children and coalesce their polls, too.")
    }
}
