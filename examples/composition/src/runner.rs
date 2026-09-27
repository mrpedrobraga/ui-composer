use glamour::{Point2, Rect, Size2};

use crate::element::{ElementEffect, ParentHints, Ui};
use std::task::Poll;

pub struct ElementEffectHandler {}

pub struct InitializationResources {
    pub secret_key: String,
}

#[pin_project::pin_project(project = ElementsRunnerProj)]
pub struct Runner<U>
where
    U: Ui,
{
    #[pin]
    ui: U,
    first_time: bool,
}

impl<U> Runner<U>
where
    U: Ui,
{
    pub fn new(ui: U) -> Self {
        let mut new_runner = Self {
            ui,
            first_time: true,
        };

        new_runner.update();

        new_runner
    }

    pub fn update(&mut self) {
        let r = InitializationResources {
            secret_key: "[EXPENSIVE RUNTIME RESOURCE]".to_string(),
        };
        let hx = ParentHints {
            rect: Rect::new(Point2::new(0.0, 0.0), Size2::new(64.0, 64.0)),
        };

        self.ui.plan(hx, &r);
    }
}

impl<U> Future for Runner<U>
where
    U: Ui,
{
    type Output = ();

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let ElementsRunnerProj { mut ui, first_time } = self.project();

        // println!("[Runner] Polling.");

        if *first_time {
            let mut e_handler = ElementEffectHandler {};
            let effects = ui.effect();
            effects.apply(&mut e_handler);
            *first_time = true;
        }

        let r = InitializationResources {
            secret_key: "[EXPENSIVE RUNTIME RESOURCE]".to_string(),
        };
        let hx = ParentHints {
            rect: Rect::new(Point2::new(0.0, 0.0), Size2::new(64.0, 64.0)),
        };

        let poll = match ui.as_mut().poll_change(cx, &r, hx) {
            std::task::Poll::Ready(Some(_)) => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(()),
            Poll::Pending => Poll::Pending,
        };

        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("-----------------------");
        let mut e_handler = ElementEffectHandler {};
        let effects = ui.effect();
        effects.apply(&mut e_handler);
        println!("-----------------------");

        poll
    }
}
