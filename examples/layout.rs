#![allow(non_snake_case)]
use ui_composer::prelude::functions::weighted_division_with_minima;
use ui_composer::prelude::*;

fn main() {
    let flex = Flex(
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 111.0 / 255.0, 145.0 / 255.0)),
            1.0,
        ),
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 199.0 / 255.0, 95.0 / 255.0)),
            2.0,
        ),
    );

    App::run(Window(Center(WithSize(Extent2::new(600.0, 300.0), flex))).with_title("Flex Example"))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem {
    Resizable::new(move |hx| hx.rect.with_color(color))
        .with_minimum_size(Extent2::new(100.0, 100.0))
}

struct FlexItem<T>(T, f32);

struct FlexContainer<A, B> {
    item_a: FlexItem<A>,
    item_b: FlexItem<B>,
}

fn Flex<A, B>(item_a: FlexItem<A>, item_b: FlexItem<B>) -> FlexContainer<A, B> {
    FlexContainer { item_a, item_b }
}

impl<A, B> LayoutItem for FlexContainer<A, B>
where
    A: LayoutItem,
    B: LayoutItem,
{
    type UINodeType = (A::UINodeType, B::UINodeType);

    fn get_natural_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.0.get_natural_size();
        let b_size = self.item_b.0.get_natural_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let a_size = self.item_a.0.get_minimum_size();
        let b_size = self.item_b.0.get_minimum_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType {
        let a_size = self.item_a.0.get_natural_size();
        let b_size = self.item_b.0.get_natural_size();

        let sizes = weighted_division_with_minima(
            parent_hints.rect.w,
            &[self.item_a.1, self.item_b.1],
            &[a_size.w, b_size.w],
        );

        let a = self.item_a.0.lay(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                sizes[0],
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        let b = self.item_b.0.lay(ParentHints {
            rect: Rect::new(
                sizes[0] + parent_hints.rect.x,
                parent_hints.rect.y,
                sizes[1],
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        (a, b)
    }
}
