use crate::{
    gpu::pipeline::text::Text,
    prelude::{LayoutItem, RectExt as _, Resizable as _, ResizableItem},
};
use vek::Rgb;

/// A simple label that visualises a String!
///
/// This element's minimum size depends on its content:
/// as the parent defines its size in the `WritingAxis`, it
/// will grow in the `WritingCrossAxis`.
///
/// ^ TODO: This is not yet implemented.
pub fn Label(text: String, color: Rgb<f32>) -> impl LayoutItem {
    ResizableItem::new(move |hx| {
        Text(
            hx.rect.expand_from_center(-8.0, -8.0, -8.0, -8.0),
            text.clone(),
            color,
        )
    })
    .with_minimum_size(vek::Extent2::new(32.0, 48.0))
}
