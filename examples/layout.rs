#![allow(non_snake_case)]

use ui_composer::items;
use ui_composer::prelude::functions::weighted_division_with_minima;
use ui_composer::prelude::*;

fn main() {
    let flex = Flex(items!(
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 111.0 / 255.0, 145.0 / 255.0)),
            2.0,
        ),
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 199.0 / 255.0, 95.0 / 255.0)),
            3.0,
        ),
        FlexItem(
            Square(Rgb::new(255.0 / 255.0, 111.0 / 255.0, 95.0 / 255.0)),
            2.0,
        ),
    ));

    App::run(Window(flex).with_title("Flex Example"))
}

fn Square(color: Rgb<f32>) -> impl LayoutItem {
    Resizable::new(move |hx| {
        (
            hx.rect.with_color(color),
            Rect::new(hx.rect.x, hx.rect.y + 4.0, 200.0, 4.0).with_color(Rgb::white()),
        )
    })
    .with_minimum_size(Extent2::new(200.0, 200.0))
}

struct FlexItem<T>(T, f32);

struct FlexContainer<TItems: FlexItemCollection> {
    items: TItems,
}

fn Flex<TItems>(items: TItems) -> FlexContainer<TItems>
where
    TItems: FlexItemCollection,
{
    FlexContainer { items }
}

trait FlexItemCollection {
    type UINodeType: ItemDescriptor;

    fn get_natural_size(&self) -> Extent2<f32>;
    fn get_minimum_size(&self) -> Extent2<f32>;
    fn weight(&self) -> f32;
    fn lay(&mut self, hx: ParentHints) -> Self::UINodeType;
}

impl<A> FlexItemCollection for FlexItem<A>
where
    A: LayoutItem,
{
    type UINodeType = A::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.0.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.0.get_minimum_size()
    }

    fn weight(&self) -> f32 {
        self.1
    }

    fn lay(&mut self, hx: ParentHints) -> Self::UINodeType {
        self.0.lay(hx)
    }
}

impl<A, B> FlexItemCollection for (A, B)
where
    A: FlexItemCollection,
    B: FlexItemCollection,
{
    type UINodeType = (A::UINodeType, B::UINodeType);

    fn get_natural_size(&self) -> Extent2<f32> {
        let a_size = self.0.get_natural_size();
        let b_size = self.1.get_natural_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        let a_size = self.0.get_minimum_size();
        let b_size = self.1.get_minimum_size();

        Extent2::new(a_size.w + b_size.w, a_size.h.max(b_size.h))
    }

    fn weight(&self) -> f32 {
        self.0.weight() + self.1.weight()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType {
        let a_natural_size = self.0.get_natural_size();
        let b_natural_size = self.1.get_natural_size();

        let sizes = weighted_division_with_minima(
            parent_hints.rect.w,
            &[self.0.weight(), self.1.weight()],
            &[a_natural_size.w, b_natural_size.w],
            1.0,
        );

        let a = self.0.lay(ParentHints {
            rect: Rect::new(
                parent_hints.rect.x,
                parent_hints.rect.y,
                sizes[0],
                parent_hints.rect.h,
            ),
            ..parent_hints
        });

        let b = self.1.lay(ParentHints {
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

impl<TItems> LayoutItem for FlexContainer<TItems>
where
    TItems: FlexItemCollection + Send,
{
    type UINodeType = TItems::UINodeType;

    fn get_natural_size(&self) -> Extent2<f32> {
        self.items.get_natural_size()
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        self.items.get_minimum_size()
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::UINodeType {
        self.items.lay(parent_hints)
    }
}
