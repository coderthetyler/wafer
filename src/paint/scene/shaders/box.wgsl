[[block]]
struct Uniforms {
    view_proj: mat4x4<f32>;
};

struct Input {
    [[location(0)]] position: vec3<f32>;

    // Model matrix: mat4x4<f32>
    [[location(1)]] model0: vec4<f32>;
    [[location(2)]] model1: vec4<f32>;
    [[location(3)]] model2: vec4<f32>;
    [[location(4)]] model3: vec4<f32>;
};

struct Output {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[stage(vertex)]]
fn vs_main(in: Input) -> Output {
    let model = mat4x4<f32>(in.model0, in.model1, in.model2, in.model3);

    var out: Output;
    out.clip_position = uniforms.view_proj * model * vec4<f32>(in.position, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
