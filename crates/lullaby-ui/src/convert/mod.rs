/// Trait that allows data types to be easily converted into a "canonical"
/// editor within the Lullaby design system.
pub trait ToDefaultUi {
    type DefaultUi;

    fn to_default_ui(&self) -> Self::DefaultUi;
}
