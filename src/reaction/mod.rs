use crate::standard::{
    primitive::Primitive,
    render::{AllocationInfo, AllocationOffset, UIFragment},
};
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::ops::Range;

/// Object that monitors states and reacts by issuing memory update commands.
#[pin_project(project = ReactorProj)]
pub struct Reactor<T, S>
where
    S: Signal<Item = T>,
    T: UIFragment,
{
    allocation_offset: AllocationOffset,
    #[pin]
    signal: S,
}

pub trait ReactorExt<T: UIFragment>: Signal<Item = T> + Send {
    fn get_allocation_offset(&self) -> AllocationOffset;
}

pub trait UnknownReactor {
    fn get_allocation_offset(&self) -> AllocationOffset;
}

impl<T, S> Signal for Reactor<T, S>
where
    S: Signal<Item = T>,
    T: UIFragment,
{
    type Item = T;

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        let ReactorProj {
            allocation_offset,
            signal,
        } = self.project();

        signal.poll_change(cx)
    }
}

impl<T, S> ReactorExt<T> for Reactor<T, S>
where
    S: Signal<Item = T> + Send,
    T: UIFragment,
{
    fn get_allocation_offset(&self) -> AllocationOffset {
        self.allocation_offset
    }
}

impl<T, S> UnknownReactor for Reactor<T, S>
where
    S: Signal<Item = T> + Send,
    T: UIFragment,
{
    fn get_allocation_offset(&self) -> AllocationOffset {
        self.allocation_offset
    }
}

pub struct React<T, S>(pub S)
where
    S: Signal<Item = T>,
    T: UIFragment;

pub trait SignalReactExt {
    fn into_fragment(self) -> impl UIFragment;
}

impl<T, S> SignalReactExt for S
where
    S: Signal<Item = T> + Send + 'static,
    T: UIFragment + 'static,
{
    fn into_fragment(self) -> impl UIFragment {
        React(self)
    }
}

impl<T, S> UIFragment for React<T, S>
where
    S: Signal<Item = T> + Send + 'static,
    T: UIFragment + 'static,
{
    fn get_allocation_info() -> crate::standard::render::AllocationInfo {
        let base_info = T::get_allocation_info();
        // TODO: Need to allocate a reactor, too, duh!
        base_info
    }

    fn splat_allocation(
        self,
        allocation_offset: AllocationOffset,
        render_module: &mut crate::standard::render::tuple_render_module::TupleRenderModule,
        temp_reactors: &mut Vec<Box<dyn UnknownReactor>>,
    ) {
        let reactor = Reactor {
            allocation_offset,
            signal: self.0,
        };
        let base_info = T::get_allocation_info();

        if render_module.primitive_buffer_cpu.len() == allocation_offset.primitive_buffer_offset {
            temp_reactors.push(Box::new(reactor));
            // Fill the primitive buffer with some dummy primitives.
            for i in 0..base_info.primitive_count {
                render_module
                    .primitive_buffer_cpu
                    .push(Primitive::default())
            }
        } else {
            temp_reactors[allocation_offset.reactor_buffer_offset] = Box::new(reactor);
        }
    }
}
