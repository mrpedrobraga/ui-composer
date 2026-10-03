use crate::primitives::graphic::{Graphic, RenderQuad};
use ::ui_composer_math::palette::rgb::Rgba;

use {
    crate::primitives::text::{RenderText, Text},
    std::{
        marker::PhantomData,
        pin::Pin,
        task::{Context, Poll},
    },
    ui_composer_core::app::composition::{
        algebra::Combine,
        elements::{Blueprint, Element, Environment},
        layout::{
            hints::{ChildHints, ParentHints},
            Ui,
        },
    },
    ui_composer_math::prelude::{Rect, Size2, Vector2},
    ui_composer_platform_tui::runner::{TerminalBlueprintResources, TerminalEnvironment},
    ui_composer_platform_winit::runner::{DesktopEnvironment, DesktopResources},
};

type Offset = u32;

// --- Contexts ---

pub struct InlineContext {
    pub local_offset: Vector2<Offset>,
    pub max_line_height: Offset,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
}

impl InlineContext {
    pub fn new_line(&mut self) {
        self.local_offset.y += self.max_line_height + self.cross_axis_gap;
        self.local_offset.x = 0;
        self.max_line_height = 1;
    }
}

pub struct MeasureContext {
    pub local_offset: Vector2<Offset>,
    pub container_width: Offset,
    pub max_line_height: Offset,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
    pub max_width_reached: Offset, // Tracks the widest line encountered
}

impl MeasureContext {
    pub fn new_line(&mut self) {
        self.local_offset.y += self.max_line_height + self.cross_axis_gap;
        self.local_offset.x = 0;
        self.max_line_height = 1;
    }
}

// --- Traits ---

pub trait InlineItem<Env>
where
    Env: Environment,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    );
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);

    type Blueprint: Blueprint<Env>;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect;

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>>;
}

pub trait InlineItemList<Env>
where
    Env: Environment,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    );
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);

    type Blueprint: Blueprint<Env>;
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect;

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &Env::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>>;
}

// --- Implementations ---

pub fn inline<Env, U>(ui: U) -> InlineAdapter<Env, U> {
    InlineAdapter(ui, PhantomData)
}

#[pin_project::pin_project]
pub struct InlineAdapter<Env, T>(#[pin] pub T, PhantomData<Env>);

impl<Env, U> InlineItem<Env> for InlineAdapter<Env, U>
where
    U: Ui<Env>,
    Env: Environment,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    ) {
        let inner_hints = self.0.prepare(hints);
        let size = inner_hints.minimum_size;
        let (w, h) = (size.width as Offset, size.height as Offset);

        if cx.local_offset.x > 0 && cx.local_offset.x + w > hints.rect.width() as u32 {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        let pos = cx.local_offset;
        cx.local_offset.x += w + cx.inline_gap;

        let size = Size2::new(w, h);
        let rect = Rect::new(pos.to_point(), size).translate(hints.rect.origin.as_().into());

        self.0.place(
            ParentHints {
                rect: rect.as_::<f32>(),
                ..hints
            },
            resources,
        );
    }

    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        let inner_hints = self.0.prepare(hints);
        let size = inner_hints.minimum_size;
        let (w, h) = (size.width as Offset, size.height as Offset);

        if cx.local_offset.x > 0 && cx.local_offset.x + w > cx.container_width {
            cx.new_line();
        }

        cx.max_line_height = cx.max_line_height.max(h);
        cx.local_offset.x += w + cx.inline_gap;
        cx.max_width_reached = cx.max_width_reached.max(cx.local_offset.x);
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let this = self.project();
        this.0.poll_change(cx, resources, parent_hints)
    }

    type Blueprint = U::Blueprint;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.0.effect()
    }
}

#[allow(non_snake_case)]
pub fn MonospaceText(text: String, color: Rgba) -> MonospaceText {
    MonospaceText {
        text,
        color,
        allocated_texts: Vec::new(),
    }
}

pub struct MonospaceText {
    text: String,
    color: Rgba,
    allocated_texts: Vec<Text>,
}

impl InlineItem<TerminalEnvironment> for MonospaceText
where
    TerminalEnvironment: Environment,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        _: &TerminalBlueprintResources,
    ) {
        let word_spacing = 1;
        let mut text_blueprints = Vec::new();
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset.x > 0
                && cx.local_offset.x + word_spacing + len > hints.rect.width() as u32
            {
                cx.new_line();
            }

            if cx.local_offset.x > 0 {
                cx.local_offset.x += word_spacing;
            }

            text_blueprints.push(
                Text()
                    .with_text(word.to_string())
                    .with_rect(
                        // TODO: Lines might have different heights?
                        Rect::new(
                            (hints.rect.origin.as_() + cx.local_offset).as_::<f32>(),
                            Size2::new(len as f32, 1.0),
                        ),
                    )
                    .with_color(self.color),
            );
            cx.max_line_height = cx.max_line_height.max(1);
            cx.local_offset.x += len;
        }

        self.allocated_texts = text_blueprints
    }

    fn measure(&mut self, cx: &mut MeasureContext, _: ParentHints) {
        let word_spacing = 1;
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset.x > 0 && cx.local_offset.x + word_spacing + len > cx.container_width
            {
                cx.new_line();
            }

            if cx.local_offset.x > 0 {
                cx.local_offset.x += word_spacing;
            }

            cx.max_line_height = cx.max_line_height.max(1);
            cx.local_offset.x += len;
            cx.max_width_reached = cx.max_width_reached.max(cx.local_offset.x);
        }
    }

    fn poll_change(
        self: Pin<&mut Self>,
        _: &mut Context,
        _: &<TerminalEnvironment as Environment>::BlueprintResources<'_>,
        _: ParentHints,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }

    type Blueprint = Vec<Text>;

    fn effect(&self) -> Vec<RenderText> {
        self.allocated_texts.iter().map(|e| e.effect()).collect()
    }
}

impl InlineItem<DesktopEnvironment> for MonospaceText {
    fn allocate(&mut self, cx: &mut InlineContext, hints: ParentHints, _: &DesktopResources) {
        let word_spacing = 1;
        let mut words_with_pos = Vec::new();
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset.x > 0
                && cx.local_offset.x + word_spacing + len > hints.rect.width() as u32
            {
                cx.new_line();
            }

            if cx.local_offset.x > 0 {
                cx.local_offset.x += word_spacing;
            }

            words_with_pos.push(
                Text()
                    .with_text(word.to_string())
                    .with_rect(
                        // TODO: Lines might have different heights?
                        Rect::new(
                            (hints.rect.origin.as_() + cx.local_offset).as_::<f32>(),
                            Size2::new(len as f32, 1.0),
                        ),
                    )
                    .with_color(self.color),
            );
            cx.max_line_height = cx.max_line_height.max(1);
            cx.local_offset.x += len;
        }

        self.allocated_texts = words_with_pos
    }

    fn measure(&mut self, cx: &mut MeasureContext, _: ParentHints) {
        let word_spacing = 1;
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset.x > 0 && cx.local_offset.x + word_spacing + len > cx.container_width
            {
                cx.new_line();
            }

            if cx.local_offset.x > 0 {
                cx.local_offset.x += word_spacing;
            }

            cx.max_line_height = cx.max_line_height.max(1);
            cx.local_offset.x += len;
            cx.max_width_reached = cx.max_width_reached.max(cx.local_offset.x);
        }
    }

    fn poll_change(
        self: Pin<&mut Self>,
        _: &mut Context,
        _: &<DesktopEnvironment as Environment>::BlueprintResources<'_>,
        _: ParentHints,
    ) -> Poll<Option<()>> {
        Poll::Ready(None)
    }

    type Blueprint = Vec<Graphic>;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<DesktopEnvironment>>::Output as Element<DesktopEnvironment>>::Effect{
        self.allocated_texts
            .iter()
            .map(|e| {
                RenderQuad(
                    Rect {
                        origin: (e.rect.origin.to_vector()
                            * DesktopEnvironment::TILE_SIZE.to_vector())
                        .to_point(),
                        // origin: e.rect.origin,
                        size: e.rect.size * DesktopEnvironment::TILE_SIZE,
                    },
                    e.color,
                )
            })
            .collect()
    }
}

impl<Env, A> InlineItemList<Env> for A
where
    A: InlineItem<Env>,
    Env: Environment,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    ) {
        InlineItem::allocate(self, cx, hints, resources)
    }
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        InlineItem::measure(self, cx, hints)
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        InlineItem::poll_change(self, cx, resources, parent_hints)
    }

    type Blueprint = A::Blueprint;

    fn effect(
        &self,
    ) -> <<<Self as InlineItemList<Env>>::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect
    {
        InlineItem::effect(self)
    }
}

impl<A, B> InlineItemList<TerminalEnvironment> for (A, B)
where
    A: InlineItemList<TerminalEnvironment>,
    B: InlineItemList<TerminalEnvironment>,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &TerminalBlueprintResources,
    ) {
        self.0.allocate(cx, hints, resources);
        self.1.allocate(cx, hints, resources);
    }
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        self.0.measure(cx, hints);
        self.1.measure(cx, hints);
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &TerminalBlueprintResources,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_change(cx, resources, parent_hints);
        let poll_b = pinned_b.poll_change(cx, resources, parent_hints);

        Combine::combine(poll_a, poll_b)
    }

    type Blueprint = (A::Blueprint, B::Blueprint);

    fn effect(
        &self,
    ) -> <<Self::Blueprint as Blueprint<TerminalEnvironment>>::Output as Element<
        TerminalEnvironment,
    >>::Effect {
        (self.0.effect(), self.1.effect())
    }
}

impl<A, B> InlineItemList<DesktopEnvironment> for (A, B)
where
    A: InlineItemList<DesktopEnvironment>,
    B: InlineItemList<DesktopEnvironment>,
{
    fn allocate(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &DesktopResources,
    ) {
        self.0.allocate(cx, hints, resources);
        self.1.allocate(cx, hints, resources);
    }
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
        self.0.measure(cx, hints);
        self.1.measure(cx, hints);
    }

    fn poll_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
        resources: &DesktopResources,
        parent_hints: ParentHints,
    ) -> Poll<Option<()>> {
        let (pinned_a, pinned_b) = {
            let mut_ref = unsafe { self.get_unchecked_mut() };
            let (a, b) = mut_ref;

            let a = unsafe { Pin::new_unchecked(a) };
            let b = unsafe { Pin::new_unchecked(b) };

            (a, b)
        };

        let poll_a = pinned_a.poll_change(cx, resources, parent_hints);
        let poll_b = pinned_b.poll_change(cx, resources, parent_hints);

        Combine::combine(poll_a, poll_b)
    }

    type Blueprint = (A::Blueprint, B::Blueprint);

    fn effect(&self) -> <<Self::Blueprint as Blueprint<DesktopEnvironment>>::Output as Element<DesktopEnvironment>>::Effect{
        (self.0.effect(), self.1.effect())
    }
}

// impl<Env, A, B> InlineItemList<Env> for (A, B)
// where
//     A: InlineItemList<Env>,
//     B: InlineItemList<Env>,
//     Env: Environment,
// {
//     fn allocate(&mut self, cx: &mut InlineContext, hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
//         self.0.allocate(cx, hints, resources);
//         self.1.allocate(cx, hints, resources);
//     }
//     fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints) {
//         self.0.measure(cx, hints);
//         self.1.measure(cx, hints);
//     }

//     fn poll_change(
//         self: Pin<&mut Self>,
//         cx: &mut Context,
//         resources: &<Env as Environment>::BlueprintResources<'_>,
//         parent_hints: ParentHints,
//     ) -> Poll<Option<()>> {
//         let (pinned_a, pinned_b) = {
//             let mut_ref = unsafe { self.get_unchecked_mut() };
//             let (a, b) = mut_ref;

//             let a = unsafe { Pin::new_unchecked(a) };
//             let b = unsafe { Pin::new_unchecked(b) };

//             (a, b)
//         };

//         let poll_a = pinned_a.poll_change(cx, resources, parent_hints);
//         let poll_b = pinned_b.poll_change(cx, resources, parent_hints);

//         Semigroup::combine(poll_a, poll_b)
//     }
// }

// --- The Flow Container ---

#[pin_project::pin_project]
pub struct LinewiseFlow<Env, Items>
where
    Items: InlineItemList<Env>,
    Env: Environment,
{
    #[pin]
    pub items: Items,
    pub inline_gap: Offset,
    pub cross_axis_gap: Offset,
    __marker: PhantomData<Env>,
}

impl<Env, Items> Ui<Env> for LinewiseFlow<Env, Items>
where
    Items: InlineItemList<Env> + Send,
    Env: Environment,
{
    fn prepare(&mut self, parent_hints: ParentHints) -> ChildHints {
        let mut min_w_cx = MeasureContext {
            container_width: 0,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            local_offset: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut min_w_cx, parent_hints);
        let true_min_w = min_w_cx.max_width_reached;

        let mut height_whem_min_w_cx = MeasureContext {
            container_width: true_min_w,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            local_offset: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut height_whem_min_w_cx, parent_hints);
        let height_when_min_w =
            height_whem_min_w_cx.local_offset.y + height_whem_min_w_cx.max_line_height;

        ChildHints {
            minimum_size: Size2::new(true_min_w as f32, height_when_min_w as f32) * Env::TILE_SIZE,
        }
    }

    fn place(&mut self, hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        let mut cx = InlineContext {
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            max_line_height: 1,
            local_offset: Vector2::new(0, 0),
        };

        self.items.allocate(&mut cx, hints, resources);
    }

    type Blueprint = Items::Blueprint;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.items.effect()
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        this.items.poll_change(cx, resources, parent_hints)
    }
}

pub fn linewise_flow<Env, Items>(items: Items) -> LinewiseFlow<Env, Items>
where
    Items: InlineItemList<Env>,
    Env: Environment,
{
    LinewiseFlow {
        items,
        inline_gap: 0,
        cross_axis_gap: 0,
        __marker: PhantomData,
    }
}
