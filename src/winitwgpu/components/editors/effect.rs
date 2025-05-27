#![allow(non_snake_case)]
use futures_signals::signal::Mutable;
use vek::Rgb;

use crate::{
    items_internal as items,
    prelude::{Effect, LayoutItem, ParentHints, Resizable, ResizableItem, Tap},
    winitwgpu::pipeline::graphics::graphic::Graphic,
};

/// A simple button which you can click!
pub fn Button<L, Fx>(mut label: L, effect: Fx) -> impl LayoutItem
where
    L: LayoutItem + 'static,
    Fx: Effect + Clone + Send + Sync,
{
    let minimum_size = label.get_minimum_size();
    let mouse_position = Mutable::new(None);

    let render = move |parent_hints: ParentHints| {
        items!(
            Graphic::from(parent_hints.rect).with_color(Rgb::new(0.3, 0.2, 0.5)),
            label.lay(parent_hints),
            Tap::new(parent_hints.rect, mouse_position.clone(), effect.clone()),
        )
    };

    ResizableItem::new(render).with_minimum_size(minimum_size)
}
