use crate::app::building_blocks::reify::Reify;
use crate::standard::backends::wgpu::pipeline::UIContext;
use crate::standard::backends::wgpu::pipeline::graphics::graphic::Graphic;
use crate::standard::backends::wgpu::pipeline::text::TextItemRe;
use crate::state::process::Pollable;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight, Wrap};
use {
    super::{RenderText, TextItem},
    crate::{
        app::{building_blocks::BuildingBlock, input::Event},
        state::process::{FutureAwaitItemRe, SignalReactItemRe},
    },
    futures_signals::signal::Signal,
    glyphon::{Color, TextArea, TextBounds},
    std::future::Future,
};
use crate::standard::prelude::{Drag, Hover, Tap, Typing};
use crate::state::effect::Effect;
//MARK: Text

impl<S: AsRef<str> + Send, Res> BuildingBlock<Res> for TextItem<S> {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Res> Pollable<Res> for TextItemRe {}

impl<Res> BuildingBlock<Res> for TextItemRe {
    fn handle_event(&mut self, #[expect(unused)] event: Event) -> bool {
        false
    }
}

impl RenderText for TextItemRe {
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<TextArea<'a>>) {
        let color: vek::Rgb<u8> = (self.color * 255.0).as_();
        let rect = self.rect;
        let buffer = &self.buffer;

        container.push(glyphon::TextArea {
            buffer,
            left: rect.x,
            top: rect.y,
            scale: 1.0,
            bounds,
            default_color: Color::rgb(color.r, color.g, color.b),
            custom_glyphs: &[],
        });
    }
}

impl<S: AsRef<str>> Reify<UIContext> for TextItem<S> {
    type Output = TextItemRe;

    fn reify(self, resources: &mut UIContext) -> Self::Output {
        let renderer = &mut resources.renderers.text_renderer;
        let mut buffer = Buffer::new(&mut renderer.font_system, Metrics::new(20.0, 20.0));

        let default_attributes = &Attrs::new()
            .family(Family::Name("Anima Sans"))
            .weight(Weight::MEDIUM);

        buffer.set_text(
            &mut renderer.font_system,
            self.text.as_ref(),
            // TODO: Allow composing this...
            default_attributes,
            Shaping::Advanced,
        );
        buffer.set_size(
            &mut renderer.font_system,
            Some(self.rect.w),
            Some(self.rect.h),
        );
        // TODO: Perhaps add another primitive that won't wrap?
        buffer.set_wrap(&mut renderer.font_system, Wrap::Word);
        // TODO: This should be configurable, too.
        buffer.shape_until_scroll(&mut renderer.font_system, false);

        TextItemRe {
            rect: self.rect,
            buffer,
            color: self.color,
        }
    }
}

impl<S: AsRef<str> + Send, Res> Pollable<Res> for TextItem<S> {}
//MARK: Graphics

impl RenderText for Graphic {
    fn push_text<'a>(
        &self,
        #[expect(unused)] bounds: glyphon::TextBounds,
        #[expect(unused)] container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
    }
}

//MARK: ()

impl RenderText for () {
    fn push_text<'a>(
        &'a self,
        #[expect(unused)] bounds: TextBounds,
        #[expect(unused)] container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
    }
}

impl<A> RenderText for Option<A>
where
    A: RenderText,
{
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<TextArea<'a>>) {
        if let Some(inner) = &self {
            inner.push_text(bounds, container);
        }
    }
}

//MARK: (A, B)

impl<A, B> RenderText for (A, B)
where
    A: RenderText,
    B: RenderText,
{
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<glyphon::TextArea<'a>>) {
        self.0.push_text(bounds, container);
        self.1.push_text(bounds, container);
    }
}

//MARK: SignalProcessor

impl<Sig, Res> RenderText for SignalReactItemRe<Sig, Res>
where
    Sig: Signal,
    Sig::Item: Reify<Res, Output: RenderText>,
{
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<TextArea<'a>>) {
        match &self.held_item {
            Some(item) => item.push_text(bounds, container),
            None => panic!("Reactor was drawn (text) without being polled first!"),
        }
    }
}

// MARK: FutureProcessor

impl<Fut, Res> RenderText for FutureAwaitItemRe<Fut, Res>
where
    Fut: Future,
    Fut::Output: Reify<Res, Output: RenderText>,
{
    fn push_text<'a>(
        &'a self,
        bounds: glyphon::TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        if let Some(item) = &self.held_item {
            item.push_text(bounds, container)
        }
    }
}

// MARK: Primitives

macro_rules! impl_render_text_nop {
    ($name:ident) => {
        impl RenderText for $name {
            fn push_text<'a>(
                &'a self,
                #[expect(unused)] bounds: glyphon::TextBounds,
                #[expect(unused)] container: &mut Vec<glyphon::TextArea<'a>>,
            ) {
            }
        }
    };
}

impl_render_text_nop!(Hover);
impl_render_text_nop!(Drag);
impl_render_text_nop!(Typing);

impl<A: Effect + Send + Sync> RenderText for Tap<A> {
    fn push_text<'a>(
        &'a self,
        #[expect(unused)] bounds: glyphon::TextBounds,
        #[expect(unused)] container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
    }
}
