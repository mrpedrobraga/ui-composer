use downcast_rs::{Downcast, impl_downcast};
use std::fmt::Debug;

pub mod future;
pub mod signal;

/// An effect that some element of a structure might produce.
///
/// For example, a `Graphic` might imply a rectangle should be drawn at some place on-screen.
/// Depending on the effect handler, this might result in quad instances being sent to the GPU
/// or rectangles drawn on the terminal or pixels in a GameBoy screen.
pub trait ElementEffect: Downcast + Debug {}
impl_downcast!(ElementEffect);

/// An [Effect] gathered into an ADT with reflection.
pub trait ElementEffectNode {
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler;
}

impl<T> ElementEffectNode for T
where
    T: ElementEffect,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler,
    {
        h.handle(self);
    }
}

impl ElementEffectNode for () {
    fn visit_with<Handler>(&self, _: &mut Handler)
    where
        Handler: EffectHandler,
    {
        /* Nothing to visit. */
    }
}

impl<A, B> ElementEffectNode for (A, B)
where
    A: ElementEffectNode,
    B: ElementEffectNode,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler,
    {
        self.0.visit_with(h);
        self.1.visit_with(h);
    }
}

impl<A> ElementEffectNode for Option<A>
where
    A: ElementEffectNode,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler,
    {
        if let Some(inner) = self {
            inner.visit_with(h);
        }
    }
}

/// Please refer to the [module level documentation](self).
pub trait EffectHandler {
    fn handle<E>(&mut self, effect: &E)
    where
        E: 'static + ElementEffect;
}

#[test]
fn visit_all() {
    use crate::items_internal;

    #[derive(Debug)]
    struct EffectA;

    #[derive(Debug)]
    struct EffectB;

    impl ElementEffect for EffectA {}
    impl ElementEffect for EffectB {}

    struct SomeHandler;

    impl EffectHandler for SomeHandler {
        fn handle<E>(&mut self, effect: &E)
        where
            E: 'static + ElementEffect,
        {
            let effect: &dyn ElementEffect = effect;

            if effect.downcast_ref::<EffectA>().is_some() {
                println!("[Handler] Holy shit is this an EffectA?");
            } else if effect.downcast_ref::<EffectB>().is_some() {
                println!("[Handler] Oh, brother, this EffeectB stinks...");
            }
        }
    }

    let mut h = SomeHandler;
    let effect_tree = items_internal!(
        EffectA,
        EffectB,
        Some(EffectA),
        None as Option<EffectB>
    );
    effect_tree.visit_with(&mut h);
}
