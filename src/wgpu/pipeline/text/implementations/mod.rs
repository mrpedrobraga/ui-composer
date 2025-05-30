use crate::app::primitives::PollProcessors;
use crate::wgpu::pipeline::graphics::graphic::Graphic;
use crate::wgpu::render_target::RenderInternal;
use glyphon::Buffer;
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
    std::{
        future::Future,
        pin::Pin,
        task::{Context, Poll},
    },
};
//MARK: Text

impl RenderText for Text {
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: TextBounds,
        container: &mut Vec<TextArea<'a>>,
    ) {
        let v: vek::Rgb<u8> = (self.2 * 255.0).as_();

        container.push(glyphon::TextArea {
            buffer,
            left: self.0.x,
            top: self.0.y,
            scale: 1.0,
            bounds,
            default_color: Color::rgb(v.r, v.g, v.b),
            custom_glyphs: &[],
        });
    }
}

impl Primitive for Text {
    fn handle_event(&mut self, _event: Event) -> bool {
        false
    }
}

impl PollProcessors for Text {
    fn poll_processors(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Option<()>> {
        Poll::Ready(Some(()))
    }
}
//MARK: Graphics

impl RenderText for Graphic {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: glyphon::TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // Nothing here!
    }
}

//MARK: ()

impl RenderText for () {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // No text here!
    }
}

impl<A> RenderText for Option<A>
where
    A: RenderText,
{
    fn push_text<'a>(
        &self,
        buffer: &'a Buffer,
        bounds: TextBounds,
        container: &mut Vec<TextArea<'a>>,
    ) {
        if let Some(inner) = &self {
            inner.push_text(buffer, bounds, container);
        }
    }
}

//MARK: (A, B)

impl<A, B> RenderText for (A, B)
where
    A: RenderText,
    B: RenderText,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        self.0.push_text(buffer, bounds, container);
        self.1.push_text(buffer, bounds, container);
    }
}

//MARK: SignalProcessor

impl<S, T> RenderText for SignalProcessor<S, T>
where
    S: Signal<Item = T>,
    T: RenderInternal + Send,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: glyphon::TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        match &self.signal.held_item {
            Some(item) => item.push_text(buffer, bounds, container),
            None => panic!("Reactor was drawn (text) without being polled first!"),
        }
    }
}

// MARK: FutureProcessor

impl<F, T> RenderText for FutureProcessor<F, T>
where
    F: Future<Output = T>,
    T: RenderInternal,
{
    fn push_text<'a>(
        &self,
        buffer: &'a glyphon::Buffer,
        bounds: glyphon::TextBounds,
        container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        if let Some(item) = &self.signal.held_item {
            item.push_text(buffer, bounds, container)
        }
    }
}

// MARK: Primitives

macro_rules! impl_render_text {
    ($name:ident) => {
        impl RenderText for $name {
            fn push_text<'a>(
                &self,
                _buffer: &'a glyphon::Buffer,
                _bounds: glyphon::TextBounds,
                _container: &mut Vec<glyphon::TextArea<'a>>,
            ) {
                // Nothing here!
            }
        }
    };
}

impl_render_text!(Hover);
impl_render_text!(Drag);

impl<A: Effect + Send + Sync> RenderText for Tap<A> {
    fn push_text<'a>(
        &self,
        _buffer: &'a glyphon::Buffer,
        _bounds: glyphon::TextBounds,
        _container: &mut Vec<glyphon::TextArea<'a>>,
    ) {
        // Nothing here!
    }
}
