use {
    crate::primitives::text::Text,
    ui_composer_core::app::composition::layout::{
        LayoutItem,
        hints::{ChildHints, ParentHints},
    },
    ui_composer_math::prelude::{Rect, Size2, Srgba, Vector2},
};

type Offset = u32;

// --- Contexts ---

pub struct InlineContext {
    pub offset: Vector2<Offset>,
    pub container_rect: Rect<Offset>,
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

pub struct MeasureContext {
    pub offset: Vector2<Offset>,
    pub container_width: Offset,
    pub max_line_height: Offset,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
    pub max_width_reached: Offset, // Tracks the widest line encountered
}

impl MeasureContext {
    pub fn new_line(&mut self) {
        self.offset.y += self.max_line_height + self.cross_axis_gap;
        self.offset.x = 0;
        self.max_line_height = 1;
    }
}

// --- Traits ---

pub trait InlineItem {
    type Blueprint;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprint;
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);
}

pub trait InlineItemList {
    type Blueprints;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprints;
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);
}

// --- Implementations ---

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
        let inner_hints = self.0.prepare(hints);
        let size = inner_hints.minimum_size;
        let (w, h) = (size.width as Offset, size.height as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_rect.width() {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.offset;
        cx.offset.x += w + cx.inline_gap;

        let size = Size2::new(w, h);
        let rect = Rect::new(pos.into(), size)
            .translate(cx.container_rect.origin.into());

        self.0.place(ParentHints {
            rect: rect.as_::<f32>(),
            ..hints
        })
    }

    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        let inner_hints = self.0.prepare(hints);
        let size = inner_hints.minimum_size;
        let (w, h) = (size.width as Offset, size.height as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_width {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        cx.offset.x += w + cx.inline_gap;
        cx.max_width_reached = cx.max_width_reached.max(cx.offset.x);
    }
}

pub struct MonospaceText(pub String, pub Srgba);

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
                && cx.offset.x + word_spacing + len
                    > cx.container_rect.size.width
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
                        // TODO: Lines might have different heights?
                        Rect::new(
                            (cx.container_rect.origin + cx.offset).as_::<f32>(),
                            Size2::new(len as f32, 1.0),
                        ),
                    )
                    .with_color(self.1),
            );
            cx.max_line_height = cx.max_line_height.max(1);
            cx.offset.x += len;
        }
        words_with_pos
    }

    fn measure(&mut self, cx: &mut MeasureContext, _: ParentHints) {
        let word_spacing = 1;
        let words = self.0.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.offset.x > 0
                && cx.offset.x + word_spacing + len > cx.container_width
            {
                cx.new_line();
            }

            if cx.offset.x > 0 {
                cx.offset.x += word_spacing;
            }

            cx.max_line_height = cx.max_line_height.max(1);
            cx.offset.x += len;
            cx.max_width_reached = cx.max_width_reached.max(cx.offset.x);
        }
    }
}

impl<A: InlineItem> InlineItemList for A {
    type Blueprints = A::Blueprint;
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
    ) -> Self::Blueprints {
        InlineItem::allocate(self, cx, hints)
    }
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        InlineItem::measure(self, cx, hints)
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
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        self.0.measure(cx, hints);
        self.1.measure(cx, hints);
    }
}

// --- The Flow Container ---

pub struct LinewiseFlow<T: InlineItemList> {
    pub items: T,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl<T: InlineItemList + Send> LayoutItem for LinewiseFlow<T> {
    type Blueprint = T::Blueprints;

    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let mut min_w_cx = MeasureContext {
            container_width: 0,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut min_w_cx, parent_hints);
        let true_min_w = min_w_cx.max_width_reached;

        let mut height_whem_min_w_cx = MeasureContext {
            container_width: true_min_w,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut height_whem_min_w_cx, parent_hints);
        let height_when_min_w = height_whem_min_w_cx.offset.y
            + height_whem_min_w_cx.max_line_height;

        ChildHints {
            minimum_size: Size2::new(
                true_min_w as f32,
                height_when_min_w as f32,
            ),
        }
    }

    fn place(&mut self, hints: ParentHints) -> Self::Blueprint {
        let mut cx = InlineContext {
            container_rect: hints.rect.as_(),
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vector2::new(0, 0),
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
