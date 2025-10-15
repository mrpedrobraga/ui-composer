#![allow(non_snake_case)]
use crate::backends::wgpu::pipeline::graphics::graphic::Graphic;
use crate::backends::wgpu::render_target::Render;
use crate::items_internal as items;
use crate::layout::hints::ParentHints;
use crate::prelude::items::Tap;
use crate::state::process::SignalReactItem;
use crate::{
    prelude::{ItemBox, LayoutItem, RectExt, Resizable},
    state::animation::spring::Spring,
};
use futures_signals::signal::{Mutable, SignalExt};
use vek::{Extent2, Lerp, Rect, Rgb, Vec4};

pub trait BoolEditGraphics {
    fn describe_render(
        rect: Rect<f32, f32>,
        anim_factor: f32,
        parent_hints: ParentHints,
    ) -> impl Render;
}

/// A barebones switch which allows the user to toggle a `Mutable<bool>`.
/// [SwitchGraphics] and a minimum size must be provided for anything to show up on screen!
pub fn BoolEditBase<G>(state: Mutable<bool>) -> impl Resizable<Content = impl Render>
where
    G: BoolEditGraphics,
{
    let anim_state = Mutable::new(0.0);

    let factory = move |parent_hints: ParentHints| {
        let rect = parent_hints.rect;
        let state_ = state.clone();
        let tap = Tap::new(rect, Mutable::new(None), move || state_.set(!state_.get()));

        let spring_animation = Spring::if_then_else(state.signal(), anim_state.clone(), 1.0, 0.0);

        let spring_square = anim_state
            .signal()
            .map(move |anim_factor| G::describe_render(rect, anim_factor, parent_hints));

        items!(
            tap,
            SignalReactItem(spring_animation),
            SignalReactItem(spring_square)
        )
    };

    ItemBox::new(factory)
}

#[derive(Clone, Copy)]
pub struct AnimatedSwitch;

/// A simple switch that edits a `bool` state.
///
/// The user can press it to toggle the underlying boolean.
pub fn Switch(state: Mutable<bool>) -> impl LayoutItem<Content = impl Render> {
    BoolEditBase::<AnimatedSwitch>(state).with_minimum_size(Extent2::new(32.0, 20.0))
}

/// Same as [Switch]
pub fn VerticalSwitch(state: Mutable<bool>) -> impl LayoutItem<Content = impl Render> {
    BoolEditBase::<AnimatedSwitch>(state).with_minimum_size(Extent2::new(20.0, 32.0))
}

impl BoolEditGraphics for AnimatedSwitch {
    fn describe_render(
        rect: Rect<f32, f32>,
        anim_factor: f32,
        parent_hints: ParentHints,
    ) -> impl Render {
        let bg_color = Rgb::new(58.0, 58.0, 58.0) / 255.0;
        let switch_color = Rgb::new(182.0, 182.0, 182.0) / 255.0;
        let bg_color_active = Rgb::new(183.0, 71.0, 71.0) / 255.0;
        let switch_color_active = Rgb::new(231.0, 231.0, 231.0) / 255.0;
        let inset_radius = 1.5;
        let rect_min_axis = f32::min(rect.w, rect.h);

        items! {
            // Background!
            Graphic::from(rect)
                .with_color(Lerp::lerp(bg_color, bg_color_active, anim_factor))
                .with_corner_radii(Vec4::one() * rect_min_axis * 0.5),
            // The switch indicator thingy!
            Graphic::from(
                rect
                .with_size(Extent2::new(rect_min_axis, rect_min_axis))
                .expand(-inset_radius * 2.0)

                // Make sure it starts at the right position...
                .translated(parent_hints.writing_origin() * (rect.w - rect_min_axis))
                .translated(parent_hints.writing_cross_origin() * (rect.h - rect_min_axis))

                // Animate it moving to the other side of the switch...
                .translated(parent_hints.writing_axis() * (rect.w - rect_min_axis) * anim_factor)
                .translated(parent_hints.writing_cross_axis() * (rect.h - rect_min_axis) * anim_factor)
            )
            // Make it pretty!
            .with_color(Lerp::lerp(switch_color, switch_color_active, anim_factor))
            .with_corner_radii(Vec4::one() * rect_min_axis * 0.5 - inset_radius * 2.0),
        }
    }
}
