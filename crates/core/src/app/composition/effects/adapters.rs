use ::std::task::Poll;

use ::either::Either;
use ::ui_composer_input::event::Event;

use crate::app::composition::{algebra::Combine, elements::Environment, layout::Ui};

use super::future::Await;

#[pin_project::pin_project]
pub struct WithEmptyState<A, E>(#[pin] pub A, #[pin] pub E);

impl<Env, U, Fut, Map, E> Ui<Env> for WithEmptyState<Await<Env, U, Fut, Map>, E>
where
    Env: Environment + Send,
    U: Ui<Env>,
    Fut: Future + Send,
    Map: FnOnce(Fut::Output) -> U + Send,
    E: Ui<Env>,
{
    type Blueprint = Either<U::Blueprint, E::Blueprint>;

    fn prepare(
        &mut self,
        expected_parent_hints: crate::app::composition::layout::hints::ParentHints,
    ) -> crate::app::composition::layout::hints::ChildHints {
        if let Some(ui) = &mut self.0.ui {
            ui.prepare(expected_parent_hints)
        } else {
            self.1.prepare(expected_parent_hints)
        }
    }

    fn place(
        &mut self,
        parent_hints: crate::app::composition::layout::hints::ParentHints,
        resources: &<Env as Environment>::BlueprintResources<'_>,
    ) {
        if let Some(ui) = &mut self.0.ui {
            ui.place(parent_hints, resources)
        } else {
            self.1.place(parent_hints, resources)
        }
    }

    fn effect(&self) -> <<Self::Blueprint as crate::prelude::Blueprint<Env>>::Output as crate::prelude::Element<Env>>::Effect{
        if let Some(ui) = &self.0.ui {
            Either::Left(ui.effect())
        } else {
            Either::Right(self.1.effect())
        }
    }

    async fn propagate(&mut self, event: &mut Event) -> bool {
        if let Some(ui) = &mut self.0.ui {
            ui.propagate(event).await
        } else {
            self.1.propagate(event).await
        }
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: crate::app::composition::layout::hints::ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        let mut awaiter = this.0.project();

        /* The future has not yet yielded! */
        // To satisfy FnOnce, we `take` the map here.
        if let Some(map) = awaiter.map.take() {
            let fut_poll = awaiter.future.poll(cx);

            match fut_poll {
                Poll::Ready(value) => {
                    let mut new_ui = map(value);
                    new_ui.place(parent_hints, resources);
                    awaiter.ui.set(Some(new_ui));
                    Poll::Ready(Some(()))
                }
                Poll::Pending => {
                    // Put the mapper back because it wasn't used hehe
                    *awaiter.map = Some(map);

                    let empty_state_poll = this.1.poll_change(cx, resources, parent_hints);
                    Combine::combine(Poll::Pending, empty_state_poll)
                }
            }
        }
        /* The future has yielded, so we just pass forth the value of the inner poll! */
        else {
            if let Some(element) = awaiter.ui.as_pin_mut() {
                element.poll_change(cx, resources, parent_hints)
            } else {
                Poll::Ready(None)
            }
        }
    }
}
