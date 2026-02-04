#![allow(non_snake_case)]

use ui_composer::Flex;
use ui_composer::standard::runners::wgpu::components::*;
use ui_composer::standard::runners::wgpu::pipeline::graphics::graphic::Graphic;
use ui_composer::standard::runners::wgpu::pipeline::text::Text;
use ui_composer::standard::runners::winitwgpu::prelude::*;
use ui_composer::standard::prelude::items::*;
use ui_composer::standard::prelude::process::React;
use ui_composer::standard::prelude::*;

extern crate serde_json;

fn main() {
    UIComposer::run(Window(App()))
}

fn App() -> impl UI {}

/*

// Goal is to eventually have something beautiful like this:

```rust
fn App() -> impl UI {
    let state = Mutable::new(String::new());

    ui! {
        <column>
            <item>
                <Label text="Write some JSON to format!"/>
            </item>

            <item grow>
                <TextEdit state={state.clone()}/>
            </item>

            <item>
            		<Button effect={state.effect(json_format_quick)}>
            			<Label text="Format"/>
            		</Button>
            </item>
        </column>
    }
}
```

 */