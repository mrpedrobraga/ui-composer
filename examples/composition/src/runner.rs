use crate::element::{ElementEffect, Ui};
use std::task::Poll;

pub struct ElementEffectHandler {}

pub struct InitializationResources {
    pub secret_key: String,
}

#[pin_project::pin_project(project = ElementsRunnerProj)]
pub struct Runner<A>
where
    A: Ui,
{
    #[pin]
    pub elements: A,
    first_time: bool,
}

impl<A> Runner<A>
where
    A: Ui,
{
    pub fn new(mut elements: A) -> Self {
        let resources = InitializationResources {
            secret_key: "fee foh fi fum".to_string(),
        };

        elements.initialize(&resources);

        Self {
            elements,
            first_time: true,
        }
    }
}

impl<A> Future for Runner<A>
where
    A: Ui,
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
