#![allow(non_snake_case)]
use std::ops::Add;

use ui_composer::{
    prelude::*,
    standard::layout::item::{LayoutItem, ReshapableFragment},
};
use winit::{dpi::LogicalSize, platform::x11::WindowAttributesExtX11, window::WindowAttributes};

fn main() {
    let app = MyApp();
    let app_minimum_size = app.get_natural_size();

    let window_aabb = AABB::new(
        (0, 0),
        (0.max(app_minimum_size.0), 0.max(app_minimum_size.1)),
    );

    AppBuilder::new(app.bake(window_aabb))
        .with_window_attributes(
            WindowAttributes::default()
                .with_name("Simple App", "Simple App")
                .with_title("Container Layout Test")
                .with_inner_size(LogicalSize {
                    width: window_aabb.size.0,
                    height: window_aabb.size.1,
                })
                .with_min_inner_size(LogicalSize {
                    width: app_minimum_size.0,
                    height: app_minimum_size.1,
                }),
        )
        .run();
}

fn MyApp() -> impl LayoutItem {
    Column((Button(), Button()))
}

fn Column<A: LayoutItem, B: LayoutItem>((item_0, item_1): (A, B)) -> impl LayoutItem {
    let gap = 10; // 10px
    let item_a_size = item_0.get_natural_size();
    let item_b_size = item_1.get_natural_size();

    ReshapableFragment::new(
        (
            Ord::max(item_a_size.0, item_b_size.0) + gap + gap,
            Add::add(item_a_size.1, item_b_size.1) + gap + gap + gap,
        ),
        move |aabb: AABB| {
            // Padding!!!
            let padded_aabb = aabb.expand_radius(-10);

            (
                Rect(aabb, (0.6, 0.6, 0.6)),
                (
                    item_0.bake(AABB {
                        top_left: (padded_aabb.top_left.0, padded_aabb.top_left.1),
                        size: (padded_aabb.size.0, item_a_size.1),
                    }),
                    item_1.bake(AABB {
                        top_left: (
                            padded_aabb.top_left.0,
                            padded_aabb.top_left.1 + item_a_size.1 + gap,
                        ),
                        size: (padded_aabb.size.0, item_b_size.1),
                    }),
                ),
            )
        },
    )
}

#[allow(unused)]
fn Row<A: LayoutItem, B: LayoutItem>((item_0, item_1): (A, B)) -> impl LayoutItem {
    let gap = 10; // 10px
    let item_a_natural_size = item_0.get_natural_size();
    let item_b_natural_size = item_1.get_natural_size();

    ReshapableFragment::new(
        (
            Add::add(item_a_natural_size.0, item_b_natural_size.0) + gap + gap + gap,
            Ord::max(item_a_natural_size.1, item_b_natural_size.1) + gap + gap,
        ),
        move |aabb: AABB| {
            // Padding!!!
            let padded_aabb = aabb.expand_radius(-10);

            (
                Rect(aabb, (0.6, 0.6, 0.6)),
                (
                    item_0.bake(AABB {
                        top_left: (padded_aabb.top_left.0, padded_aabb.top_left.1),
                        size: (item_a_natural_size.0, padded_aabb.size.1),
                    }),
                    item_1.bake(AABB {
                        top_left: (
                            padded_aabb.top_left.0 + item_a_natural_size.0 + gap,
                            padded_aabb.top_left.1,
                        ),
                        size: (item_b_natural_size.0, padded_aabb.size.1),
                    }),
                ),
            )
        },
    )
}

fn Button() -> impl LayoutItem {
    ReshapableFragment::new((320, 64), |aabb| {
        let base = Rect(aabb, (0.2, 0.2, 0.2));
        let text = Rect(aabb.expand_radius(-8), (0.3, 0.3, 0.3));

        return (base, text);
    })
}

fn Rect(aabb: AABB, color: (f32, f32, f32)) -> Primitive {
    Primitive {
        transform: [
            [aabb.size.0 as f32, 0.0, 0.0, 0.0],
            [0.0, aabb.size.1 as f32, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [aabb.top_left.0 as f32, aabb.top_left.1 as f32, 0.0, 1.0],
        ],
        color: [color.0, color.1, color.2],
    }
}
