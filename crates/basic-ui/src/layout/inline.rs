use {
    crate::primitives::text::Text,
    ui_composer_core::app::composition::layout::{
        LayoutItem,
        hints::{ChildHints, ParentHints},
    },
    ui_composer_geometry::RectExt,
    vek::{Extent2, Rect, Rgba, Vec2},
};

type Offset = u32;

// --- Contexts ---

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

pub struct MeasureContext {
    pub offset: Vec2<Offset>,
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
        let size = inner_hints.natural_size;
        let (w, h) = (size.w as Offset, size.h as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_rect.w {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.offset;
        cx.offset.x += w + cx.inline_gap;

        let size = Extent2::new(size.w, size.h);
        let rect = Rect::new(pos.x as f32, pos.y as f32, size.w, size.h)
            .translated(cx.container_rect.position().as_());

        self.0.place(ParentHints { rect, ..hints })
    }

    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        let inner_hints = self.0.prepare(hints);
        let size = inner_hints.natural_size;
        let (w, h) = (size.w as Offset, size.h as Offset);

        if cx.offset.x > 0 && cx.offset.x + w > cx.container_width {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        cx.offset.x += w + cx.inline_gap;
        cx.max_width_reached = cx.max_width_reached.max(cx.offset.x);
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
        // PASS 1: Find the Intrinsic Minimum Width
        // By setting width to 0, we force a wrap before every single word/item.
        // The max_width_reached will flawlessly capture the widest unbreakable element.
        let mut min_w_cx = MeasureContext {
            container_width: 0,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vec2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut min_w_cx, parent_hints);
        let true_min_w = min_w_cx.max_width_reached;

        // PASS 2: Find the Minimum Height at that Width
        // Now that we know the container will never be smaller than `true_min_w`,
        // we run it again to see how many short words can pack into that width.
        let mut min_h_cx = MeasureContext {
            container_width: true_min_w,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vec2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut min_h_cx, parent_hints);
        let true_min_h = min_h_cx.offset.y + min_h_cx.max_line_height;

        // PASS 3: Calculate Natural Size (fit-content)
        // We use the parent's actual suggestion to find the preferred layout.
        let mut nat_cx = MeasureContext {
            container_width: parent_hints.rect.w as Offset,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            offset: Vec2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut nat_cx, parent_hints);
        let nat_w = nat_cx.max_width_reached;
        let nat_h = nat_cx.offset.y + nat_cx.max_line_height;

        ChildHints {
            minimum_size: Extent2::new(true_min_w as f32, true_min_h as f32),
            natural_size: Extent2::new(nat_w as f32, nat_h as f32),
        }
    }

    fn place(&mut self, hints: ParentHints) -> Self::Blueprint {
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
