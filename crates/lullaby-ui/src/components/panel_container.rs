use {
    crate::list_internal,
    ui_composer_basic_ui::primitives::graphic::Graphic,
    ui_composer_core::{
        app::composition::layout::hints::ParentHints, prelude::LayoutItem,
    },
    ui_composer_math::prelude::Srgba,
};

static SURFACE_COLOR: Srgba = Srgba::new(255.0, 253.0, 248.0, 255.0);
#[allow(unused)]
static SURFACE_COLOR_2: Srgba = Srgba::new(255.0, 241.0, 231.0, 255.0);

pub fn PanelContainer<Item>(item: Item) -> PanelContainer<Item> {
    PanelContainer { item }
}

pub struct PanelContainer<Item> {
    item: Item,
}
impl<Item> LayoutItem for PanelContainer<Item>
where
    Item: LayoutItem,
{
    type Blueprint = (Graphic, Item::Blueprint);

    fn prepare(
        &mut self,
        expected_parent_hints: ParentHints,
    ) -> ui_composer_core::app::composition::layout::hints::ChildHints {
        self.item.prepare(expected_parent_hints)
    }

    fn place(
        &mut self,
        // TODO: Reflect on whether it's necessary to pass any context when calling `place`.
        parent_hints: ParentHints,
    ) -> Self::Blueprint {
        list_internal![
            Graphic::new(parent_hints.rect, SURFACE_COLOR / 255.0),
            self.item.place(parent_hints)
        ]
    }
}
