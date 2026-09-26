use crate::element::{Element, ElementEffect};
use std::task::Poll;

pub struct ElementEffectHandler {}

#[pin_project::pin_project(project = ElementsRunnerProj)]
pub struct ElementsRunner<E>
where
    E: Element,
{
    #[pin]
    pub elements: E,
    first_time: bool,
}

impl<E> ElementsRunner<E>
where
    E: Element,
{
    pub fn new(elements: E) -> Self {
        Self {
            elements,
            first_time: true,
        }
    }
}

impl<E> Future for ElementsRunner<E>
where
    E: Element,
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let ElementsRunnerProj {
            mut elements,
            first_time,
        } = self.project();

        // println!("[Runner] Polling.");

        if *first_time {
            let mut e_handler = ElementEffectHandler {};
            let effects = elements.effect();
            effects.apply(&mut e_handler);
            *first_time = false;
        }

        let poll = match elements.as_mut().poll_change(cx) {
            std::task::Poll::Ready(Some(_)) => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        };

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("-----------------------");
        let mut e_handler = ElementEffectHandler {};
        let effects = elements.effect();
        effects.apply(&mut e_handler);
        println!("-----------------------");

        poll
    }
}
