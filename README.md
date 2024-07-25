# UI Composer

> This repository is so WIP dude

Rust-based library for fast native UI rendering.

## Getting started

After installing the library, you should be able to create a simple window like this:

```rust
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(()).run()
}
```

This is a functional library, the way you tell the app to render anything is by _returning_ some `UIFragment`s to it in `AppBuilder::new`.

```rust
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(
        Primitive::rect(
            Rect::new(0.0, 0.0, 64.0, 64.0),
            Rgb::red()
        )
    ).run()
}
```

Let's move the fragments to a new function.

```rust
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(
        App()
    ).run()
}

fn App() -> impl UIFragment {
    Primitive::rect(
        Rect::new(0.0, 0.0, 64.0, 64.0),
        Rgb::red()
    )
}
```

We can return move than a single UIFragment by using tuples.
For example, we can return an Interactor.

```rust
use ui_composer::prelude::*;

fn main() {
    AppBuilder::new(
        App()
    ).run()
}

fn App() -> impl UIFragment {
    let rect = Rect::new(0.0, 0.0, 200.0, 200.0);
    let hover_interaction = HoverInteraction::rect(rect);

    (
        // Listening on the hover signal
        hover_interaction
            .get_signal()
            // We 'map' to generate an UIFragment in a reactive fashion!
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

        // We also return the interactor itself
        // so that the app holds it to send events to it
        hover_interaction,
    )
}
```

Between Interactors and Primitives everything is possible.
