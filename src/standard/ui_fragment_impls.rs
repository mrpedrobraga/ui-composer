use super::render::{
    tuple_render_module::TupleRenderModule, AllocationInfo, AllocationOffset, UIFragment,
};
use crate::{
    interaction::InteractorNodeContainer,
    reaction::UnknownReactor,
    render_module::{self, RenderModule},
};

impl<T, const N: usize> UIFragment for [T; N]
where
    T: UIFragment + Clone,
{
    fn get_allocation_info() -> AllocationInfo {
        let inner_alloc_info = T::get_allocation_info();
        AllocationInfo {
            buffer_size: inner_alloc_info.buffer_size * N,
            primitive_count: inner_alloc_info.primitive_count * N,
        }
    }

    fn splat_allocation(
        self,
        allocation_offset: AllocationOffset,
        render_module: &mut TupleRenderModule,
        temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
    ) {
        let mut offset = allocation_offset;
        let allocation_info = T::get_allocation_info();

        for fragment in self.iter().cloned() {
            fragment.splat_allocation(allocation_offset, render_module, temp_reactors);
            offset.offset_by_allocation(&allocation_info);
        }
    }
}

impl<T, E> UIFragment for Result<T, E>
where
    T: UIFragment,
    E: UIFragment,
{
    fn get_allocation_info() -> AllocationInfo {
        let allocation_info_ok = T::get_allocation_info();
        let allocation_info_err = T::get_allocation_info();

        AllocationInfo {
            buffer_size: Ord::max(
                allocation_info_ok.buffer_size,
                allocation_info_err.buffer_size,
            ),
            primitive_count: Ord::max(
                allocation_info_err.primitive_count,
                allocation_info_err.primitive_count,
            ),
        }
    }

    fn splat_allocation(
        self,
        allocation_offset: AllocationOffset,
        render_module: &mut TupleRenderModule,
        temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
    ) {
        match self {
            Ok(ok) => ok.splat_allocation(allocation_offset, render_module, temp_reactors),
            Err(err) => err.splat_allocation(allocation_offset, render_module, temp_reactors),
        }
    }
}

pub struct SizedVec<T: UIFragment, const N: usize>(Vec<T>);

impl<T: UIFragment, const N: usize> SizedVec<T, N> {
    pub fn new(value: Vec<T>) -> Self {
        Self(value)
    }
}

impl<A: UIFragment, const N: usize> FromIterator<A> for SizedVec<A, N> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        Self::new(Vec::from_iter(iter))
    }
}

impl<T, const N: usize> UIFragment for SizedVec<T, N>
where
    T: UIFragment + Clone,
{
    fn get_allocation_info() -> AllocationInfo {
        let inner_alloc_info = T::get_allocation_info();
        AllocationInfo {
            buffer_size: inner_alloc_info.buffer_size * N,
            primitive_count: inner_alloc_info.primitive_count * N,
        }
    }

    fn splat_allocation(
        self,
        allocation_offset: AllocationOffset,
        render_module: &mut TupleRenderModule,
        temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
    ) {
        let mut offset = allocation_offset;
        let allocation_info = T::get_allocation_info();

        for fragment in self.0.into_iter().take(N) {
            fragment.splat_allocation(offset, render_module, temp_reactors);
            offset.offset_by_allocation(&allocation_info);
        }
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: UIFragment),+> UIFragment for ($($name,)+)
        {
            fn get_allocation_info() -> AllocationInfo {
                [$($name::get_allocation_info()),+]
                    .iter()
                    .fold(
                        AllocationInfo {
                            buffer_size: 0,
                            primitive_count: 0,
                        },
                        |acc, fragment| AllocationInfo {
                            buffer_size: acc.buffer_size + fragment.buffer_size,
                            primitive_count: acc.primitive_count + fragment.primitive_count,
                        },
                    )
            }

            fn splat_allocation(
                self,
                allocation_offset: AllocationOffset,
                render_module: &mut TupleRenderModule,
                temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
            ) {
                let mut offset = allocation_offset;
                #[allow(non_snake_case)]
                let ($($name,)+) = self;

                $({
                    $name.splat_allocation(offset, render_module, temp_reactors);
                    offset.offset_by_allocation(&A::get_allocation_info());
                })+
            }
        }
    };
}

tuple_impls! { A }
tuple_impls! { A B }
tuple_impls! { A B C }
tuple_impls! { A B C D }
tuple_impls! { A B C D E }
tuple_impls! { A B C D E F }
tuple_impls! { A B C D E F G }
tuple_impls! { A B C D E F G H }
tuple_impls! { A B C D E F G H I }
tuple_impls! { A B C D E F G H I J }
tuple_impls! { A B C D E F G H I J K }
tuple_impls! { A B C D E F G H I J K L }
tuple_impls! { A B C D E F G H I J K L M }
tuple_impls! { A B C D E F G H I J K L M N }
tuple_impls! { A B C D E F G H I J K L M N O }
tuple_impls! { A B C D E F G H I J K L M N O P }
