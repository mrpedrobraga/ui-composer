use futures_signals::signal::Signal;
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

        self.ui.place(hx, &r);
    }
}

impl<U> Signal for Runner<U>
where
    U: Ui,
{
    type Item = ();

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> Poll<Option<Self::Item>> {
        let ElementsRunnerProj { mut ui, first_time } = self.project();

        let r = InitializationResources {
            secret_key: "[EXPENSIVE RUNTIME RESOURCE]".to_string(),
        };
        let hx = ParentHints {
            rect: Rect::new(Point2::new(0.0, 0.0), Size2::new(64.0, 64.0)),
        };

        let ui_poll = ui.as_mut().poll_change(cx, &r, hx);

        if ui_poll.is_pending() {
            return Poll::Pending;
        };
        if let Poll::Ready(None) = ui_poll
            && !*first_time {
                return Poll::Ready(None);
            };

        *first_time = false;

        //print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        println!("\n-----------------------");
        let mut e_handler = ElementEffectHandler {};
        let effects = ui.effect();
        effects.apply(&mut e_handler);
        println!("-----------------------\n");

        Poll::Ready(Some(()))
    }
}
