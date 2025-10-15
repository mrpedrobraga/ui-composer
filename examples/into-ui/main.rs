use ui_composer::standard::prelude::UIComposer;
use ui_composer::standard::backends::wgpu::components::{Label, TextLayoutItem};
use ui_composer::prelude::Window;

fn main() {
    UIComposer::run(Window(
        "Hello, world!".into_default_ui(), // Becomes a `Label`
    ));
}

/// Trait that represents something that can edit some data.
trait Editor<Data> {
    fn edit(data: Data) -> Self;
}
/// Reverse trait of [Editor], can transform data on its editor.
trait IntoUI<Editor> {
    fn into_ui(self) -> Editor;
}
impl<D, E> IntoUI<E> for D
where
    E: Editor<D>,
{
    fn into_ui(self) -> E {
        E::edit(self)
    }
}
/// "Default" version of [IntoUI], where a "canon" editor is chosen
/// for the data type.
trait IntoDefaultUI {
    type DefaultEditor;

    fn into_default_ui(self) -> Self::DefaultEditor
    where
        Self: Sized,
        Self::DefaultEditor: Editor<Self>,
    {
        Self::DefaultEditor::edit(self)
    }
}

impl<S> Editor<S> for TextLayoutItem<S>
where
    S: AsRef<str>,
{
    fn edit(data: S) -> Self {
        Label(data)
    }
}

impl<S> IntoDefaultUI for S
where
    S: AsRef<str>,
{
    type DefaultEditor = TextLayoutItem<S>;
}
