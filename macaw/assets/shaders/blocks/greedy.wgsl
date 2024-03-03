struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4f,
    @location(0) scaled_uv: vec2f,
};

@group(0) @binding(0) var<uniform> block_count: vec2f;

@vertex // triangle corner points
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    output.position = vec4(input.position, 1.0);
    output.scaled_uv = input.uv * block_count; // uvs * vec2(x, y)

    return output;
}

@group(1) @binding(0) var my_texture: texture_2d<f32>;
@group(1) @binding(1) var my_sampler: sampler;

@fragment // converted vertices, helping to find a color for each pixel
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(my_texture, my_sampler, input.scaled_uv);
    // TODO?: apply more effects?
    return color;
}