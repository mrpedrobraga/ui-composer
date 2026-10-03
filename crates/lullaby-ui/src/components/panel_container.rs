use {
    ::ui_composer_platform_winit::runner::{DesktopEnvironment, DesktopResources},
    ui_composer_basic_ui::primitives::graphic::{Graphic, RenderQuad},
    ui_composer_core::{
        app::composition::{
            elements::{Blueprint, Element},
            layout::hints::ParentHints,
        },
        prelude::Ui,
    },
    ui_composer_math::{glamour::Rect, prelude::Srgba},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
};

static SURFACE_COLOR: Srgba = Srgba::new(255.0, 253.0, 248.0, 255.0);
#[allow(unused)]
static SURFACE_COLOR_2: Srgba = Srgba::new(255.0, 241.0, 231.0, 255.0);

pub fn PanelContainer<Item>(item: Item) -> PanelContainer<Item> {
    PanelContainer {
        item,
        rect: Rect::ZERO,
    }
}

#[pin_project::pin_project]
pub struct PanelContainer<Item> {
    #[pin]
    item: Item,
    rect: Rect,
}
impl<Item> Ui<TerminalEnvironment> for PanelContainer<Item>
where
    Item: Ui<TerminalEnvironment>,
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
        resources: &TerminalBlueprintResources,
    ) {
        self.rect = parent_hints.rect;
        self.item.place(parent_hints, resources);
    }

    fn effect(
        &self,
    ) -> (
        RenderQuad,
        <<Item::Blueprint as Blueprint<TerminalEnvironment>>::Output as Element<
            TerminalEnvironment,
        >>::Effect,
    ) {
        (RenderQuad(self.rect, SURFACE_COLOR), self.item.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &TerminalBlueprintResources,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.item.poll_change(cx, resources, parent_hints)
    }
}
impl<Item> Ui<DesktopEnvironment> for PanelContainer<Item>
where
    Item: Ui<DesktopEnvironment>,
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
        resources: &DesktopResources,
    ) {
        self.rect = parent_hints.rect;
        self.item.place(parent_hints, resources);
    }

    fn effect(
        &self,
    ) -> (
        RenderQuad,
        <<Item::Blueprint as Blueprint<DesktopEnvironment>>::Output as Element<
            DesktopEnvironment,
        >>::Effect,
    ) {
        (RenderQuad(self.rect, SURFACE_COLOR), self.item.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &DesktopResources,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.item.poll_change(cx, resources, parent_hints)
    }
}
