#![allow(non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::animation::spring::*;
use ui_composer::prelude::*;

fn main() {
    App::run(
        Window(Center(Row(
            Row(
                SmoothSquare(Rgb::new(255.0, 172.0, 183.0) / 255.0),
                SmoothSquare(Rgb::new(251.0, 200.0, 167.0) / 255.0),
            ),
            Row(
                SmoothSquare(Rgb::new(93.0, 212.0, 223.0) / 255.0),
                SmoothSquare(Rgb::new(255.0, 249.0, 245.0) / 255.0),
            ),
        )))
        .with_title("Interactive Animation"),
    )
}

fn SmoothSquare(color: Rgb<f32>) -> impl LayoutItem {
    let is_hovered_state = Editable::new(false);
    let anim_state = Editable::new(0.0);

    Resizable::new(move |hx| {
        let is_hovered_state = is_hovered_state.clone();
        items!(
            Spring::if_then_else(is_hovered_state.signal(), anim_state.clone(), 50.0, 0.0)
                .process(),
            anim_state
                .signal()
                .map(move |animation_factor| {
                    hover_square(hx.rect, color, animation_factor, is_hovered_state.clone())
                })
                .process()
        )
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

fn hover_square(
    original_rect: Rect<f32, f32>,
    original_color: Rgb<f32>,
    animation_factor: f32,
    is_hovered_state: Editable<bool>,
) -> impl ItemDescriptor {
    let hover_rect = original_rect.expanded_to_contain_point(Vec2::new(
        original_rect.x,
        original_rect.y - animation_factor,
    ));
    let hover = Hover::new(hover_rect, is_hovered_state);
    let rect = original_rect
        .translated(-animation_factor * Vec2::unit_y())
        .expand(8.0 * animation_factor / 50.0);

    #[cfg(feature = "debug")]
    let hover_rect_graphic = hover_rect.with_color(Rgb::new(1.0, 0.1, 0.2));
    #[cfg(not(feature = "debug"))]
    let hover_rect_graphic = ();

    let animation_factor_pct = animation_factor / 50.0;

    items!(
        hover,
        rect.with_color(Lerp::lerp(
            original_color,
            original_color,
            animation_factor_pct
        )),
        hover_rect_graphic
    )
}
