[[block]]
struct Uniforms {
    cursor_pos: vec2<f32>;
};
[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

struct VertexInput {
    [[builtin(vertex_index]] index: u32;
};

struct VertexOutput {
    [[builtin(position)]] vertex_pos: vec4<f32>;
};

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.vertex_pos = in.vertex_pos;
    out.clip_position = uniforms.view_proj * vec4<f32>(input.position, 1.0);
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}