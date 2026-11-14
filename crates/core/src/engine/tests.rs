#[test]
fn layered_start_keeps_other_scenes() {
    let mut show = Show::default();
    let z = show.zones.first().map(|z| z.id);
    let a = show.add_scene("A", z, vec![crate::show::Step::new(Vec::new())]);
    let b = show.add_scene("B", z, vec![crate::show::Step::new(Vec::new())]);
    let mut e = Engine::new(show);
    e.activate(a);
    e.activate_layered(b);
    assert!(e.is_active(a) && e.is_active(b));
}

use super::*;
use crate::library::builtin_fixture_types;
use crate::show::{ChannelValue, FixtureId, Step};

const MS: fn(u64) -> Duration = Duration::from_millis;

fn cv(fixture: FixtureId, channel: u16, value: u8) -> ChannelValue {
    ChannelValue {
        fixture,
        channel,
        value,
    }
}

/// Show with a D+RGB PAR (addr. 1) and an RGB PAR without dimmer (addr. 10).
fn setup() -> (Show, FixtureId, FixtureId) {
    let mut s = Show::new("T", 2);
    for t in builtin_fixture_types() {
        s.upsert_fixture_type(t);
    }
    let a = s.add_fixture("A", "generic/drgb-4ch", 0, 1).unwrap();
    let b = s.add_fixture("B", "generic/rgb-3ch", 1, 10).unwrap();
    (s, a, b)
}

#[test]
fn virtual_dimmer_scales_colors_and_keeps_ratio() {
    let (s, a, b) = setup();
    assert!(s.virtual_dimmer(b).is_some());
    assert!(s.virtual_dimmer(a).is_none(), "own dimmer");
    let mut e = Engine::new(s);
    let r = ChannelRef {
        fixture: b,
        channel: 0,
    };
    let g = ChannelRef {
        fixture: b,
        channel: 1,
    };
    e.set_live(r, 200);
    e.set_live(g, 100);
    let out = |e: &Engine| {
        let f = e.render_frame();
        (f.get(1, 10).unwrap(), f.get(1, 11).unwrap())
    };
    assert_eq!(out(&e), (200, 100), "without dimmer value: unchanged");
    e.set_live(ChannelRef::virtual_dimmer(b), 128);
    assert_eq!(out(&e), (100, 50));
    e.set_live(ChannelRef::virtual_dimmer(b), 0);
    assert_eq!(out(&e), (0, 0));
    // stored color values are kept
    assert_eq!(e.live().get(&r), Some(&200));
    e.set_live(ChannelRef::virtual_dimmer(b), 255);
    assert_eq!(out(&e), (200, 100));
    // the virtual channel is not output (no address overflow)
    assert_eq!(
        e.output_value(&e.render_frame(), ChannelRef::virtual_dimmer(b)),
        None
    );
}

#[test]
fn super_scene_plays_clips_on_its_timeline() {
    let (mut s, a, _) = setup();
    let inner = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    tl.add_clip(0, crate::timeline::ClipContent::Scene(inner), 1000, 2000);
    tl.add_clip(1, crate::timeline::ClipContent::Scene(inner), 4000, 1000);
    sc.sync_timeline_length();
    assert_eq!(sc.duration_ms(), 5000);
    let mut e = Engine::new(s);
    e.activate(sup);
    e.tick(Duration::from_millis(500));
    assert_eq!(e.render_frame().get(0, 1), Some(0));
    e.tick(Duration::from_millis(1000));
    assert_eq!(e.render_frame().get(0, 1), Some(200));
    e.seek_timeline(sup, 3500.0);
    assert_eq!(e.render_frame().get(0, 1), Some(0));
    assert!((e.timeline_time(sup).unwrap() - 3500.0).abs() < 1.0);
}

#[test]
fn upper_track_wins_in_super_scene() {
    let (mut s, a, _) = setup();
    let top = s.add_scene("Top", None, vec![Step::new(vec![cv(a, 0, 50)])]);
    let below = s.add_scene("Below", None, vec![Step::new(vec![cv(a, 0, 200)])]);
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    tl.add_clip(1, crate::timeline::ClipContent::Scene(below), 0, 2000);
    let id = tl.add_clip(0, crate::timeline::ClipContent::Scene(top), 0, 2000);
    sc.sync_timeline_length();
    let mut e = Engine::new(s.clone());
    e.activate(sup);
    e.tick(Duration::from_millis(500));
    // blending: the brighter value wins
    assert_eq!(e.render_frame().get(0, 1), Some(200));
    // covering: track 1 has priority
    let sc = s.scene_mut(sup).unwrap();
    sc.timeline_mut().unwrap().clip_mut(id).unwrap().layer = crate::timeline::Layer::Cover;
    let mut e = Engine::new(s);
    e.activate(sup);
    e.tick(Duration::from_millis(500));
    assert_eq!(e.render_frame().get(0, 1), Some(50), "track 1 has priority");
}

#[test]
fn a_pulsing_dimmer_effect_does_not_pull_the_position_down() {
    let (mut s, _, _) = setup();
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 1, 100)
        .unwrap();
    let ty = s.type_of(s.fixture(mh).unwrap()).unwrap().clone();
    let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
    let ch = |k: ChannelKind| ChannelRef {
        fixture: mh,
        channel: idx(k),
    };
    let (pan, dim) = (ch(ChannelKind::Pan), ch(ChannelKind::Dimmer));
    // below: static position, full light
    let below: HashMap<ChannelRef, u8> = [(pan, 40), (dim, 255)].into_iter().collect();
    // above: moving position, the dimmer effect is down at 30 just now,
    // the scene's step has full dimmer
    let above: HashMap<ChannelRef, u8> = [(pan, 200), (dim, 30)].into_iter().collect();
    let step: HashMap<FixtureId, f64> = [(mh, 1.0)].into_iter().collect();
    let mut out = HashMap::default();
    Engine::layer(&s, &mut out, below.clone(), 1.0, false, None);
    Engine::layer(&s, &mut out, above.clone(), 1.0, true, Some(&step));
    assert_eq!(out[&pan], 200, "position of the upper block, not mixed");
    // a fading block still crosses back to the position below
    let mut out = HashMap::default();
    Engine::layer(&s, &mut out, below, 1.0, false, None);
    Engine::layer(&s, &mut out, above, 0.5, true, Some(&step));
    assert_eq!(out[&pan], 120);
}

#[test]
fn prepare_moves_heads_in_black_before_the_block() {
    let (mut s, _, _) = setup();
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 1, 100)
        .unwrap();
    let ty = s.type_of(s.fixture(mh).unwrap()).unwrap().clone();
    let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
    let (pan, dim) = (idx(ChannelKind::Pan), idx(ChannelKind::Dimmer));
    let up = s.add_scene(
        "Up",
        None,
        vec![Step::new(vec![cv(mh, pan, 40), cv(mh, dim, 255)])],
    );
    let lit = s.add_scene("Lit", None, vec![Step::new(vec![cv(mh, dim, 100)])]);
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    let c = tl.add_clip(0, crate::timeline::ClipContent::Scene(up), 2000, 1000);
    tl.clip_mut(c).unwrap().prepare = Some(crate::timeline::Prepare::default());
    let l = tl.add_clip(1, crate::timeline::ClipContent::Scene(lit), 0, 300);
    sc.sync_timeline_length();
    let tl = s.scene(sup).unwrap().timeline().unwrap().clone();
    let e = Engine::new(s.clone());
    let at = |t: f64| e.timeline_values(&s, &tl, t, 0);
    let ch = |c: u16| ChannelRef {
        fixture: mh,
        channel: c,
    };
    // before the lead time: nothing
    assert_eq!(at(400.0).get(&ch(pan)), None);
    // during the lead time: pan already there, no light
    let v = at(1000.0);
    assert_eq!(v.get(&ch(pan)), Some(&40));
    assert_eq!(v.get(&ch(dim)), None);
    // only while dark: the lit block keeps the head where it is
    let mut tl2 = tl.clone();
    let lc = tl2.clip_mut(l).unwrap();
    lc.start_ms = 800;
    assert_eq!(e.timeline_values(&s, &tl2, 1000.0, 0).get(&ch(pan)), None);
}

#[test]
fn tracking_aims_heads_at_the_point_and_releases_softly() {
    let mut s = Show::new("T", 1);
    s.fixture_types = builtin_fixture_types();
    let mut heads = Vec::new();
    for (i, x) in [-2.0f32, 2.0].into_iter().enumerate() {
        let id = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1 + 20 * i as u16)
            .unwrap();
        s.fixture_mut(id).unwrap().place3d = Some(crate::venue::Placement {
            position: [x, 4.0, 0.0],
            rotation: [0.0, -90.0, 0.0],
        });
        heads.push(id);
    }
    let tr = s.add_tracker("A", heads.clone());
    let target = [0.5, 1.0, 2.0];
    s.tracker_mut(tr).unwrap().target = target;
    let mut e = Engine::new(s);
    let pan_tilt = |e: &Engine, id: FixtureId| {
        let ty = e.show().type_of(e.show().fixture(id).unwrap()).unwrap();
        let at = |k: ChannelKind| {
            let i = ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
            let f = e.show().fixture(id).unwrap();
            e.render_frame()
                .get(usize::from(f.universe), f.address + i)
                .unwrap()
        };
        (at(ChannelKind::Pan), at(ChannelKind::Tilt))
    };
    let rest = pan_tilt(&e, heads[0]);
    e.show_mut().tracker_mut(tr).unwrap().active = true;
    e.tick(MS(500));
    for id in &heads {
        let want = e.show().aim_moving_head(*id, target).unwrap();
        let lim = e.show().fixture(*id).unwrap().limits;
        let p = FixtureLimits::map16(want.0, lim.pan_min, lim.pan_max, lim.invert_pan);
        let t = FixtureLimits::map16(want.1, lim.tilt_min, lim.tilt_max, lim.invert_tilt);
        assert_eq!(pan_tilt(&e, *id), ((p >> 8) as u8, (t >> 8) as u8));
    }
    // different heads, different angles for the same point
    assert_ne!(pan_tilt(&e, heads[0]), pan_tilt(&e, heads[1]));
    // the target brings its own light
    let dim = |e: &Engine| {
        let f = e.show().fixture(heads[0]).unwrap();
        let ty = e.show().type_of(f).unwrap();
        let i = ty
            .channels
            .iter()
            .position(|c| c.kind == ChannelKind::Dimmer)
            .unwrap() as u16;
        e.render_frame()
            .get(usize::from(f.universe), f.address + i)
            .unwrap()
    };
    // the own dimmer waits until the heads have arrived
    assert_eq!(dim(&e), 0);
    assert!(e.tracking()[&heads[0]].wait_ms > 0.0);
    e.tick(MS(5000));
    e.tick(MS(400));
    assert_eq!(dim(&e), 255);
    e.show_mut().tracker_mut(tr).unwrap().light = false;
    e.tick(MS(10));
    assert_eq!(dim(&e), 0);
    // released: first dark, then back to the scenes, then lit again
    e.show_mut().tracker_mut(tr).unwrap().light = true;
    e.tick(MS(10));
    e.show_mut().tracker_mut(tr).unwrap().active = false;
    let held = pan_tilt(&e, heads[0]);
    e.tick(MS(450));
    assert_eq!(dim(&e), 0, "dark before it moves");
    assert_eq!(pan_tilt(&e, heads[0]), held, "still at the target");
    e.tick(MS(1100));
    assert_eq!(pan_tilt(&e, heads[0]), rest);
    assert!(
        e.tracking()[&heads[0]].wait_ms > 0.0,
        "waits for the way back"
    );
    for _ in 0..10 {
        e.tick(MS(1000));
    }
    assert!(e.tracking().is_empty());
    assert_eq!(pan_tilt(&e, heads[0]), rest);
}

#[test]
fn blending_does_not_depend_on_the_order_of_values() {
    let (s, _, b) = setup();
    let vd = ChannelRef::virtual_dimmer(b);
    let red = ChannelRef {
        fixture: b,
        channel: 0,
    };
    // nothing below, the block fades: the virtual dimmer follows the fade
    // every time (each map has its own random order)
    for _ in 0..64 {
        let mut out = HashMap::default();
        let values: HashMap<ChannelRef, u8> = [(vd, 255), (red, 255)].into_iter().collect();
        Engine::layer(&s, &mut out, values, 0.5, true, None);
        assert_eq!(out[&vd], 128);
        assert_eq!(out[&red], 255);
    }
}

#[test]
fn fading_flash_returns_to_the_color_below() {
    let (mut s, a, b) = setup();
    // a: dimmer + RGB, b: RGB without dimmer
    let blue = s.add_scene(
        "Blue",
        None,
        vec![Step::new(vec![
            cv(a, 0, 255),
            cv(a, 1, 0),
            cv(a, 2, 0),
            cv(a, 3, 255),
            cv(b, 0, 0),
            cv(b, 1, 0),
            cv(b, 2, 255),
        ])],
    );
    // flash: white, then dark (the scene fades out by itself)
    let white = |lvl: u8| {
        vec![
            cv(a, 0, lvl),
            cv(a, 1, 255),
            cv(a, 2, 255),
            cv(a, 3, 255),
            cv(b, 0, lvl),
            cv(b, 1, lvl),
            cv(b, 2, lvl),
        ]
    };
    let flash = s.add_scene(
        "Flash",
        None,
        vec![Step::new(white(255)), Step::new(white(0))],
    );
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    tl.add_clip(1, crate::timeline::ClipContent::Scene(blue), 0, 10_000);
    let top = tl.add_clip(0, crate::timeline::ClipContent::Scene(flash), 0, 10_000);
    tl.clip_mut(top).unwrap().looped = false;
    let mut tl = s.scene(sup).unwrap().timeline().unwrap().clone();
    let e = Engine::new(s.clone());
    let ch = |f: FixtureId, c: u16| ChannelRef {
        fixture: f,
        channel: c,
    };
    let at = |tl: &crate::timeline::Timeline, t: f64| e.timeline_values(&s, tl, t, 0);
    // the flash is white
    let v = at(&tl, 10.0);
    assert_eq!((v[&ch(a, 0)], v[&ch(a, 1)], v[&ch(a, 3)]), (255, 255, 255));
    assert_eq!((v[&ch(b, 0)], v[&ch(b, 2)]), (255, 255));
    // after the flash: blue again, not dark
    let v = at(&tl, 9_000.0);
    assert_eq!(
        (v[&ch(a, 0)], v[&ch(a, 1)], v[&ch(a, 2)], v[&ch(a, 3)]),
        (255, 0, 0, 255)
    );
    assert_eq!((v[&ch(b, 0)], v[&ch(b, 1)], v[&ch(b, 2)]), (0, 0, 255));
    // covering: dark covers the blue
    tl.clip_mut(top).unwrap().layer = crate::timeline::Layer::Cover;
    assert_eq!(at(&tl, 9_000.0)[&ch(a, 0)], 0);
    // the block's own fade out goes back to the blue as well
    let c = tl.clip_mut(top).unwrap();
    c.looped = true;
    c.fade_out_ms = 1000;
    c.duration_ms = 1000;
    let v = at(&tl, 990.0);
    assert!(v[&ch(a, 0)] > 240 && v[&ch(a, 1)] < 20, "{v:?}");
}

#[test]
fn scene_group_block_plays_members_with_offsets() {
    let (mut s, a, _) = setup();
    let one = s.add_scene("One", None, vec![Step::new(vec![cv(a, 0, 100)])]);
    let two = s.add_scene(
        "Two",
        None,
        vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 50)])],
    );
    s.scene_mut(one).unwrap().steps[0].hold_ms = 1000;
    s.scene_mut(two).unwrap().steps[0].hold_ms = 1000;
    let g = s.add_scene_group("G");
    s.scene_group_mut(g).unwrap().members = vec![
        crate::scene_group::GroupMember::new(one, 500),
        crate::scene_group::GroupMember::new(two, 0),
    ];
    assert_eq!(s.scene_group_length(g), 1500);
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    let tl = sc.timeline_mut().unwrap();
    tl.add_group_clip(0, g, 1500, 0);
    let tl = s.scene(sup).unwrap().timeline().unwrap().clone();
    let e = Engine::new(s.clone());
    let ch = |c: u16| ChannelRef {
        fixture: a,
        channel: c,
    };
    // only Two at first, then One wins on channel 0 (first member)
    let v = e.timeline_values(&s, &tl, 200.0, 0);
    assert_eq!(v.get(&ch(0)), Some(&200));
    let v = e.timeline_values(&s, &tl, 700.0, 0);
    assert_eq!(v.get(&ch(0)), Some(&100));
    assert_eq!(v.get(&ch(1)), Some(&50));
    // own length and fade-out of a member
    s.scene_group_mut(g).unwrap().members[1].duration_ms = Some(400);
    s.scene_group_mut(g).unwrap().members[1].fade_out_ms = 200;
    let e = Engine::new(s.clone());
    let v = e.timeline_values(&s, &tl, 300.0, 0);
    assert_eq!(v.get(&ch(1)), Some(&50), "channel 1 is no intensity");
    assert_eq!(e.timeline_values(&s, &tl, 450.0, 0).get(&ch(0)), None);
    // pre-position of a group block: the starting values of its scenes
    let mut s2 = s.clone();
    s2.scene_group_mut(g).unwrap().members[0].offset_ms = 0;
    let mut tl2 = tl.clone();
    let cid = tl2.tracks[0].clips[0].id;
    let c = tl2.clip_mut(cid).unwrap();
    c.start_ms = 3000;
    c.prepare = Some(crate::timeline::Prepare {
        position: false,
        beam: false,
        ..Default::default()
    });
    assert!(
        !tl2.preparing_at(2000).is_empty(),
        "group blocks prepare too"
    );
    // deleting a scene removes it from the group
    s.remove_scene(two);
    assert_eq!(s.scene_group(g).unwrap().members.len(), 1);
}

#[test]
fn finished_super_scene_restarts_on_play() {
    let (mut s, a, _) = setup();
    let inner = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
    let sup = s.add_scene("S", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(sup).unwrap();
    sc.add_effect(crate::show::EffectKind::SuperScene);
    sc.loop_mode = crate::show::LoopMode::Once;
    let tl = sc.timeline_mut().unwrap();
    tl.add_clip(0, crate::timeline::ClipContent::Scene(inner), 0, 1000);
    sc.sync_timeline_length();
    let mut e = Engine::new(s);
    e.toggle_timeline(sup, 0.0);
    e.tick(Duration::from_millis(1500));
    assert!(!e.timeline_running(sup), "finished");
    e.toggle_timeline(sup, 0.0);
    assert!(e.timeline_running(sup), "restarts");
    e.tick(Duration::from_millis(300));
    assert!(e.timeline_time(sup).unwrap() < 500.0);
    e.toggle_timeline(sup, 0.0);
    assert!(!e.timeline_running(sup), "pausiert");
}

#[test]
fn end_scene_follows_jump_to() {
    let (mut s, a, _) = setup();
    let next = s.add_scene("B", None, vec![Step::new(vec![cv(a, 0, 50)])]);
    let first = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 200)])]);
    s.scene_mut(first).unwrap().settings.jump_to_scenes = vec![next];
    let mut e = Engine::new(s);
    e.activate(first);
    e.end_scene(first);
    assert!(!e.is_active(first));
    assert!(e.is_active(next));
}

#[test]
fn color_fx_drives_rgb_and_moves() {
    let (mut s, a, b) = setup();
    let id = s.add_scene("FX", None, vec![Step::new(vec![cv(a, 0, 255)])]);
    let sc = s.scene_mut(id).unwrap();
    let fx = sc.add_effect(crate::show::EffectKind::ColorFx);
    sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = vec![a, b];
    let mut e = Engine::new(s);
    e.activate(id);
    let rgb = |f: &DmxFrame| (f.get(0, 2), f.get(0, 3), f.get(0, 4), f.get(1, 10));
    let first = rgb(&e.render_frame());
    e.tick(Duration::from_millis(300));
    let later = rgb(&e.render_frame());
    assert_ne!(first, later, "colors run");
    assert_eq!(
        e.render_frame().get(0, 1),
        Some(255),
        "Dimmer from the step"
    );
}

#[test]
fn preview_effects_only_run_when_active() {
    let (mut s, a, b) = setup();
    let id = s.add_scene("FX", None, vec![Step::new(vec![cv(a, 0, 255)])]);
    let sc = s.scene_mut(id).unwrap();
    let fx = sc.add_effect(crate::show::EffectKind::ColorFx);
    sc.effects.iter_mut().find(|e| e.id == fx).unwrap().fixtures = vec![a, b];
    let mut e = Engine::new(s);
    let rgb = |e: &Engine| {
        let f = e.render_monitor();
        (f.get(0, 2), f.get(0, 3), f.get(0, 4), f.get(1, 10))
    };
    // edit mode, scene not active: effect shown, but standing still
    e.set_preview(vec![id]);
    let first = rgb(&e);
    e.tick(Duration::from_millis(300));
    assert_eq!(first, rgb(&e), "inactive: effect stands still");
    // active: runs
    e.activate(id);
    e.tick(Duration::from_millis(300));
    let running = rgb(&e);
    assert_ne!(first, running, "active: effect runs");
    // paused: stands still
    e.toggle_pause(id);
    e.tick(Duration::from_millis(300));
    assert_eq!(running, rgb(&e), "paused: effect stands still");
}

#[test]
fn limits_leave_rest_position_alone() {
    use crate::show::FixtureLimits;
    let mut show = Show::new("t", 1);
    show.fixture_types = crate::library::builtin_fixture_types();
    let id = show
        .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
        .unwrap();
    show.fixture_mut(id).unwrap().limits = FixtureLimits {
        invert_pan: true,
        invert_tilt: true,
        ..FixtureLimits::default()
    };
    let mut e = Engine::new(show);
    // nothing active: base values unchanged (pan/tilt 128, not mirrored to 127)
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(128));
    // set by the programmer: mirrored
    e.set_programmer(
        ChannelRef {
            fixture: id,
            channel: 0,
        },
        0,
    );
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(255));
}

#[test]
fn defaults_are_rendered() {
    let mut s = Show::new("T", 1);
    for t in builtin_fixture_types() {
        s.upsert_fixture_type(t);
    }
    s.add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
        .unwrap();
    let e = Engine::new(s);
    let f = e.render_frame();
    assert_eq!(f.get(0, 100), Some(128)); // Pan
    assert_eq!(f.get(0, 102), Some(128)); // Tilt
    assert_eq!(f.get(0, 105), Some(0)); // Dimmer
}

#[test]
fn removing_steps_while_playing_does_not_panic() {
    let (mut s, a, _) = setup();
    let id = s.add_scene(
        "S",
        None,
        vec![
            Step::new(vec![cv(a, 0, 255)]),
            Step::new(vec![cv(a, 0, 100)]),
            Step::new(vec![cv(a, 0, 50)]),
        ],
    );
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(Duration::from_millis(2_100));
    e.show_mut().scene_mut(id).unwrap().steps = vec![Step::new(Vec::new())];
    e.tick(Duration::from_millis(20));
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(0));
    assert!(e.progress(id).is_some());
}

#[test]
fn scene_values_land_on_correct_addresses() {
    let (mut s, a, b) = setup();
    let id = s.add_scene(
        "S",
        None,
        vec![Step::new(vec![cv(a, 0, 255), cv(a, 1, 200), cv(b, 2, 50)])],
    );
    let mut e = Engine::new(s);
    e.activate(id);
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(255));
    assert_eq!(f.get(0, 2), Some(200));
    assert_eq!(f.get(1, 12), Some(50));
}

#[test]
fn latest_scene_wins_for_stored_channels() {
    let (mut s, a, _) = setup();
    let s1 = s.add_scene(
        "1",
        None,
        vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 10)])],
    );
    let s2 = s.add_scene(
        "2",
        None,
        vec![Step::new(vec![cv(a, 0, 100), cv(a, 1, 99)])],
    );
    let mut e = Engine::new(s);
    e.activate(s1);
    e.activate(s2);
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(100), "Dimmer LTP");
    assert_eq!(f.get(0, 2), Some(99), "Rot LTP");
    e.activate(s1); // restarted -> now the newest
    assert_eq!(e.render_frame().get(0, 2), Some(10));
}

#[test]
fn unstored_channels_stay_free() {
    // The red scene stores only red, the blue scene only blue -> both visible.
    let (mut s, a, _) = setup();
    let red = s.add_scene("R", None, vec![Step::new(vec![cv(a, 1, 255)])]);
    let blue = s.add_scene("B", None, vec![Step::new(vec![cv(a, 3, 255)])]);
    let mut e = Engine::new(s);
    e.activate(red);
    e.activate(blue);
    let f = e.render_frame();
    assert_eq!(f.get(0, 2), Some(255));
    assert_eq!(f.get(0, 4), Some(255));
    // if the blue scene stores red = 0, it wins (started last)
    e.show_mut().scene_mut(blue).unwrap().steps[0]
        .values
        .push(cv(a, 1, 0));
    assert_eq!(e.render_frame().get(0, 2), Some(0));
}

#[test]
fn zones_are_exclusive() {
    let (mut s, a, _) = setup();
    let z = s.add_zone("Color");
    let s1 = s.add_scene("Red", Some(z), vec![Step::new(vec![cv(a, 1, 255)])]);
    let s2 = s.add_scene("Blue", Some(z), vec![Step::new(vec![cv(a, 3, 255)])]);
    let free = s.add_scene("Free", None, vec![Step::new(vec![cv(a, 0, 255)])]);
    let mut e = Engine::new(s);
    e.activate(free);
    e.activate(s1);
    e.activate(s2);
    assert!(!e.is_active(s1));
    assert!(e.is_active(s2));
    assert!(e.is_active(free));
    e.toggle(s2);
    assert!(!e.is_active(s2));
}

#[test]
fn fades_between_steps_and_loops() {
    let (mut s, a, _) = setup();
    let mut st1 = Step::new(vec![cv(a, 1, 0)]);
    st1.fade_ms = 0;
    st1.hold_ms = 100;
    let mut st2 = Step::new(vec![cv(a, 1, 200)]);
    st2.fade_ms = 1000;
    st2.hold_ms = 0;
    let id = s.add_scene("Fade", None, vec![st1, st2]);
    let mut e = Engine::new(s);
    e.activate(id);
    assert_eq!(e.render_frame().get(0, 2), Some(0));
    e.tick(MS(100)); // Step 2 starts
    assert_eq!(e.progress(id).unwrap().step, 1);
    e.tick(MS(500)); // Half of the crossfade
    assert_eq!(e.render_frame().get(0, 2), Some(100));
    e.tick(MS(500)); // End -> loop to step 1 (fade 0)
    assert_eq!(e.progress(id).unwrap().step, 0);
    assert_eq!(e.render_frame().get(0, 2), Some(0));
}

#[test]
fn first_step_fades_intensity_in() {
    let (mut s, a, _) = setup();
    let mut st = Step::new(vec![cv(a, 0, 200), cv(a, 1, 80)]);
    st.fade_ms = 1000;
    let id = s.add_scene("In", None, vec![st]);
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(MS(250));
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(50), "Dimmer fades in");
    assert_eq!(f.get(0, 2), Some(80), "color jumps");
}

#[test]
fn once_mode_stops_on_last_step() {
    let (mut s, a, _) = setup();
    let id = s.add_scene(
        "Once",
        None,
        vec![Step::new(vec![cv(a, 1, 1)]), Step::new(vec![cv(a, 1, 2)])],
    );
    s.scene_mut(id).unwrap().loop_mode = LoopMode::Once;
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(MS(10_000));
    let p = e.progress(id).unwrap();
    assert!(p.finished);
    assert_eq!(p.step, 1);
    assert_eq!(e.render_frame().get(0, 2), Some(2));
}

#[test]
fn finished_scene_holds_effects_at_their_end() {
    let (mut s, a, _) = setup();
    let id = s.add_scene("Once", None, vec![Step::new(vec![cv(a, 1, 1)])]);
    let sc = s.scene_mut(id).unwrap();
    sc.loop_mode = LoopMode::Once;
    sc.steps[0].hold_ms = 2000;
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(MS(1000));
    assert!((e.fx_cycles(id) - 0.5).abs() < 0.01, "running");
    for _ in 0..10 {
        e.tick(MS(700));
    }
    assert!(e.progress(id).unwrap().finished);
    let c = e.fx_cycles(id);
    assert!(c < 1.0 && c > 0.999, "held just before the wrap: {c}");
}

#[test]
fn move_hold_does_not_stop_a_playing_scene() {
    let (mut s, _, _) = setup();
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
        .unwrap();
    let id = s.add_scene("Hold", None, vec![Step::new(Vec::new())]);
    let sc = s.scene_mut(id).unwrap();
    let fx = sc.add_effect(crate::show::EffectKind::MoveFx);
    sc.effects[0].fixtures = vec![mh];
    let mut e = Engine::new(s);
    e.set_move_hold(Some((id, fx, [0.1, 0.1])));
    let scene = e.show().scene(id).unwrap().clone();
    let fx_of = |e: &Engine| {
        let mut out = HashMap::default();
        e.apply_effects(e.show(), &scene, 0.25, &mut out);
        out
    };
    let held = fx_of(&e);
    e.activate(id);
    assert_ne!(fx_of(&e), held, "playing: the hold is ignored");
}

#[test]
fn zero_length_steps_do_not_hang() {
    let (mut s, a, _) = setup();
    let mut st = Step::new(vec![cv(a, 1, 1)]);
    st.hold_ms = 0;
    let mut st2 = st.clone();
    st2.values[0].value = 2;
    let id = s.add_scene("Z", None, vec![st, st2]);
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(Duration::from_secs(3600)); // must not hang
    assert!(e.is_active(id));
}

#[test]
fn crossfade_is_a_morph() {
    let (mut s, a, _) = setup();
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
        .unwrap();
    let ty = s.type_of(s.fixture(mh).unwrap()).unwrap();
    let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
    let (dim, pan) = (idx(ChannelKind::Dimmer), idx(ChannelKind::Pan));
    let addr = |c: u16| 100 + c;
    // A: lamp a full; B: the same lamp the same + a moving head from dark
    let sa = s.add_scene("A", None, vec![Step::new(vec![cv(a, 0, 255)])]);
    let sb = s.add_scene(
        "B",
        None,
        vec![Step::new(vec![
            cv(a, 0, 255),
            cv(mh, dim, 255),
            cv(mh, pan, 200),
        ])],
    );
    for id in [sa, sb] {
        let st = &mut s.scene_mut(id).unwrap().settings;
        st.fade_in_ms = 1000;
        st.fade_out_ms = 1000;
    }
    let mut e = Engine::new(s);
    e.activate(sa);
    e.tick(Duration::from_millis(1500));
    e.activate(sb);
    e.tick(Duration::from_millis(500));
    let f = e.render_frame();
    assert_eq!(
        f.get(0, 1),
        Some(255),
        "same value: no dip in the crossfade"
    );
    assert_eq!(f.get(0, addr(pan)), Some(200), "dark head stands at once");
    assert!(
        f.get(0, addr(dim)).unwrap() < 200,
        "only its dimmer fades in"
    );
    // B goes: the head keeps its position while it fades out
    e.release(sb);
    e.tick(Duration::from_millis(500));
    let f = e.render_frame();
    assert_eq!(
        f.get(0, addr(pan)),
        Some(200),
        "no swing back while fading out"
    );
    assert!(f.get(0, addr(dim)).unwrap() < 200);
}

#[test]
fn strobe_channel_does_not_fade() {
    let (mut s, _, _) = setup();
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 100)
        .unwrap();
    let ty = s.type_of(s.fixture(mh).unwrap()).unwrap();
    let idx = |k: ChannelKind| ty.channels.iter().position(|c| c.kind == k).unwrap() as u16;
    let (dim, strobe) = (idx(ChannelKind::Dimmer), idx(ChannelKind::Strobe));
    let addr = |c: u16| 100 + c;
    let id = s.add_scene(
        "S",
        None,
        vec![Step::new(vec![cv(mh, dim, 255), cv(mh, strobe, 255)])],
    );
    s.scene_mut(id).unwrap().settings.fade_in_ms = 1000;
    s.scene_mut(id).unwrap().settings.fade_out_ms = 1000;
    let mut e = Engine::new(s);
    e.activate(id);
    e.tick(Duration::from_millis(300));
    let f = e.render_frame();
    assert_eq!(f.get(0, addr(strobe)), Some(255), "open from the start");
    assert!(f.get(0, addr(dim)).unwrap() < 255, "dimmer fades in");
    e.tick(Duration::from_millis(1000));
    e.release(id);
    e.tick(Duration::from_millis(700));
    let f = e.render_frame();
    assert_eq!(f.get(0, addr(strobe)), Some(255), "stays open");
    assert!(f.get(0, addr(dim)).unwrap() < 255, "dimmer fades out");
    e.tick(Duration::from_millis(400));
    assert_eq!(e.render_frame().get(0, addr(dim)), Some(0));
}

#[test]
fn scene_dimmer_fades_and_seek() {
    let (mut s, a, _) = setup();
    let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 200)])]);
    s.scene_mut(id).unwrap().settings.dimmer = 128;
    s.scene_mut(id).unwrap().settings.fade_in_ms = 1000;
    s.scene_mut(id).unwrap().settings.fade_out_ms = 1000;
    let mut e = Engine::new(s);
    e.activate(id);
    assert_eq!(e.render_frame().get(0, 1), Some(0), "Fade-in starts at 0");
    e.tick(Duration::from_millis(500));
    assert_eq!(
        e.render_frame().get(0, 1),
        Some(50),
        "half faded in, dimmer 50 %"
    );
    e.tick(Duration::from_millis(600));
    assert_eq!(e.render_frame().get(0, 1), Some(100));
    e.release(id);
    assert!(!e.is_active(id));
    e.tick(Duration::from_millis(500));
    assert_eq!(e.render_frame().get(0, 1), Some(50), "fades out");
    e.tick(Duration::from_millis(600));
    assert_eq!(e.render_frame().get(0, 1), Some(0));
    assert!(e.playing.is_empty());

    // Seek within a sequence
    let (mut s, a, _) = setup();
    let mut st = Step::new(vec![cv(a, 1, 1)]);
    st.hold_ms = 1000;
    let mut st2 = st.clone();
    st2.values[0].value = 2;
    let id = s.add_scene("Z", None, vec![st, st2]);
    let mut e = Engine::new(s);
    e.activate(id);
    e.seek(id, 0.75);
    assert_eq!(e.render_frame().get(0, 2), Some(2));
    assert!((e.scene_position(id).unwrap() - 0.75).abs() < 0.01);
}

#[test]
fn reverse_plays_steps_backwards() {
    let (mut s, a, _) = setup();
    let steps: Vec<Step> = (1..=3)
        .map(|v| {
            let mut st = Step::new(vec![cv(a, 1, v)]);
            st.hold_ms = 100;
            st
        })
        .collect();
    let id = s.add_scene("R", None, steps);
    s.scene_mut(id).unwrap().settings.reverse = true;
    let mut e = Engine::new(s);
    e.activate(id);
    assert_eq!(e.render_frame().get(0, 2), Some(3));
    e.tick(Duration::from_millis(110));
    assert_eq!(e.render_frame().get(0, 2), Some(2));
    e.tick(Duration::from_millis(100));
    e.tick(Duration::from_millis(100));
    assert_eq!(e.render_frame().get(0, 2), Some(3), "Loop backwards");
}

#[test]
fn release_modes_and_protection() {
    use crate::show::BankScope;
    let (mut s, a, _) = setup();
    let z1 = s.add_zone("1");
    let z2 = s.add_zone("2");
    let one = |s: &mut Show, z, v| s.add_scene("x", Some(z), vec![Step::new(vec![cv(a, 1, v)])]);
    let a1 = one(&mut s, z1, 1);
    let a2 = one(&mut s, z1, 2);
    let b1 = one(&mut s, z2, 3);
    let mut e = Engine::new(s);
    // Default: bank -> only scenes of the same bank
    e.activate(a1);
    e.activate(b1);
    e.activate(a2);
    assert!(!e.is_active(a1) && e.is_active(a2) && e.is_active(b1));
    // All
    e.show_mut().scene_mut(a1).unwrap().settings.release_mode = BankScope::All;
    e.activate(a1);
    assert!(e.is_active(a1) && !e.is_active(a2) && !e.is_active(b1));
    // Protection: b1 cannot be released from outside its bank
    e.show_mut().scene_mut(b1).unwrap().settings.protect = BankScope::OutsideBank;
    e.activate(b1);
    e.activate(a1);
    assert!(e.is_active(b1));
    // Off
    e.show_mut().scene_mut(a2).unwrap().settings.release_mode = BankScope::Off;
    e.activate(a2);
    assert!(e.is_active(a1) && e.is_active(a2));
}

#[test]
fn priority_beats_start_order() {
    let (mut s, a, _) = setup();
    let hi = s.add_scene("hi", None, vec![Step::new(vec![cv(a, 1, 200)])]);
    let lo = s.add_scene("lo", None, vec![Step::new(vec![cv(a, 1, 50)])]);
    s.scene_mut(hi).unwrap().settings.priority = 5;
    let mut e = Engine::new(s);
    e.activate(hi);
    e.activate(lo);
    assert_eq!(e.render_frame().get(0, 2), Some(200));
}

#[test]
fn repeat_then_release_and_jump() {
    let (mut s, a, _) = setup();
    let mut st = Step::new(vec![cv(a, 1, 9)]);
    st.hold_ms = 100;
    let first = s.add_scene("1", None, vec![st]);
    let next = s.add_scene("2", None, vec![Step::new(vec![cv(a, 1, 7)])]);
    {
        let sc = s.scene_mut(first).unwrap();
        sc.loop_mode = LoopMode::Once;
        sc.settings.repeat_count = 2;
        sc.settings.release_at_end = true;
        sc.settings.jump_to_scenes = vec![next];
    }
    let mut e = Engine::new(s);
    e.activate(first);
    e.tick(Duration::from_millis(150));
    assert!(e.is_active(first), "first of two runs");
    e.tick(Duration::from_millis(100));
    assert!(!e.is_active(first));
    assert!(e.is_active(next));
}

#[test]
fn blind_keeps_output_but_monitor_shows_edits() {
    let (mut s, a, _) = setup();
    let red = s.add_scene("Red", None, vec![Step::new(vec![cv(a, 1, 255)])]);
    let mut e = Engine::new(s);
    e.activate(red);
    e.set_blind(true);
    // Edit the scene and set the programmer: the output stays
    e.show_mut().scene_mut(red).unwrap().steps[0].values[0].value = 10;
    e.set_programmer(
        ChannelRef {
            fixture: a,
            channel: 3,
        },
        200,
    );
    let out = e.render_frame();
    assert_eq!((out.get(0, 2), out.get(0, 4)), (Some(255), Some(0)));
    let mon = e.render_monitor();
    assert_eq!((mon.get(0, 2), mon.get(0, 4)), (Some(10), Some(200)));
    e.set_blind(false);
    assert_eq!(e.render_frame().get(0, 2), Some(10));
}

#[test]
fn live_overrides_scenes_and_resets() {
    let (mut s, a, _) = setup();
    let red = s.add_scene(
        "Red",
        None,
        vec![Step::new(vec![cv(a, 1, 255), cv(a, 2, 0), cv(a, 3, 0)])],
    );
    let mut e = Engine::new(s);
    e.activate(red);
    let blue = ChannelRef {
        fixture: a,
        channel: 3,
    };
    e.set_live(blue, 255);
    let f = e.render_frame();
    assert_eq!((f.get(0, 2), f.get(0, 4)), (Some(255), Some(255)));
    e.clear_live();
    assert_eq!(e.render_frame().get(0, 4), Some(0));
}

#[test]
fn preview_outputs_inactive_scene() {
    let (mut s, a, _) = setup();
    let id = s.add_scene("P", None, vec![Step::new(vec![cv(a, 1, 77)])]);
    let mut e = Engine::new(s);
    let ch = ChannelRef {
        fixture: a,
        channel: 1,
    };
    let before = e.render_frame();
    e.set_preview(vec![id]);
    let f = e.render_frame();
    assert_eq!(e.output_value(&f, ch), Some(77));
    assert!(!e.is_active(id));
    e.set_preview(vec![]);
    assert_eq!(e.render_frame(), before);
}

#[test]
fn programmer_overrides_and_master_scales() {
    let (mut s, a, b) = setup();
    let id = s.add_scene(
        "S",
        None,
        vec![Step::new(vec![cv(a, 0, 200), cv(a, 1, 200), cv(b, 0, 200)])],
    );
    let mut e = Engine::new(s);
    e.activate(id);
    e.set_programmer(
        ChannelRef {
            fixture: a,
            channel: 0,
        },
        100,
    );
    e.set_master(0.5);
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(50), "Programmer 100 * Master 0.5");
    assert_eq!(
        f.get(0, 2),
        Some(200),
        "Color unchanged on fixture with dimmer"
    );
    assert_eq!(
        f.get(1, 10),
        Some(100),
        "Fixture without dimmer: color scaled"
    );
    e.set_blackout(true);
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(0));
    assert_eq!(f.get(1, 10), Some(0));
}

#[test]
fn pause_and_step_jumps() {
    let (mut s, a, _) = setup();
    let steps = (0..3u8)
        .map(|i| {
            let mut st = Step::new(vec![cv(a, 1, i * 10)]);
            st.hold_ms = 1000;
            st
        })
        .collect();
    let z = s.add_zone("Z");
    let id = s.add_scene("S", Some(z), steps);
    let mut e = Engine::new(s);
    e.activate(id);
    assert_eq!(e.active_in_zone(z), Some(id));
    e.toggle_pause(id);
    e.tick(MS(5000));
    assert_eq!(e.progress(id).unwrap().step, 0, "pausiert");
    e.jump_step(id, 1);
    assert_eq!(e.render_frame().get(0, 2), Some(10));
    e.jump_step(id, -2);
    assert_eq!(
        e.progress(id).unwrap().step,
        2,
        "backwards with wrap-around"
    );
    e.toggle_pause(id);
    e.tick(MS(1000));
    assert_eq!(e.progress(id).unwrap().step, 0);
}

#[test]
fn limits_cap_dimmer_and_position() {
    let mut s = Show::new("T", 1);
    for t in builtin_fixture_types() {
        s.upsert_fixture_type(t);
    }
    let mh = s
        .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
        .unwrap();
    // Channels: 0 pan, 1 pan fine, 2 tilt, 3 tilt fine, 5 dimmer
    let id = s.add_scene(
        "S",
        None,
        vec![Step::new(vec![
            cv(mh, 0, 255),
            cv(mh, 1, 255),
            cv(mh, 2, 0),
            cv(mh, 5, 200),
        ])],
    );
    {
        let l = &mut s.fixture_mut(mh).unwrap().limits;
        l.dimmer_max = 128;
        l.pan_min = 64;
        l.pan_max = 191;
        l.invert_tilt = true;
    }
    let mut e = Engine::new(s);
    e.activate(id);
    let f = e.render_frame();
    assert_eq!(f.get(0, 6), Some(100), "Dimmer 200 * 128/255");
    e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 64;
    assert_eq!(
        e.render_frame().get(0, 6),
        Some((64 + 200u32 * 64 / 255) as u8),
        "range 64..128"
    );
    e.show_mut().fixture_mut(mh).unwrap().limits.dimmer_min = 0;
    assert_eq!(f.get(0, 1), Some(191), "pan full -> pan max");
    assert_eq!(f.get(0, 2), Some(255), "pan fine at the upper end");
    assert_eq!(f.get(0, 3), Some(255), "tilt 0 inverted -> full");
    // Swap
    e.show_mut().fixture_mut(mh).unwrap().limits = FixtureLimits {
        swap_pan_tilt: true,
        ..Default::default()
    };
    let f = e.render_frame();
    assert_eq!(f.get(0, 1), Some(0), "pan gets the tilt value");
    assert_eq!(f.get(0, 3), Some(255), "tilt gets the pan value");
}

#[test]
fn map16_ranges() {
    assert_eq!(FixtureLimits::map16(0, 0, 255, false), 0);
    assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, false), u16::MAX);
    assert_eq!(FixtureLimits::map16(u16::MAX, 0, 255, true), 0);
    assert_eq!(FixtureLimits::map16(0, 64, 191, false) >> 8, 64);
}

#[test]
fn speed_multiplies_time() {
    let (mut s, a, _) = setup();
    let mut st = Step::new(vec![cv(a, 0, 200)]);
    st.fade_ms = 1000;
    let id = s.add_scene("S", None, vec![st]);
    let mut e = Engine::new(s);
    e.set_speed(2.0);
    e.activate(id);
    e.tick(MS(250));
    assert_eq!(e.render_frame().get(0, 1), Some(100));
}

#[test]
fn sync_drops_removed_scenes_and_fixtures() {
    let (mut s, a, _) = setup();
    let id = s.add_scene("S", None, vec![Step::new(vec![cv(a, 0, 1)])]);
    let mut e = Engine::new(s);
    e.activate(id);
    e.set_programmer(
        ChannelRef {
            fixture: a,
            channel: 0,
        },
        5,
    );
    e.show_mut().remove_scene(id);
    e.show_mut().remove_fixture(a);
    e.sync();
    assert!(e.active_scenes().is_empty());
    assert!(e.programmer().is_empty());
}
