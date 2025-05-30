#![allow(non_snake_case)]

use ui_composer::layout::{LayoutItem, Resizable, ResizableItem};
use ui_composer::wgpu::pipeline::{graphics::graphic::Graphic, text::Text};
use ui_composer::wgpu::render_target::Render;
use vek::Vec3;
use {
    ui_composer::{
        app::{
            input::items::{Hover, Tap},
            UIComposer,
        },
        components::{Center, Column, Row},
        geometry::RectExt,
        items,
        prelude::animation::spring::*,
        state::{
            process::{UIFutureExt, UISignalExt},
            Mutable, SignalExt,
        },
        winitwgpu::window::Window,
    },
    vek::{Extent2, Lerp, Rect, Rgb, Vec2, Vec4},
};

fn main() {
    let app = Center(Column(
        Row(
            SmoothSquare("A", Rgb::new(126.0, 46.0, 132.0) / 255.0),
            SmoothSquare("B", Rgb::new(209.0, 64.0, 129.0) / 255.0),
        ),
        Row(
            SmoothSquare("C", Rgb::new(239.0, 121.0, 138.0) / 255.0),
            SmoothSquare("D", Rgb::new(249.0, 245.0, 227.0) / 255.0),
        ),
    ));

    let window = Window(app).with_title("Interactive Animation".into());

    UIComposer::run(window)
}

fn SmoothSquare(name: &'static str, color: Rgb<f32>) -> impl LayoutItem<Content = impl Render> {
    let is_hovered_state = Mutable::new(false);
    let mouse_position_state = Mutable::new(None);
    let tap_state = Mutable::new(None);
    let anim_state = Mutable::new(0.0);

    ResizableItem::new(move |hx| {
        let is_hovered_state = is_hovered_state.clone();
        let mouse_position_state_anim = mouse_position_state.clone();
        let tap_state_anim = tap_state.clone();

        let animation =
            Spring::if_then_else(is_hovered_state.signal(), anim_state.clone(), 50.0, 0.0);

        items!(
            animation.process(),
            anim_state
                .signal()
                .map(move |animation_factor| {
                    hover_square(
                        hx.rect,
                        color,
                        animation_factor,
                        is_hovered_state.clone(),
                        mouse_position_state_anim.clone(),
                        tap_state_anim.clone(),
                    )
                })
                .process(),
            tap_state
                .signal()
                .for_each(move |tap| {
                    if tap.is_some() {
                        println!("Tapped {}!", name);
                    }
                    async {}
                })
                .process(),
        )
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}

fn hover_square(
    original_rect: Rect<f32, f32>,
    original_color: Rgb<f32>,
    animation_factor: f32,
    is_hovered_state: Mutable<bool>,
    mouse_position_state: Mutable<Option<Vec2<f32>>>,
    tap_state: Mutable<Option<()>>,
) -> impl Render {
    let hover_rect = original_rect.expanded_to_contain_point(Vec2::new(
        original_rect.x,
        original_rect.y - animation_factor,
    ));
    let hover = Hover::new(hover_rect, is_hovered_state);
    let rect = original_rect
        .translated(-animation_factor * Vec2::unit_y())
        .expand(8.0 * animation_factor / 50.0);
    let tap = Tap::new(rect, mouse_position_state, tap_state);

    #[cfg(feature = "debug")]
    let hover_rect_graphic = Graphic::from(hover_rect).with_color(Rgb::new(1.0, 0.1, 0.2));
    #[cfg(not(feature = "debug"))]
    let hover_rect_graphic = ();

    let animation_factor_pct = animation_factor / 50.0;

    items! {
        hover,
        tap,
        Graphic::from(rect).with_color(Lerp::lerp(
            original_color,
            original_color,
            animation_factor_pct
        ))
        .translated_3d(Vec3::new(0.0, 0.0, -0.1 * animation_factor_pct))
        .with_corner_radii(Lerp::lerp(
            Vec4::zero(),
            Vec4::one() * 50.0,
            animation_factor_pct
        )),
        hover_rect_graphic,
        Text(rect, "Dummy text!".to_owned(), Rgb::new(1.0, 1.0, 1.0) - original_color)
    }
}
