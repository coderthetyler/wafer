
[[block]]
struct Uniforms {
    ortho_proj: mat4x4<f32>;
};

struct VsVertex {
    [[location(0)]] position: vec2<f32>;
};

struct VsInstance {
    [[location(1)]] size: vec2<f32>;
    [[location(2)]] progress: f32;
    [[location(3)]] top_left: vec2<f32>;
};

struct VsOutput {
    [[builtin(position)]] position: vec4<f32>;
    [[location(0)]] size: vec2<f32>;
    [[location(1)]] progress: f32;
    [[location(2)]] top_left: vec2<f32>;
};

[[group(0), binding(0)]]
var<uniform> uniforms: Uniforms;

[[stage(vertex)]]
fn vs_main(vertex: VsVertex, inst: VsInstance) -> VsOutput {
    var out: VsOutput;
    out.position = uniforms.ortho_proj * vec4<f32>(vertex.position, 0.0, 1.0);
    out.size = inst.size;
    out.progress = inst.progress;
    out.top_left = inst.top_left;
    return out;
}

// stolen from https://stackoverflow.com/a/30545544
// i'll try to understand this later
fn dist_to_rect(uv: vec2<f32>, tl: vec2<f32>, br: vec2<f32>) -> f32 {
    let zero2 = vec2<f32>(0.0, 0.0);
    let d = max(tl-uv, uv-br);
    return length(max(zero2, d)) + min(0.0, max(d.x, d.y));
}

[[stage(fragment)]]
fn fs_main(in: VsOutput) -> [[location(0)]] vec4<f32> {
    let width = in.size.x;
    let height = in.size.y;
    let frag: vec2<f32> = in.position.xy - in.top_left;
    
    // knob parameters
    let knob_radius = in.size.y / 2.0;
    let knob_color = vec4<f32>(1.0, 1.0, 1.0, 1.0);
    let knob = vec2<f32>(width * in.progress, height * 0.5);

    // rail parameters
    let rail_size = vec2<f32>(width, height * 0.5);
    let rail_radius = rail_size.y / 2.0;
    let rail_tl = vec2<f32>(0.0 + rail_radius, knob.y - rail_size.y / 2.0 + rail_radius);
    let rail_br = vec2<f32>(width - rail_radius, knob.y + rail_size.y / 2.0 - rail_radius);
    let rail_left_color = vec4<f32>(0.613, 0.766, 0.901, 1.0);
    let rail_right_color = vec4<f32>(0.921, 0.921, 0.921, 1.0);

    // border parameters
    let border_width: f32 = 2.0;
    let border_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let border_aa_color = vec4<f32>(0.1, 0.1, 0.1, 1.0);

    // draw knob
    let dist: f32 = length(frag - knob);
    if (dist < knob_radius) {
        if (dist >= knob_radius - border_width) {
            return border_color;
        }else{
            return knob_color;
        }
    }

    // draw rail
    let rail_dist = dist_to_rect(frag, rail_tl, rail_br);
    if (rail_dist <= rail_radius) {
        if (rail_dist >= rail_radius - border_width) {
            let falloff = min(1.0, rail_radius - rail_dist);
            return vec4<f32>(border_color.rgb + (1.0 - sqrt(falloff)), 1.0);
        }elseif (frag.x <= knob.x) {
            return rail_left_color;
        }else{
            return rail_right_color;
        }
    }

    // neither knob nor rail
    // return vec4<f32>(1.0, 1.0, 0.0, 1.0);
    discard;
}