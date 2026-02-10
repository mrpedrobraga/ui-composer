pub mod future;
pub mod signal;

/// An effect that some element of a structure might produce.
///
/// For example, a `Graphic` might imply a rectangle should be drawn at some place on-screen.
/// Depending on the effect handler, this might result in quad instances being sent to the GPU
/// or rectangles drawn on the terminal or pixels in a GameBoy screen.
#[diagnostic::on_unimplemented(
    message = "{Self} is not an effect applicable to {Environment}."
)]
pub trait ElementEffect<Environment> {
    /// Applies the effect to an environment — this can be something like queueing a render,
    /// a sound effect, a task, etc.
    fn apply(&self, env: &mut Environment);
}

/// An [Effect] gathered into an ADT with reflection.
#[diagnostic::on_unimplemented(
    message = "{Self} is not an effect node applicable to {Environment}."
)]
pub trait ElementEffectNode<Environment> {
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Environment>;
}

impl<Env> ElementEffectNode<Env> for () {
    fn visit_with<Handler>(&self, _: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        /* Nothing to visit. */
    }
}

impl<Env, A, B> ElementEffectNode<Env> for (A, B)
where
    A: ElementEffectNode<Env>,
    B: ElementEffectNode<Env>,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        self.0.visit_with(h);
        self.1.visit_with(h);
    }
}

impl<Env, A> ElementEffectNode<Env> for Option<A>
where
    A: ElementEffectNode<Env>,
{
    fn visit_with<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        if let Some(inner) = self {
            inner.visit_with(h);
        }
    }
}

/// Please refer to the [module level documentation](self).
pub trait EffectHandler<Environment> {
    fn handle_one<E>(&mut self, effect: &E)
    where
        E: 'static + ElementEffect<Environment>;
}

#[test]
fn visit_all() {
    use crate::list_internal;

    #[derive(Debug)]
    struct SomeEnvironment {
        state: usize,
    }

    #[derive(Debug)]
    struct SomeHandler {
        env: SomeEnvironment,
    }

    impl EffectHandler<SomeEnvironment> for SomeHandler {
        fn handle_one<E>(&mut self, effect: &E)
        where
            E: 'static + ElementEffect<SomeEnvironment>,
        {
            effect.apply(&mut self.env);
        }
    }

    struct EffectA;

    struct EffectB;

    impl ElementEffect<SomeEnvironment> for EffectA {
        fn apply(&self, env: &mut SomeEnvironment) {
            env.state += 1;
        }
    }
    impl<Env> ElementEffectNode<Env> for EffectA
    where
        EffectA: ElementEffect<Env>,
    {
        fn visit_with<Handler>(&self, h: &mut Handler)
        where
            Handler: EffectHandler<Env>,
        {
            h.handle_one(self);
        }
    }
    impl ElementEffect<SomeEnvironment> for EffectB {
        fn apply(&self, _: &mut SomeEnvironment) {
            /* Nothing! */
        }
    }
    impl ElementEffectNode<SomeEnvironment> for EffectB {
        fn visit_with<Handler>(&self, h: &mut Handler)
        where
            Handler: EffectHandler<SomeEnvironment>,
        {
            h.handle_one(self);
        }
    }

    /* Using */

    let mut h = SomeHandler {
        env: SomeEnvironment { state: 0 },
    };
    let effect_tree = list_internal!(
        EffectA,
        EffectB,
        Some(EffectA),
        None as Option<EffectB>
    );
    effect_tree.visit_with(&mut h);
    dbg!(&h);
    assert_eq!(h.env.state, 2);
}
