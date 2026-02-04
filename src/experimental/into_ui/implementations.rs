use crate::experimental::into_ui::{Editor, IntoDefaultUI};
use crate::runners::wgpu::components::{Label, TextLayoutItem};
use crate::standard::RowContainer;

/* Strings */

impl<S> Editor<S> for TextLayoutItem<S>
where
    S: AsRef<str>,
{
    fn edit(data: S) -> Self {
        Label(data)
    }
}

impl IntoDefaultUI for &str
{
    type DefaultEditor = TextLayoutItem<Self>;
}

/* Containers */

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
