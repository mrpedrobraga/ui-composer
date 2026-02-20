# UI Composer Mockup

```rust
use ui_composer::prelude::*;
use ui_composer::runners::tui::Terminal;

fn main() {
	let counter = State::new(0);

	UIComposer::run_tui(Terminal(Center(Counter(counter))))
}

fn Counter(counter: State<i32>) -> impl UI {
	let label = react!( Label("Counter: {}", *counter) );
	let decr = Button("Take 1", fx!( *counter -= 1 ));
	let incr = Button("Add 1", fx!( *counter += 1 ));
	
	flex(list![ item(label), item(decr), item(incr) ])
}

fn Label(text: impl Signal<Item = String>) -> impl UI {
	let text = text.broadcast();
	
	ItemBox::new(move |cx| {
		react!( Text(*text, cx.rect, Color::white()) )
	})
	.with_minimum_size(Extent2::new(15.0, 1.0))
}

fn Button(label: impl UI, effect: impl Effect + 'static) -> impl UI {
	let hover_state = State::empty();

	ItemBox::new(move |cx| {
		let hover = Hover::new(cx.rect, hover_state);
		let tap = Tap::new(cx.rect, effect);

		let rect = react!(
			if *hover_state {
				Graphic::new(cx.rect, Color::gray(0.3))
			} else {
				Graphic::new(cx.rect, Color::gray(0.0))
			}
		);
	});
}
```