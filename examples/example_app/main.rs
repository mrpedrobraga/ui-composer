#![allow(non_snake_case)]

use serde_json::Value;
use ui_composer::prelude::*;
use ui_composer::state::process::React;
use ui_composer::wgpu::components::*;
use ui_composer::wgpu::pipeline::text::Text;
use ui_composer::winitwgpu::prelude::*;
use ui_composer::Flex2;
use ui_composer::prelude::items::Typing;
use ui_composer::wgpu::image::Image;
use ui_composer::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::wgpu::pipeline::UIContext;

extern crate serde_json;

fn main() {
    UIComposer::run(Window(App()))
}

/*

The idea for this app is a text encoding and decoding tool;

*/

fn App() -> impl UI {
    let state = Mutable::new(String::new());

    let l0 = Label("Write some JSON to format!");
    let t1 = TextEdit(state.clone());
    let b2 = Button(Label("Format"), state.effect(json_format_quick));

    (Flex2! { 3;
        [_] l0,
        [1.0] t1,
        [_] b2
    })
    .with_vertical_flow()
}

fn json_format_quick(input: &mut String) {
    let mut parse = move || -> Option<()> {
        let parsed: Value = serde_json::from_str(input).ok()?;
        let mut bytes: Vec<_> = Vec::new();
        serde_json::to_writer_pretty(&mut bytes, &parsed).ok()?;
        input.clear();
        input.push_str(String::from_utf8(bytes).unwrap().as_str());
        Some(())
    };
    parse();
}

fn TextEdit(state: Mutable<String>) -> impl UI {
    ReactiveLabel(state.clone())
}

fn ReactiveLabel(state: Mutable<String>) -> impl UI
{
    ResizableItem::new(move |hx| {
        let text_typer = Typing::new(state.clone());
        let text_view = state.signal_cloned().map(move |s| Text(hx.rect.expand_from_center(-10.0, -10.0, -10.0, -10.0), s, vek::Rgb::black()));

        ui_composer::items! (
            Graphic::new(hx.rect, Rgb::gray(0.9)),
            React(text_view),
            text_typer
        )
    })
}