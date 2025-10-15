#![allow(non_snake_case)]
use futures_signals::signal::{Mutable, SignalExt};
use vek::Rgb;

use crate::prelude::process::SignalReactItem;
use crate::wgpu::pipeline::graphics::graphic::Graphic;
use crate::wgpu::render_target::Render;
use crate::{
    items_internal as items,
    prelude::{
        Effect, LayoutItem, ParentHints, Resizable, ResizableItem,
        items::{Hover, Tap},
    },
};
use crate::state::process::React;

/// A simple button which you can click!
pub fn Button<L, Fx>(mut label: L, effect: Fx) -> impl LayoutItem<Content = impl Render>
where
    L: LayoutItem + Clone + Send + Sync + 'static,
    L::Content: Render,
    Fx: Effect + Clone + Send + Sync,
{
    #[allow(deprecated)]
    let minimum_size = label.get_minimum_size();
    let mouse_position = Mutable::new(None);
    let is_hovered_state = Mutable::new(false);

    let color_hovered = Rgb::new(0.5, 0.7, 1.0);
    let color_normal = Rgb::new(0.3, 0.3, 0.7);

    let render_ui = move |parent_hints: ParentHints| {
        let hover = Hover::new(parent_hints.rect, is_hovered_state.clone());
        let tap = Tap::new(parent_hints.rect, mouse_position.clone(), effect.clone());

        items!(
            tap,
            hover,
            React(is_hovered_state.signal().map(move |is_hovered| {
                Graphic::from(parent_hints.rect).with_color(if is_hovered {color_hovered} else {color_normal})
            })),
            label.lay(ParentHints { ..parent_hints }),
        )
    };

    ResizableItem::new(render_ui).with_minimum_size(minimum_size)
}
