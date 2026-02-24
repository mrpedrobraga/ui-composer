/// Trait for a Ui component that can edit some type T.
pub trait Edit<T: ?Sized> {
    fn edit(value: &T) -> Self;
}

/// Trait for a value which can be transformed into UI.
pub trait ToUi<Ui: Edit<Self>> {
    fn to_ui(&self) -> Ui;
}
impl<Ui, Item> ToUi<Ui> for Item
where
    Ui: Edit<Item>,
{
    fn to_ui(&self) -> Ui {
        Ui::edit(self)
    }
}
