use crate::{
    render_module::RenderModule,
    standard::{
        primitive::Primitive,
        render::{AllocationInfo, AllocationOffset, UIFragment, UIFragmentLive},
    },
};
use futures_signals::signal::{Signal, SignalExt};
use pin_project::pin_project;
use std::{ops::Range, pin::Pin};

/// Object that monitors states and reacts by issuing memory update commands.
#[pin_project(project = ReactorProj)]
pub struct Reactor {
    pub allocation_offset: AllocationOffset,
    #[pin]
    pub signal: Pin<Box<dyn Signal<Item = Box<dyn UIFragmentLive>> + Send>>,
}

impl Signal for Reactor {
    type Item = Box<dyn UIFragmentLive>;

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

#[derive(Clone)]
pub struct React<T, S>(pub Option<S>)
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
        React(Some(self))
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
}

impl<T, S> UIFragmentLive for React<T, S>
where
    S: Signal<Item = T> + Send + 'static,
    T: UIFragment + 'static,
{
    fn splat_allocation(
        &mut self,
        allocation_offset: AllocationOffset,
        render_module: &mut dyn RenderModule,
        initial: bool,
    ) {
        // TODO: This is awful... I wonder if I can rewrite this to remove the indirection.
        let dyn_signal: Pin<Box<dyn Signal<Item = Box<dyn UIFragmentLive>> + Send>> = Box::pin(
            self.0
                .take()
                .unwrap()
                .map(|fragment| Box::new(fragment) as Box<dyn UIFragmentLive>),
        );
        let reactor = Reactor {
            allocation_offset,
            signal: dyn_signal,
        };
        let base_info = T::get_allocation_info();

        if render_module.primitive_buffer().len() == allocation_offset.primitive_buffer_offset {
            render_module.reactors().push(reactor);
            // Fill the primitive buffer with some dummy primitives.
            for i in 0..base_info.primitive_count {
                render_module.primitive_buffer().push(Primitive::default())
            }
        } else {
            render_module.reactors()[allocation_offset.reactor_buffer_offset] = reactor;
        }
    }
}
