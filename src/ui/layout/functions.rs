//! For a lot of these containers, the meaning of the x, y, w and h coordinates
//! is a little different. 'x' and 'w' mean 'in-axis' metrics. 'y' and 'h' mean 'cross-axis' metrics.
//! So a container that disposes its items vertically is increasing the 'x' coordinate.

use cgmath::BaseFloat;
use vek::{Extent2, Rect, Vec2};

struct StackContext {
    offset: Vec2<f32>,
}

impl Default for StackContext {
    fn default() -> Self {
        Self {
            offset: Vec2::default(),
        }
    }
}

/// Stacks several sizes ([`Extent2`]s) one after another.
/// The resulting [`Rect`]s will not be stretched at all.
#[inline(always)]
pub fn stack_rects<I>(sizes: I, gap: f32, vertical: bool) -> impl Iterator<Item = Rect<f32, f32>>
where
    I: Iterator<Item = Extent2<f32>>,
{
    sizes.scan(StackContext::default(), move |cx, item| {
        let rect = Rect::new(cx.offset.x, cx.offset.y, item.w, item.h);

        if (vertical) {
            cx.offset.y += item.h;
            cx.offset.y += gap;
        } else {
            cx.offset.x += item.w;
            cx.offset.x += gap;
        }

        Some(rect)
    })
}

/// Divides a total number of shares for n elements, where the elements can be biased with a weight, or have a minimum share.
/// It does three dynamic allocations, and runs in O(n * log(n)).
pub fn weighted_division_with_minima<T: BaseFloat + std::iter::Sum>(
    total: T,
    item_weights: &[T],
    item_minima: &[T],
) -> Vec<T> {
    let item_count = item_weights.len();
    // Imagine a container with size x on the lim x -> Infinity.
    // In such a container, minimum size doesn't matter.
    // If you shrink this container, eventually *some* element will hit its
    // minimum size. The elements need to be addressed in the order they hit the minimum size.
    let mut indices = (0..item_count).collect::<Vec<usize>>();
    indices.sort_by(|&i_a, &i_b| {
        let ratio_a = item_minima[i_a] * item_weights[i_a];
        let ratio_b = item_minima[i_b] * item_weights[i_b];
        ratio_b.partial_cmp(&ratio_a).unwrap()
    });
    let total_weight_count = item_weights.iter().copied().sum();
    // After that, we know the characteristics of which elements
    // will be taken off the total, so we can pre-calculate the sums of the weights
    // of the remaining objects.
    let remaining_weight_sums = indices.iter().scan(total_weight_count, |acc, i| {
        let result = Some(*acc);
        *acc -= item_weights[*i];
        result
    });
    // Then, each element will calculate how much they take from the total
    // which will either be their minimum size, or a calculated fraction of the
    // remaining space;
    let sizes = indices.iter().zip(remaining_weight_sums).scan(
        total,
        |space_left, (i, remaining_weight_sum)| {
            let el_share_count = *space_left * item_weights[*i] / remaining_weight_sum;
            let size = item_minima[*i].max(el_share_count);
            *space_left -= size;
            Some(size)
        },
    );
    // Finally, you need to return the sizes in the original order.
    let mut result = vec![T::zero(); item_count];
    for (index, size) in indices.iter().zip(sizes) {
        result[*index] = size;
    }
    result
}
