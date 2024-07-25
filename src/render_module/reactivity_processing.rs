use crate::{
    reaction::{self, Reactor},
    render_module::RenderModule,
    standard::render::UIFragmentLive,
};
use futures_signals::signal::{Signal, SignalExt as _};
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    task::Poll,
};

/// Processes a render module, looking for Reactivity events such as Reactors changing
/// or (futurely) futures resolving.
pub struct ReactorProcessor {
    dirty: bool,
    render_module: Arc<Mutex<Box<dyn RenderModule>>>,
}

pub enum ReactorProcessorEvent {
    DoNothing,
    Redraw,
}

impl ReactorProcessor {
    pub fn new(reactors: Arc<Mutex<Box<dyn RenderModule>>>) -> Self {
        Self {
            dirty: false,
            render_module: reactors,
        }
    }
}

impl Signal for ReactorProcessor {
    type Item = ReactorProcessorEvent;

    fn poll_change(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Self::Item>> {
        let mut any_pending = false;

        let mut render_module = self
            .render_module
            .lock()
            .expect("Could not lock Render Module to poll reactors.");

        for reactor in render_module.reactors().iter_mut() {
            match reactor.poll_change_unpin(cx) {
                Poll::Ready(opt) => match opt {
                    Some(mut value) => {
                        value.splat_allocation(
                            reactor.allocation_offset,
                            render_module.as_mut(),
                            false,
                        );
                        drop(render_module);
                        self.dirty = true;
                        return Poll::Ready(Some(ReactorProcessorEvent::DoNothing));
                    }
                    None => (),
                },
                Poll::Pending => {
                    any_pending = true;
                }
            }
        }

        drop(render_module);

        // TODO: I have no idea if this works, to be honest, but the idea is that
        // you only send a redraw signal when you processed *all* consecutive reactive events
        // available.
        if self.dirty {
            self.dirty = false;
            return Poll::Ready(Some(ReactorProcessorEvent::Redraw));
        }

        if any_pending {
            return Poll::Pending;
        }

        return Poll::Ready(None);
    }
}
