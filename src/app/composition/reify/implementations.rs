use super::Emit;

impl<Cx> Emit<Cx> for () {
    type Output = ();

    fn reify(self, #[expect(unused)] context: &mut Cx) -> Self::Output {
        ()
    }
}

impl<Cx, A, B> Emit<Cx> for (A, B)
where
    A: Emit<Cx>,
    B: Emit<Cx>,
{
    type Output = (A::Output, B::Output);

    fn reify(self, context: &mut Cx) -> Self::Output {
        (self.0.reify(context), self.1.reify(context))
    }
}

impl<Cx, A, const N: usize> Emit<Cx> for [A; N]
where
    A: Emit<Cx>,
{
    type Output = [A::Output; N];

    fn reify(self, _resources: &mut Cx) -> Self::Output {
        todo!()
    }
}

impl<Cx, T> Emit<Cx> for Option<T>
where
    T: Emit<Cx>,
{
    type Output = Option<T::Output>;

    fn reify(self, context: &mut Cx) -> Self::Output {
        self.map(|v| v.reify(context))
    }
}

impl<Cx, T, E> Emit<Cx> for Result<T, E>
where
    T: Emit<Cx>,
    E: Emit<Cx>,
{
    type Output = Result<T::Output, E::Output>;

    fn reify(self, context: &mut Cx) -> Self::Output {
        match self {
            Ok(v) => Ok(Emit::reify(v, context)),
            Err(e) => Err(Emit::reify(e, context)),
        }
    }
}

#[cfg(feature = "std")]
impl<Cx, A> Emit<Cx> for Box<A>
where
    A: Emit<Cx>,
{
    type Output = A::Output;

    fn reify(self, context: &mut Cx) -> Self::Output {
        (*self).reify(context)
    }
}