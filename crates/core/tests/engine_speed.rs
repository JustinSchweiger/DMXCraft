//! Speed guard for the engine: a large show (4 universes, 216 fixtures,
//! scenes with effects, a SUPER SCENE with blend blocks, crossfades) must
//! render well within one output tick (20 ms at 50 Hz; debug builds get
//! 10×). The limit is generous for slow CI machines; it catches work that
//! grows out of hand (e.g. a scan over all channels per fixture), not small
//! changes. `cargo test --release -p dmxcraft-core --test engine_speed --
//! --nocapture` prints the time.

use std::time::{Duration, Instant};

use dmxcraft_core::library::builtin_fixture_types;
use dmxcraft_core::show::EffectKind;
use dmxcraft_core::timeline::{ClipContent, Layer};
use dmxcraft_core::{ChannelValue, DmxFrame, Engine, Show, Step};

fn big_show() -> (Show, Vec<dmxcraft_core::SceneId>, dmxcraft_core::SceneId) {
    let mut s = Show::new("Speed", 4);
    for t in builtin_fixture_types() {
        s.upsert_fixture_type(t);
    }
    let mut pars = Vec::new();
    let mut heads = Vec::new();
    // 200 RGBW pars (6 ch) and 16 moving heads (11 ch) over 4 universes
    for i in 0..200u16 {
        let (u, a) = ((i / 80) as u8, (i % 80) * 6 + 1);
        pars.push(
            s.add_fixture(format!("Par {i}"), "generic/rgbw-par-6ch", u, a)
                .unwrap(),
        );
    }
    for i in 0..16u16 {
        heads.push(
            s.add_fixture(
                format!("MH {i}"),
                "generic/moving-head-spot-11ch",
                3,
                100 + i * 11,
            )
            .unwrap(),
        );
    }
    let all = |chs: &[(u16, u8)], fx: &[dmxcraft_core::FixtureId]| -> Vec<ChannelValue> {
        fx.iter()
            .flat_map(|f| {
                chs.iter().map(|&(channel, value)| ChannelValue {
                    fixture: *f,
                    channel,
                    value,
                })
            })
            .collect()
    };
    let mut scenes = Vec::new();
    for n in 0..12u8 {
        let mut values = all(&[(0, 255), (1, n * 20), (2, 255 - n * 20), (3, 80)], &pars);
        values.extend(all(&[(0, 128), (2, 64), (5, 255)], &heads));
        let id = s.add_scene(format!("S{n}"), None, vec![Step::new(values)]);
        let sc = s.scene_mut(id).unwrap();
        sc.settings.fade_in_ms = 800;
        sc.settings.fade_out_ms = 800;
        if n % 3 == 0 {
            let fx = sc.add_effect(EffectKind::ColorFx);
            sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = pars.clone();
        }
        if n % 4 == 1 {
            let fx = sc.add_effect(EffectKind::MoveFx);
            sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = heads.clone();
        }
        scenes.push(id);
    }
    let sup = s.add_scene("Super", None, vec![Step::new(Vec::new())]);
    let blocks: Vec<_> = scenes.iter().take(6).copied().collect();
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    for (i, id) in blocks.iter().enumerate() {
        let c = tl.add_clip(i % 4, ClipContent::Scene(*id), (i as u64) * 500, 20_000);
        if i % 2 == 1 {
            tl.clip_mut(c).unwrap().layer = Layer::Blend;
        }
    }
    sc.sync_timeline_length();
    (s, scenes, sup)
}

#[test]
fn large_show_renders_within_a_tick() {
    let (show, scenes, sup) = big_show();
    let mut e = Engine::new(show);
    e.activate(sup);
    for id in &scenes[6..12] {
        e.activate(*id);
    }
    // let the scenes fade in, then cross-fade two of them away
    for _ in 0..20 {
        e.tick(Duration::from_millis(20));
    }
    e.activate(scenes[0]);
    e.activate(scenes[1]);
    let mut frame = DmxFrame::new(4);
    let n = 50;
    let t = Instant::now();
    for _ in 0..n {
        e.tick(Duration::from_millis(20));
        e.render(&mut frame);
    }
    let per = t.elapsed() / n;
    println!("tick + render: {per:?} per frame");
    // release about 1.6 ms, debug about 20 ms on a laptop core; the debug
    // limit is 10× slower code than the release one
    let limit = if cfg!(debug_assertions) {
        Duration::from_millis(200)
    } else {
        Duration::from_millis(20)
    };
    assert!(
        per < limit,
        "a frame of the large show takes {per:?} (limit {limit:?})"
    );
}
