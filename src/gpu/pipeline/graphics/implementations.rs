use super::{GraphicItem, GraphicItemDescriptor};
use crate::prelude::Graphic;
use vek::Rect;

// --- Empty Graphics ---

impl GraphicItemDescriptor for () {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl GraphicItem for () {
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        /* No quads to write */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

// --- (A, B) ---

impl<A, B> GraphicItemDescriptor for (A, B)
where
    A: GraphicItemDescriptor,
    B: GraphicItemDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT + B::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match (self.0.get_render_rect(), self.1.get_render_rect()) {
            (None, None) => None,
            (None, Some(b)) => Some(b),
            (Some(a), None) => Some(a),
            (Some(a), Some(b)) => Some(a.union(b)),
        }
    }
}

impl<A, B> GraphicItem for (A, B)
where
    A: GraphicItemDescriptor,
    B: GraphicItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        let (slice_a, slice_b) = quad_buffer.split_at_mut(A::QUAD_COUNT);
        self.0.write_quads(slice_a);
        self.1.write_quads(slice_b);
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

// --- [A; N] ---

impl<A, const N: usize> GraphicItemDescriptor for [A; N]
where
    A: GraphicItemDescriptor,
{
    const QUAD_COUNT: usize = N * A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        let mut iterator = self.iter();
        let first = iterator.next()?.get_render_rect();
        iterator.fold(first, |acc, item| Some(acc?.union(item.get_render_rect()?)))
    }
}

impl<A, const N: usize> GraphicItem for [A; N]
where
    A: GraphicItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        for (idx, item) in self.iter().enumerate() {
            item.write_quads(&mut quad_buffer[(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)])
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

// --- Box<A> ---

impl<A> GraphicItemDescriptor for Box<A>
where
    A: GraphicItemDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

impl<A> GraphicItem for Box<A>
where
    A: GraphicItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        self.as_ref().write_quads(quad_buffer)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

// --- Option<A> ---

impl<A> GraphicItemDescriptor for Option<A>
where
    A: GraphicItemDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref()
            .and_then(GraphicItemDescriptor::get_render_rect)
    }
}

impl<A> GraphicItem for Option<A>
where
    A: GraphicItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Some(inner) => inner.write_quads(quad_buffer),
            None => {
                for idx in 0..Self::QUAD_COUNT {
                    quad_buffer[idx] = Graphic::default()
                }
            }
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

// --- Result<A> ---

impl<T, E> GraphicItemDescriptor for Result<T, E>
where
    T: GraphicItemDescriptor,
    E: GraphicItemDescriptor,
{
    const QUAD_COUNT: usize = max(T::QUAD_COUNT, E::QUAD_COUNT);

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match self {
            Ok(v) => v.get_render_rect(),
            Err(e) => e.get_render_rect(),
        }
    }
}

impl<T, E> GraphicItem for Result<T, E>
where
    T: GraphicItemDescriptor,
    E: GraphicItemDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Ok(v) => v.write_quads(quad_buffer),
            Err(e) => e.write_quads(quad_buffer),
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

pub const fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
