use {
    crate::primitives::text::Text,
    ui_composer_core::app::composition::layout::{
        LayoutItem, hints::ParentHints,
    },
    vek::{Extent2, Rect, Rgba, Vec2},
};

type Offset = u32;

pub struct InlineContext {
    pub offset: Vec2<Offset>,
    pub container_size: Extent2<Offset>,
    pub max_line_height: Offset,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl InlineContext {
    pub fn new_line(&mut self) {
        self.offset.y += self.max_line_height + self.cross_axis_gap;
        self.offset.x = 0;
        self.max_line_height = 1;
    }
}

pub trait InlineItem {
    type Blueprint;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprint;
}

pub struct Inline<T>(pub T);

impl<T: LayoutItem> InlineItem for Inline<T> {
    type Blueprint = T::Blueprint;

    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprint {
        let size = self.0.get_natural_size();
        let (w, h) = (size.w as Offset, size.h as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_size.w {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.offset;
        cx.offset.x += w + cx.inline_gap;

        let rect = Rect::new(pos.x as f32, pos.y as f32, size.w, size.h);
        self.0.lay(ParentHints { rect, ..hints })
    }
}

pub struct MonospaceText(pub String);

impl InlineItem for MonospaceText {
    type Blueprint = Vec<Text>;

    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        _: ParentHints,
    ) -> Self::Blueprint {
        let mut words_with_pos = Vec::new();
        let words = self.0.split_whitespace();

        for word in words {
            let s = word.to_string();
            let len = s.chars().count() as Offset;

            if cx.offset.x > 0 && cx.offset.x + len > cx.container_size.w {
                cx.new_line();
            }

            words_with_pos.push(
                Text()
                    .with_text(word.to_string())
                    .with_rect(Rect::new(
                        cx.offset.x as f32,
                        cx.offset.y as f32,
                        word.len() as f32,
                        1.0,
                    ))
                    .with_color(Rgba::white()), //TODO: Get this colour from somewhere else.
            );
            cx.max_line_height = cx.max_line_height.max(1);
            cx.offset.x += len + cx.inline_gap;
        }
        words_with_pos
    }
}

pub trait InlineItemList {
    type Blueprints;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprints;
}

impl<A: InlineItem> InlineItemList for A {
    type Blueprints = A::Blueprint;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprints {
        self.allocate(cx, hints)
    }
}

impl<A, B> InlineItemList for (A, B)
where
    A: InlineItemList,
    B: InlineItemList,
{
    type Blueprints = (A::Blueprints, B::Blueprints);

    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprints {
        (self.0.allocate(cx, hints), self.1.allocate(cx, hints))
    }
}

pub struct InlineFlow<T: InlineItemList> {
    pub items: T,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl<T: InlineItemList + Send> LayoutItem for InlineFlow<T> {
    type Blueprint = T::Blueprints;

    fn get_natural_size(&self) -> Extent2<f32> {
        Extent2::default() // Simplified for brevity
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::default()
    }

    fn lay(&mut self, hints: ParentHints) -> Self::Blueprint {
        let mut cx = InlineContext {
            container_size: hints.rect.extent().as_(),
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vec2::new(hints.rect.x as Offset, hints.rect.y as Offset),
        };

        self.items.allocate(&mut cx, hints)
    }
}

pub fn inline_flow<T: InlineItemList>(items: T) -> InlineFlow<T> {
    InlineFlow {
        items,
        inline_gap: 0,
        cross_axis_gap: 0,
    }
}
