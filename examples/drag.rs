#![allow(unused, non_snake_case)]

use futures_signals::map_ref;
use ui_composer::app::input::items::Drag;
use ui_composer::items;
use ui_composer::prelude::items::DragState;
use ui_composer::prelude::process::React;
use ui_composer::prelude::*;
use ui_composer::wgpu::pipeline::UIReifyResources;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::render_target::RenderDescriptor;

fn main() {
    let window = Window(App())
        .with_title("Custom window!".into())
        .with_decorations(false);

    UIComposer::run(window);
}

fn App() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let mut drag_bar = Drag();

    ResizableItem::new(move |parent| {
        drag_bar.lay(ParentHints {
            rect: Rect::from((parent.rect.center(), Extent2::zero()))
                .expand_from_center(64.0, 64.0, 64.0, 64.0),
            ..parent
        })
    })
    .with_minimum_size(Extent2::new(64.0, 64.0))
}

fn Drag() -> impl LayoutItem<Content = impl RenderDescriptor> {
    let drag_state = Mutable::new(Default::default());
    let offset_state = Mutable::new(Vec2::zero());
    let mouse_position = Mutable::new(Vec2::zero());

    let factory = move |parent: ParentHints| {
        let drag_state = drag_state.clone();
        let mouse_position = mouse_position.clone();
        let offset_state = offset_state.clone();

        React(map_ref! {
            let offset = offset_state.signal(),
            let drag = drag_state.signal() => {
                let dis_rect = parent.rect.translated(*offset);

                let window_drag = Drag::new(
                    dis_rect,
                    drag_state.clone(),
                    mouse_position.clone(),
                    offset_state.clone(),
                );

                let color = match *drag {
                    DragState::None => Rgb::new(1.0, 1.0, 1.0),
                    DragState::Hovering => Rgb::new(0.7, 0.6, 0.5),
                    DragState::Dragging => Rgb::new(0.5, 0.6, 0.7),
                };

                items! {
                    window_drag,
                    Graphic::from(if let DragState::Dragging = *drag { dis_rect.expand_from_center(4.0, 4.0, 4.0, 4.0) } else { dis_rect }).with_color(color)
                }
            }
        })
    };

    ResizableItem::<_, _, UIReifyResources>::new(factory)
        .with_minimum_size(Extent2::new(10.0, 32.0))
}
