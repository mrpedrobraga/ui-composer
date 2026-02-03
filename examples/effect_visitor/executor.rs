use std::any::Any;
use futures_signals::signal::Signal;
use pin_project::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};
use crate::elements::{DummyEnvironment, Element, ElementEffect};

/// Has a reference to a runner, serving as an Executor for its [`Future`]s and [`Signal`]s.
#[pin_project(project=AsyncExecutorProj)]
pub struct DummyExecutor<A> {
    #[pin]
    pub element: A,
}

impl<A> Signal for DummyExecutor<A> where A: Element<DummyEnvironment> {
    type Item = ();

    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let AsyncExecutorProj { element } = self.project();
        element.poll(cx, &DummyEnvironment())
    }
}

pub trait EffectHandler {
    fn handle<E>(&mut self, event: E) where E: 'static + ElementEffect;
}

#[test]
fn test_effect_handler() {
    pub struct DummyEffect;
    impl ElementEffect for DummyEffect {}

    pub struct NullEffect;
    impl ElementEffect for NullEffect {}

    pub struct DummyEffectHandler {}

    impl EffectHandler for DummyEffectHandler {
        fn handle<E>(&mut self, effect: E) where E: 'static + ElementEffect {
            let effect: &dyn ElementEffect = &effect;

            if let Some(dummy) = effect.downcast_ref::<DummyEffect>() {
                println!("Holy shit!")
            }
        }
    }

    let a = DummyEffect;
    let b = DummyEffect;
    let c = NullEffect;
    //let effects = (a, (b, c));

    let mut handler = DummyEffectHandler {};
    handler.handle(a);
}