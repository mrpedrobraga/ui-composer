#![allow(non_snake_case)]
use itertools::Itertools as _;
use ui_composer::{app::App, items, prelude::*, ui::node::SizedVec};

pub fn main() {
    App::run(Window(Squares()));
}

fn grid(width: i32, height: i32) -> impl Iterator<Item = (f32, f32)> {
    (0..width).cartesian_product(0..height)
        .map(|(x, y)| (x as f32, y as f32))
}

pub fn Squares() -> impl LayoutItem {
    Center(
        Resizable::new(move |hx| {
            grid(10, 10).map(|(x, y)| {
                let rect = hx.rect
                    .with_size(Extent2::new(32.0, 32.0))
                    .translated(Vec2::new(x as f32 * 32.0, y as f32 * 32.0));
                let hover_area = Hover::new(rect);

                return items![
                    hover_area.signal().map(move |is_hovering| {
                        if is_hovering {
                            rect.with_color(Rgb::cyan())
                        } else {
                            rect.with_color(Rgb::new(x / 10.0, y / 10.0, 0.0))
                        }
                    }).collect_ui(),
                    hover_area,
                ]
            }).collect::<SizedVec<_, 100>>()
        }).with_minimum_size(Extent2::one() * 32.0 * 10.0),
    )
}
