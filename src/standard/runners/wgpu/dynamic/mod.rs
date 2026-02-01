use crate::app::composition::algebra::Bubble;
use crate::app::composition::reify::Reify;
use crate::standard::runners::wgpu::pipeline::graphics::{
    graphic::Graphic, RenderGraphic, RenderGraphicDescriptor,
};
use crate::standard::runners::wgpu::pipeline::RendererBuffers;
use crate::state::process::Pollable;
use futures_signals::signal_vec::{SignalVec, VecDiff};
use std::pin::Pin;
use std::task::{Context, Poll};
use {crate::app::input::Event, vek::Rect};
// TODO: Move this out of `wgpu`...
// Vec Items should still be barred behind `alloc`,
// but it shouldn't be inherently bound to the gpu!

pub struct VecItemDescriptor<Sig: SignalVec> {
    rect: Rect<f32, f32>,
    items: Sig,
}

pub struct VecItem<Sig: SignalVec> {
    rect: Rect<f32, f32>,
    // TODO: Use a construct called 'HoldSignalVec' that keeps holding the `<Sig::Item as PrimitiveDescriptor<_>>::Primitive` values.
    items: Sig,
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

impl<Res, Sig> Reify<Res> for VecItemDescriptor<Sig>
where
    Sig: SignalVec + Send,
{
    type Output = VecItem<Sig>;

    fn reify(self, _resources: &mut Res) -> Self::Output {
        todo!()
    }
}

impl<Res, Sig> RenderGraphicDescriptor<Res> for VecItemDescriptor<Sig>
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
    fn poll(self: Pin<&mut Self>, cx: &mut Context, _resources: &mut Res) -> Poll<Option<()>> {
        let signal_poll = self.items.poll_vec_change(cx);

        match signal_poll {
            Poll::Ready(Some(diff)) => {
                self.apply_diff(diff);
            }
            Poll::Ready(None) => return Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<Sig> VecItem<Sig>
where
    Sig: SignalVec,
{
    fn apply_diff(diff: VecDiff<Sig::Item>) {
        // TODO: Do something with the diffs like updating or recreating the buffers with new sizes...
        match diff {
            VecDiff::Replace { .. } => {}
            VecDiff::InsertAt { .. } => {}
            VecDiff::UpdateAt { .. } => {}
            VecDiff::RemoveAt { .. } => {}
            VecDiff::Move { .. } => {}
            VecDiff::Push { .. } => {}
            VecDiff::Pop { .. } => {}
            VecDiff::Clear { .. } => {}
        }
    }
}

impl<Sig> Bubble<Event, bool> for VecItem<Sig>
where
    Sig: SignalVec + Send,
    Sig::Item: Bubble<Event, bool>,
{
    fn bubble(&mut self, event: &mut Event) -> bool {
        todo!("Broadcast event to children...")
    }
}
