// shader/blit.wgsl

struct VertOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// uniform: center_x, center_y, width_ndc, height_ndc
@group(0) @binding(2)
var<uniform> u_rect: vec4<f32>;

@vertex
fn vs_main(@builtin(vertex_index) vid: u32) -> VertOut {
    // triangle strip ordering:
    // 0 (-1,-1), 1 (1,-1), 2 (-1,1), 3 (1,1)
    var pos_table = array<vec2<f32>, 4>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>( 1.0,  1.0)
    );
    let p = pos_table[vid];
    // uv: map [-1,1] -> [0,1]
    let uv = (p * 0.5) + vec2<f32>(0.5, 0.5);

    // expand around center: pos = center + p * 0.5 * size
    let cx = u_rect.x;
    let cy = u_rect.y;
    let w  = u_rect.z;
    let h  = u_rect.w;

    let ndc_x = cx + p.x * 0.5 * w;
    let ndc_y = cy + p.y * 0.5 * h;

    var out: VertOut;
    out.pos = vec4<f32>(ndc_x, ndc_y, 0.0, 1.0);
    out.uv = uv;
    return out;
}

@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;

@fragment
fn fs_main(in: VertOut) -> @location(0) vec4<f32> {
    return textureSample(tex, samp, in.uv);
}