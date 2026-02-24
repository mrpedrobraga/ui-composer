#![allow(unused)]

use {std::marker::PhantomData, ui_composer_derive_ui::ToDefaultUi};

#[derive(ToDefaultUi)]
#[container(Container)]
struct User {
    name: String,
    #[to_ui(DummyEdit2)]
    #[props(foo = 4.0, bar = 5.0)]
    description: String,
    #[wrap(Wrapper(baz = 4.0, qux = 2.0))]
    subscribed: bool,
    #[ui_exclude]
    other: i32,
}

#[test]
fn test_to_ui() {
    let user = User {
        name: "Pedro Braga".into(),
        description: "Best Game Dev!".into(),
        subscribed: true,
        other: 4,
    };

    let ui = user.to_default_ui();

    dbg!(ui);
}

// --- Mocked versions of the traits --- //

trait ToDefaultUi {
    type Ui;
    fn to_default_ui(&self) -> Self::Ui;
}
pub trait Edit<T: ?Sized> {
    fn edit(value: &T) -> Self;
}

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

#[derive(Debug, Clone)]
pub struct DummyEdit<T>(T);
impl<T> Edit<T> for DummyEdit<T>
where
    T: Clone,
{
    fn edit(value: &T) -> Self {
        Self(value.clone())
    }
}

#[derive(Debug, Clone)]
pub struct DummyEdit2<T>(T);
impl<T> Edit<T> for DummyEdit2<T>
where
    T: Clone,
{
    fn edit(value: &T) -> Self {
        Self(value.clone())
    }
}
impl<T> DummyEdit2<T> {
    fn with_foo<U>(self, _: U) -> Self {
        self
    }
    fn with_bar<U>(self, _: U) -> Self {
        self
    }
}

#[derive(Debug, Clone)]
pub struct Wrapper<T>(T);
impl<T> Wrapper<T> {
    fn with_baz<U>(self, _: U) -> Self {
        self
    }
    fn with_qux<U>(self, _: U) -> Self {
        self
    }
}

#[derive(Debug)]
pub struct Container<Items> {
    items: Items,
}
impl<Items> Edit<Items> for Container<Items>
where
    Items: Clone,
{
    fn edit(value: &Items) -> Self {
        Container {
            items: value.clone(),
        }
    }
}

impl ToDefaultUi for String {
    type Ui = DummyEdit<Self>;

    fn to_default_ui(&self) -> Self::Ui {
        DummyEdit::edit(self)
    }
}
impl ToDefaultUi for bool {
    type Ui = DummyEdit<Self>;

    fn to_default_ui(&self) -> Self::Ui {
        DummyEdit::edit(self)
    }
}
