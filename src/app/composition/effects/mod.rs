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
pub trait Drive<Environment> {
    fn drive_thru<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Environment>;
}

impl<Env> Drive<Env> for () {
    fn drive_thru<Handler>(&self, _: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        /* Nothing to visit. */
    }
}

impl<Env, A, B> Drive<Env> for (A, B)
where
    A: Drive<Env>,
    B: Drive<Env>,
{
    fn drive_thru<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        self.0.drive_thru(h);
        self.1.drive_thru(h);
    }
}

impl<Env, A> Drive<Env> for Option<A>
where
    A: Drive<Env>,
{
    fn drive_thru<Handler>(&self, h: &mut Handler)
    where
        Handler: EffectHandler<Env>,
    {
        if let Some(inner) = self {
            inner.drive_thru(h);
        }
    }
}

/// Please refer to the [module level documentation](self).
pub trait EffectHandler<Environment> {
    fn visit<E>(&mut self, effect: &E)
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
        fn visit<E>(&mut self, effect: &E)
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
    impl<Env> Drive<Env> for EffectA
    where
        EffectA: ElementEffect<Env>,
    {
        fn drive_thru<Handler>(&self, h: &mut Handler)
        where
            Handler: EffectHandler<Env>,
        {
            h.visit(self);
        }
    }
    impl ElementEffect<SomeEnvironment> for EffectB {
        fn apply(&self, _: &mut SomeEnvironment) {
            /* Nothing! */
        }
    }
    impl Drive<SomeEnvironment> for EffectB {
        fn drive_thru<Handler>(&self, h: &mut Handler)
        where
            Handler: EffectHandler<SomeEnvironment>,
        {
            h.visit(self);
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
    effect_tree.drive_thru(&mut h);
    dbg!(&h);
    assert_eq!(h.env.state, 2);
}
