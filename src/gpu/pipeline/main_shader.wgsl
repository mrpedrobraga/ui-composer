struct StandardUniform {
    world_to_wgpu_mat_x: vec4<f32>,
    world_to_wgpu_mat_y: vec4<f32>,
    world_to_wgpu_mat_z: vec4<f32>,
    world_to_wgpu_mat_w: vec4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: StandardUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
};

struct InstanceInput {
    @location(5) transform_mat_x: vec4<f32>,
    @location(6) transform_mat_y: vec4<f32>,
    @location(7) transform_mat_z: vec4<f32>,
    @location(8) transform_mat_w: vec4<f32>,
    @location(9) color: vec3<f32>
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
};

struct FragmentInput {
    @builtin(position) fragment_position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.color = instance.color;

    let transform_mat = mat4x4<f32>(
        instance.transform_mat_x,
        instance.transform_mat_y,
        instance.transform_mat_z,
        instance.transform_mat_w
    );
    let world_mat = mat4x4<f32>(
        uniforms.world_to_wgpu_mat_x,
        uniforms.world_to_wgpu_mat_y,
        uniforms.world_to_wgpu_mat_z,
        uniforms.world_to_wgpu_mat_w
    );

    let position = (world_mat * transform_mat * vec4(model.position, 1.0)).xyz;

    out.clip_position = vec4<f32>(position, 1.0);
    return out;
}

@fragment
fn fs_main(
    in: FragmentInput,
) -> @location(0) vec4<f32> {
    // TODO: Inquire the bit depth;
    let inv_bit_depth = 1.0/255.0;
    let debanding = inv_bit_depth * interleaved_gradient_noise(in.fragment_position.xy);
    // NOTE: Debanding should *not* be applied when rendering user images.
    // Or at the very least should be made optional.
    return vec4<f32>(1.0 * debanding + srgb_to_linear(in.color), 1.0);
}

fn srgb_to_linear(color_srgb: vec3<f32>) -> vec3<f32> {
    let color_linear = pow(color_srgb, vec3<f32>(2.2));
    return min(max(color_linear, vec3<f32>(0.0)), vec3<f32>(1.0));
}

fn interleaved_gradient_noise(frag_coord: vec2<f32>) -> f32
{
	return fract(52.9829189 * fract(dot(frag_coord, vec2(0.06711056, 0.00583715))));
}
