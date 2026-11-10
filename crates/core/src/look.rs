//! Copy a fixture's "look" (color, intensity, position, …) and paste it onto
//! other fixtures – also of a different type. Values are translated by
//! meaning, not by channel number:
//!
//! * same fixture type: channels are copied 1:1,
//! * color: RGB/W/A/UV components are converted (e.g. RGBW → RGB adds the
//!   white to R, G and B; RGB → RGBW extracts the common white), a color
//!   wheel gets the nearest slot,
//! * intensity: dimmer (or virtual dimmer) value,
//! * position: pan/tilt as degrees from the center (if the ranges are
//!   known), otherwise as the same fraction of the range,
//! * strobe/shutter: same state (open, closed, same rate) using the ranges.

use std::collections::HashMap;

use crate::fixture::{ChannelDef, ChannelKind as K, FixtureType, RangeEffect};

/// Which part of a look to paste.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LookPart {
    All,
    Color,
    Intensity,
    Position,
}

impl LookPart {
    pub const ALL: [LookPart; 4] = [
        LookPart::All,
        LookPart::Color,
        LookPart::Intensity,
        LookPart::Position,
    ];

    fn includes(self, kind: K) -> bool {
        match self {
            LookPart::All => true,
            LookPart::Color => is_color(kind),
            LookPart::Intensity => kind == K::Dimmer,
            LookPart::Position => matches!(kind, K::Pan | K::PanFine | K::Tilt | K::TiltFine),
        }
    }
}

fn is_color(kind: K) -> bool {
    matches!(
        kind,
        K::Red | K::Green | K::Blue | K::White | K::Amber | K::Uv | K::ColorWheel
    )
}

/// Target of a pasted value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LookTarget {
    /// Channel index within the fixture.
    Channel(u16),
    /// The virtual dimmer of fixtures without a dimmer channel.
    VirtualDimmer,
}

/// A captured look (from one fixture).
#[derive(Clone, Debug, PartialEq)]
pub struct Look {
    /// Name of the source fixture (for messages).
    pub source: String,
    type_key: String,
    /// All channel values of the source (index → value).
    raw: Vec<u8>,
    /// Dimmer 0–1 (dimmer channel or virtual dimmer).
    intensity: Option<f32>,
    /// Color components 0–1 per kind (R, G, B, W, A, UV).
    color: HashMap<K, f32>,
    /// Pan/tilt: fraction 0–1 and range in degrees (0 = unknown).
    pan: Option<(f32, f32)>,
    tilt: Option<(f32, f32)>,
    /// Shutter: `None` open, `Some(0.0)` closed, `Some(hz)` strobe.
    shutter: Option<Option<f32>>,
    /// Other channels by kind (zoom, focus, fan, fog).
    other: HashMap<K, u8>,
}

impl Look {
    /// Capture the look of a fixture. `value(i)` = current value of channel
    /// `i`, `virtual_dimmer` = value of the virtual dimmer (if any).
    pub fn capture(
        source: impl Into<String>,
        ty: &FixtureType,
        value: impl Fn(u16) -> u8,
        virtual_dimmer: Option<u8>,
    ) -> Self {
        let raw: Vec<u8> = (0..ty.channel_count()).map(&value).collect();
        let get = |kind: K| -> Option<u8> {
            ty.channels
                .iter()
                .position(|c| c.kind == kind)
                .map(|i| raw[i])
        };
        let f = |v: u8| f32::from(v) / 255.0;
        let intensity = get(K::Dimmer).or(virtual_dimmer).map(f);
        let mut color = HashMap::new();
        for k in [K::Red, K::Green, K::Blue, K::White, K::Amber, K::Uv] {
            if let Some(v) = get(k) {
                color.insert(k, f(v));
            }
        }
        // only a color wheel: its slot color as RGB
        if !color.iter().any(|(k, _)| *k != K::Uv) {
            if let Some((i, def)) = ty
                .channels
                .iter()
                .enumerate()
                .find(|(_, c)| c.kind == K::ColorWheel)
            {
                if let Some(rgb) = slot_color(def, raw[i]) {
                    color.insert(K::Red, rgb[0]);
                    color.insert(K::Green, rgb[1]);
                    color.insert(K::Blue, rgb[2]);
                }
            }
        }
        let phys = ty.physical.unwrap_or_default();
        let axis = |coarse: K, fine: K, range: f32| {
            get(coarse).map(|c| {
                let v = (u16::from(c) << 8) | u16::from(get(fine).unwrap_or(0));
                (f32::from(v) / 65535.0, range)
            })
        };
        let shutter = ty
            .channels
            .iter()
            .enumerate()
            .find(|(_, c)| c.kind == K::Strobe)
            .and_then(|(i, def)| {
                let r = def.range_at(raw[i])?;
                r.effect.shutter(r, raw[i])
            });
        let mut other = HashMap::new();
        for k in [K::Zoom, K::Focus, K::Fan, K::Fog] {
            if let Some(v) = get(k) {
                other.insert(k, v);
            }
        }
        let pan = axis(K::Pan, K::PanFine, phys.pan_range);
        let tilt = axis(K::Tilt, K::TiltFine, phys.tilt_range);
        Self {
            source: source.into(),
            type_key: ty.key.clone(),
            raw,
            intensity,
            color,
            pan,
            tilt,
            shutter,
            other,
        }
    }

    /// Values for a fixture of type `ty` (only the chosen part).
    pub fn apply(&self, ty: &FixtureType, part: LookPart) -> Vec<(LookTarget, u8)> {
        let mut out = Vec::new();
        // same type: 1:1
        if ty.key == self.type_key && ty.channels.len() == self.raw.len() {
            for (i, c) in ty.channels.iter().enumerate() {
                if part.includes(c.kind) {
                    out.push((LookTarget::Channel(i as u16), self.raw[i]));
                }
            }
            if matches!(part, LookPart::All | LookPart::Intensity) && ty.needs_virtual_dimmer() {
                if let Some(v) = self.intensity {
                    out.push((LookTarget::VirtualDimmer, to_u8(v)));
                }
            }
            return out;
        }
        let set = |out: &mut Vec<(LookTarget, u8)>, kind: K, v: u8| {
            for (i, c) in ty.channels.iter().enumerate() {
                if c.kind == kind {
                    out.push((LookTarget::Channel(i as u16), v));
                }
            }
        };
        if matches!(part, LookPart::All | LookPart::Intensity) {
            if let Some(v) = self.intensity {
                if ty.has_kind(K::Dimmer) {
                    set(&mut out, K::Dimmer, to_u8(v));
                } else if ty.needs_virtual_dimmer() {
                    out.push((LookTarget::VirtualDimmer, to_u8(v)));
                }
            }
        }
        if matches!(part, LookPart::All | LookPart::Color) {
            self.apply_color(ty, &mut out);
        }
        if matches!(part, LookPart::All | LookPart::Position) {
            let phys = ty.physical.unwrap_or_default();
            for (src, coarse, fine, range) in [
                (self.pan, K::Pan, K::PanFine, phys.pan_range),
                (self.tilt, K::Tilt, K::TiltFine, phys.tilt_range),
            ] {
                let Some((frac, src_range)) = src else {
                    continue;
                };
                if !ty.has_kind(coarse) {
                    continue;
                }
                // degrees from the center, if both ranges are known
                let t = if src_range > 0.0 && range > 0.0 {
                    (0.5 + (frac - 0.5) * src_range / range).clamp(0.0, 1.0)
                } else {
                    frac
                };
                let v = (t * 65535.0).round() as u16;
                set(&mut out, coarse, (v >> 8) as u8);
                set(&mut out, fine, (v & 0xff) as u8);
            }
        }
        if part == LookPart::All {
            if let Some(state) = self.shutter {
                if let Some((i, def)) = ty
                    .channels
                    .iter()
                    .enumerate()
                    .find(|(_, c)| c.kind == K::Strobe)
                {
                    if let Some(v) = shutter_value(def, state) {
                        out.push((LookTarget::Channel(i as u16), v));
                    }
                }
            }
            for (k, v) in &self.other {
                set(&mut out, *k, *v);
            }
        }
        out
    }

    fn apply_color(&self, ty: &FixtureType, out: &mut Vec<(LookTarget, u8)>) {
        let has = |k: K| ty.has_kind(k);
        let src = |k: K| self.color.get(&k).copied();
        let mut c = self.color.clone();
        let mut fold = |k: K, mix: [f32; 3]| {
            if let Some(v) = c.remove(&k) {
                for (kk, m) in [(K::Red, mix[0]), (K::Green, mix[1]), (K::Blue, mix[2])] {
                    *c.entry(kk).or_insert(0.0) += v * m;
                }
            }
        };
        // components the target lacks are mixed from R, G, B
        if !has(K::White) {
            fold(K::White, [1.0, 1.0, 1.0]);
        }
        if !has(K::Amber) {
            fold(K::Amber, [1.0, 0.75, 0.0]);
        }
        let rgb = [K::Red, K::Green, K::Blue].map(|k| c.get(&k).copied().unwrap_or(0.0).min(1.0));
        let target_rgb = has(K::Red) && has(K::Green) && has(K::Blue);
        let put = |out: &mut Vec<(LookTarget, u8)>, k: K, v: f32| {
            for (i, d) in ty.channels.iter().enumerate() {
                if d.kind == k {
                    out.push((LookTarget::Channel(i as u16), to_u8(v)));
                }
            }
        };
        if target_rgb {
            let mut rgb = rgb;
            // target has white the source did not: take the common white
            if has(K::White) && src(K::White).is_none() {
                let w = rgb[0].min(rgb[1]).min(rgb[2]);
                rgb = rgb.map(|v| v - w);
                put(out, K::White, w);
            } else if let Some(w) = c.get(&K::White) {
                put(out, K::White, *w);
            }
            put(out, K::Red, rgb[0]);
            put(out, K::Green, rgb[1]);
            put(out, K::Blue, rgb[2]);
            if let Some(a) = c.get(&K::Amber) {
                put(out, K::Amber, *a);
            } else if has(K::Amber) {
                put(out, K::Amber, 0.0);
            }
        } else {
            // single colors without full RGB (e.g. W only, UV bar)
            for k in [K::Red, K::Green, K::Blue, K::White, K::Amber] {
                if has(k) {
                    let v = match k {
                        K::White => c
                            .get(&K::White)
                            .copied()
                            .unwrap_or(rgb[0].min(rgb[1]).min(rgb[2])),
                        K::Amber => c.get(&K::Amber).copied().unwrap_or(0.0),
                        K::Red => rgb[0],
                        K::Green => rgb[1],
                        _ => rgb[2],
                    };
                    put(out, k, v);
                }
            }
            // color wheel: nearest slot
            if let Some((i, def)) = ty
                .channels
                .iter()
                .enumerate()
                .find(|(_, d)| d.kind == K::ColorWheel)
            {
                if let Some(v) = nearest_slot(def, rgb) {
                    out.push((LookTarget::Channel(i as u16), v));
                }
            }
        }
        if has(K::Uv) {
            put(out, K::Uv, src(K::Uv).unwrap_or(0.0));
        }
    }
}

fn to_u8(v: f32) -> u8 {
    (v.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// Color of the wheel slot at `v` (0–1 per component).
fn slot_color(def: &ChannelDef, v: u8) -> Option<[f32; 3]> {
    match def.range_at(v)?.effect {
        RangeEffect::Color { rgb } => Some(rgb.map(|c| f32::from(c) / 255.0)),
        RangeEffect::WheelOpen | RangeEffect::Open => Some([1.0; 3]),
        _ => None,
    }
}

/// Wheel slot closest to `rgb` (by hue direction and brightness).
fn nearest_slot(def: &ChannelDef, rgb: [f32; 3]) -> Option<u8> {
    let max = rgb[0].max(rgb[1]).max(rgb[2]);
    if max <= 0.0 {
        return None;
    }
    let want = rgb.map(|c| c / max);
    def.ranges
        .iter()
        .filter_map(|r| {
            let c = match r.effect {
                RangeEffect::Color { rgb } => rgb.map(|c| f32::from(c) / 255.0),
                RangeEffect::WheelOpen | RangeEffect::Open => [1.0; 3],
                _ => return None,
            };
            let m = c[0].max(c[1]).max(c[2]).max(1e-3);
            let d: f32 = (0..3).map(|i| (c[i] / m - want[i]).powi(2)).sum();
            Some((d, r.from + (r.to - r.from) / 2))
        })
        .min_by(|a, b| a.0.total_cmp(&b.0))
        .map(|(_, v)| v)
}

/// Strobe channel value for a shutter state (open, closed, rate).
fn shutter_value(def: &ChannelDef, state: Option<f32>) -> Option<u8> {
    let mid = |r: &crate::fixture::ChannelRange| r.from + (r.to - r.from) / 2;
    match state {
        None => def
            .ranges
            .iter()
            .find(|r| r.effect == RangeEffect::Open)
            .map(mid),
        Some(hz) if hz <= 0.0 => def
            .ranges
            .iter()
            .find(|r| r.effect == RangeEffect::Closed)
            .map(mid),
        Some(hz) => def.ranges.iter().find_map(|r| match r.effect {
            RangeEffect::Strobe { from_hz, to_hz } => {
                let (lo, hi) = (from_hz.min(to_hz), from_hz.max(to_hz));
                if hz < lo || hz > hi {
                    return None;
                }
                let k = if (to_hz - from_hz).abs() < 1e-3 {
                    0.0
                } else {
                    (hz - from_hz) / (to_hz - from_hz)
                };
                Some((f32::from(r.from) + k * f32::from(r.to - r.from)).round() as u8)
            }
            _ => None,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::{ChannelRange, Physics};

    fn ty(key: &str, kinds: &[K]) -> FixtureType {
        FixtureType {
            key: key.into(),
            manufacturer: "T".into(),
            model: key.into(),
            mode: String::new(),
            channels: kinds
                .iter()
                .map(|k| ChannelDef::new(format!("{k:?}"), *k))
                .collect(),
            physical: None,
        }
    }

    fn val(out: &[(LookTarget, u8)], t: &FixtureType, k: K) -> Option<u8> {
        let i = t.channels.iter().position(|c| c.kind == k)? as u16;
        out.iter()
            .find(|(tg, _)| *tg == LookTarget::Channel(i))
            .map(|(_, v)| *v)
    }

    #[test]
    fn same_type_copies_raw() {
        let t = ty("a", &[K::Dimmer, K::Red, K::Green, K::Blue, K::Pan]);
        let look = Look::capture("A", &t, |i| [200, 10, 20, 30, 99][i as usize], None);
        let out = look.apply(&t, LookPart::Color);
        assert_eq!(out.len(), 3);
        assert_eq!(val(&out, &t, K::Green), Some(20));
        let all = look.apply(&t, LookPart::All);
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn rgbw_to_rgb_and_back() {
        let rgbw = ty("rgbw", &[K::Dimmer, K::Red, K::Green, K::Blue, K::White]);
        let rgb = ty("rgb", &[K::Red, K::Green, K::Blue]);
        // warm white-ish: R 255, G 0, B 0, W 100 → RGB: R 255, G 100, B 100
        let look = Look::capture("A", &rgbw, |i| [255, 255, 0, 0, 100][i as usize], None);
        let out = look.apply(&rgb, LookPart::All);
        assert_eq!(val(&out, &rgb, K::Red), Some(255));
        assert_eq!(val(&out, &rgb, K::Green), Some(100));
        // no dimmer channel: intensity goes to the virtual dimmer
        assert!(out.contains(&(LookTarget::VirtualDimmer, 255)));
        // back: the common white is extracted
        let look = Look::capture("B", &rgb, |i| [255, 100, 100][i as usize], Some(128));
        let out = look.apply(&rgbw, LookPart::All);
        assert_eq!(val(&out, &rgbw, K::White), Some(100));
        assert_eq!(val(&out, &rgbw, K::Red), Some(155));
        assert_eq!(val(&out, &rgbw, K::Green), Some(0));
        assert_eq!(val(&out, &rgbw, K::Dimmer), Some(128));
    }

    #[test]
    fn color_wheel_gets_nearest_slot() {
        let rgb = ty("rgb", &[K::Red, K::Green, K::Blue]);
        let mut wheel = ty("wheel", &[K::Dimmer, K::ColorWheel]);
        wheel.channels[1].ranges = vec![
            ChannelRange::new(0, 9, RangeEffect::WheelOpen),
            ChannelRange::new(10, 19, RangeEffect::Color { rgb: [255, 0, 0] }),
            ChannelRange::new(20, 29, RangeEffect::Color { rgb: [0, 0, 255] }),
        ];
        let look = Look::capture("A", &rgb, |i| [0, 0, 200][i as usize], None);
        let out = look.apply(&wheel, LookPart::Color);
        assert_eq!(val(&out, &wheel, K::ColorWheel), Some(24));
        // and from the wheel back to RGB
        let look = Look::capture("W", &wheel, |i| [255, 15][i as usize], None);
        let out = look.apply(&rgb, LookPart::Color);
        assert_eq!(val(&out, &rgb, K::Red), Some(255));
        assert_eq!(val(&out, &rgb, K::Blue), Some(0));
    }

    #[test]
    fn position_in_degrees() {
        let mut a = ty("a", &[K::Pan, K::Tilt]);
        a.physical = Some(Physics {
            pan_range: 540.0,
            tilt_range: 270.0,
            ..Default::default()
        });
        let mut b = ty("b", &[K::Pan, K::PanFine, K::Tilt]);
        b.physical = Some(Physics {
            pan_range: 360.0,
            tilt_range: 270.0,
            ..Default::default()
        });
        // pan: 90° right of center on a 540° fixture
        let pan = ((0.5 + 90.0 / 540.0) * 255.0_f32).round() as u8;
        let look = Look::capture("A", &a, |i| [pan, 128][i as usize], None);
        let out = look.apply(&b, LookPart::Position);
        let p = val(&out, &b, K::Pan).unwrap();
        // 90° on a 360° fixture: 0.75
        assert!((f32::from(p) / 255.0 - 0.75).abs() < 0.01, "{p}");
        assert!(val(&out, &b, K::PanFine).is_some());
        assert!(look.apply(&b, LookPart::Color).is_empty());
    }

    #[test]
    fn strobe_by_meaning() {
        let mut a = ty("a", &[K::Strobe]);
        a.channels[0].ranges = vec![
            ChannelRange::new(0, 9, RangeEffect::Open),
            ChannelRange::new(
                10,
                255,
                RangeEffect::Strobe {
                    from_hz: 1.0,
                    to_hz: 20.0,
                },
            ),
        ];
        let mut b = ty("b", &[K::Strobe]);
        b.channels[0].ranges = vec![
            ChannelRange::new(0, 0, RangeEffect::Closed),
            ChannelRange::new(
                1,
                100,
                RangeEffect::Strobe {
                    from_hz: 1.0,
                    to_hz: 20.0,
                },
            ),
            ChannelRange::new(101, 255, RangeEffect::Open),
        ];
        let open = Look::capture("A", &a, |_| 0, None);
        assert_eq!(
            val(&open.apply(&b, LookPart::All), &b, K::Strobe),
            Some(178)
        );
        let fast = Look::capture("A", &a, |_| 255, None);
        assert_eq!(
            val(&fast.apply(&b, LookPart::All), &b, K::Strobe),
            Some(100)
        );
    }
}
