// ---------- Tonemapping (Vollbild-Dreieck) ----------

@group(0) @binding(0) var hdr: texture_2d<f32>;
@group(0) @binding(1) var hdr_sampler: sampler;

struct FsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_full(@builtin(vertex_index) id: u32) -> FsOut {
    let p = vec2<f32>(f32((id << 1u) & 2u), f32(id & 2u));
    var o: FsOut;
    o.clip = vec4<f32>(p * 2.0 - 1.0, 0.0, 1.0);
    o.uv = vec2<f32>(p.x, 1.0 - p.y);
    return o;
}

fn aces(x: vec3<f32>) -> vec3<f32> {
    let a = 2.51;
    let b = 0.03;
    let c = 2.43;
    let d = 0.59;
    let e = 0.14;
    return clamp((x * (a * x + b)) / (x * (c * x + d) + e), vec3<f32>(0.0), vec3<f32>(1.0));
}

@fragment
fn fs_tonemap(i: FsOut) -> @location(0) vec4<f32> {
    let c = textureSample(hdr, hdr_sampler, i.uv).rgb;
    return vec4<f32>(aces(c), 1.0);
}
