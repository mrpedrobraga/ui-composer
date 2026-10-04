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

/// A horizontal, writing order stack of items.
///
/// ### Sizing
/// The width of the container is the sum of the widths
/// of the items inside (accounting for gap).
///
/// The height of the container is the max height between the items.
/// TODO: Allow it to take more than two items.
pub fn row<Env, A, B>((item_a, item_b): (A, B)) -> RowContainer<Env, A, B> {
    RowContainer {
        item_a,
        item_b,
        gap: 0.0,
        __item_a_hints_cache: ChildHints::default(),
        __item_b_hints_cache: ChildHints::default(),
        __marker: PhantomData,
    }
}

#[pin_project::pin_project]
pub struct RowContainer<Env, A, B> {
    #[pin]
    pub item_a: A,
    #[pin]
    pub item_b: B,
    pub gap: f32,
    __item_a_hints_cache: ChildHints,
    __item_b_hints_cache: ChildHints,
    __marker: PhantomData<Env>,
}

impl<Env, A, B> RowContainer<Env, A, B> {
    pub fn with_gap(self, gap: f32) -> Self {
        Self { gap, ..self }
    }
}

impl<Env, A, B> Ui<Env> for RowContainer<Env, A, B>
where
    A: Ui<Env>,
    B: Ui<Env>,
    Env: Environment,
{
    type Blueprint = (A::Blueprint, B::Blueprint);

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let inner_hints = ParentHints {
            rect: Rect::new(
                parent_hints.rect.origin,
                Size2::new(f32::INFINITY, parent_hints.rect.size.height),
            ),
            ..parent_hints
        };
        let a = self.item_a.prepare(inner_hints);
        let b = self.item_b.prepare(inner_hints);

        self.__item_a_hints_cache = a;
        self.__item_b_hints_cache = b;

        let minimum_size = Size2::new(
            a.minimum_size.width + self.gap + b.minimum_size.width, // Min width with gap
            a.minimum_size.height.max(b.minimum_size.height),       // Max height
        );
        ChildHints { minimum_size }
    }

    fn place(&mut self, parent_hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        // TODO: Maybe "split" resources?

        self.item_a.place(
            ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin,
                    Size2::new(
                        self.__item_a_hints_cache.minimum_size.width,
                        parent_hints.rect.size.height,
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
                        self.__item_a_hints_cache.minimum_size.width + self.gap,
                        0.0,
                    )),
                    Size2::new(
                        self.__item_b_hints_cache.minimum_size.width,
                        parent_hints.rect.size.height,
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
                        this.__item_a_hints_cache.minimum_size.width,
                        parent_hints.rect.size.height,
                    ),
                ),
                ..parent_hints
            }),
            this.item_b.poll_change(cx, resources, ParentHints {
                rect: Rect::new(
                    parent_hints.rect.origin.translate(Vector2::new(
                        this.__item_a_hints_cache.minimum_size.width + *this.gap,
                        0.0,
                    )),
                    Size2::new(
                        this.__item_b_hints_cache.minimum_size.width,
                        parent_hints.rect.size.height,
                    ),
                ),
                ..parent_hints
            }),
        )
    }
}
