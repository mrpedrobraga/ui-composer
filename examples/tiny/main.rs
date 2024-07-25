use ui_composer::{interaction::hover::HoverInteraction, prelude::*};

fn main() {
    AppBuilder::new(App()).run()
}

fn App() -> impl UIFragment {
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let hover_interaction = HoverInteraction::rect(rect);

    (
        hover_interaction
            .get_signal()
            .map(move |is_hovering| {
                Primitive::rect(
                    rect,
                    if is_hovering {
                        Rgb::red()
                    } else {
                        Rgb::green()
                    },
                )
            })
            .into_fragment(),
        hover_interaction,
    )
}
