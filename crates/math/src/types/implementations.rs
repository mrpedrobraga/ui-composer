//! # Extensions
//!
//! Extension traits to make working with some foreign types a little bit more ergonomic.

use {
    crate::types::{
        Aabb2, Rect, RectExt, SideOffsets, Size2, UAabb2, URect, USize2,
    },
    glam::{UVec2, Vec2},
};

impl RectExt for Rect {
    type Scalar = f32;
    type Size = Size2;
    type Vector = Vec2;

    fn offset(self, offsets: SideOffsets<Self::Scalar>) -> Self {
        Self {
            pos: Vec2 {
                x: self.pos.x + offsets.left,
                y: self.pos.y + offsets.top,
            },
            size: Size2 {
                x: self.size.x - offsets.left + offsets.right,
                y: self.size.y - offsets.top + offsets.bottom,
            },
        }
    }

    fn with_size(self, size: Size2) -> Self {
        Self { size, ..self }
    }

    fn translated(self, vector: Vec2) -> Self {
        Self {
            pos: self.pos + vector,
            ..self
        }
    }
}

impl RectExt for Aabb2 {
    type Scalar = f32;
    type Size = Size2;
    type Vector = Vec2;

    fn offset(self, offsets: SideOffsets<Self::Scalar>) -> Self {
        Self {
            min: Vec2 {
                x: self.min.x + offsets.left,
                y: self.min.y + offsets.top,
            },
            max: Size2 {
                x: self.max.x + offsets.right,
                y: self.max.y + offsets.bottom,
            },
        }
    }

    fn with_size(self, size: Size2) -> Self {
        let current_size = self.max - self.min;
        let delta = size - current_size;
        let half_size = delta / 2.0;
        let offsets = SideOffsets {
            top: -half_size.y,
            bottom: half_size.y,
            left: -half_size.x,
            right: half_size.x,
        };
        self.offset(offsets)
    }

    fn translated(self, vector: Vec2) -> Self {
        Self {
            min: self.min + vector,
            max: self.max + vector,
        }
    }
}

impl RectExt for URect {
    type Scalar = u32;

    fn offset(self, offsets: SideOffsets<Self::Scalar>) -> Self {
        Self {
            pos: UVec2 {
                x: self.pos.x + offsets.left,
                y: self.pos.y + offsets.top,
            },
            size: USize2 {
                x: self.size.x - offsets.left + offsets.right,
                y: self.size.y - offsets.top + offsets.bottom,
            },
        }
    }

    fn with_size(self, size: USize2) -> Self {
        Self { size, ..self }
    }

    fn translated(self, vector: UVec2) -> Self {
        Self {
            pos: self.pos + vector,
            ..self
        }
    }
}

impl RectExt for UAabb2 {
    type Scalar = u32;

    fn offset(self, offsets: SideOffsets<Self::Scalar>) -> Self {
        Self {
            min: Vec2 {
                x: self.min.x + offsets.left,
                y: self.min.y + offsets.top,
            },
            max: Size2 {
                x: self.max.x + offsets.right,
                y: self.max.y + offsets.bottom,
            },
        }
    }

    fn with_size(self, size: Size2) -> Self {
        let current_size = self.max - self.min;
        let delta = size - current_size;
        let half_size = delta / 2.0;
        let offsets = SideOffsets {
            top: -half_size.y,
            bottom: half_size.y,
            left: -half_size.x,
            right: half_size.x,
        };
        self.offset(offsets)
    }

    fn translated(self, vector: Vec2) -> Self {
        Self {
            min: self.min + vector,
            max: self.max + vector,
        }
    }
}
