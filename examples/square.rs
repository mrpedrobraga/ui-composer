#![allow(non_snake_case)]
use itertools::Itertools as _;
use ui_composer::{app::UIComposer, items, prelude::*, ui::node::SizedVec};

pub fn main() {
    UIComposer::run(Window(Squares()));
}

fn grid_range(width: i32, height: i32) -> impl Iterator<Item = (f32, f32)> {
    (0..width)
        .cartesian_product(0..height)
        .map(|(x, y)| (x as f32, y as f32))
}

pub fn Squares() -> impl LayoutItem {
    Center(
        ResizableItem::new(move |hx| {
            grid_range(10, 10)
                .map(|(x, y)| {
                    let is_hovered_state = Mutable::new(false);
                    let rect = Rect::new(hx.rect.x + 32.0 * x, hx.rect.y + 32.0 * y, 32.0, 32.0);
                    let color = Rgb::new(x / 10.0, y / 10.0, 0.0);
                    let hover_area = Hover::new(rect, is_hovered_state.clone());

                    return items![
                        is_hovered_state
                            .signal()
                            .map(move |is_hovering| {
                                if is_hovering {
                                    rect.expand(-4.0).with_color(color)
                                } else {
                                    rect.with_color(color)
                                }
                            })
                            .process(),
                        hover_area,
                    ];
                })
                .collect::<SizedVec<_, 100>>()
        })
        .with_minimum_size(Extent2::one() * 32.0 * 10.0),
    )
}
