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
