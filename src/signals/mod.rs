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

pub struct ReactorProcessor {
    render_module: Arc<Mutex<Box<dyn RenderModule>>>,
}

impl ReactorProcessor {
    pub fn new(reactors: Arc<Mutex<Box<dyn RenderModule>>>) -> Self {
        Self {
            render_module: reactors,
        }
    }
}

impl Signal for ReactorProcessor {
    type Item = ();

    fn poll_change(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Self::Item>> {
        let mut any_pending = false;

        let mut render_module = self
            .render_module
            .lock()
            .expect("Could not lock Render Module to process reactors.");

        for reactor in render_module.reactors().iter_mut() {
            match reactor.poll_change_unpin(cx) {
                Poll::Ready(opt) => match opt {
                    Some(mut value) => {
                        value.splat_allocation(
                            reactor.allocation_offset,
                            render_module.as_mut(),
                            false,
                        );
                        return Poll::Ready(Some(()));
                    }
                    None => (),
                },
                Poll::Pending => {
                    any_pending = true;
                }
            }
        }

        return if any_pending {
            Poll::Pending
        } else {
            Poll::Ready(None)
        };
    }
}
