#![allow(non_snake_case)]
use futures_signals::signal::{Mutable, SignalExt};
use vek::Rgb;

use crate::prelude::process::SignalReactItem;
use crate::wgpu::pipeline::graphics::graphic::Graphic;
use crate::wgpu::render_target::RenderDescriptor;
use crate::{
    items_internal as items,
    prelude::{
        Effect, LayoutItem, ParentHints, Resizable, ResizableItem,
        items::{Hover, Tap},
    },
};

/// A simple button which you can click!
pub fn Button<L, Fx>(mut label: L, effect: Fx) -> impl LayoutItem<Content = impl RenderDescriptor>
where
    L: LayoutItem + Clone + Send + Sync + 'static,
    L::Content: RenderDescriptor,
    Fx: Effect + Clone + Send + Sync,
{
    #[allow(deprecated)]
    let minimum_size = label.get_minimum_size();
    let mouse_position = Mutable::new(None);
    let is_hovered_state = Mutable::new(false);

    let render_ui = move |parent_hints: ParentHints| {
        let hover = Hover::new(parent_hints.rect, is_hovered_state.clone());
        let tap = Tap::new(parent_hints.rect, mouse_position.clone(), effect.clone());

        items!(
            tap,
            hover,
            SignalReactItem(is_hovered_state.signal().map(move |is_hovered| {
                if is_hovered {
                    items!(Graphic::from(parent_hints.rect).with_color(Rgb::new(0.6, 0.6, 0.6)),)
                } else {
                    items!(Graphic::from(parent_hints.rect).with_color(Rgb::new(0.2, 0.2, 0.2)),)
                }
            })),
            label.lay(ParentHints { ..parent_hints }),
        )
    };

    ResizableItem::new(render_ui).with_minimum_size(minimum_size)
}
