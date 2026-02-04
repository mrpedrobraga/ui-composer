use crate::runners::wgpu::components::{Label, TextLayoutItem};
use crate::runners::winitwgpu::prelude::UI;
use crate::standard::{ColumnContainer, RowContainer};

pub mod implementations;

/// Trait that represents something that can edit some data.
pub trait Editor<Data> {
    fn edit(data: Data) -> Self;
}
/// Reverse trait of [Editor], can transform data on its editor.
pub trait IntoUI<Editor> {
    fn into_ui(self) -> Editor;
}

/// "Default" version of [IntoUI], where a "canon" editor is chosen
/// for the data type.
pub trait IntoDefaultUI: Sized {
    type DefaultEditor: UI + Sync + Editor<Self>;

    fn into_default_ui(self) -> Self::DefaultEditor
    where
        Self: Sized,
    {
        Self::DefaultEditor::edit(self)
    }
}

impl<D, E> IntoUI<E> for D
where
    E: Editor<D>,
{
    fn into_ui(self) -> E {
        E::edit(self)
    }
}





