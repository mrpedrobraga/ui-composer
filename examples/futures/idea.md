```rust
#![allow(non_snake_case)]

/* When Layout reactivity is implemented... */

fn App2() -> impl UI {
    fetch_person("https://mrpedrobraga.com/api")
        .map(PersonView)
        .into_ui()
}

fn PersonView() -> impl UI {
    Label(format!("{}, {} years old.", person.name, person.age))
}

fn Label() -> impl UI {}
```

| Data              | Editor              | Condition          |
|-------------------|---------------------|--------------------|
| `String`          | `Label<String>`     |                    |
| `Mutable<String>` | `TextEdit<String>`  |                    |
| `bool`            | `BoolView`          |                    |
| `Mutable<bool>`   | `Checkbox`          |                    |
| `Mutable<S>`      | `MaskedTextEdit<S>` | `where S: FromStr` |
| `Future<A>`       | `Future<F<A>>`      |                    |

```rust
#[derive(IntoUI)]
#[into_ui(editor = "Column")]
struct Person {
    #[into_ui(editor = "Label")]
    name: String,

    #[into_ui(editor = "Checkbox")]
    is_alive: Mutable<bool>,
}
```