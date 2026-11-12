//! Calibrating a moving head in the 3D venue: the real head is pointed at
//! marks whose place in the venue is known (stage corner, edge of a table …)
//! and the pan/tilt values are noted. From several such points the position
//! and rotation of the head in the room are computed so that the venue
//! matches reality (least squares over the beam directions).

use crate::show::{FixtureId, Show};
use crate::venue::{rotation, Placement};

/// A mark in the room and the pan/tilt (programmer values) that hit it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CalPoint {
    pub target: [f32; 3],
    pub pan: u16,
    pub tilt: u16,
}

/// Result of a calibration.
#[derive(Clone, Debug, PartialEq)]
pub struct Calibration {
    pub placement: Placement,
    /// Angle between beam and mark per point (degrees) …
    pub errors: Vec<f32>,
    /// … and their root mean square.
    pub rms: f32,
}

/// What the head needs for the beam direction.
struct Head {
    pan_range: f32,
    tilt_range: f32,
    /// Distance of the turning point below the mounting point (m).
    drop: f32,
    lim: crate::show::FixtureLimits,
}

impl Head {
    /// Turning point and beam direction (world, unit) for a placement and
    /// programmer pan/tilt.
    fn beam(&self, p: &Placement, pan: u16, tilt: u16) -> ([f32; 3], [f32; 3]) {
        let r = rotation(p.rotation);
        let origin = [0, 1, 2].map(|i| p.position[i] - r[1][i] * self.drop);
        let (pan, tilt) = if self.lim.swap_pan_tilt {
            (tilt, pan)
        } else {
            (pan, tilt)
        };
        let l = self.lim;
        let po = crate::show::FixtureLimits::map16(pan, l.pan_min, l.pan_max, l.invert_pan);
        let to = crate::show::FixtureLimits::map16(tilt, l.tilt_min, l.tilt_max, l.invert_tilt);
        let pf = f32::from(po) / 65535.0;
        let tf = f32::from(to) / 65535.0;
        let pa = (0.5 - pf) * self.pan_range.to_radians();
        let ta = (0.5 - tf) * self.tilt_range.to_radians();
        // in the fixture (see venue::moving_head_aim)
        let local = [-ta.sin() * pa.sin(), -ta.cos(), -ta.sin() * pa.cos()];
        let d = [0, 1, 2].map(|i| (0..3).map(|j| r[j][i] * local[j]).sum::<f32>());
        (origin, d)
    }
}

fn norm(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt().max(1e-9);
    v.map(|x| x / l)
}

fn residuals(head: &Head, p: &Placement, pts: &[CalPoint]) -> Vec<f64> {
    let mut out = Vec::with_capacity(pts.len() * 3);
    for c in pts {
        let (o, d) = head.beam(p, c.pan, c.tilt);
        let want = norm([0, 1, 2].map(|i| c.target[i] - o[i]));
        for i in 0..3 {
            out.push(f64::from(d[i] - want[i]));
        }
    }
    out
}

fn angles(head: &Head, p: &Placement, pts: &[CalPoint]) -> Vec<f32> {
    pts.iter()
        .map(|c| {
            let (o, d) = head.beam(p, c.pan, c.tilt);
            let want = norm([0, 1, 2].map(|i| c.target[i] - o[i]));
            let dot = (0..3).map(|i| d[i] * want[i]).sum::<f32>().clamp(-1.0, 1.0);
            dot.acos().to_degrees()
        })
        .collect()
}

/// Solve `a x = b` (small, dense) by Gaussian elimination.
#[allow(clippy::needless_range_loop)]
fn solve(mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Option<Vec<f64>> {
    let n = b.len();
    for c in 0..n {
        let piv = (c..n).max_by(|&x, &y| a[x][c].abs().total_cmp(&a[y][c].abs()))?;
        if a[piv][c].abs() < 1e-12 {
            return None;
        }
        a.swap(c, piv);
        b.swap(c, piv);
        for r in c + 1..n {
            let f = a[r][c] / a[c][c];
            for k in c..n {
                a[r][k] -= f * a[c][k];
            }
            b[r] -= f * b[c];
        }
    }
    let mut x = vec![0.0; n];
    for r in (0..n).rev() {
        let s: f64 = (r + 1..n).map(|k| a[r][k] * x[k]).sum();
        x[r] = (b[r] - s) / a[r][r];
    }
    Some(x)
}

/// Parameters: rotation (deg) and, when solved, position (m).
fn placement_of(base: &Placement, x: &[f64], with_pos: bool) -> Placement {
    let mut p = *base;
    for i in 0..3 {
        p.rotation[i] = x[i] as f32;
        if with_pos {
            p.position[i] = x[3 + i] as f32;
        }
    }
    p
}

/// Levenberg–Marquardt from a starting placement.
#[allow(clippy::needless_range_loop)]
fn fit(head: &Head, start: &Placement, pts: &[CalPoint], with_pos: bool) -> (Placement, f64) {
    let n = if with_pos { 6 } else { 3 };
    let mut x: Vec<f64> = start.rotation.iter().map(|v| f64::from(*v)).collect();
    if with_pos {
        x.extend(start.position.iter().map(|v| f64::from(*v)));
    }
    let cost = |x: &[f64]| -> f64 {
        residuals(head, &placement_of(start, x, with_pos), pts)
            .iter()
            .map(|r| r * r)
            .sum()
    };
    let mut c = cost(&x);
    let mut lambda = 1e-3;
    for _ in 0..200 {
        let r0 = residuals(head, &placement_of(start, &x, with_pos), pts);
        // numeric Jacobian
        let mut jac = vec![vec![0.0; n]; r0.len()];
        for k in 0..n {
            let h = if k < 3 { 0.01 } else { 0.001 };
            let mut xp = x.clone();
            xp[k] += h;
            let rp = residuals(head, &placement_of(start, &xp, with_pos), pts);
            for (row, (a, b)) in jac.iter_mut().zip(rp.iter().zip(&r0)) {
                row[k] = (a - b) / h;
            }
        }
        let mut jtj = vec![vec![0.0; n]; n];
        let mut jtr = vec![0.0; n];
        for (row, r) in jac.iter().zip(&r0) {
            for i in 0..n {
                jtr[i] -= row[i] * r;
                for j in 0..n {
                    jtj[i][j] += row[i] * row[j];
                }
            }
        }
        let mut improved = false;
        for _ in 0..10 {
            let mut a = jtj.clone();
            for (i, row) in a.iter_mut().enumerate() {
                row[i] += lambda * (1.0 + jtj[i][i]);
            }
            let Some(step) = solve(a, jtr.clone()) else {
                lambda *= 10.0;
                continue;
            };
            let xn: Vec<f64> = x.iter().zip(&step).map(|(a, b)| a + b).collect();
            let cn = cost(&xn);
            if cn < c {
                let done = (c - cn) < 1e-14;
                x = xn;
                c = cn;
                lambda = (lambda * 0.3).max(1e-9);
                improved = !done;
                break;
            }
            lambda *= 10.0;
        }
        if !improved {
            break;
        }
    }
    (placement_of(start, &x, with_pos), c)
}

impl Show {
    fn calibration_head(&self, id: FixtureId) -> Option<Head> {
        let f = self.fixture(id)?;
        let ty = self.type_of(f)?;
        if !ty.has_kind(crate::ChannelKind::Pan) || !ty.has_kind(crate::ChannelKind::Tilt) {
            return None;
        }
        let ph = ty.physical.unwrap_or_default();
        let k = if ph.size_mm[1] > 100.0 {
            (ph.size_mm[1] / 420.0).clamp(0.6, 1.6)
        } else {
            1.0
        };
        Some(Head {
            pan_range: ph.pan_range,
            tilt_range: ph.tilt_range,
            drop: 0.33 * k,
            lim: f.limits,
        })
    }

    /// Beam of a moving head with this placement and pan/tilt (programmer
    /// values): turning point and direction.
    pub fn beam_of(
        &self,
        id: FixtureId,
        p: &Placement,
        pan: u16,
        tilt: u16,
    ) -> Option<([f32; 3], [f32; 3])> {
        Some(self.calibration_head(id)?.beam(p, pan, tilt))
    }

    /// Placement of a moving head that fits the points best. With
    /// `with_position` (needs 3 points, better 4+) position and rotation,
    /// otherwise only the rotation (2 points). Tries several starting
    /// rotations around the current placement.
    pub fn calibrate_fixture(
        &self,
        id: FixtureId,
        points: &[CalPoint],
        with_position: bool,
    ) -> Option<Calibration> {
        let head = self.calibration_head(id)?;
        if points.len() < if with_position { 3 } else { 2 } {
            return None;
        }
        let current = self.placement3d(id);
        let mut best: Option<(Placement, f64)> = None;
        for flip in [0.0f32, 180.0] {
            for yaw in [0.0f32, 90.0, 180.0, 270.0] {
                let mut start = current;
                start.rotation[1] += yaw;
                start.rotation[0] += flip;
                let (p, c) = fit(&head, &start, points, with_position);
                if best.as_ref().is_none_or(|b| c < b.1) {
                    best = Some((p, c));
                }
            }
        }
        let (mut placement, _) = best?;
        for r in &mut placement.rotation {
            *r = (*r + 180.0).rem_euclid(360.0) - 180.0;
        }
        let errors = angles(&head, &placement, points);
        let rms = (errors.iter().map(|e| e * e).sum::<f32>() / errors.len() as f32).sqrt();
        Some(Calibration {
            placement,
            errors,
            rms,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::builtin_fixture_types;

    #[test]
    fn finds_position_and_rotation_from_four_marks() {
        let mut s = Show::new("T", 1);
        s.fixture_types = builtin_fixture_types();
        let id = s
            .add_fixture("MH", "generic/moving-head-spot-11ch", 0, 1)
            .unwrap();
        // the real head hangs here, a bit turned
        let real = Placement {
            position: [1.2, 4.6, -0.5],
            rotation: [4.0, -71.0, -3.0],
        };
        s.fixture_mut(id).unwrap().place3d = Some(real);
        let marks = [
            [0.0, 0.0, 0.0],
            [3.0, 0.0, 2.5],
            [-2.0, 0.8, 4.0],
            [2.0, 0.0, -3.0],
            [-3.5, 0.0, -1.0],
        ];
        let points: Vec<CalPoint> = marks
            .iter()
            .map(|m| {
                let (pan, tilt) = s.aim_moving_head(id, *m).unwrap();
                CalPoint {
                    target: *m,
                    pan,
                    tilt,
                }
            })
            .collect();
        // the plan is off by half a meter and 20°
        s.fixture_mut(id).unwrap().place3d = Some(Placement {
            position: [0.8, 4.2, 0.0],
            rotation: [0.0, -90.0, 0.0],
        });
        let cal = s.calibrate_fixture(id, &points, true).unwrap();
        assert!(cal.rms < 0.2, "{cal:?}");
        for i in 0..3 {
            assert!(
                (cal.placement.position[i] - real.position[i]).abs() < 0.05,
                "{cal:?}"
            );
        }
        // only the rotation, position known
        s.fixture_mut(id).unwrap().place3d = Some(Placement {
            position: real.position,
            rotation: [0.0, -90.0, 0.0],
        });
        let cal = s.calibrate_fixture(id, &points[..2], false).unwrap();
        assert!(cal.rms < 0.2, "{cal:?}");
    }
}
