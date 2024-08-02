#![allow(non_snake_case)]
use std::ops::{Add, Sub};

use futures::StreamExt as _;
use ui_composer::{
    interaction::tap::TapInteraction,
    prelude::*,
    standard::layout::item::{LayoutItem, ResizableItem},
};
use vek::{Extent2, Rect, Rgb};
use winit::{dpi::PhysicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    let state = Editable::new(false);

    AppBuilder::new(App(state))
        .with_window_attributes(
            WindowAttributes::default()
                .with_title("Layout test")
                .with_name("Layout Test", "Layout Test")
                .with_inner_size(PhysicalSize::new(256, 256)),
        )
        .run();
}

fn App(state: Editable<bool>) -> impl LayoutItem {
    Center(Checkbox(state))
}

fn Center<I: LayoutItem + 'static>(item: I) -> impl LayoutItem {
    ResizableItem::new(Extent2::default(), move |rect| {
        let extents = item.get_natural_size();
        let position = rect.center() - extents / 2.0;
        item.bake(Rect::new(position.x, position.y, extents.w, extents.h))
    })
}

fn Checkbox(state: Editable<bool>) -> impl LayoutItem {
    return ResizableItem::new(Extent2::new(32.0, 32.0), move |rect| {
        let tap_interaction = TapInteraction::new(rect);
        let tap_clone = tap_interaction.clone();
        let state_clone = state.clone();
        std::thread::spawn(move || {
            pollster::block_on(tap_clone.stream().for_each(move |_| {
                state_clone.set(!state_clone.get());
                async {}
            }))
        });

        (
            tap_interaction,
            state
                .signal()
                .map(move |isTrue| {
                    (
                        Primitive::rect(rect, Rgb::gray(0.7)),
                        if isTrue {
                            Some(Primitive::rect(rect.expand_radius(-4.0), Rgb::gray(0.2)))
                        } else {
                            None
                        },
                    )
                })
                .into_fragment(),
        )
    });
}

trait RectExt<P, E> {
    fn expand_radius(self, radius: P) -> Self
    where
        P: Copy,
        P: Sub<P, Output = P>,
        E: Add<P, Output = P>,
        P: Add<P, Output = P>,
        E: From<P>;
}

impl<P, E> RectExt<P, E> for Rect<P, E> {
    fn expand_radius(self, radius: P) -> Self
    where
        P: Copy,
        P: Sub<P, Output = P>,
        E: Add<P, Output = P>,
        P: Add<P, Output = P>,
        E: From<P>,
    {
        Rect::new(
            self.x - radius,
            self.y - radius,
            (self.w + radius + radius).into(),
            (self.h + radius + radius).into(),
        )
    }
}
