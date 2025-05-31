use crate::app::primitives::{PrimitiveDescriptor, Processor};
use crate::wgpu::pipeline::graphics::graphic::Graphic;
use crate::wgpu::pipeline::text::TextItem;
use crate::wgpu::pipeline::UIReifyResources;
use glyphon::cosmic_text::Align;
use glyphon::{Attrs, Buffer, Family, Metrics, Shaping, Weight, Wrap};
use {
    super::{RenderText, Text},
    crate::{
        app::{input::Event, primitives::Primitive},
        prelude::items::{Drag, Hover, Tap},
        state::{
            process::{FutureProcessor, SignalProcessor},
            Effect,
        },
    },
    futures_signals::signal::Signal,
    glyphon::{Color, TextArea, TextBounds},
    std::future::Future,
};
//MARK: Text

impl<Res> Primitive<Res> for Text {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl<Res> Processor<Res> for TextItem {}

impl<Res> Primitive<Res> for TextItem {
    fn handle_event(&mut self, #[expect(unused)] event: Event) -> bool {
        false
    }
}

impl RenderText for TextItem {
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

impl PrimitiveDescriptor<UIReifyResources> for Text {
    type Primitive = TextItem;

    fn reify(self, resources: &mut UIReifyResources) -> Self::Primitive {
        let renderer = &mut resources.renderers.text_renderer;
        let mut buffer = Buffer::new(&mut renderer.font_system, Metrics::new(16.0, 20.0));

        buffer.set_text(
            &mut renderer.font_system,
            self.1.as_ref(),
            // TODO: Allow composing this...
            &Attrs::new()
                .family(Family::Name("Work Sans"))
                .weight(Weight::NORMAL),
            Shaping::Advanced,
        );
        buffer.lines[0].set_align(Some(Align::Center));
        buffer.set_size(&mut renderer.font_system, Some(self.0.w), Some(self.0.h));
        // TODO: Perhaps add another primitive that won't wrap?
        buffer.set_wrap(&mut renderer.font_system, Wrap::Word);
        // TODO: This should be configurable, too.
        buffer.shape_until_scroll(&mut renderer.font_system, false);

        TextItem {
            rect: self.0,
            buffer,
            color: self.2,
        }
    }
}

impl<Res> Processor<Res> for Text {}
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

impl<Sig, Res> RenderText for SignalProcessor<Sig, Res>
where
    Sig: Signal,
    Sig::Item: PrimitiveDescriptor<Res, Primitive: RenderText>,
{
    fn push_text<'a>(&'a self, bounds: TextBounds, container: &mut Vec<TextArea<'a>>) {
        match &self.held_item {
            Some(item) => item.push_text(bounds, container),
            None => panic!("Reactor was drawn (text) without being polled first!"),
        }
    }
}

// MARK: FutureProcessor

impl<Fut, Res> RenderText for FutureProcessor<Fut, Res>
where
    Fut: Future,
    Fut::Output: PrimitiveDescriptor<Res, Primitive: RenderText>,
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

impl<A: Effect + Send + Sync> RenderText for Tap<A> {
    fn push_text<'a>(
        &'a self,
        #[expect(unused)] bounds: glyphon::TextBounds,
        #[expect(unused)] container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
    }
}
