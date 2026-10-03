use ui_composer_math::glamour::{Matrix4, Vector4};

// TODO: Move this struct somewhere else!
pub struct RenderModuleEffectVisitor<'fx> {
    pub quads: &'fx mut Vec<QuadInstance>
}

/* Rendering */
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, bytemuck::Zeroable, bytemuck::Pod)]
pub struct QuadInstance {
    pub matrix: Matrix4<f32>,
    pub color: Vector4<f32>
}

impl QuadInstance {
    pub fn new(matrix: Matrix4<f32>, color: Vector4<f32>) -> Self {
        Self { matrix, color }
    }
}