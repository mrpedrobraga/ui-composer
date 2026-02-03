use std::pin::Pin;
use std::task::{Context, Poll};
use downcast_rs::{impl_downcast, Downcast};
use crate::app::composition::algebra::Semigroup;

pub mod implementations;

pub struct DummyEnvironment();

pub trait Blueprint<Environment> {
    type Element: Element<Environment>;
    fn make(self, env: &Environment) -> Self::Element;
}

pub trait Element<Environment> {
    type Effect: ElementEffect;

    fn effect(&self) -> Self::Effect;

    fn poll(
        self: Pin<&mut Self>,
        #[expect(unused)] cx: &mut Context,
        #[expect(unused)] env: &Environment,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }
}

/// An effect that some element of a structure might produce.
///
/// For example, a `Graphic` might imply a rectangle should be drawn at some place on-screen.
/// Depending on the effect handler, this might result in quad instances being sent to the GPU
/// or rectangles drawn on the terminal or pixels in a GameBoy screen.
pub trait ElementEffect: Downcast {}
impl_downcast!(ElementEffect);

impl ElementEffect for () {}

impl<A, B> ElementEffect for (A, B)
where
    A: ElementEffect,
    B: ElementEffect,
{
}

impl<A> ElementEffect for Option<A> where A: ElementEffect, {}
