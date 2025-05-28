use {
    super::{graphic::Graphic, RenderGraphic, RenderGraphicDescriptor},
    crate::{
        app::primitives::PrimitiveDescriptor,
        prelude::items::{Drag, Hover, Tap},
        state::{
            process::{FutureProcessor, SignalProcessor},
            Effect,
        },
        winitwgpu::pipeline::text::Text,
    },
    std::future::Future,
};
use {crate::winitwgpu::render_target::RenderInternal, futures_signals::signal::Signal, vek::Rect};

//MARK: Graphics

impl RenderGraphic for Graphic {
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        quad_buffer[0] = *self;
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

impl RenderGraphicDescriptor for Graphic {
    const QUAD_COUNT: usize = 1;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        // Beautifully calculating the bounds of this Primitive
        // as a Rect!
        let matrix = self.transform.as_col_slice();
        Some(Rect::new(matrix[12], matrix[13], matrix[0], matrix[4]))
    }
}

//MARK: Text

impl RenderGraphicDescriptor for Text {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.0)
    }
}

impl RenderGraphic for Text {
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {}

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: ()

impl RenderGraphicDescriptor for () {
    const QUAD_COUNT: usize = 0;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl RenderGraphic for () {
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
        /* No quads to write */
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: (A, B)

impl<A, B> RenderGraphicDescriptor for (A, B)
where
    A: RenderGraphicDescriptor,
    B: RenderGraphicDescriptor,
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

impl<A, B> RenderGraphic for (A, B)
where
    A: RenderGraphicDescriptor,
    B: RenderGraphicDescriptor,
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

// MARK: [A; N]

impl<A, const N: usize> RenderGraphicDescriptor for [A; N]
where
    A: RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = N * A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        let mut iterator = self.iter();
        let first = iterator.next()?.get_render_rect();
        iterator.fold(first, |acc, item| Some(acc?.union(item.get_render_rect()?)))
    }
}

impl<A, const N: usize> RenderGraphic for [A; N]
where
    A: RenderGraphicDescriptor,
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

//MARK: Box<A>

impl<A> RenderGraphicDescriptor for Box<A>
where
    A: RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

impl<A> RenderGraphic for Box<A>
where
    A: RenderGraphicDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        self.as_ref().write_quads(quad_buffer)
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: Option<A>

impl<A> RenderGraphicDescriptor for Option<A>
where
    A: RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref()
            .and_then(RenderGraphicDescriptor::get_render_rect)
    }
}

impl<A> RenderGraphic for Option<A>
where
    A: RenderGraphicDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Some(inner) => inner.write_quads(quad_buffer),
            None => {
                for slot in quad_buffer {
                    *slot = Graphic::default()
                }
            }
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: Result<A>

impl<T, E> RenderGraphicDescriptor for Result<T, E>
where
    T: RenderGraphicDescriptor,
    E: RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = max(T::QUAD_COUNT, E::QUAD_COUNT);

    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match self {
            Ok(v) => v.get_render_rect(),
            Err(e) => e.get_render_rect(),
        }
    }
}

impl<T, E> RenderGraphic for Result<T, E>
where
    T: RenderGraphicDescriptor,
    E: RenderGraphicDescriptor,
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

//MARK: SignalProcessor

impl<S, T> RenderGraphicDescriptor for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: RenderInternal + RenderGraphicDescriptor + Send,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        match &self.signal.held_item {
            Some(item) => item.get_render_rect(),
            None => panic!("Reactor was asked for its render rect before being polled!"),
        }
    }
}
impl<S, T> RenderGraphic for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: RenderInternal + RenderGraphicDescriptor + Send,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match &self.signal.held_item {
            Some(item) => item.write_quads(quad_buffer),
            None => panic!("Reactor was drawn (graphics) without being polled first!"),
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: FutureProcessor

impl<F, T> RenderGraphicDescriptor for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: RenderInternal + RenderGraphicDescriptor,
{
    const QUAD_COUNT: usize = T::QUAD_COUNT;

    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        match &self.signal.held_item {
            Some(item) => item.get_render_rect(),
            None => panic!("Reactor was asked for its render rect before being polled!"),
        }
    }
}
impl<F, T> RenderGraphic for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: RenderInternal + RenderGraphicDescriptor + PrimitiveDescriptor,
{
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        if let Some(item) = &self.signal.held_item {
            item.write_quads(quad_buffer)
        }
    }

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}

//MARK: Primitives

macro_rules! impl_render_graphic {
    ($name:ident) => {
        impl RenderGraphicDescriptor for $name {
            const QUAD_COUNT: usize = 0;

            fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
                None // Some(self.area))
            }
        }

        impl RenderGraphic for $name {
            fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
                /* Maybe push something here in Debug mode? */
            }

            fn get_quad_count(&self) -> usize {
                Self::QUAD_COUNT
            }
        }
    };
}

impl_render_graphic!(Hover);
impl_render_graphic!(Drag);

impl<A> RenderGraphicDescriptor for Tap<A>
where
    A: Effect,
{
    const QUAD_COUNT: usize = 0;
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}
impl<A> RenderGraphic for Tap<A>
where
    A: Effect,
{
    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {}

    fn get_quad_count(&self) -> usize {
        Self::QUAD_COUNT
    }
}
