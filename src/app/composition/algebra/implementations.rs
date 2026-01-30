use crate::app::composition::algebra::{Bubble, Gather, Monoid, Semigroup};
use std::task::Poll;
use std::task::Poll::Pending;
/* Combine */

impl Semigroup for bool {
    fn combine(self, other: Self) -> Self {
        self || other
    }
}

impl Monoid for bool {
    fn empty() -> Self {
        false
    }
}

impl Semigroup for Poll<Option<()>> {
    /// If any of the values are Poll::Ready(Some(())), the result will be Poll::Ready(Some(()));
    /// If not and any of the values are Poll::Pending, the result will be Poll::Pending;
    /// Otherwise (if both values are Poll::Ready(None)), the result will be Poll::Ready(None);
    fn combine(self, other: Self) -> Self {
        use std::task::Poll::*;

        match (self, other) {
            (Ready(Some(())), _) | (_, Ready(Some(()))) => Ready(Some(())),
            (Pending, _) | (_, Pending) => Pending,
            _ => Ready(None),
        }
    }
}

impl Monoid for Poll<Option<()>> {
    fn empty() -> Self {
        Poll::Ready(None)
    }
}

pub mod bubble {
    use super::{Bubble, Monoid, Semigroup};

    impl<Down, Up> Bubble<Down, Up> for ()
    where
        Up: Monoid,
    {
        fn bubble(&mut self, #[allow(unused)] cx: &mut Down) -> Up {
            Monoid::empty()
        }
    }

    impl<A, B, Down, Up> Bubble<Down, Up> for (A, B)
    where
        A: Bubble<Down, Up>,
        B: Bubble<Down, Up>,
        Up: Semigroup,
    {
        fn bubble(&mut self, cx: &mut Down) -> Up {
            let a = self.0.bubble(cx);
            let b = self.1.bubble(cx);
            a.combine(b)
        }
    }

    impl<A, Down, Up, const N: usize> Bubble<Down, Up> for [A; N]
    where
        A: Bubble<Down, Up>,
        Up: Monoid,
    {
        fn bubble(&mut self, cx: &mut Down) -> Up {
            self.into_iter().fold(Monoid::empty(), |acc, el| {
                Semigroup::combine(acc, el.bubble(cx))
            })
        }
    }

    impl<A, Down, Up> Bubble<Down, Up> for Option<A>
    where
        A: Bubble<Down, Up>,
        Up: Monoid,
    {
        fn bubble(&mut self, cx: &mut Down) -> Up {
            match self {
                None => Up::empty(),
                Some(inner) => inner.bubble(cx),
            }
        }
    }

    impl<T, E, Down, Up> Bubble<Down, Up> for Result<T, E>
    where
        T: Bubble<Down, Up>,
        E: Bubble<Down, Up>,
        Up: Monoid,
    {
        fn bubble(&mut self, cx: &mut Down) -> Up {
            match self {
                Err(inner) => inner.bubble(cx),
                Ok(inner) => inner.bubble(cx),
            }
        }
    }

    #[cfg(feature = "std")]
    impl<A, Down, Up> Bubble<Down, Up> for Box<A>
    where
        A: Bubble<Down, Up>,
    {
        fn bubble(&mut self, cx: &mut Down) -> Up {
            self.as_mut().bubble(cx)
        }
    }
}

pub mod gather {
    use std::mem::MaybeUninit;
    use super::{Gather};

    impl<Context, Item> Gather<Context, Item> for ()
    {
        const SIZE: usize = 0;

        fn gather(&mut self, #[expect(unused)] cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), 0);
        }
    }

    impl<A, B, Context, Item> Gather<Context, Item> for (A, B)
    where
        A: Gather<Context, Item>,
        B: Gather<Context, Item>,
    {
        const SIZE: usize = A::SIZE + B::SIZE;

        fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), Self::SIZE);
            let (left, right) = acc.split_at_mut(A::SIZE);
            let a = self.0.gather(cx, left);
            let b = self.1.gather(cx, right);
        }
    }

    impl<A, Context, Item, const N: usize> Gather<Context, Item> for [A; N]
    where
        A: Gather<Context, Item>,
    {
        const SIZE: usize = A::SIZE * N;

        fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), Self::SIZE);

            for (i, elem) in self.iter_mut().enumerate() {
                let start = i * A::SIZE;
                let end = start + A::SIZE;
                elem.gather(cx, &mut acc[start..end]);
            }
        }
    }

    impl<A, Context, Item> Gather<Context, Item> for Option<A>
    where
        A: Gather<Context, Item>,
    {
        const SIZE: usize = A::SIZE;

        fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), Self::SIZE);
            match self {
                None => { /* Leave buffer initialized. */ },
                Some(inner) => inner.gather(cx, acc),
            }
        }
    }

    impl<T, E, Context, Item> Gather<Context, Item> for Result<T, E>
    where
        T: Gather<Context, Item>,
        E: Gather<Context, Item>,
    {
        const SIZE: usize = if T::SIZE > E::SIZE { T::SIZE } else { E::SIZE };

        fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), Self::SIZE);

            /* Assuming space that isn't written to is initialized! */

            match self {
                Err(inner) => inner.gather(cx, &mut acc[..E::SIZE]),
                Ok(inner) => inner.gather(cx, &mut acc[..T::SIZE]),
            }
        }
    }

    impl<A, Context, Item> Gather<Context, Item> for Box<A>
    where
        A: Gather<Context, Item>,
    {
        const SIZE: usize = A::SIZE;

        fn gather(&mut self, cx: &mut Context, acc: &mut [MaybeUninit<Item>]) {
            debug_assert_eq!(acc.len(), Self::SIZE);
            let a = self.as_mut().gather(cx, acc);
        }
    }
}
