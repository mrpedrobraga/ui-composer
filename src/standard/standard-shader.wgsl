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
    let position = ((transform_mat * vec4(model.position, 1.0)).xyz + vec3<f32>(-1.0, -1.0, 0.0)) * vec3<f32>(1.0, -1.0, 1.0);

    out.clip_position = vec4<f32>(position, 1.0);
    return out;
}

// Fragment shader

@fragment
fn fs_main(
    in: VertexOutput,
) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
