# Ideal syntax for the best UI Composer DX.

## Functorial `for` syntax.

For deriving functors, the `for` syntax allows getting a `T` out of a `Foo<T>`
while handling the "Foo"ness.

For example, `Label` only accepts `String`, so you can't pass a `Option<String>` to it,
it wouldn't be sound because what if there is no string?

But you can map a `Option<String>` to get an `Option<Label>`!

```rust
for string of string_opt {
    Label(string)
}
```

This works with any type implementing `UiMap` from `ui-composer-core`, and the functiorial aspect will be handled for you.

There's also the trait `WithEmptyState` and the syntax:

```rust
for string of string_opt {
    Label(string)
} else {
    EmptyState ()
}
```

`UiMap` (or equivalent) is implemented for `Option<_>`, `Vec<_>` and other collections, `F: Future`, `S: Signal`. `WithEmptyState` is implemented for the applicable ones.

### Fine-grained reactivity

Reactivity is achieved mapping, say, a `Signal<i32>` into a `Signal<Label>`!

> [!NOTE] Well, actually...
>
> Because of limitations regarding blanket implementations for foreign traits, `Ui` can't be impl'd for `<T> Signal<T> where T: Ui`... instead, mapping a `Signal<i32>` gets you a `React<Label, _, _>`, a type defined in `ui-composer-state` which `Ui` can be impl'd for.

```rust
fn App() -> impl Ui {
    let count_state = mutable(0);
    let increment = count_state.effect(|e| *e += 1);
    let decrement = count_state.effect(|e| *e -= 1);

    view !{
        row [
            Button (Label("Take 1"), decrement)
            
            // Fine-grained reactivity.
            // Only this part of the UI updates
            // when count_state changes.
            for count of count_state.signal() {
                Label(format!("Count: {count}"))
            }
            
            Button (Label("Give 1"), increment)
        ]
    }
}
```

For futures, you can use the `else` clause to render a loading state.

```rust
fn App() -> impl Ui {
    let response = http_request("https://myapi.com/value");

    view! {
        row [
            for value of response {
                Label f!("Got {value}!")
            } else {
                Label { text_style = Faint } "Loading..."
            }
        ]
    }
}
```