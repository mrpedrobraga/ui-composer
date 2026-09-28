use async_std::task::block_on;
use futures_signals::signal::Map;
use futures_signals::signal::Mutable;
use futures_signals::signal::MutableSignal;
use futures_signals::signal::SignalExt;
use std::fmt::Formatter;
use ui_composer_view_macro::view;

macro_rules! list {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, ::ui_composer::list!($($rest)*))
    };
}

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
    pub fn with_on_click<NewEffect>(self, eff: NewEffect) -> ButtonBlueprint<Label, NewEffect> {
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
        flex {vertical_layout} [
            Label (( "Hello, world!" ))
            row [
                Label (( "Click me:" ))
                Button { on_click: || println!("Hello!") }
                    Label (( "Click me!" ))
            ]
        ]
    };
    dbg!(_ui);

    let Add = |a, b| a + b;
    let Mul = |a, b| a * b;

    let sum = view! {
        Add (
            (1)
            Mul (
                (2)
                (3)
            )
        )
    };
    dbg!(sum);

    let me_button = view! { Button {on_click: || println!("I was clicked!")} ((())) };
    me_button.trigger();
}

/// Dummy trait so the tests work.
/// In practice, there will be an IntoBlueprint trait in scope
/// whenever you use `view` that adapts the monadic traits to implement `Blueprint`.
trait ForOf<A, F, B>
where
    F: FnMut(A) -> B,
{
    type Output;
    fn for_of(self, map: F) -> Self::Output;
}

impl<const N: usize, A, F, B> ForOf<A, F, B> for [A; N]
where
    F: FnMut(A) -> B,
{
    type Output = [B; N];

    fn for_of(self, map: F) -> Self::Output {
        self.map(map)
    }
}

impl<A, F, B> ForOf<A, F, B> for MutableSignal<A>
where
    F: FnMut(A) -> B,
    Self: futures_signals::signal::Signal<Item = A>,
{
    type Output = Map<MutableSignal<A>, F>;

    fn for_of(self, map: F) -> Self::Output {
        self.map(map)
    }
}

impl<A, F, B> ForOf<A, F, B> for Option<A>
where
    F: FnMut(A) -> B,
{
    type Output = Option<B>;

    fn for_of(self, map: F) -> Self::Output {
        self.map(map)
    }
}

pub trait IntoBlueprint {
    fn into_blueprint(self) -> Self
    where
        Self: Sized,
    {
        self
    }
}
impl<T> IntoBlueprint for T {}

pub trait WithEmptyState<E> {
    type Output;
    fn with_empty_state(self, empty_state: E) -> Self::Output;
}

#[derive(Debug)]
pub struct Meanwhile<A, E>(A, E);

impl<A, E> WithEmptyState<E> for Option<A> {
    type Output = Meanwhile<Option<A>, E>;

    fn with_empty_state(self, empty_state: E) -> Self::Output {
        Meanwhile(self, empty_state)
    }
}

#[test]
fn test_blocks() {
    /* Options */
    let option = Some(3);

    let optional = view! {
        for value of option {
            Label (( format!("The value is {}", value) ))
        } else {
            Label (("Loading..."))
        }
    };

    dbg!(optional);

    /* Collections */
    let collection = [(1, 2), (3, 4)];

    let iterated = view! {
        for (l, r) of &collection {
            Label (( format!("The tuple has {} and {}", l, r) ))
        }
    };

    dbg!(iterated);

    /* Signals */

    let message_st = Mutable::new("Hi there!");
    let message_sig = message_st.signal();

    let derived = view! {
        column [
            Label ("Message 1!")
            for message of message_sig {
                Label (( message ))
            }
        ]
    };

    // (Hacky way to listen to the signal to prove it updates right!)
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
