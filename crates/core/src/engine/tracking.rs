//! LIVE TRACKING in the engine: aiming following fixtures at their targets,
//! travel times, writing pan/tilt and the tracker's own light.

#[allow(unused_imports)]
use super::*;

impl Engine {
    /// Convenience function: create a new frame.
    /// LIVE TRACKING: aim the fixtures of following trackers at their
    /// points; released ones fade back to the other sources.
    pub(super) fn update_tracking(&mut self, real_ms: f64) {
        type Wanted = ([f32; 3], Option<TrackLight>, Option<u8>);
        let mut wanted: HashMap<FixtureId, Wanted> = HashMap::default();
        for t in self.show.trackers.iter().filter(|t| t.active) {
            let light = t.light.then(|| TrackLight {
                dimmer: (t.dimmer.clamp(0.0, 1.0) * 255.0).round() as u8,
                color: t.color,
                cells: t.cells,
            });
            let zoom = t
                .own_zoom
                .then(|| (t.zoom.clamp(0.0, 1.0) * 255.0).round() as u8);
            for f in &t.fixtures {
                wanted.entry(*f).or_insert((t.target, light, zoom));
            }
        }
        let step_in = (real_ms / TRACK_IN_MS) as f32;
        let step_out = (real_ms / TRACK_OUT_MS) as f32;
        // where the heads stand now (only needed when one starts to follow)
        let starting = wanted.keys().any(|f| !self.tracking.contains_key(f));
        let frame = starting.then(|| self.render_frame());
        for (f, (target, light, zoom)) in &wanted {
            let prev = self.tracking.get(f).copied();
            let Some((pan, tilt)) =
                self.show
                    .aim_moving_head_near(*f, *target, prev.map(|p| (p.pan, p.tilt)))
            else {
                continue;
            };
            let weight = prev.map_or(0.0, |p| p.weight) + step_in;
            let (wait_ms, light_weight) = match prev {
                Some(p) => {
                    let wait = (p.wait_ms - real_ms as f32).max(0.0);
                    let lw = if wait > 0.0 {
                        p.light_weight
                    } else {
                        p.light_weight + step_in
                    };
                    (wait, lw)
                }
                None => (
                    frame
                        .as_ref()
                        .map_or(0.0, |fr| self.travel_ms(fr, *f, pan, tilt)),
                    0.0,
                ),
            };
            self.tracking.insert(
                *f,
                Track {
                    pan,
                    tilt,
                    weight: weight.min(1.0),
                    light: *light,
                    zoom: *zoom,
                    wait_ms,
                    light_weight: light_weight.min(1.0),
                    dark: prev.map_or(0.0, |p| (p.dark - step_in).max(0.0)),
                },
            );
        }
        // released: first dark, then back, then (arrived) the light again
        let step_dark = (real_ms / RELEASE_DARK_MS) as f32;
        let step_light = (real_ms / RELEASE_LIGHT_MS) as f32;
        let mut arrived_back = Vec::new();
        self.tracking.retain(|f, t| {
            if wanted.contains_key(f) {
                return true;
            }
            if t.weight > 0.0 && t.dark < 1.0 {
                t.dark = (t.dark + step_dark).min(1.0);
            } else if t.weight > 0.0 {
                t.weight -= step_out;
                if t.weight <= 0.0 {
                    t.weight = 0.0;
                    arrived_back.push(*f);
                }
            } else if t.wait_ms > 0.0 {
                t.wait_ms = (t.wait_ms - real_ms as f32).max(0.0);
            } else {
                t.dark -= step_light;
            }
            t.light_weight = t.light_weight.min(t.weight);
            t.weight > 0.0 || t.wait_ms > 0.0 || t.dark > 0.0
        });
        if !arrived_back.is_empty() {
            // the output now shows the scenes' position: how far is that?
            let frame = self.render_frame();
            for f in arrived_back {
                let Some(&t) = self.tracking.get(&f) else {
                    continue;
                };
                let ms = self.travel_ms(&frame, f, t.pan, t.tilt);
                if let Some(t) = self.tracking.get_mut(&f) {
                    t.wait_ms = ms;
                }
            }
        }
    }

    /// Estimated time (ms) a moving head needs from its current output
    /// position in `frame` to pan/tilt `to_pan`/`to_tilt` (programmer values).
    pub(super) fn travel_ms(
        &self,
        frame: &DmxFrame,
        f: FixtureId,
        to_pan: u16,
        to_tilt: u16,
    ) -> f32 {
        use crate::fixture::ChannelKind as K;
        let Some(fx) = self.show.fixture(f) else {
            return 0.0;
        };
        let Some(ty) = self.show.type_of(fx) else {
            return 0.0;
        };
        let ph = ty.physical.unwrap_or_default();
        let lim = fx.limits;
        let read = |k: K, kf: K| -> Option<u16> {
            let i = ty.channels.iter().position(|c| c.kind == k)?;
            let c = frame.get(usize::from(fx.universe), fx.address + i as u16)?;
            let fine = ty
                .channels
                .iter()
                .position(|c| c.kind == kf)
                .and_then(|j| frame.get(usize::from(fx.universe), fx.address + j as u16))
                .unwrap_or(0);
            Some(u16::from(c) << 8 | u16::from(fine))
        };
        let (pan_to, tilt_to) = if lim.swap_pan_tilt {
            (to_tilt, to_pan)
        } else {
            (to_pan, to_tilt)
        };
        let deg = |now: Option<u16>, to: u16, min: u8, max: u8, inv: bool, range: f32| {
            now.map_or(range, |n| {
                let to = crate::show::FixtureLimits::map16(to, min, max, inv);
                f32::from(n.abs_diff(to)) / 65535.0 * range
            })
        };
        let pan = deg(
            read(K::Pan, K::PanFine),
            pan_to,
            lim.pan_min,
            lim.pan_max,
            lim.invert_pan,
            ph.pan_range,
        );
        let tilt = deg(
            read(K::Tilt, K::TiltFine),
            tilt_to,
            lim.tilt_min,
            lim.tilt_max,
            lim.invert_tilt,
            ph.tilt_range,
        );
        (pan / PAN_DEG_PER_S).max(tilt / TILT_DEG_PER_S) * 1000.0 + SETTLE_MS
    }

    /// Fixtures that follow a tracker now (or are fading back).
    pub fn tracking(&self) -> &HashMap<FixtureId, Track> {
        &self.tracking
    }

    /// Lay the pan/tilt of following fixtures over the other values.
    pub(super) fn apply_tracking(
        &self,
        show: &Show,
        values: &mut HashMap<ChannelRef, u8>,
        touched: &mut FastSet<ChannelRef>,
    ) {
        use crate::fixture::ChannelKind as K;
        for (f, t) in &self.tracking {
            let Some(ty) = show.fixture(*f).and_then(|x| show.type_of(x)) else {
                continue;
            };
            let find = |k: K| {
                ty.channels
                    .iter()
                    .position(|c| c.kind == k)
                    .map(|i| ChannelRef {
                        fixture: *f,
                        channel: i as u16,
                    })
            };
            // ease in and out
            let w = f64::from(t.weight.clamp(0.0, 1.0));
            let w = w * w * (3.0 - 2.0 * w);
            if let Some(z) = t.zoom {
                for (i, def) in ty.channels.iter().enumerate() {
                    if matches!(def.kind, K::Zoom | K::Focus) {
                        let ch = ChannelRef {
                            fixture: *f,
                            channel: i as u16,
                        };
                        let base = values.get(&ch).copied().unwrap_or(0);
                        values.insert(ch, lerp(base, z, w));
                        touched.insert(ch);
                    }
                }
            }
            if let Some(l) = t.light {
                // the dimmer only comes up once the head has arrived
                let lw = f64::from(t.light_weight.clamp(0.0, 1.0));
                let lw = lw * lw * (3.0 - 2.0 * lw);
                let mut put = |ch: ChannelRef, to: u8, k: f64| {
                    let base = values.get(&ch).copied().unwrap_or(0);
                    values.insert(ch, lerp(base, to, k));
                    touched.insert(ch);
                };
                let white = l.color.iter().copied().min().unwrap_or(0);
                // n-th channel of a color = color cell n
                let mut seen: HashMap<K, u32> = HashMap::default();
                for (i, def) in ty.channels.iter().enumerate() {
                    let ch = ChannelRef {
                        fixture: *f,
                        channel: i as u16,
                    };
                    let mut cell = |k: K| {
                        let n = seen.entry(k).or_insert(0);
                        let on = *n >= 32 || l.cells & (1 << *n) != 0;
                        *n += 1;
                        on
                    };
                    match def.kind {
                        K::Dimmer => put(ch, l.dimmer, lw),
                        K::Red => {
                            let v = if cell(K::Red) { l.color[0] } else { 0 };
                            put(ch, v, w);
                        }
                        K::Green => {
                            let v = if cell(K::Green) { l.color[1] } else { 0 };
                            put(ch, v, w);
                        }
                        K::Blue => {
                            let v = if cell(K::Blue) { l.color[2] } else { 0 };
                            put(ch, v, w);
                        }
                        K::White => {
                            let v = if cell(K::White) { white } else { 0 };
                            put(ch, v, w);
                        }
                        K::Amber | K::Uv => put(ch, 0, w),
                        // shutter open as soon as it follows
                        K::Strobe if w > 0.0 => put(ch, def.open_value(), 1.0),
                        _ => {}
                    }
                }
                if ty.needs_virtual_dimmer() {
                    put(ChannelRef::virtual_dimmer(*f), l.dimmer, lw);
                }
            }
            // released: the intensity is down while the head moves back
            if t.dark > 0.0 {
                let d = f64::from(t.dark.clamp(0.0, 1.0));
                let k = 1.0 - d * d * (3.0 - 2.0 * d);
                let mut chans: Vec<ChannelRef> = ty
                    .channels
                    .iter()
                    .enumerate()
                    .filter(|(_, c)| c.kind == K::Dimmer)
                    .map(|(i, _)| ChannelRef {
                        fixture: *f,
                        channel: i as u16,
                    })
                    .collect();
                if ty.needs_virtual_dimmer() {
                    chans.push(ChannelRef::virtual_dimmer(*f));
                }
                for ch in chans {
                    let full = if ch.is_virtual() { 255 } else { 0 };
                    let v = values.get(&ch).copied().unwrap_or(full);
                    values.insert(ch, (f64::from(v) * k).round() as u8);
                    touched.insert(ch);
                }
            }
            for (coarse, fine, to) in [(K::Pan, K::PanFine, t.pan), (K::Tilt, K::TiltFine, t.tilt)]
            {
                let Some(c) = find(coarse) else {
                    continue;
                };
                let fine = find(fine);
                let base = u16::from(values.get(&c).copied().unwrap_or(0)) << 8
                    | u16::from(fine.and_then(|x| values.get(&x).copied()).unwrap_or(0));
                let v = (f64::from(base) + (f64::from(to) - f64::from(base)) * w).round() as u16;
                values.insert(c, (v >> 8) as u8);
                touched.insert(c);
                if let Some(fc) = fine {
                    values.insert(fc, (v & 0xFF) as u8);
                    touched.insert(fc);
                }
            }
        }
    }

    pub fn render_frame(&self) -> DmxFrame {
        let mut f = DmxFrame::new(usize::from(self.show.universes));
        self.render(&mut f);
        f
    }

    /// Current output value of a fixture channel (for fader displays).
    pub fn output_value(&self, frame: &DmxFrame, ch: ChannelRef) -> Option<u8> {
        if ch.is_virtual() {
            return None;
        }
        let f = self.show.fixture(ch.fixture)?;
        frame.get(usize::from(f.universe), f.address + ch.channel)
    }
}
