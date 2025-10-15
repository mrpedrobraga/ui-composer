#![allow(non_snake_case)]

use crate::app::building_blocks::BuildingBlock;
use crate::prelude::{Event, LayoutItem, ParentHints};
use crate::wgpu::pipeline::text::{Text, TextItem};
use vek::Rgb;
use crate::state::process::Pollable;

/// A simple label that visualises a String!
///
/// This element's minimum size depends on its content:
/// as the parent defines its size in the `WritingAxis`, it
/// will grow in the `WritingCrossAxis`.
///
/// ^ TODO: This is not yet implemented.
pub fn Label<S>(text: S) -> TextLayoutItem<S>
where
    S: AsRef<str>,
{
    TextLayoutItem {
        text,
        own_color: None,
    }
}

#[derive(Clone)]
pub struct TextLayoutItem<AsStr>
where
    AsStr: AsRef<str>,
{
    text: AsStr,
    own_color: Option<Rgb<f32>>,
}

impl<AsStr> TextLayoutItem<AsStr>
where
    AsStr: AsRef<str>,
{
    /// Returns this [TextLayoutItem], with a different colour.
    pub fn with_color(self, new_color: Rgb<f32>) -> Self {
        Self {
            own_color: Some(new_color),
            ..self
        }
    }
}

impl<AsStr: AsRef<str> + Send + Clone> LayoutItem for TextLayoutItem<AsStr> {
    type Content = TextItem<AsStr>;

    fn get_natural_size(&self) -> vek::Extent2<f32> {
        vek::Extent2::new(120.0, 32.0)
    }

    fn get_minimum_size(&self) -> vek::Extent2<f32> {
        vek::Extent2::new(120.0, 32.0)
    }

    fn lay(&mut self, parent_hints: ParentHints) -> Self::Content {
        Text (
            parent_hints.rect,
            self.text.clone(),
            self.own_color.unwrap_or(Rgb::white()), // TODO: Use the current foreground colour!
        )
    }
}

impl<S: AsRef<str> + Send, Res> Pollable<Res> for TextLayoutItem<S> {}

impl<S: AsRef<str> + Send, Res> BuildingBlock<Res> for TextLayoutItem<S> {
    fn handle_event(&mut self, _: Event) -> bool {
        // Event was not handled
        false
    }
}
