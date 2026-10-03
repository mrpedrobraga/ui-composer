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
    /// The current offset relative to the inline container,
    /// represented in pixels.
    pub local_offset_px: Vector2<Offset>,
    /// The running maximum height for all the items in the current line.
    pub current_line_max_item_height: Offset,
    /// The gap between items in line.
    pub inline_gap: Offset,
    /// The gap between lines.
    pub cross_axis_gap: Offset,
}

impl InlineContext {
    pub fn new_line(&mut self) {
        self.local_offset_px.y += self.current_line_max_item_height + self.cross_axis_gap;
        self.local_offset_px.x = 0;
        self.current_line_max_item_height = 1;
    }
}

pub struct MeasureContext {
    /// The current offset relative to the inline container,
    /// represented in pixels.
    pub local_offset_px: Vector2<Offset>,
    /// The width of the container.
    pub container_width: Offset,
    /// The running maximum height for all the items in the current line.
    pub current_line_max_item_height: Offset,
    /// The gap between items in line.
    pub inline_gap: Offset,
    /// The gap between lines.
    pub cross_axis_gap: Offset,
    /// Widest line reached so far.
    pub max_width_reached: Offset,
}

impl MeasureContext {
    /// Moves to a new line and returns the "carriage" to the start, like a typewriter.
    pub fn new_line(&mut self) {
        self.local_offset_px.y += self.current_line_max_item_height + self.cross_axis_gap;
        self.local_offset_px.x = 0;
        self.current_line_max_item_height = 1;
    }
}

// --- Traits ---

pub trait InlineItem<Env>
where
    Env: Environment,
{
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    );
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);

    type Blueprint: Blueprint<Env>;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect;

    #[allow(async_fn_in_trait)]
    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool;

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
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    );
    fn measure(&mut self, cx: &mut MeasureContext, hints: ParentHints);

    type Blueprint: Blueprint<Env>;
    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect;

    #[allow(async_fn_in_trait)]
    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool;

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
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    ) {
        let inner_hints = self.0.prepare(hints);
        let item_size = inner_hints.minimum_size.as_::<Offset>();

        if cx.local_offset_px.x > 0 && cx.local_offset_px.x + item_size.width > hints.rect.width() as u32 {
            cx.new_line();
        }

        cx.current_line_max_item_height = cx.current_line_max_item_height.max(item_size.height);
        let pos = cx.local_offset_px;
        cx.local_offset_px.x += item_size.width + cx.inline_gap;

        let rect = Rect::new(pos.to_point(), item_size).translate(hints.rect.origin.as_().into());

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
        let item_size = inner_hints.minimum_size.as_::<Offset>();

        if cx.local_offset_px.x > 0 && cx.local_offset_px.x + item_size.width > cx.container_width {
            cx.new_line();
        }

        cx.current_line_max_item_height = cx.current_line_max_item_height.max(item_size.height);
        cx.local_offset_px.x += item_size.width + cx.inline_gap;
        cx.max_width_reached = cx.max_width_reached.max(cx.local_offset_px.x);
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

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        self.0.propagate(event).await
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
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        _: &TerminalBlueprintResources,
    ) {
        let word_spacing = 1;
        let mut text_elements = Vec::new();
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset_px.x > 0
                && cx.local_offset_px.x + word_spacing + len > hints.rect.width() as u32
            {
                cx.new_line();
            }

            if cx.local_offset_px.x > 0 {
                cx.local_offset_px.x += word_spacing;
            }

            text_elements.push(
                Text()
                    .with_text(word.to_string())
                    .with_rect(
                        // TODO: Lines might have different heights?
                        Rect::new(
                            (hints.rect.origin.as_() + cx.local_offset_px).as_::<f32>(),
                            Size2::new(len as f32, 1.0),
                        ),
                    )
                    .with_color(self.color),
            );
            cx.current_line_max_item_height = cx.current_line_max_item_height.max(1);
            cx.local_offset_px.x += len;
        }

        self.allocated_texts = text_elements
    }

    fn measure(&mut self, cx: &mut MeasureContext, _: ParentHints) {
        let word_spacing = 1;
        let words = self.text.split_whitespace();

        for word in words {
            let len = word.len() as Offset;

            if cx.local_offset_px.x > 0 && cx.local_offset_px.x + word_spacing + len > cx.container_width
            {
                cx.new_line();
            }

            if cx.local_offset_px.x > 0 {
                cx.local_offset_px.x += word_spacing;
            }

            cx.current_line_max_item_height = cx.current_line_max_item_height.max(1);
            cx.local_offset_px.x += len;
            cx.max_width_reached = cx.max_width_reached.max(cx.local_offset_px.x);
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

    async fn propagate(&mut self, _: &mut ui_composer_input::event::Event) -> bool {
        false
    }
}

impl InlineItem<DesktopEnvironment> for MonospaceText {
    fn place(&mut self, cx: &mut InlineContext, hints: ParentHints, _: &DesktopResources) {
        let word_spacing = DesktopEnvironment::TILE_SIZE.width as u32;
        let mut text_elements = Vec::new();

        for word in self.text.split_whitespace() {
            let len = word.len() as Offset;

            // Look ahead to see if the word will fit in this line.
            // If it won't, break into a new line.
            //
            // If the word itself is bigger than the whole line,
            // it will be left as is.
            if cx.local_offset_px.x + word_spacing + len > hints.rect.width() as u32
                && cx.local_offset_px.x > 0
            {
                cx.new_line();
            }

            // Add a space before the previous word in this line if there's such.
            if cx.local_offset_px.x > 0 {
                cx.local_offset_px.x += word_spacing;
            }

            text_elements.push(
                Text()
                    // TODO: Make this zero-copy?
                    .with_text(word.to_string())
                    .with_rect(
                        // TODO: Lines might have different heights?
                        Rect::new(
                            hints.rect.origin + cx.local_offset_px.as_(),
                            Size2::new(len as f32, 1.0) * DesktopEnvironment::TILE_SIZE,
                        ),
                    )
                    .with_color(self.color),
            );
            cx.current_line_max_item_height = cx.current_line_max_item_height.max(DesktopEnvironment::TILE_SIZE.height as u32);
            cx.local_offset_px.x += len * DesktopEnvironment::TILE_SIZE.width as u32;
        }

        self.allocated_texts = text_elements
    }

    fn measure(&mut self, cx: &mut MeasureContext, _: ParentHints) {
        /* Same as [place], but no allocation happens. */

        let word_spacing = DesktopEnvironment::TILE_SIZE.width as u32;
        for word in self.text.split_whitespace() {
            let len = word.len() as Offset;

            if cx.local_offset_px.x > 0 && cx.local_offset_px.x + word_spacing + len > cx.container_width
            {
                cx.new_line();
            }

            if cx.local_offset_px.x > 0 {
                cx.local_offset_px.x += word_spacing;
            }

            cx.current_line_max_item_height = cx.current_line_max_item_height.max(DesktopEnvironment::TILE_SIZE.height as u32);
            cx.local_offset_px.x += len * DesktopEnvironment::TILE_SIZE.width as u32;
            cx.max_width_reached = cx.max_width_reached.max(cx.local_offset_px.x);
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
                        origin: (e.rect.origin.to_vector()).to_point(),
                        size: e.rect.size,
                    },
                    e.color,
                )
            })
            .collect()
    }

    async fn propagate(&mut self, _: &mut ui_composer_input::event::Event) -> bool {
        false
    }
}

impl<Env, A> InlineItemList<Env> for A
where
    A: InlineItem<Env>,
    Env: Environment,
{
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &Env::BlueprintResources<'_>,
    ) {
        InlineItem::place(self, cx, hints, resources)
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

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        InlineItem::propagate(self, event).await
    }
}

impl<A, B> InlineItemList<TerminalEnvironment> for (A, B)
where
    A: InlineItemList<TerminalEnvironment>,
    B: InlineItemList<TerminalEnvironment>,
{
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &TerminalBlueprintResources,
    ) {
        self.0.place(cx, hints, resources);
        self.1.place(cx, hints, resources);
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

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        Combine::combine(self.0.propagate(event).await, self.1.propagate(event).await)
    }
}

impl<A, B> InlineItemList<DesktopEnvironment> for (A, B)
where
    A: InlineItemList<DesktopEnvironment>,
    B: InlineItemList<DesktopEnvironment>,
{
    fn place(
        &mut self,
        cx: &mut InlineContext,
        hints: ParentHints,
        resources: &DesktopResources,
    ) {
        self.0.place(cx, hints, resources);
        self.1.place(cx, hints, resources);
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

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        Combine::combine(self.0.propagate(event).await, self.1.propagate(event).await)
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
        // Calculates the minimum width of this container,
        // that is, the biggest of its items minimum sizes.
        //
        // You can imagine the `container_width: 0` here as forcing
        // every single item inside into its own line.
        let mut cx_measure_max_inner_min = MeasureContext {
            container_width: 0,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            current_line_max_item_height: 1,
            local_offset_px: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut cx_measure_max_inner_min, parent_hints);
        let true_min_w = cx_measure_max_inner_min.max_width_reached;

        // Calculates the minimum height of the container,
        // which might be smaller than the height above,
        // because when the container is at its minimum width
        // some items might be in the same line.
        let mut cx_measure_height_when_min_width = MeasureContext {
            container_width: true_min_w,
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            current_line_max_item_height: 1,
            local_offset_px: Vector2::new(0, 0),
            max_width_reached: 0,
        };
        self.items.measure(&mut cx_measure_height_when_min_width, parent_hints);
        // The height is the vertical offset accumulated from the previous lines
        // plus the height of the current line.
        let height_when_min_w =
            cx_measure_height_when_min_width.local_offset_px.y + cx_measure_height_when_min_width.current_line_max_item_height;

        ChildHints {
            minimum_size: Size2::new(true_min_w as f32, height_when_min_w as f32),
        }
    }

    fn place(&mut self, hints: ParentHints, resources: &Env::BlueprintResources<'_>) {
        let mut cx = InlineContext {
            inline_gap: self.inline_gap,
            cross_axis_gap: self.cross_axis_gap,
            current_line_max_item_height: 1,
            local_offset_px: Vector2::new(0, 0),
        };

        self.items.place(&mut cx, hints, resources);
    }

    type Blueprint = Items::Blueprint;

    fn effect(&self) -> <<Self::Blueprint as Blueprint<Env>>::Output as Element<Env>>::Effect {
        self.items.effect()
    }

    async fn propagate(&mut self, event: &mut ui_composer_input::event::Event) -> bool {
        self.items.propagate(event).await
    }

    fn poll_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
        resources: &<Env as Environment>::BlueprintResources<'_>,
        parent_hints: ParentHints,
    ) -> std::task::Poll<Option<()>> {
        let this = self.project();
        // TODO: Cache the item rects and send them correct parent hints.
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
