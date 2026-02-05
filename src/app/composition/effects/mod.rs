use std::fmt::Debug;
use downcast_rs::{impl_downcast, Downcast};

pub mod future;
pub mod signal;
pub mod executor;

/// An effect that some element of a structure might produce.
///
/// For example, a `Graphic` might imply a rectangle should be drawn at some place on-screen.
/// Depending on the effect handler, this might result in quad instances being sent to the GPU
/// or rectangles drawn on the terminal or pixels in a GameBoy screen.
pub trait ElementEffect: Downcast + Debug {}

impl ElementEffect for () {}

impl<A, B> ElementEffect for (A, B)
where
    A: ElementEffect,
    B: ElementEffect,
{
}

impl<A> ElementEffect for Option<A> where A: ElementEffect, {}

impl_downcast!(ElementEffect);

/// Please refer to the [module level documentation](self).
pub trait EffectHandler {
    fn handle<E>(&mut self, event: E) where E: 'static + ElementEffect;
}

#[test]
fn test_effect_handler() {
    #[derive(Debug)]
    pub struct DummyEffect;
    impl ElementEffect for DummyEffect {}

    #[derive(Debug)]
    pub struct NullEffect;
    impl ElementEffect for NullEffect {}

    pub struct DummyEffectHandler {}

    impl EffectHandler for DummyEffectHandler {
        fn handle<E>(&mut self, effect: E) where E: 'static + ElementEffect {
            let effect: &dyn ElementEffect = &effect;

            if let Some(dummy) = effect.downcast_ref::<DummyEffect>() {
                println!("Holy shit!")
            }
        }
    }

    let a = DummyEffect;
    let b = DummyEffect;
    let c = NullEffect;
    //let effects = (a, (b, c));

    let mut handler = DummyEffectHandler {};
    handler.handle(a);
}