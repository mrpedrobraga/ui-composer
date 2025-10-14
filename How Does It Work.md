# How does it work?

The fundamental principle behind how ui-composer works is "concepts" encoded in types.

See, the UI as rendered consists of a bunch of primitives. Think triangles, splats, pixels or lines of text. You could simply push a `&[Primitive]` to the engine, but, as apps becomes more complex it's impossible to manually keep track of how `<your app state> -> <primitives>`.

So, instead, ui-composer allows you to create UI by adapting the ADT-based structure of your app.

- Primitive values (`i32`, `bool`, `&str`) -> Primitives;
- `struct`, `enum`, `Vec` -> Containers;
- Monads remain the same, i.e. `Future`, `Iterator`, `Mutable` -> `Future`, `Iterator`, `Mutable`.

Technically speaking, it's even possible to automatically generate UI from a data specification... But, of course, we like to customise how things actually look.

Custom visual functionality such as clipping, shaders, scrolling, can be achieved with custom components, adapters and newtypes.