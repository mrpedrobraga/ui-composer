//! Module for layout "allocators", types useful for distributing
//! a container's "real estate" to its children.
//!
//! These functions are supposed to be incredibly optimized for inlining and
//! no_std environments.

use glamour::Vector2;
use num_traits::Float;
use {
    arrayvec::ArrayVec,
    glamour::{Rect, Size2},
};

/// Struct that can be used to allocate rects for layout items in
/// simple stacks.
pub struct RectStackArranger {
    offset: Vector2,
}

impl RectStackArranger {
    #[inline(always)]
    pub fn stack<I>(
        sizes: I,
        gap: f32,
        vertical: bool,
    ) -> impl Iterator<Item = Rect>
    where
        I: Iterator<Item = Size2>,
    {
        sizes.scan(
            RectStackArranger {
                offset: Vector2::ZERO,
            },
            move |cx, item_size| {
                let rect = Rect::new(cx.offset.to_point(), item_size);

                // TODO: Use a [`CoordinateSystemProvider`] for this.
                if vertical {
                    cx.offset.y += item_size.height;
                    cx.offset.y += gap;
                } else {
                    cx.offset.x += item_size.width;
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
pub fn arrange_stretchy_rects_with_minimum_sizes<
    const SIZE: usize,
    Num: Float + core::iter::Sum,
>(
    container_size: Num,
    stretch_weights: &[Num; SIZE],
    minima: &[Num; SIZE],
    tolerance: Num,
) -> ArrayVec<Num, SIZE> {
    let combined_minimum_size: Num = minima.iter().copied().sum();
    let combined_weights: Num = stretch_weights.iter().copied().sum();

    // If the minimum size is equal or greater to the total size,
    // there is no stretching to be done and every element is sized to their minimum size.
    //
    // This also happens if no elements have positive weight.
    if combined_minimum_size >= container_size
        || combined_weights <= Num::zero()
    {
        return ArrayVec::from(*minima);
    }

    // Precompute normalized weights (the fraction of the remaining
    // space that every element should receive.)
    // For example, if there are two elements with weights [2, 3],
    // the normalized weights will be [2/5, 3/5] = [0.4, 0.6] = [40%, 60%].
    //
    // Notice how the weights now add up to 100%.
    let normalized_stretch_weights: Vec<Num> = stretch_weights
        .iter()
        .map(|&w| w / combined_weights)
        .collect();

    // This is the crux of the layouting function.
    //
    // (∃ equilibrium) container_size - ∑ max(minima[idx], container_size * normalized_weights[idx] * equilibrium) = 0
    //
    // Let's understand this equation. You have a container_size.
    // And the SUM of the sizes of the arranged elements must equal this container size.
    //
    // container_size = ∑ <element_size[idx]>;
    // container_size - ∑ <element_size[idx]> = 0;
    //
    // Now what is the size of an element. Well, we know it's at least its minimum size.
    //
    // container_size - ∑ max(minima[idx], <element_size[idx]>)
    //
    // For the other part imagine this you have three elements with minima [0, 0, 0] and normalized weights [1/3, 2/3, 0].
    //
    // Arranging these elements should result in each weighted element getting a share of the total based on their weights.
    // Something like `container_size * normalized_weights[idx]`.
    //
    // However, we if we change the minima to [0, 0, container_size/2].
    // The stretchy elements have less real estate to share amongst themselves...
    // but the ratios of their sizes are still the same!
    // So you can imagine calculating their sizes based on the total container size...
    // and then just shrinking them uniformly to fit the container.
    // There is some value of the `equilibrium` which satisfies the equation.
    //
    // ![Desmos link.](https://www.desmos.com/calculator/vmnnfmdxhd)
    // ---
    //
    // Another way to imagine this is like this:
    // imagine the container is 0-sized.
    // Each element is sized to its minimum and overflows the container.
    //
    // As you increase the size of the container, you'll eventually hit
    // a point where the container size is equal to the combined
    // minima of the elements, the first legal state.
    //
    // As you increase the container further, elements will suddenly SNAP and start growing,
    // as now there's space left in the container...
    // The element's growth rate is proportional to the container size...
    // `max(minimum, T * ...)`
    // And respects the ratio relationships between the other stretchy elements... so it's proportional to its normalized weight.
    // `max(minimum, T * normalized_weight * ...)`
    // All that's left is some value `equilibrium` that happens to make the elements fit the container.
    let flex_equation = |x| {
        container_size
            - normalized_stretch_weights
                .iter()
                .zip(minima.iter())
                .map(|(weight, minimum)| {
                    minimum.max(container_size * *weight * x)
                })
                .sum::<Num>()
    };

    // The equation is not a [closed form expression](https://en.wikipedia.org/wiki/Closed-form_expression)
    // as it includes `max`, which itself isn't closed form, thus we have to solve it iteratively.
    //
    // Here we do binary search.
    // There are other ways of solving this (like splitting the function into many individually analytic parts)
    // but binary search is Good Enough.
    let mut lower_bound = Num::zero();
    let mut upper_bound = container_size;

    let mut equilibrium = Num::zero();
    // Find the value for equilibrium in up to 64 iterations.
    // Okay maybe later I'll replace this with something else.
    for _ in 0..64 {
        equilibrium = (lower_bound + upper_bound) / Num::from(2).unwrap();
        let error = flex_equation(equilibrium);

        if error.abs() < tolerance {
            break;
        }

        if error > Num::zero() {
            lower_bound = equilibrium;
        } else {
            upper_bound = equilibrium;
        }
    }

    // And each element is sized according to the previous equation.
    let element_sizes: ArrayVec<Num, SIZE> = normalized_stretch_weights
        .iter()
        .zip(minima.iter())
        .map(|(weight, minimum)| {
            minimum.max(container_size * *weight * equilibrium)
        })
        .collect();

    // The previous result has values as floating point numbers, which isn't ideal
    // when rendering to a pixel canvas.
    let mut pixel_element_sizes: ArrayVec<Num, SIZE> = ArrayVec::new();
    // We start sweeping from the start of the container to the end.
    let mut offset_from_start = Num::zero();
    let mut pixel_offset_from_start = Num::zero();

    for size in element_sizes {
        // Move the float offset along.
        offset_from_start = offset_from_start + size;
        // Calculate the pixel perfect position based on the float offset.
        let next_pixel_offset = offset_from_start.round();
        // What makes this pixel perfect is that the pixel start edge of the current element
        // is the pixel end edge of the previous element.
        pixel_element_sizes.push(next_pixel_offset - pixel_offset_from_start);
        pixel_offset_from_start = next_pixel_offset;
    }

    // We _have_ been rounding float falls, it's entirely possible that the ignored fractional part of the floats
    // has resulted in one single pixel of error at the end.
    let remaining_error = container_size.round() - pixel_offset_from_start;
    if remaining_error != Num::zero()
        && let Some(last) = pixel_element_sizes.last_mut()
    {
        // We just slightly stretch the last element to cover the empty space.
        // It's just one pixel, so it's fine.
        *last = *last + remaining_error;
    }

    pixel_element_sizes
}

pub fn arrange_stretchy_rects_with_minimum_sizes_dirty_alloc<
    Num: Float + core::iter::Sum,
>(
    t: Num,
    w: &[Num],
    m: &[Num],
    tol: Num,
) -> Vec<Num> {
    let total_m: Num = m.iter().copied().sum();
    let total_w: Num = w.iter().copied().sum();

    if total_m >= t || total_w <= Num::zero() {
        return m.to_vec();
    }

    let inv_total_w = Num::one() / total_w;
    let t_inv_w = t * inv_total_w;

    let mut equ_0 = Num::zero();
    let mut equ_1 = t * Num::from(2).unwrap();
    let mut equ = t;

    for _ in 0..32 {
        equ = (equ_0 + equ_1) / Num::from(2).unwrap();

        let sum: Num = w
            .iter()
            .zip(m.iter())
            .map(|(&weight, &min)| {
                let scaled = weight * t_inv_w * equ;
                if scaled > min { scaled } else { min }
            })
            .sum();

        if (t - sum).abs() < tol {
            break;
        }

        if t > sum {
            equ_0 = equ;
        } else {
            equ_1 = equ;
        }
    }

    let mut s_off_px = Num::zero();
    let mut res: Vec<Num> = w
        .iter()
        .zip(m.iter())
        .scan(Num::zero(), |s_off, (&weight, &min)| {
            let float_size = min.max(weight * t_inv_w * equ);
            *s_off = *s_off + float_size;
            let next_off_px = s_off.round();
            let pixel_size = next_off_px - s_off_px;
            s_off_px = next_off_px;
            Some(pixel_size)
        })
        .collect();

    let diff = t.round() - s_off_px;
    if diff != Num::zero()
        && let Some(last) = res.last_mut()
    {
        *last = *last + diff;
    }

    res
}

pub fn arrange_stretchy_rects_with_minimum_sizes_dirty_alloc_alt<
    Num: Float + core::iter::Sum,
>(
    t: Num,
    w: &[Num],
    m: &[Num],
) -> Vec<Num> {
    let x = (t - m.iter().copied().sum()) / w.iter().copied().sum();
    m.iter()
        .zip(w.iter())
        .map(|(m, w)| *m + *w * Num::max(x, Num::zero()))
        .collect()
}
