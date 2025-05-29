# UI Composer

Rust-based library for fast native UI rendering.

## Getting started

After adding the library, you should be able to create a simple window like this:

```rust
use ui_composer::prelude::*;

fn main() {
    UIComposer::run(Window(()));
}
```