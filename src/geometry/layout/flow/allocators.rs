//! Module for layout "allocators", types useful for distributing
//! a container's "real estate" to its children.
//!
//! These functions are supposed to be incredibly optimized for inlining and
//! no_std environments.

use arrayvec::ArrayVec;
use cgmath::BaseFloat;
use vek::{Extent2, Rect, Vec2};

/// Struct that can be used to allocate rects for layout items in
/// simple stacks.
pub struct RectStackAllocator {
    offset: Vec2<f32>,
}

impl RectStackAllocator {
    #[inline(always)]
    pub fn stack<I>(sizes: I, gap: f32, vertical: bool) -> impl Iterator<Item = Rect<f32, f32>>
    where
        I: Iterator<Item = Extent2<f32>>,
    {
        sizes.scan(
            RectStackAllocator {
                offset: Vec2::zero(),
            },
            move |cx, item| {
                let rect = Rect::new(cx.offset.x, cx.offset.y, item.w, item.h);

                // TODO: Use a [`CoordinateSystemProvider`] for this.
                if vertical {
                    cx.offset.y += item.h;
                    cx.offset.y += gap;
                } else {
                    cx.offset.x += item.w;
                    cx.offset.x += gap;
                }

                Some(rect)
            },
        )
    }
}

/// Utility function that divides a total amount of real estate amongst n elements,
/// where the elements can be biased with a weight, or have a minimum share.
///
/// For example, if you have 100.0 of real estate, and four elements with equal weight,
/// you'd distribute 25.0 for each...
///
/// ```[AAAAABBBBBCCCCCDDDDD]```
///
/// ...but if one of those elements has a weight of 2.0,
/// it gets "twice" as much space as the others.
///
/// ```[AAAAAAAABBBBCCCCDDDD]```
pub fn weighted_division_with_minima<const SIZE: usize, T: BaseFloat + core::iter::Sum>(
    total: T,
    w: &[T; SIZE],
    m: &[T; SIZE],
    tolerance: T,
) -> ArrayVec<T, SIZE> {
    let total_m: T = m.iter().copied().sum();
    let total_w: T = w.iter().copied().sum();

    if total_m >= total || total_w <= T::zero() {
        return ArrayVec::from(*m);
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
