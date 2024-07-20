use super::render_stack::{AllocationInfo, UIFragment};

impl<'a, T, const N: usize> UIFragment for [T; N]
where
    T: UIFragment,
{
    fn get_allocation_info(&self) -> AllocationInfo {
        self.iter()
            .map(|fragment| fragment.get_allocation_info())
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

    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
        self.iter()
            .for_each(|fragment| fragment.push_allocation(primitive_buffer));
    }
}

impl<'a, T> UIFragment for &'a [T]
where
    T: UIFragment,
{
    fn get_allocation_info(&self) -> AllocationInfo {
        self.iter()
            .map(|fragment| fragment.get_allocation_info())
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

    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
        self.iter()
            .for_each(|fragment| fragment.push_allocation(primitive_buffer));
    }
}

impl<'a, T> UIFragment for Vec<T>
where
    T: UIFragment,
{
    fn get_allocation_info(&self) -> AllocationInfo {
        self.iter()
            .map(|fragment| fragment.get_allocation_info())
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

    fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
        self.iter()
            .for_each(|fragment| fragment.push_allocation(primitive_buffer));
    }
}

macro_rules! tuple_impls {
    ( $( $name:ident )+ ) => {
        impl<$($name: UIFragment),+> UIFragment for ($($name,)+)
        {
            fn get_allocation_info(&self) -> AllocationInfo {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;

                [$($name.get_allocation_info()),+]
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

            fn push_allocation(&self, primitive_buffer: &mut Vec<u8>) {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;
                $($name.push_allocation(primitive_buffer);)+
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
