use crate::app::primitives::PrimitiveDescriptor;
use crate::prelude::items::Typing;
use crate::prelude::process::React;
use crate::state::process::Await;
use crate::wgpu::pipeline::UIReifyResources;
use crate::wgpu::pipeline::text::{Text, TextItemRe};
use {
    super::{RenderGraphic, RenderGraphicDescriptor, graphic::Graphic},
    crate::{
        prelude::items::{Drag, Hover, Tap},
        state::{
            Effect,
            process::{FutureProcessor, SignalProcessor},
        },
    },
    std::future::Future,
};
use {futures_signals::signal::Signal, vek::Rect};
//MARK: Graphics

impl<Res> PrimitiveDescriptor<Res> for Graphic {
    type Primitive = Graphic;

    fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
        self
    }
}

impl<Res> RenderGraphicDescriptor<Res> for Graphic {
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        // TODO: Calculate the bounds for a general transform matrix.
        let matrix = self.transform.as_col_slice();
        Some(Rect::new(matrix[12], matrix[13], matrix[0], matrix[4]))
    }
}

impl RenderGraphic for Graphic {
    const QUAD_COUNT: usize = 1;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        quad_buffer[0] = *self;
    }
}

//MARK: Text

impl<S: AsRef<str>> RenderGraphicDescriptor<UIReifyResources> for Text<S> {
    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        Some(self.0)
    }
}

impl RenderGraphic for TextItemRe {
    const QUAD_COUNT: usize = 0;

    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {}
}

//MARK: ()

impl<Res> RenderGraphicDescriptor<Res> for () {
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}

impl RenderGraphic for () {
    const QUAD_COUNT: usize = 0;

    #[expect(unused)]
    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        /* No quads to write */
    }
}

//MARK: (A, B)

impl<A, B, Res> RenderGraphicDescriptor<Res> for (A, B)
where
    A: RenderGraphicDescriptor<Res>,
    B: RenderGraphicDescriptor<Res>,
{
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
    A: RenderGraphic,
    B: RenderGraphic,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT + B::QUAD_COUNT;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        let (slice_a, slice_b) = quad_buffer.split_at_mut(A::QUAD_COUNT);
        self.0.write_quads(slice_a);
        self.1.write_quads(slice_b);
    }
}

// MARK: [A; N]

impl<A, const N: usize, Res> RenderGraphicDescriptor<Res> for [A; N]
where
    A: RenderGraphicDescriptor<Res>,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        let mut iterator = self.iter();
        let first = iterator.next()?.get_render_rect();
        iterator.fold(first, |acc, item| Some(acc?.union(item.get_render_rect()?)))
    }
}

impl<A, const N: usize> RenderGraphic for [A; N]
where
    A: RenderGraphic,
{
    const QUAD_COUNT: usize = N * A::QUAD_COUNT;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        for (idx, item) in self.iter().enumerate() {
            item.write_quads(&mut quad_buffer[(idx * A::QUAD_COUNT)..((idx + 1) * A::QUAD_COUNT)])
        }
    }
}

//MARK: Box<A>

#[cfg(feature = "std")]
impl<A, Res> RenderGraphicDescriptor<Res> for Box<A>
where
    A: RenderGraphicDescriptor<Res>,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref().get_render_rect()
    }
}

#[cfg(feature = "std")]
impl<A> RenderGraphic for Box<A>
where
    A: RenderGraphic,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        self.as_ref().write_quads(quad_buffer)
    }
}

//MARK: Option<A>

impl<A, Res> RenderGraphicDescriptor<Res> for Option<A>
where
    A: RenderGraphicDescriptor<Res>,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        self.as_ref()
            .and_then(RenderGraphicDescriptor::get_render_rect)
    }
}

impl<A> RenderGraphic for Option<A>
where
    A: RenderGraphic,
{
    const QUAD_COUNT: usize = A::QUAD_COUNT;

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
}

//MARK: Result<A>

impl<T, E, Res> RenderGraphicDescriptor<Res> for Result<T, E>
where
    T: RenderGraphicDescriptor<Res>,
    E: RenderGraphicDescriptor<Res>,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        match self {
            Ok(v) => v.get_render_rect(),
            Err(e) => e.get_render_rect(),
        }
    }
}

impl<T, E> RenderGraphic for Result<T, E>
where
    T: RenderGraphic,
    E: RenderGraphic,
{
    const QUAD_COUNT: usize = max(T::QUAD_COUNT, E::QUAD_COUNT);

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match self {
            Ok(v) => v.write_quads(quad_buffer),
            Err(e) => e.write_quads(quad_buffer),
        }
    }
}

pub const fn max(a: usize, b: usize) -> usize {
    if a > b { a } else { b }
}

//MARK: React
impl<S, Res> RenderGraphicDescriptor<Res> for React<S>
where
    S: Signal + Send,
    S::Item: RenderGraphicDescriptor<Res>,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None //TODO: Figure this out!
    }
}

impl<S, T, Res> RenderGraphic for SignalProcessor<S, Res>
where
    S: Signal<Item = T>,
    T: RenderGraphicDescriptor<Res>,
{
    const QUAD_COUNT: usize = <S::Item as PrimitiveDescriptor<Res>>::Primitive::QUAD_COUNT;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        match &self.held_item {
            Some(item) => item.write_quads(quad_buffer),
            None => panic!("Reactor was drawn (graphics) without being polled first!"),
        }
    }
}

//MARK: FutureProcessor
impl<Res, Fut> PrimitiveDescriptor<Res> for FutureProcessor<Fut, Res>
where
    Fut: Future,
    Fut::Output: PrimitiveDescriptor<Res>,
{
    type Primitive = <Fut::Output as PrimitiveDescriptor<Res>>::Primitive;

    fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
        // Of course, by default,
        todo!()
    }
}

impl<Fut, Res> RenderGraphicDescriptor<Res> for Await<Fut>
where
    Fut: Future + Send,
    Fut::Output: PrimitiveDescriptor<Res>,
    <Fut::Output as PrimitiveDescriptor<Res>>::Primitive: RenderGraphic,
{
    fn get_render_rect(&self) -> Option<vek::Rect<f32, f32>> {
        None
    }
}
impl<Fut, Res> RenderGraphic for FutureProcessor<Fut, Res>
where
    Fut: Future,
    Fut::Output: PrimitiveDescriptor<Res>,
    <Fut::Output as PrimitiveDescriptor<Res>>::Primitive: RenderGraphic,
{
    const QUAD_COUNT: usize = <Fut::Output as PrimitiveDescriptor<Res>>::Primitive::QUAD_COUNT;

    fn write_quads(&self, quad_buffer: &mut [Graphic]) {
        if let Some(item) = &self.held_item {
            item.write_quads(quad_buffer)
        }
    }
}

//MARK: Primitives

macro_rules! impl_render_graphic_nop {
    ($name:ident) => {
        impl<Res> PrimitiveDescriptor<Res> for $name {
            type Primitive = Self;
            fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
                self
            }
        }

        impl<Res> RenderGraphicDescriptor<Res> for $name {
            fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
                None // Some(self.area))
            }
        }

        impl RenderGraphic for $name {
            const QUAD_COUNT: usize = 0;

            fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
                /* Maybe push something here in Debug mode? */
            }
        }
    };
}

impl_render_graphic_nop!(Hover);
impl_render_graphic_nop!(Drag);
impl_render_graphic_nop!(Typing);

impl<A, Res> PrimitiveDescriptor<Res> for Tap<A>
where
    A: Effect,
{
    type Primitive = Self;

    fn reify(self, #[expect(unused)] resources: &mut Res) -> Self::Primitive {
        self
    }
}

impl<A, Res> RenderGraphicDescriptor<Res> for Tap<A>
where
    A: Effect,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        None
    }
}
impl<A> RenderGraphic for Tap<A>
where
    A: Effect,
{
    const QUAD_COUNT: usize = 0;

    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {}
}
