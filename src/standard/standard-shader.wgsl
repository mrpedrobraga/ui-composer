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
    let window_size = vec2<f32>(800.0, 800.0);
    let world_mat = mat4x4<f32>(
        vec4<f32>(1.0 / window_size.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, -1.0 / window_size.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(-1.0, 1.0, 0.0, 1.0),
    );

    let position = (world_mat * transform_mat * vec4(model.position, 1.0)).xyz;

    out.clip_position = vec4<f32>(position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(srgb_to_linear(in.color), 1.0);
}

// Function to convert sRGB color to linear RGB
fn srgb_to_linear(color_srgb: vec3<f32>) -> vec3<f32> {
    // Apply inverse gamma correction
    let color_linear = pow(color_srgb, vec3<f32>(2.2));

    // Ensure the resulting linear RGB color values are clamped between 0 and 1
    return min(max(color_linear, vec3<f32>(0.0)), vec3<f32>(1.0));
}
