use {
    crate::primitives::text::Text,
    ui_composer_core::app::composition::layout::{
        LayoutItem, hints::ParentHints,
    },
    ui_composer_geometry::RectExt,
    vek::{Extent2, Rect, Rgba, Vec2},
};

type Offset = u32;

pub struct InlineContext {
    pub offset: Vec2<Offset>,
    pub container_rect: Rect<Offset, Offset>,
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

pub fn inline<T>(item: T) -> InlineAdapter<T> {
    InlineAdapter(item)
}
pub struct InlineAdapter<T>(pub T);

impl<T: LayoutItem> InlineItem for InlineAdapter<T> {
    type Blueprint = T::Blueprint;

    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprint {
        let size = self.0.get_natural_size();
        let (w, h) = (size.w as Offset, size.h as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_rect.w {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.offset;
        cx.offset.x += w + cx.inline_gap;

        let rect = Rect::new(pos.x as f32, pos.y as f32, size.w, size.h)
            .translated(cx.container_rect.position().as_());
        self.0.lay(ParentHints { rect, ..hints })
    }
}

pub struct MonospaceText(pub String, pub Rgba<f32>);

impl InlineItem for MonospaceText {
    type Blueprint = Vec<Text>;

    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        _: ParentHints,
    ) -> Self::Blueprint {
        let word_spacing = 1;
        let mut words_with_pos = Vec::new();
        let words = self.0.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.offset.x > 0
                && cx.offset.x + word_spacing + len > cx.container_rect.w
            {
                cx.new_line();
            }

            if cx.offset.x > 0 {
                cx.offset.x += word_spacing;
            }

            words_with_pos.push(
                Text()
                    .with_text(word.to_string())
                    .with_rect(
                        Rect::new(
                            cx.offset.x as f32,
                            cx.offset.y as f32,
                            len as f32,
                            1.0,
                        )
                        .translated(cx.container_rect.position().as_()),
                    )
                    .with_color(self.1),
            );
            cx.max_line_height = cx.max_line_height.max(1);
            cx.offset.x += len;
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

pub struct LinewiseFlow<T: InlineItemList> {
    pub items: T,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl<T: InlineItemList + Send> LayoutItem for LinewiseFlow<T> {
    type Blueprint = T::Blueprints;

    fn get_natural_size(&self) -> Extent2<f32> {
        Extent2::default() // Simplified for brevity
    }

    fn get_minimum_size(&self) -> Extent2<f32> {
        Extent2::default()
    }

    fn lay(&mut self, hints: ParentHints) -> Self::Blueprint {
        let mut cx = InlineContext {
            container_rect: hints.rect.as_(),
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vec2::new(0, 0),
        };

        self.items.allocate(&mut cx, hints)
    }
}

pub fn linewise_flow<T: InlineItemList>(items: T) -> LinewiseFlow<T> {
    LinewiseFlow {
        items,
        inline_gap: 0,
        cross_axis_gap: 0,
    }
}
