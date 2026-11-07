// ---------- Tonemapping (Vollbild-Dreieck) ----------

@group(0) @binding(0) var hdr: texture_2d<f32>;
@group(0) @binding(1) var hdr_sampler: sampler;
@group(0) @binding(2) var bloom: texture_2d<f32>;
struct Post {
    // x Bloom-Stärke, y Belichtung
    p: vec4<f32>,
};
@group(0) @binding(3) var<uniform> post: Post;

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
    let b = textureSample(bloom, hdr_sampler, i.uv).rgb;
    return vec4<f32>(aces((c + b * post.p.x) * post.p.y), 1.0);
}

// ---------- Bloom (Dual-Filter: verkleinern, dann vergrößern) ----------

struct BloomPass {
    // xy Texelgröße der Quelle, z Schwelle (nur erster Schritt), w –
    p: vec4<f32>,
};
@group(1) @binding(0) var src: texture_2d<f32>;
@group(1) @binding(1) var src_sampler: sampler;
@group(1) @binding(2) var<uniform> bp: BloomPass;

fn bright(c: vec3<f32>) -> vec3<f32> {
    if (bp.p.z <= 0.0) {
        return c;
    }
    let l = max(c.r, max(c.g, c.b));
    let k = max(l - bp.p.z, 0.0) / max(l, 1e-4);
    return c * k;
}

@fragment
fn fs_down(i: FsOut) -> @location(0) vec4<f32> {
    let h = bp.p.xy;
    var c = bright(textureSample(src, src_sampler, i.uv).rgb) * 4.0;
    c = c + bright(textureSample(src, src_sampler, i.uv - h).rgb);
    c = c + bright(textureSample(src, src_sampler, i.uv + h).rgb);
    c = c + bright(textureSample(src, src_sampler, i.uv + vec2<f32>(h.x, -h.y)).rgb);
    c = c + bright(textureSample(src, src_sampler, i.uv - vec2<f32>(h.x, -h.y)).rgb);
    return vec4<f32>(c / 8.0, 1.0);
}

@fragment
fn fs_up(i: FsOut) -> @location(0) vec4<f32> {
    let h = bp.p.xy;
    var c = textureSample(src, src_sampler, i.uv + vec2<f32>(-h.x * 2.0, 0.0)).rgb;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(-h.x, h.y)).rgb * 2.0;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(0.0, h.y * 2.0)).rgb;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(h.x, h.y)).rgb * 2.0;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(h.x * 2.0, 0.0)).rgb;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(h.x, -h.y)).rgb * 2.0;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(0.0, -h.y * 2.0)).rgb;
    c = c + textureSample(src, src_sampler, i.uv + vec2<f32>(-h.x, -h.y)).rgb * 2.0;
    return vec4<f32>(c / 12.0, 1.0);
}
