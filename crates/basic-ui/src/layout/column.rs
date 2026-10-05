use core::f32;
use std::marker::PhantomData;

use ui_composer_core::app::composition::{
    algebra::Combine,
    elements::{Blueprint, Element, Environment},
    layout::{
        hints::{ChildHints, ParentHints},
        Ui,
    },
};
use ui_composer_math::prelude::{Rect, Size2, Vector2};

/// A vertical, writing order stack of items.
///
/// ### Sizing
/// The height of the container is the sum of the heights
/// of the items inside (accounting for gap).
///
/// The width of the container is the max width between the items.
/// TODO: Allow to take more than two items.
pub fn Column<Env, A, B>((item_a, item_b): (A, B)) -> ColumnContainer<Env, A, B> {
    ColumnContainer {
        item_a,
        item_b,
        gap: 0.0,
        __item_a_hints_cache: ChildHints::default(),
        __item_b_hints_cache: ChildHints::default(),
        __marker: PhantomData,
    }
}

#[pin_project::pin_project]
pub struct ColumnContainer<Env, A, B> {
    #[pin]
    pub item_a: A,
    #[pin]
    pub item_b: B,
    pub gap: f32,
    __item_a_hints_cache: ChildHints,
    __item_b_hints_cache: ChildHints,
    __marker: PhantomData<Env>,
}

impl<Env, A, B> ColumnContainer<Env, A, B> {
    /// Adds some spacing between elements.
    pub fn with_gap(self, gap: f32) -> Self {
        Self { gap, ..self }
    }
}

impl<Env, A, B> Ui<Env> for ColumnContainer<Env, A, B>
where
    A: Ui<Env>,
    B: Ui<Env>,
    Env: Environment,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn prepare(
        &mut self,
        parent_hints: ParentHints,
    ) -> ui_composer_core::app::composition::layout::hints::ChildHints {
        let inner_hints = ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin,
                Size2::new(parent_hints.rect.size.width, f32::INFINITY),
            ),
            ..parent_hints
        };
        let a = self.item_a.prepare(inner_hints);
        let b = self.item_b.prepare(inner_hints);

        self.__item_a_hints_cache = a;
        self.__item_b_hints_cache = b;

        let minimum_size = Size2::new(
            a.minimum_size.width.max(b.minimum_size.width), // Max width
            a.minimum_size.height + self.gap + b.minimum_size.height, // Min height with gap
        );
        ChildHints { minimum_size }
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        // TODO: Maybe split `resources`?

        self.item_a.place(
            ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin,
                    Size2::new(
                        parent_hints.rect.size.width,
                        self.__item_a_hints_cache.minimum_size.height,
                    ),
                ),
                ..parent_hints
            },
            resources,
        );

        self.item_b.place(
            ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin.translate(Vector2::new(
                        0.0,
                        self.__item_a_hints_cache.minimum_size.height + self.gap,
                    )),
                    Size2::new(
                        parent_hints.rect.size.width,
                        self.__item_b_hints_cache.minimum_size.height,
                    ),
                ),
                ..parent_hints
            },
            resources,
        );
    }

    fn effect(
        &self,
    ) -> (
        <<A::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect,
        <<B::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect,
    ) {
        (self.item_a.effect(), self.item_b.effect())
    }

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        Combine::combine(self.item_a.propagate(event).await, self.item_b.propagate(event).await)
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        Combine::combine(
            this.item_a.poll_change(cx, resources, ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin,
                    Size2::new(
                        parent_hints.rect.size.width,
                        this.__item_a_hints_cache.minimum_size.height,
                    ),
                ),
                ..parent_hints
            }),
            this.item_b.poll_change(cx, resources, ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin.translate(Vector2::new(
                        0.0,
                        this.__item_a_hints_cache.minimum_size.height + *this.gap,
                    )),
                    Size2::new(
                        parent_hints.rect.size.width,
                        this.__item_b_hints_cache.minimum_size.height,
                    ),
                ),
                ..parent_hints
            }),
        )
    }
}
