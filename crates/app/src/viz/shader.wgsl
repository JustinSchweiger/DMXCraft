// 3D-Visualisierer: Szene (beleuchtet), Lichtstrahlen (additiv) und Tonemapping.

struct Globals {
    view_proj: mat4x4<f32>,
    camera: vec4<f32>,
    // x = Umgebungslicht, y = Strahlen, z = Nebel, w = Anzahl Lichter
    env: vec4<f32>,
    // x = Zeit (s), y = Belichtung
    misc: vec4<f32>,
};

struct Light {
    // xyz Position, w Reichweite
    pos: vec4<f32>,
    // xyz Richtung (normiert), w cos(äußerer Winkel)
    dir: vec4<f32>,
    // rgb Farbe · Helligkeit, w cos(innerer Winkel)
    color: vec4<f32>,
    // x Schattenebene (< 0 = ohne Schatten)
    extra: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var<storage, read> lights: array<Light>;
@group(0) @binding(2) var shadow_maps: texture_depth_2d_array;
@group(0) @binding(3) var shadow_sampler: sampler_comparison;
@group(0) @binding(4) var<storage, read> shadow_mats: array<mat4x4<f32>>;

// Anteil Licht (0 = im Schatten), weich über 3×3 Abtastungen
fn shadow(layer: i32, p: vec3<f32>, n: vec3<f32>, ld: vec3<f32>) -> f32 {
    if (layer < 0) {
        return 1.0;
    }
    // etwas entlang der Normalen versetzen gegen Schattenakne
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
    // rgb linear, a Deckkraft
    @location(6) color: vec4<f32>,
    // x Glanz, y Selbstleuchten (0–1, bzw. > 1 = HDR)
    @location(7) params: vec4<f32>,
};

struct VOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) color: vec4<f32>,
    @location(3) params: vec4<f32>,
    @location(4) local: vec3<f32>,
};

@vertex
fn vs_main(v: VIn) -> VOut {
    let m = mat4x4<f32>(v.m0, v.m1, v.m2, v.m3);
    let w = m * vec4<f32>(v.pos, 1.0);
    // Normale mit der inversen Transponierten (ohne Verschiebung)
    let n3 = mat3x3<f32>(m[0].xyz, m[1].xyz, m[2].xyz);
    let nrm = normalize(transpose(inverse3(n3)) * v.normal);
    var o: VOut;
    o.clip = g.view_proj * w;
    o.world = w.xyz;
    o.normal = nrm;
    o.color = v.color;
    o.params = v.params;
    o.local = v.pos;
    return o;
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

// Lichtkegel eines Scheinwerfers auf einer Fläche
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
    // physikalisch: Abnahme mit dem Quadrat, sanft bis zur Reichweite
    let fall = 1.0 / (1.0 + d * d * 0.15);
    // nur kurz vor der Reichweite ausblenden (Reichweite endet am Hindernis)
    let edge = smoothstep(l.pos.w, l.pos.w - 0.6, d);
    let diff = max(dot(n, ld), 0.0);
    let h = normalize(ld + view);
    let spec = pow(max(dot(n, h), 0.0), mix(8.0, 96.0, gloss)) * gloss;
    let sh = shadow(i32(l.extra.x), p, n, ld);
    return l.color.rgb * cone * fall * edge * (diff + spec) * sh;
}

@fragment
fn fs_scene(i: VOut, @builtin(front_facing) front: bool) -> @location(0) vec4<f32> {
    var n = normalize(i.normal);
    if (!front) {
        n = -n;
    }
    let view = normalize(g.camera.xyz - i.world);
    let base = i.color.rgb;
    // Raumlicht: leicht von oben, damit Formen lesbar bleiben
    let sky = 0.75 + 0.25 * n.y;
    var c = base * (0.02 + g.env.x * 1.4) * sky;
    let count = u32(g.env.w);
    for (var k = 0u; k < count; k = k + 1u) {
        c = c + base * spot(lights[k], i.world, n, view, i.params.x);
    }
    c = c + base * i.params.y * 4.0;
    // Dunst: Ferne wird matter
    let dist = length(g.camera.xyz - i.world);
    let haze = 1.0 - exp(-dist * 0.02 * g.env.z);
    c = mix(c, vec3<f32>(0.02, 0.02, 0.025) * g.env.x, haze);
    return vec4<f32>(c, i.color.a);
}

// ---------- Lichtstrahl (Kegel, additiv) ----------
// Netz: Spitze im Ursprung, Achse −y, Länge 1, Radius 1 am Ende.

@fragment
fn fs_beam(i: VOut) -> @location(0) vec4<f32> {
    let t = clamp(-i.local.y, 0.0, 1.0);
    let view = normalize(g.camera.xyz - i.world);
    // weiche Ränder: am Rand des Kegels (Blick streift) dunkler
    let rim = pow(abs(dot(normalize(i.normal), view)), 1.6);
    // nahe der Linse heller, zum Ende auslaufen
    let along = pow(1.0 - t, 1.8) * smoothstep(0.0, 0.03, t);
    let air = g.env.y * (0.15 + 0.85 * g.env.z);
    let a = rim * along * air * i.color.a;
    return vec4<f32>(i.color.rgb * a, 0.0);
}


// ---------- Schattenkarten (nur Tiefe) ----------

struct ShadowPass {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) var<uniform> sp: ShadowPass;

@vertex
fn vs_shadow(v: VIn) -> @builtin(position) vec4<f32> {
    let m = mat4x4<f32>(v.m0, v.m1, v.m2, v.m3);
    return sp.view_proj * (m * vec4<f32>(v.pos, 1.0));
}
