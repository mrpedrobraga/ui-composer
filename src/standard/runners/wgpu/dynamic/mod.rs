use crate::app::composition::reify::Reify;
use crate::standard::runners::wgpu::pipeline::graphics::{
    graphic::Graphic, RenderGraphic, RenderGraphicDescriptor,
};
use crate::standard::runners::wgpu::pipeline::RendererBuffers;
use crate::state::process::Pollable;
use futures_signals::signal_vec::SignalVec;
use std::pin::Pin;
use std::task::{Context, Poll};
use {crate::app::input::Event, vek::Rect};
use crate::app::composition::algebra::Bubble;
// TODO: Move this out of `wgpu`...
// Vec Items should still be barred behind `alloc`,
// but it shouldn't be inherently bound to the gpu!

pub struct VecItemsDescriptor<Sig: SignalVec> {
    rect: Rect<f32, f32>,
    items: Sig,
}

pub struct VecItem<Sig: SignalVec> {
    rect: Rect<f32, f32>,
    // Use a construct called 'HoldSignalVec' that keeps holding
    // the `<Sig::Item as PrimitiveDescriptor<_>>::Primitive` values.
    items: Sig,
    // Stuff you need to render the sub items...
    render_buffers: Option<RendererBuffers>,
}

impl<Sig> VecItem<Sig>
where
    Sig: SignalVec,
{
    pub fn new(rect: Rect<f32, f32>, items: Sig) -> Self {
        Self {
            rect,
            items,
            render_buffers: None,
        }
    }
}

impl<Res, Sig> Reify<Res> for VecItemsDescriptor<Sig>
where
    Sig: SignalVec + Send,
{
    type Output = VecItem<Sig>;

    fn reify(self, _resources: &mut Res) -> Self::Output {
        todo!()
    }
}

impl<Res, Sig> RenderGraphicDescriptor<Res> for VecItemsDescriptor<Sig>
where
    Sig: SignalVec + Send,
{
    fn get_render_rect(&self) -> Option<Rect<f32, f32>> {
        Some(self.rect)
    }
}

// TODO: This is supposed to write one quad to the GPU, binding its texture
// to the internal texture of this vec...
impl<Sig> RenderGraphic for VecItem<Sig>
where
    Sig: SignalVec,
{
    const QUAD_COUNT: usize = 0;

    fn write_quads(&self, _quad_buffer: &mut [Graphic]) {
        todo!("Write a single quad, binding the image")
    }
}

impl<Sig, Res> Pollable<Res> for VecItem<Sig>
where
    Sig: SignalVec + Send,
{
    fn poll(self: Pin<&mut Self>, _cx: &mut Context, _resources: &mut Res) -> Poll<Option<()>> {
        todo!("Poll the items inside, like any other collection does...")
    }
}

impl<Sig> Bubble<Event, bool> for VecItem<Sig>
where
    Sig: SignalVec + Send,
    Sig::Item: Bubble<Event, bool>
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        todo!("Broadcast event to children...")
    }
}
