#![allow(non_snake_case)]
use itertools::Itertools as _;
use ui_composer::{app::App, prelude::*, ui::node::SizedVec};

pub fn main() {
    App::run(Window(Squares()));
}

pub fn Squares() -> impl LayoutItem {
    let range_length = 10;
    let range = 0..range_length;

    Center(
        Resizable::new(move |hx| {
            return SizedVec::<_, 100>::new(
                range
                    .clone()
                    .cartesian_product(range.clone())
                    .map(|(x, y)| {
                        hx.rect
                            .with_size(Extent2::new(32.0, 32.0))
                            .translated(Vec2::new(x as f32 * 32.0, y as f32 * 32.0))
                            .expand_radius(-4.0)
                            .with_color(Rgb::new(
                                x as f32 / range_length as f32,
                                y as f32 / range_length as f32,
                                0.0,
                            ))
                    })
                    .collect_vec(),
            );
        })
        .with_minimum_size(Extent2::one() * 32.0 * range_length as f32),
    )
}
