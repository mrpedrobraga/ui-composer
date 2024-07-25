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
        let mut base_info = T::get_allocation_info();
        base_info.reactor_count += 1;
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
        // TODO: I wonder if I can remove this indirection?
        let dyn_signal: Pin<Box<dyn Signal<Item = Box<dyn UIFragmentLive>> + Send>> = Box::pin(
            self.0
                .take()
                .unwrap()
                .map(|fragment| Box::new(fragment) as Box<dyn UIFragmentLive>),
        );
        let mut inner_offset = allocation_offset;
        inner_offset.reactor_buffer_offset += 1;

        let reactor = Reactor {
            allocation_offset: inner_offset,
            signal: dyn_signal,
        };

        dbg!(initial, allocation_offset);
        if initial {
            render_module.reactors().push(Some(reactor));
            T::splat_allocation_empty(inner_offset, render_module, initial)
        } else {
            render_module.reactors()[allocation_offset.reactor_buffer_offset] = Some(reactor);
        }
    }
}
