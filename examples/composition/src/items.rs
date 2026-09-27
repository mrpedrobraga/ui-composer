use crate::{
    element::{effects::LogEffect, Blueprint, Element, ParentHints, Ui},
    runner::InitializationResources,
};

#[pin_project::pin_project]
pub struct Resizable<B, F>
where
    B: Blueprint,
    F: FnMut(ParentHints) -> B,
{
    #[pin]
    elements: Option<B::Output>,
    maker: F,
}

#[pin_project::pin_project]
pub struct Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    #[pin]
    future: Fut,
    #[pin]
    ui: Option<U>,
    map: Option<Map>,
}

impl<U, Fut, Map> Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    pub fn new(future: Fut, map: Map) -> Self {
        Self {
            future,
            ui: None,
            map: Some(map),
        }
    }
}

impl<U, Fut, Map> Ui for Await<U, Fut, Map>
where
    U: Ui,
    Fut: Future,
    Map: FnOnce(Fut::Output) -> U,
{
    type Blueprint = Option<U::Blueprint>;

    fn plan(&mut self, parent_hints: ParentHints, resources: &InitializationResources) {
        if let Some(inner) = &mut self.ui {
            inner.plan(parent_hints, resources);
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect {
        self.ui.as_ref().map(|inner| inner.effect())
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &InitializationResources,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let mut this = self.project();

        /* The future has not yet yielded! */
        // To satisfy FnOnce, we `take` the map here.
        if let Some(map) = this.map.take() {
            let fut_poll = this.future.poll(cx);

            match fut_poll {
                std::task::Poll::Ready(value) => {
                    let mut inner_ui = map(value);
                    /* TODO: Plan with the right resources! */
                    inner_ui.plan(parent_hints.clone(), resources);

                    this.ui.set(Some(inner_ui));
                    // Safe to unwrap because we just set it, duh.
                    return this
                        .ui
                        .as_pin_mut()
                        .unwrap()
                        .poll_change(cx, resources, parent_hints);
                }
                std::task::Poll::Pending => {
                    // Put the mapper back because it wasn't used hehe
                    *this.map = Some(map);
                }
            }
        }
        /* The future has yielded! */
        else {
            if let Some(element) = this.ui.as_pin_mut() {
                let inner_poll = element.poll_change(cx, resources, parent_hints);

                return inner_poll;
            } else {
                return std::task::Poll::Ready(None);
            }
        }

        std::task::Poll::Pending
    }
}

#[allow(non_snake_case)]
pub fn Text(content: String) -> TextBlueprint {
    TextBlueprint(content)
}

pub struct TextBlueprint(pub String);

pub struct TextElement(pub String, TextResources);

pub struct TextResources {
    content: String,
}

impl<B, F> Resizable<B, F>
where
    B: Blueprint,
    F: FnMut(ParentHints) -> B,
{
    pub fn new(maker: F) -> Self {
        Self {
            elements: None,
            maker,
        }
    }
}

impl<B, F> Ui for Resizable<B, F>
where
    B: Blueprint,
    F: FnMut(ParentHints) -> B,
{
    type Blueprint = B;

    fn plan(&mut self, parent_hints: ParentHints, resources: &InitializationResources) {
        let new_blueprint = (self.maker)(parent_hints);
        if let Some(elements) = &mut self.elements {
            elements.update(new_blueprint, resources);
        } else {
            self.elements = Some(new_blueprint.make(resources))
        }
    }

    fn effect(&self) -> <<Self::Blueprint as Blueprint>::Output as Element>::Effect {
        self.elements.as_ref().unwrap().effect()
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        _: &InitializationResources,
        _: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.elements.poll_change(cx)
    }
}

impl Blueprint for TextBlueprint {
    type Output = TextElement;

    fn make(self, resources: &crate::runner::InitializationResources) -> Self::Output {
        TextElement(
            self.0,
            TextResources {
                content: resources.secret_key.clone(),
            },
        )
    }
}

impl Element for TextElement {
    type Effect = LogEffect;
    type Blueprint = TextBlueprint;

    fn update(&mut self, blueprint: Self::Blueprint, _: &InitializationResources) {
        /* Updates the content, but the `TextResources` remains the same yay! */
        self.0 = blueprint.0;
    }

    fn effect(&self) -> Self::Effect {
        LogEffect(format!("{} - ({})", self.0, self.1.content))
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        _: &mut std::task::Context,
    ) -> std::task::Poll<Option<()>> {
        std::task::Poll::Ready(None)
    }
}

#[macro_export]
macro_rules! list {
    ($a:expr $(,)?) => { $a };
    ($a:expr, $b:expr) => {($a, $b)};
    ($a:expr, $($rest:tt)*) => {
        ($a, list!($($rest)*))
    };
}
