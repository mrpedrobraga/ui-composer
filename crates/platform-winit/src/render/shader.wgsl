struct Uniforms {
    view_matrix: mat4x4<f32>,
}

struct QuadInstance {
    model_matrix: mat4x4<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;
@group(0) @binding(1) var<storage, read> quads: array<QuadInstance>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) @interpolate(perspective) uv: vec2<f32>,
    @location(1) @interpolate(flat) quad_color: vec4<f32>,
    @location(2) @interpolate(flat) quad_size: vec2<f32>,
    @location(3) @interpolate(flat) instance_index: u32,
}

@vertex
fn vs_main(
    @builtin(vertex_index) vertex_index: u32,
    @builtin(instance_index) instance_index: u32
) -> VertexOutput {
    let POSITIONS = array<vec2<f32>, 6>(
        vec2<f32>( 0.0, 0.0), // 0\ 3--5
        vec2<f32>( 0.0, 1.0), // | \ \ |
        vec2<f32>( 1.0, 1.0), // 1--2 \4
        vec2<f32>( 0.0, 0.0),
        vec2<f32>( 1.0, 1.0), 
        vec2<f32>( 1.0, 0.0)
    );
    
    let UVS = array<vec2<f32>, 6>(
        vec2<f32>(0.0, 0.0),
        vec2<f32>(0.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(0.0, 0.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 0.0)
    );

    let local_pos = POSITIONS[vertex_index];
    let uv = UVS[vertex_index];

    let quad = quads[instance_index];

    let world_pos = quad.model_matrix * vec4<f32>(local_pos, 0.0, 1.0);
    let quad_size = (quad.model_matrix * vec4<f32>(1.0, 1.0, 0.0, 0.0)).xy;
    let clip_pos = uniforms.view_matrix * world_pos;

    var out: VertexOutput;
    out.position = clip_pos;
    out.uv = uv;
    out.quad_color = quad.color;
    out.quad_size = quad_size;
    out.instance_index = instance_index;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let colors = array<vec4<f32>, 2>(
        vec4<f32>(1.0, 0.2, 0.0, 1.0),
        vec4<f32>(0.2, 1.0, 0.0, 1.0)
    );

    // let border_radius = min(vec2<f32>(10.0, 10.0), in.quad_size / 2);
    // let local_coord_px = in.uv * in.quad_size;
    // let local_coord_px_from_center = (in.uv - 0.5) * in.quad_size;
    // let sdf = sd_rounded_box(local_coord_px_from_center, in.quad_size / 2, min(border_radius.x, border_radius.y));
    // let stencil_alpha = 1.0 - smoothstep(-0.01, 0.0, sdf);
    let stencil_alpha = 1.0;

    return vec4(in.quad_color.rgb, in.quad_color.a * stencil_alpha);
}

fn sd_rounded_box(p: vec2f, b: vec2f, r: f32) -> f32 {
    let q = abs(p) - b + vec2f(r);
    return min(max(q.x, q.y), 0.0) + length(max(q, vec2f(0.0))) - r;
}