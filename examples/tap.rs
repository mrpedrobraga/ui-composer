#![allow(non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(Center(MyApp())))
}

fn MyApp() -> impl LayoutItem {
    let mouse_position = Mutable::new(None);
    ResizableItem::new(move |hx| {
        let rect = hx.rect;
        let mouse_position = mouse_position.clone();
        let tap = Tap::new(rect, mouse_position.clone(), move || {
            println!("{:?}", mouse_position.get())
        });

        items! {
            rect.with_color(Rgb::new(126.0, 46.0, 132.0) / 255.0),
            tap
        }
    })
    .with_minimum_size(Extent2::new(100.0, 100.0))
}
