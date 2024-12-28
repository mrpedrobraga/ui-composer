// Each view can populate a different set of values for the uniforms...
struct StandardUniform {
    // The transformation matrix can be used for
    // panning, zooming, rotating, skewing UI.
    world_to_wgpu_mat_x: vec4<f32>,
    world_to_wgpu_mat_y: vec4<f32>,
    world_to_wgpu_mat_z: vec4<f32>,
    world_to_wgpu_mat_w: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: StandardUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(5) transform_mat_x: vec4<f32>,
    @location(6) transform_mat_y: vec4<f32>,
    @location(7) transform_mat_z: vec4<f32>,
    @location(8) transform_mat_w: vec4<f32>,
    @location(9) color: vec3<f32>,
    @location(10) corner_radii: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) quad_size: vec2<f32>,
    @location(3) corner_radii: vec4<f32>,
};

struct FragmentInput {
    @builtin(position) fragment_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) quad_size: vec2<f32>,
    @location(3) corner_radii: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color;
    out.uv = model.position.xy;
    out.corner_radii = instance.corner_radii;

    let transform_mat = mat4x4<f32>(
        instance.transform_mat_x,
        instance.transform_mat_y,
        instance.transform_mat_z,
        instance.transform_mat_w
    );
    out.quad_size = vec2<f32>( length(transform_mat[0].xyz), length(transform_mat[1].xyz) );

    let world_mat = mat4x4<f32>(
        uniforms.world_to_wgpu_mat_x,
        uniforms.world_to_wgpu_mat_y,
        uniforms.world_to_wgpu_mat_z,
        uniforms.world_to_wgpu_mat_w
    );
    let final_mat = world_mat * transform_mat;
    let position = (final_mat * vec4(model.position, 1.0)).xyz;
    out.clip_position = vec4<f32>(position, 1.0);
    return out;
}

@fragment
fn fs_main(
    in: FragmentInput,
) -> @location(0) vec4<f32> {
    let px_coord = in.uv * in.quad_size;

    // TODO: Inquire the bit depth;
    let inv_bit_depth = 1.0/255.0;
    let debanding = inv_bit_depth * interleaved_gradient_noise(in.fragment_position.xy);
    let corner_radii = in.corner_radii;

    // Corner Radius Stencil
    let sdf = rounded_box(px_coord - in.quad_size * 0.5, in.quad_size * 0.5, corner_radii);
    //let sdf = rounded_box_squircle(px_coord - in.quad_size * 0.5, in.quad_size * 0.5, corner_radii, 4.0);

    var a = 1.0;

    // Outline Stencil
    let border_width = 20.0;
    a = smoothstep(-border_width - 1.0, -border_width, sdf) - smoothstep(-1.0, 0.0, sdf);

    // NOTE: Debanding should *not* be applied when rendering user images.
    // Or at the very least should be made optional.
    return vec4<f32>(1.0 * debanding + srgb_to_linear(in.color), a);
}

fn rounded_box(position: vec2f, box_size: vec2f, corner_radii: vec4f) -> f32 {
  var x = corner_radii.x;
  var y = corner_radii.y;
  x = select(corner_radii.z, corner_radii.x, position.x > 0.);
  y = select(corner_radii.w, corner_radii.y, position.x > 0.);
  x  = select(y, x, position.y > 0.);
  let q = abs(position) - box_size + x;
  return min(max(q.x, q.y), 0.) + length(max(q, vec2f(0.))) - x;
}

fn rounded_box_squircle(
    position: vec2f,
    box_size: vec2f,
    corner_radii: vec4f,
    squircle_strength: f32
) -> f32 {
    var x = corner_radii.x;
    var y = corner_radii.y;
    x = select(corner_radii.z, corner_radii.x, position.x > 0.);
    y = select(corner_radii.w, corner_radii.y, position.x > 0.);
    x = select(y, x, position.y > 0.);
    let q = abs(position) - box_size + x;

    // Compute squircle distance using squircle_strength
    let squircle_distance = pow(pow(max(q.x, 0.), squircle_strength) + pow(max(q.y, 0.), squircle_strength), 1.0 / squircle_strength);

    return min(max(q.x, q.y), 0.) + squircle_distance - x;
}

fn srgb_to_linear(color_srgb: vec3<f32>) -> vec3<f32> {
    let gamma = 2.2;
    let color_linear = pow(color_srgb, vec3<f32>(gamma));
    return min(max(color_linear, vec3<f32>(0.0)), vec3<f32>(1.0));
}

fn interleaved_gradient_noise(frag_coord: vec2<f32>) -> f32
{
	return fract(52.9829189 * fract(dot(frag_coord, vec2(0.06711056, 0.00583715))));
}
