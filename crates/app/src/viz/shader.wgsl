// 3D visualizer: scene (lit), light beams (additive) and tone mapping.

struct Globals {
    view_proj: mat4x4<f32>,
    camera: vec4<f32>,
    // x = ambient light, y = beams, z = fog, w = number of lights
    env: vec4<f32>,
    // x = time (s), y = exposure
    misc: vec4<f32>,
};

struct Light {
    // xyz position, w range
    pos: vec4<f32>,
    // xyz direction (normalized), w cos(outer angle)
    dir: vec4<f32>,
    // rgb color · brightness, w cos(inner angle)
    color: vec4<f32>,
    // x shadow layer (< 0 = no shadow), y outer angle (deg), z gobo, w gobo sharpness
    extra: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var<storage, read> lights: array<Light>;
@group(0) @binding(2) var shadow_maps: texture_depth_2d_array;
@group(0) @binding(3) var shadow_sampler: sampler_comparison;
@group(0) @binding(4) var<storage, read> shadow_mats: array<mat4x4<f32>>;

// Share of light (0 = in shadow), soft over 3×3 samples
fn shadow(layer: i32, p: vec3<f32>, n: vec3<f32>, ld: vec3<f32>) -> f32 {
    if (layer < 0) {
        return 1.0;
    }
    // offset a little along the normal against shadow acne
    let q = shadow_mats[layer] * vec4<f32>(p + n * 0.03, 1.0);
    if (q.w <= 0.0) {
        return 1.0;
    }
    let ndc = q.xyz / q.w;
    let uv = vec2<f32>(ndc.x * 0.5 + 0.5, 0.5 - ndc.y * 0.5);
    if (uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || ndc.z > 1.0) {
        return 1.0;
    }
    let dims = vec2<f32>(textureDimensions(shadow_maps));
    let texel = 1.0 / dims;
    var sum = 0.0;
    for (var y = -1; y <= 1; y = y + 1) {
        for (var x = -1; x <= 1; x = x + 1) {
            sum = sum + textureSampleCompareLevel(
                shadow_maps, shadow_sampler, uv + vec2<f32>(f32(x), f32(y)) * texel * 1.5, layer, ndc.z - 0.0008);
        }
    }
    return sum / 9.0;
}

struct VIn {
    @location(0) pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) m0: vec4<f32>,
    @location(3) m1: vec4<f32>,
    @location(4) m2: vec4<f32>,
    @location(5) m3: vec4<f32>,
    // rgb linear, a opacity
    @location(6) color: vec4<f32>,
    // x gloss, y self-illumination (0–1, or > 1 = HDR),
    // z tile size (m) or gobo, w image: 0 none, 1 projected, 2 image coordinates
    @location(7) params: vec4<f32>,
    @location(8) uv: vec2<f32>,
};

// Image of the material (without image: 1 × 1 white)
@group(1) @binding(0) var tex: texture_2d<f32>;
@group(1) @binding(1) var tex_sampler: sampler;

struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) local: vec3<f32>,
    @location(5) uv: vec2<f32>,
    // position in the object in meters (scale included) and normal in the object
    @location(6) metric: vec3<f32>,
    @location(7) lnormal: vec3<f32>,
};

@vertex
fn vs_main(v: VIn) -> VOut {
    let m = mat4x4<f32>(v.m0, v.m1, v.m2, v.m3);
    let w = m * vec4<f32>(v.pos, 1.0);
    // normal with the inverse transpose (without translation)
    let n3 = mat3x3<f32>(m[0].xyz, m[1].xyz, m[2].xyz);
    let nrm = normalize(transpose(inverse3(n3)) * v.normal);
    var o: VOut;
    o.clip = g.view_proj * w;
    o.world = w.xyz;
    o.normal = nrm;
    o.color = v.color;
    o.params = v.params;
    o.local = v.pos;
    o.uv = v.uv;
    o.metric = v.pos * vec3<f32>(length(m[0].xyz), length(m[1].xyz), length(m[2].xyz));
    o.lnormal = v.normal;
    return o;
}

// Color from the image: projected (three directions, weighted by the
// normal) or via the image coordinates of the model
fn texel(i: VOut) -> vec3<f32> {
    // always sample (uniform control flow), then choose
    let s = i.metric / max(i.params.z, 0.01);
    var w = pow(abs(normalize(i.lnormal)), vec3<f32>(4.0));
    w = w / max(w.x + w.y + w.z, 1e-4);
    let a = textureSample(tex, tex_sampler, vec2<f32>(s.z, -s.y)).rgb;
    let b = textureSample(tex, tex_sampler, vec2<f32>(s.x, s.z)).rgb;
    let c = textureSample(tex, tex_sampler, vec2<f32>(s.x, -s.y)).rgb;
    let d = textureSample(tex, tex_sampler, i.uv).rgb;
    let mode = i32(i.params.w + 0.5);
    if (mode == 2) {
        return d;
    }
    if (mode == 1) {
        return a * w.x + b * w.y + c * w.z;
    }
    return vec3<f32>(1.0);
}

fn inverse3(m: mat3x3<f32>) -> mat3x3<f32> {
    let a = m[0];
    let b = m[1];
    let c = m[2];
    let r0 = cross(b, c);
    let r1 = cross(c, a);
    let r2 = cross(a, b);
    let det = dot(a, r0);
    let inv = 1.0 / select(det, 1e-8, abs(det) < 1e-8);
    return transpose(mat3x3<f32>(r0 * inv, r1 * inv, r2 * inv));
}

// Gobo pattern (approximated): angle around the axis, distance from the center 0–1
fn gobo(id: f32, ang: f32, r: f32) -> f32 {
    let k = i32(id + 0.5);
    var v = 1.0;
    if (k == 1) {
        v = smoothstep(-0.1, 0.3, cos(ang * 6.0)); // star
    } else if (k == 2) {
        v = smoothstep(0.2, 0.5, sin(r * 20.0)) * smoothstep(-0.2, 0.2, cos(ang * 8.0)); // dots
    } else if (k == 3) {
        v = smoothstep(-0.2, 0.4, sin(r * 26.0)); // rings
    } else if (k == 4) {
        v = smoothstep(-0.1, 0.4, sin(ang * 5.0 + r * 9.0) * sin(ang * 3.0 - r * 13.0) + 0.2); // breakup
    } else if (k == 5) {
        v = smoothstep(-0.2, 0.2, cos(ang * 2.0)) * 0.9 + 0.1; // bars
    } else if (k == 6) {
        v = smoothstep(-0.2, 0.3, sin(ang * 3.0 + r * 14.0)); // spiral
    }
    return v;
}

// Light cone of a spotlight on a surface
fn spot(l: Light, p: vec3<f32>, n: vec3<f32>, view: vec3<f32>, gloss: f32) -> vec3<f32> {
    let to = l.pos.xyz - p;
    let d = length(to);
    if (d > l.pos.w) {
        return vec3<f32>(0.0);
    }
    let ld = to / max(d, 1e-4);
    let cosang = dot(-ld, l.dir.xyz);
    let cone = smoothstep(l.dir.w, l.color.w, cosang);
    if (cone <= 0.0) {
        return vec3<f32>(0.0);
    }
    // physical: falls off with the square of the distance, softly up to the range
    let fall = 1.0 / (1.0 + d * d * 0.15);
    // fade out only just before the range (the range ends at the obstacle)
    let edge = smoothstep(l.pos.w, l.pos.w - 0.6, d);
    let diff = max(dot(n, ld), 0.0);
    let h = normalize(ld + view);
    let spec = pow(max(dot(n, h), 0.0), mix(8.0, 96.0, gloss)) * gloss;
    let sh = shadow(i32(l.extra.x), p, n, ld);
    var pattern = 1.0;
    if (l.extra.z > 0.5) {
        let ax = l.dir.xyz;
        let up = select(vec3<f32>(0.0, 1.0, 0.0), vec3<f32>(1.0, 0.0, 0.0), abs(ax.y) > 0.95);
        let u = normalize(cross(ax, up));
        let w = cross(ax, u);
        let q = -ld;
        let r = acos(clamp(cosang, -1.0, 1.0)) / max(acos(clamp(l.dir.w, -1.0, 1.0)), 1e-3);
        // out of focus: the pattern blurs towards its mean
        pattern = mix(0.6, gobo(l.extra.z, atan2(dot(q, w), dot(q, u)), r), clamp(l.extra.w, 0.0, 1.0));
    }
    return l.color.rgb * cone * fall * edge * (diff + spec) * sh * pattern;
}

@fragment
fn fs_scene(i: VOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    var n = normalize(i.normal);
    if (!front) {
        n = -n;
    }
    let view = normalize(g.camera.xyz - i.world);
    let base = i.color.rgb * texel(i);
    // room light: slightly from above so shapes stay readable
    let sky = 0.75 + 0.25 * n.y;
    var c = base * (0.02 + g.env.x * 1.4) * sky;
    // Direct light: real surfaces reflect stage light much less tinted than
    // their color suggests (rough, partly specular), so the spot keeps most of
    // the light's color; very dark materials stay dark.
    let lum = dot(base, vec3<f32>(0.2126, 0.7152, 0.0722));
    let refl = mix(vec3<f32>(lum), base, 0.4) * 1.1;
    var direct = vec3<f32>(0.0);
    let count = u32(g.env.w);
    for (var k = 0u; k < count; k = k + 1u) {
        direct = direct + spot(lights[k], i.world, n, view, i.params.x);
    }
    c = c + refl * direct;
    c = c + base * i.params.y * 4.0;
    // haze: the distance gets dimmer
    let dist = length(g.camera.xyz - i.world);
    let haze = 1.0 - exp(-dist * 0.02 * g.env.z);
    c = mix(c, vec3<f32>(0.02, 0.02, 0.025) * g.env.x, haze);
    return vec4<f32>(c, i.color.a);
}

// ---------- Light beam (cone, additive) ----------
// Mesh: tip at the origin, axis −y, length 1, radius 1 at the end.

@fragment
fn fs_beam(i: VOut) -> @location(0) vec4<f32> {
    let t = clamp(-i.local.y, 0.0, 1.0);
    let view = normalize(g.camera.xyz - i.world);
    // soft edges: darker at the rim of the cone (grazing view)
    let rim = pow(abs(dot(normalize(i.normal), view)), 1.6);
    // brighter near the lens, fading out towards the end
    let along = pow(1.0 - t, 1.8) * smoothstep(0.0, 0.03, t);
    let air = g.env.y * (0.15 + 0.85 * g.env.z);
    var a = rim * along * air * i.color.a;
    if (i.params.z > 0.5) {
        // gobo in the beam: shafts around the axis
        let gb = mix(0.6, gobo(i.params.z, atan2(i.local.z, i.local.x), 0.9), clamp(i.params.w, 0.0, 1.0));
        a = a * mix(0.15, 1.0, gb);
    }
    return vec4<f32>(i.color.rgb * a, 0.0);
}


// ---------- Shadow maps (depth only) ----------

struct ShadowPass {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) var<uniform> sp: ShadowPass;

@vertex
fn vs_shadow(v: VIn) -> @builtin(position) vec4<f32> {
    let m = mat4x4<f32>(v.m0, v.m1, v.m2, v.m3);
    return sp.view_proj * (m * vec4<f32>(v.pos, 1.0));
}
