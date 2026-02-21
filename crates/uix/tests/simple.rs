use async_std::task::block_on;
use futures_signals::signal::Mutable;
use futures_signals::signal::SignalExt;
use std::fmt::Formatter;
use uix::uix as view;

/// A container which stacks its items linearly within a rectangular bound,
/// allowing some of the items to grow to fill up remaining space.
/// ```html
/// <flex>
///     <item />
///     <item />
/// </flex>
/// ```
#[allow(non_snake_case)]
pub fn flex<A, B>((item_a, item_b): (A, B)) -> FlexContainer<A, B> {
    FlexContainer {
        a: item_a,
        b: item_b,
    }
}
#[derive(Debug)]
pub struct FlexContainer<A, B> {
    pub a: A,
    pub b: B,
}
impl<A, B> FlexContainer<A, B> {
    /// Arranges items vertically instead of the default, which is horizontally.
    pub fn with_vertical_layout(self) -> Self {
        self
    }
}

/// A container which stacks its items horizontally, in line writing order.
/// ```html
/// <row>
///     <item />
///     <item />
/// </row>
/// ```
#[allow(non_snake_case)]
pub fn row<A, B>((a, b): (A, B)) -> Row<A, B> {
    Row { a, b }
}
#[derive(Debug)]
pub struct Row<A, B> {
    pub a: A,
    pub b: B,
}

/// A container which stacks its items vertically, in paragraph writing order.
/// ```html
/// <column>
///     <item />
///     <item />
/// </column>
/// ```
#[allow(non_snake_case)]
pub fn column<A, B>((a, b): (A, B)) -> Column<A, B> {
    Column { a, b }
}
#[derive(Debug)]
pub struct Column<A, B> {
    pub a: A,
    pub b: B,
}

/// A humble label, displays some text.
/// ```html
/// <Label>"Hello, world!"</Label>
/// ````
#[allow(non_snake_case)]
pub fn Label<S>(text: S) -> LabelBlueprint
where
    S: Into<String>,
{
    LabelBlueprint { text: text.into() }
}
#[derive(Debug)]
pub struct LabelBlueprint {
    pub text: String,
}

pub trait Effect {
    fn trigger(&self);
}
impl<F> Effect for F
where
    F: Fn(),
{
    fn trigger(&self) {
        self();
    }
}

/// A `Button`, which displays as a clickable `label`...
/// When the user taps, or clicks, or FOCUS+SELECT's it, it will `trigger` an effect.
#[allow(non_snake_case)]
pub fn Button<Label>(label: Label) -> ButtonBlueprint<Label, impl Effect> {
    ButtonBlueprint {
        label,
        effect: || {},
    }
}
pub struct ButtonBlueprint<Label, Eff> {
    label: Label,
    effect: Eff,
}
impl<Label, Eff> std::fmt::Debug for ButtonBlueprint<Label, Eff>
where
    Label: std::fmt::Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Button")
            .field("label", &self.label)
            .finish()
    }
}
impl<Label, Eff> ButtonBlueprint<Label, Eff>
where
    Eff: Effect,
{
    /// This effect will be [triggered](Effect::trigger) when the button is pressed.
    pub fn with_effect<NewEffect>(self, eff: NewEffect) -> ButtonBlueprint<Label, NewEffect> {
        ButtonBlueprint {
            label: self.label,
            effect: eff,
        }
    }

    pub fn trigger(&self) {
        self.effect.trigger()
    }
}

#[test]
pub fn test_simple() {
    #![allow(non_snake_case)]

    let _ui = view! {
        <flex vertical_layout>
            <Label>"Hello, world!"</Label>
            <row>
                <Label>"Click me:"</Label>
                <Button effect=|| println!("Hello!") ><Label>"Click me!"</Label></Button>
            </row>
        </flex>
    };

    let Add = |a, b| a + b;
    let Mul = |a, b| a * b;

    let sum = view! {
        <Add>
            {1}
            <Mul>
                {2}
                {3}
            </Mul>
        </Add>
    };
    dbg!(sum);

    let me_button = view! { <Button effect=|| println!("I was clicked!") >{()}</Button> };
    me_button.trigger();
}

#[test]
fn test_blocks() {
    let collection = [(1, 2), (3, 4)];

    let iterated = view! {
        @for (l, r) in &collection {
            <Label>{ format!("The tuple has {} and {}", l, r) }</Label>
        }
    };

    dbg!(iterated);

    let message_st = Mutable::new("Hi there!");
    let message_sig = message_st.signal();

    let derived = view! {
        <column>
            // This label never changes
            <Label>"Message 1!"</Label>
            // This one does in sync with `message_sig`
            @for message in message_sig {
                <Label>{message}</Label>
            }
        </column>
    };

    std::thread::spawn(move || {
        block_on(
            derived
                .b
                .for_each(|item| async move { println!("Item: {:?}", item) }),
        );
    });

    message_st.set("How are you doing?");
    std::thread::sleep(std::time::Duration::from_secs(1));
    message_st.set("Everything fine?");
    std::thread::sleep(std::time::Duration::from_secs(1));
}
