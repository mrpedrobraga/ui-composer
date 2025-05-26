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
pub fn weighted_division_with_minima<T: BaseFloat + std::iter::Sum>(
    total: T,
    w: &[T],
    m: &[T],
    tolerance: T,
) -> Vec<T> {
    let total_m: T = m.iter().copied().sum();
    let total_w: T = w.iter().copied().sum();

    if total_m >= total || total_w <= T::zero() {
        return m.to_vec();
    }

    // Precompute normalized weights
    let normalized_w: Vec<T> = w.iter().map(|&w| w / total_w).collect();

    let equation = |x| {
        total
            - normalized_w
                .iter()
                .zip(m.iter())
                .map(|(nw, m)| m.max(total * *nw * x))
                .sum::<T>()
    };

    let mut lower_bound = T::zero();
    let mut upper_bound = total;

    loop {
        let sample_point = (lower_bound + upper_bound) / T::from(2).unwrap();
        let error = equation(sample_point);

        if error.abs() < tolerance {
            return normalized_w
                .iter()
                .zip(m.iter())
                .map(|(nw, m)| m.max(total * *nw * sample_point))
                .collect();
        }

        if error > T::zero() {
            lower_bound = sample_point;
        } else {
            upper_bound = sample_point;
        }
    }
}
