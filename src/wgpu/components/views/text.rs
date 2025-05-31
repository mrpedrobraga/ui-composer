#![allow(non_snake_case)]

use crate::app::primitives::{Primitive, Processor};
use crate::prelude::{Event, LayoutItem, ParentHints, RectExt as _};
use crate::wgpu::pipeline::text::Text;
use vek::{Rgb, Vec2};

/// A simple label that visualises a String!
///
/// This element's minimum size depends on its content:
/// as the parent defines its size in the `WritingAxis`, it
/// will grow in the `WritingCrossAxis`.
///
/// ^ TODO: This is not yet implemented.
pub fn Label<S>(text: S) -> TextLayoutItem
where
    S: Into<String>,
{
    let text = text.into();
    TextLayoutItem {
        text,
        own_color: None,
    }
}

#[derive(Clone)]
pub struct TextLayoutItem {
    text: String,
    own_color: Option<Rgb<f32>>,
}

impl TextLayoutItem {
    /// Returns this [TextLayoutItem], with a different colour.
    pub fn with_color(self, new_color: Rgb<f32>) -> Self {
        Self {
            own_color: Some(new_color),
            ..self
        }
    }
}

impl LayoutItem for TextLayoutItem {
    type Content = Text;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        vek::Extent2::new(120.0, 32.0)
    }

    fn get_minimum_size(&self) -> vek::Extent2<f32> {
        vek::Extent2::new(120.0, 32.0)
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        Text(
            parent_hints.rect.translated(Vec2::new(0.0, 4.0)),
            self.text.clone(),
            self.own_color.unwrap_or(Rgb::white()), // TODO: Use the current foreground colour!
        )
    }
}

impl<Res> Processor<Res> for TextLayoutItem {}

impl<Res> Primitive<Res> for TextLayoutItem {
    fn handle_event(&mut self, _: Event) -> bool {
        // Event was not handled
        false
    }
}
