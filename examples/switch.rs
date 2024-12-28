#![allow(unused, non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::animation::spring::Spring;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(App()).with_title("Switches!"))
}

fn App() -> impl LayoutItem {
    let state = Mutable::new(false);

    Center(Switch(state))
}

fn Switch(state: Mutable<bool>) -> impl LayoutItem {
    SwitchBase(state).with_minimum_size(Extent2::new(32.0, 20.0))
}

fn VerticalSwitch(state: Mutable<bool>) -> impl LayoutItem {
    SwitchBase(state).with_minimum_size(Extent2::new(20.0, 32.0))
}

fn SwitchBase(state: Mutable<bool>) -> impl Resizable + LayoutItem {
    let anim_state = Mutable::new(0.0);
    let mposs = Mutable::new(None);

    ResizableItem::new(move |parent_hints| {
        let rect = parent_hints.rect;
        let state_ = state.clone();
        let tap = Tap::new(rect, mposs.clone(), move || state_.set(!state_.get()));

        items! {
            tap,
            Spring::if_then_else(state.signal(), anim_state.clone(), 1.0, 0.0).process(),
            anim_state.signal().map(move |anim_factor| {
                SwitchStyle(rect, anim_factor, &parent_hints)
            }).process()
        }
    })
}

fn SwitchStyle(
    rect: Rect<f32, f32>,
    anim_factor: f32,
    parent_hints: &ParentHints,
) -> impl ItemDescriptor {
    let bg_color = Rgb::new(58.0, 58.0, 58.0) / 255.0;
    let switch_color = Rgb::new(182.0, 182.0, 182.0) / 255.0;
    let bg_color_active = Rgb::new(183.0, 71.0, 71.0) / 255.0;
    let switch_color_active = Rgb::new(231.0, 231.0, 231.0) / 255.0;
    let inset_radius = 1.5;
    let rect_min_axis = f32::min(rect.w, rect.h);

    items! {
        rect
            .with_color(Lerp::lerp(bg_color, bg_color_active, anim_factor))
            .with_corner_radii(Vec4::one() * rect_min_axis * 0.5),
        rect
            .with_size(Extent2::new(rect_min_axis, rect_min_axis))
            .expand(-inset_radius * 2.0)
            .translated(parent_hints.writing_origin() * (rect.w - rect_min_axis))
            .translated(parent_hints.writing_cross_origin() * (rect.h - rect_min_axis))
            .translated(parent_hints.writing_axis() * (rect.w - rect_min_axis) * anim_factor)
            .translated(parent_hints.writing_cross_axis() * (rect.h - rect_min_axis) * anim_factor)
            .with_color(Lerp::lerp(switch_color, switch_color_active, anim_factor))
            .with_corner_radii(Vec4::one() * rect_min_axis * 0.5 - inset_radius * 2.0),
    }
}
