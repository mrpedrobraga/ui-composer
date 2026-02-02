use ui_composer::runners::winitwgpu::prelude::UI;
use ui_composer::standard::prelude::*;
use ui_composer::standard::runners::wgpu::components::*;

fn main() {
    UIComposer::run(Window(App().into_default_ui()));
}

fn App() -> impl IntoDefaultUI {
    ("Hello, world!", "Also hello!")
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
trait IntoDefaultUI: Sized {
    type DefaultEditor: UI + Sync + Editor<Self>;

    fn into_default_ui(self) -> Self::DefaultEditor
    where
        Self: Sized,
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

/*impl<S> IntoDefaultUI for S
where
    S: AsRef<str> + Clone + Send + Sync,
{
    type DefaultEditor = TextLayoutItem<S>;
}*/

impl IntoDefaultUI for &str
{
    type DefaultEditor = TextLayoutItem<Self>;
}

impl<A, B> Editor<(A, B)> for RowContainer<A::DefaultEditor, B::DefaultEditor> where
    A: IntoDefaultUI,
    B: IntoDefaultUI,   {
    fn edit(data: (A, B)) -> Self {
        Self {
            item_a: data.0.into_default_ui(),
            item_b: data.1.into_default_ui(),
            gap: 0.0,
        }
    }
}

impl<A, B> IntoDefaultUI for (A, B)
where
    A: IntoDefaultUI,
    B: IntoDefaultUI,
{
    type DefaultEditor = RowContainer<A::DefaultEditor, B::DefaultEditor>;
}


impl<A, B> Editor<ColumnContainer<A, B>> for ColumnContainer<A::DefaultEditor, B::DefaultEditor> where
    A: IntoDefaultUI,
    B: IntoDefaultUI,   {
    fn edit(data: ColumnContainer<A, B>) -> Self {
        Self {
            item_a: data.item_a.into_default_ui(),
            item_b: data.item_b.into_default_ui(),
            gap: data.gap,
        }
    }
}

impl<A, B> IntoDefaultUI for ColumnContainer<A, B>
where
    A: IntoDefaultUI,
    B: IntoDefaultUI,
{
    type DefaultEditor = ColumnContainer<A::DefaultEditor, B::DefaultEditor>;
}
