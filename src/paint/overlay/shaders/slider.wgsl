
struct VsInput {
    // Per vertex
    [[location(0)]] position: vec2<f32>;

    // Per instance
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] progress: f32;
};

struct FsInput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] progress: f32;
    [[location(3)]] left: f32;
};

[[stage(vertex)]]
fn vs_main(in: VsInput) -> FsInput {
    var out: FsInput;
    out.clip_position = vec4<f32>(in.position, 0.0, 1.0);
    out.size = in.size;
    out.progress = in.progress;
    out.left = in.size.x;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: FsInput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.left, 0.0, 0.0, 1.0);
}
