use super::Reify;

impl<Cx> Reify<Cx> for () {
    type Output = ();

    fn reify(self, #[expect(unused)] context: &mut Cx) -> Self::Output {
        ()
    }
}

impl<Cx, A, B> Reify<Cx> for (A, B)
where
    A: Reify<Cx>,
    B: Reify<Cx>,
{
    type Output = (A::Output, B::Output);

    fn reify(self, context: &mut Cx) -> Self::Output {
        (self.0.reify(context), self.1.reify(context))
    }
}

impl<Cx, A, const N: usize> Reify<Cx> for [A; N]
where
    A: Reify<Cx>,
{
    type Output = [A::Output; N];

    fn reify(self, _resources: &mut Cx) -> Self::Output {
        todo!()
    }
}

impl<Cx, T> Reify<Cx> for Option<T>
where
    T: Reify<Cx>,
{
    type Output = Option<T::Output>;

    fn reify(self, context: &mut Cx) -> Self::Output {
        self.map(|v| v.reify(context))
    }
}

impl<Cx, T, E> Reify<Cx> for Result<T, E>
where
    T: Reify<Cx>,
    E: Reify<Cx>,
{
    type Output = Result<T::Output, E::Output>;

    fn reify(self, context: &mut Cx) -> Self::Output {
        match self {
            Ok(v) => Ok(Reify::reify(v, context)),
            Err(e) => Err(Reify::reify(e, context)),
        }
    }
}

#[cfg(feature = "std")]
impl<Cx, A> Reify<Cx> for Box<A>
where
    A: Reify<Cx>,
{
    type Output = A::Output;

    fn reify(self, context: &mut Cx) -> Self::Output {
        (*self).reify(context)
    }
}