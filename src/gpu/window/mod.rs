use std::sync::Arc;

use futures_signals::signal::{Mutable, Signal, SignalExt};
use vek::{Extent2, Rect, Rgb};
use winit::{
    event_loop::{self, ActiveEventLoop},
    window::{Window, WindowId},
};

use super::{
    engine::{GPUResources, LiveNode, Node},
    render_target::WindowRenderTarget,
    view::{View, ViewNode},
};
use crate::ui::{
    graphics::Primitive,
    layout::LayoutItem,
    node::{LiveUINode, UINode},
    react::{React, UISignalExt},
};

/// A node that describes the existence of a new window in the UI tree.
pub struct WindowNode<T: UINode> {
    attributes: WindowAttributes,
    state: WindowState,
    content: T,
}

impl<T> Node for WindowNode<T>
where
    T: UINode + 'static,
{
    type LiveType = LiveWindowNode;

    /// Transforms a WindowNode descriptor into a live window node which can be really used!
    fn reify(self, event_loop: &ActiveEventLoop, gpu_resources: GPUResources) -> Self::LiveType {
        let window_default_size = Extent2::new(100, 100);

        let window = event_loop
            .create_window(
                winit::window::WindowAttributes::default()
                    .with_title(self.attributes.title.get_cloned()),
            )
            .expect("Couldn't reify window!");

        let window = std::sync::Arc::new(window);

        LiveWindowNode {
            content: Box::new(self.content),
            render_target: WindowRenderTarget::new(
                gpu_resources,
                window.clone(),
                window_default_size,
            ),
            window,
        }
    }
}

/// A live window which contains a UI tree inside.
pub struct LiveWindowNode {
    content: Box<dyn LiveUINode>,
    render_target: WindowRenderTarget,
    window: Arc<Window>,
}

impl<'window> LiveNode for LiveWindowNode {
    fn handle_window_event(&mut self, window_id: WindowId, event: crate::ui::node::UIEvent) {
        self.content.handle_ui_event(event);
    }
}

#[derive(Debug, Default, Clone)]
pub struct WindowAttributes {
    pub title: Mutable<String>,
}

pub struct WindowState {
    pub window_size: Mutable<Extent2<f32>>,
}

impl WindowState {
    fn new(window_size: Extent2<f32>) -> Self {
        Self {
            window_size: Mutable::new(window_size),
        }
    }
}

/// Creates a new window as the render target for the nodes inside.
#[allow(non_snake_case)]
pub fn Window<T>(attributes: WindowAttributes, item: T) -> WindowNode<impl UINode>
where
    T: LayoutItem + 'static,
{
    let window_state = WindowState {
        window_size: Mutable::new(item.get_natural_size()),
    };

    let window_size_signal = window_state.window_size.signal();

    let minimum_size = item.get_natural_size();
    let item = window_state
        .window_size
        .signal()
        .map(move |window_size| item.bake(Rect::new(0.0, 0.0, window_size.w, window_size.h)))
        .into_ui();

    WindowNode {
        attributes,
        state: window_state,
        content: View(minimum_size, item).bake_react(window_size_signal),
    }
}
