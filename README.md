# UI Composer

UI composer is a fast, modern native UI Rendering library. It combines the performance of native rendering,
with the flexibility and expressiveness of web frontend development.

It achieves its speed by bypassing the need for a DOM and issuing direct calls to the GPU in response to events.

> As of now, UI Composer is in an early _"research stage"_ and should not be used for anything critical. I really do
> welcome experimentation and discussion, though!

## Getting Started

Add `ui-composer` as a git dependency from this repository. ~~In the future, you'll be able
to `cargo add ui-composer`.~~

```rust
use ui_composer::{prelude::*, std::*};

fn main() {
    UIComposer::run(Window(Label("Hello, World!")))
}
```

And you're ready to ship!

## API Design

Where this library shines is on _its API design_. Instead of having a gigantic library of features powered by hacky
magic, UI Composer's core has only handful of concepts — quanta of functionality each which focuses on doing a
_single thing well_.

Apps are creating by _composing_ (eh?) these primitives together, UNIX-style. In fact, this library comes with zero "
builtin widgets". The `Button`, `Label`, `Flex`, etc you see here are all implemented with primitives in the user-facing
API.

There are three types of components to be precise: _State_, _Input_ and _Output_.

```rust
use ui_composer::prelude::*;
use ui_composer::std::{Label, Button};

fn main() {
    // State
    let text_state = Mutable::new("");

    let app = Row(
        // Input - Modifies state...
        Button(
            Label("Click me!"),
            text_state.effect(|_| "Thank you!")
        ),
        // Output - Visualizes state, possibly reactively!
        text_state.imply(Label),
    );

    UIComposer::run(Window(app));
}
```

This means that sometimes you'll miss the granularity that dozens of _React Hooks_ give you, and the library has a
steeper learning curve. But as a result, after you learn `ui-composer`, the code is simple to read _reason about_. In
the example above it is clear what gets re-rendered when `text_state` changes (only the `Label`, and not the `Button`,
since that's what inside `.map`).

### The Editor Pattern (Algebraic UI)

The star of the show of UI Composer is the "Editor" pattern, which dictates how the API is designed from the ground up.
In this paradigm, the UI is _stateless_, but follows the same algebraic structure as the data it views/edits.

This is not only a good way of organizing code, but it gives the user superpowers: being able to reason locally about
state, and being able to compose parts endlessly.

```rust
// Data
struct PersonForm {
    name: Mutable<String>,
    description: Mutable<String>,
}

// Editor
#[allow(non_snake_case)]
fn PersonFormEditor(person_form: PersonForm, send_fx: impl Effect) -> impl UI {
    // notice how instead of using CSS we *compose* layout by using newtypes.
    Center(
        WithSize(
            Extent2::new(300.0, 300.0),
            Flex![
                [ _ ] Label("Log in!"),
                [ _ ] Row![ Label("Name"), TextEdit(form.name) ],
                [1.0] ( // <- `flex-grow: 1;`
                    Column![
                        Label("Tell us about yourself"),
                        TextEdit(form.description)
                    ]
                ),
                [ _ ] Button(Label("Send"), send_fx),
                // Notice how instead of passing a &str,
                // we pass an item: `Label`. This is so we could
                // make a button containing anything else we desire.
                //
                // Good design is in the little things, isn't it?
            ]
        )
    )
}
```

## Features

- [x] Fast;
    - [x] GPU Accelerated;
    - [x] Small binary sizes;
    - [x] Modular design with Cargo features for conditional compilation;
- [ ] Standard library of common components and data-structures, inspired by [shadcn/ui](https://ui.shadcn.com/);
    - [x] `Button`, `Label`, `Switch`, `TextEdit`, `ColorEdit`, `CalendarView` and common basic components like these.
    - [ ] `Toast`, `Sonner`, `CommandPalette` and rich components like these.
    - [x] `Center`, `Row`, `Flex`, `FlexWrap`, `Grid`, `Masonry` and many other containers!
    - [ ] `Symphony UI` and `Lullaby UI` as two beautiful styling toolkits.
- [x] Create your own components using primitives.
- [x] First class support for 3D rendering (that lives _in_ the layout context);
- [ ] First class custom shaders support;
- [x] First-class `Animator` API inspired by `std::iter::Iterator`, [Motion Canvas](https://motioncanvas.io/) and
  the [View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions) and can animate literally
  any `State`;
- [ ] Attention to detail and polish;
    - [x] Full Unicode support courtesy of `cosmic-text`.
    - [x] Gradient Debanding;
    - [ ] Support for squircles as border radii;
- [ ] Focus on Accessibility;
    - [ ] Support for screen-readers and navigation using [accesskit](https://github.com/AccessKit/accesskit);
    - [x] Semantic layouting that responds to locale;
- [ ] Composition over configuration. — "Widgets", "DOM", "Objects" are a non-concept in this library.
- [ ] Cross-Platform — it's not "write once, run anywhere", but it's "learn once, apply anywhere";
    - [x] Desktop — Windows, Linux, Mac OS;
    - [ ] Mobile — Android, iOS;
    - [ ] WASM — Web;
    - [ ] Embedded (`no-std`) — think Raspberry PI, microcontrollers, the Game Boy Advance;
    - [ ] TUI — terminals, using [crossterm](https://github.com/crossterm-rs/crossterm);

## Implementation Details

- Signals from the marvellous `futures-signals` crate;
- `cosmic-text` for rendering text in most platforms;
- `winit` and `wgpu` for targeting modern desktops with GPU acceleration;
- `winit` and `pixels` for targeting desktops without GPU acceleration;
- `crossterm` for drawing to the terminal;

## Contributing

I don't accept code contributions _yet_, but discussions are welcome. Reach me
at [Bluesky](https://bsky.app/profile/mrpedrobraga.com) or email [my email](mailto:mrhenribraga@gmail.com);
