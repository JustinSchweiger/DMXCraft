//! Helper functions for the stage plan.
//!
//! The plan is unbounded. One unit corresponds to the width of the area in
//! which new fixtures are arranged automatically (0.0..=1.0); fixtures may
//! lie anywhere outside of it.

/// Margin automatically arranged fixtures keep from the plan edge.
pub const MARGIN: f32 = 0.06;

pub fn clamp(p: [f32; 2]) -> [f32; 2] {
    [p[0].clamp(0.0, 1.0), p[1].clamp(0.0, 1.0)]
}

/// Snaps a position to a grid with `cells` cells per unit.
pub fn snap(p: [f32; 2], cells: u32) -> [f32; 2] {
    let n = cells.max(1) as f32;
    [(p[0] * n).round() / n, (p[1] * n).round() / n]
}

/// `n` positions evenly in a row at height `y`.
pub fn row(n: usize, y: f32) -> Vec<[f32; 2]> {
    match n {
        0 => Vec::new(),
        1 => vec![[0.5, y]],
        _ => {
            let span = 1.0 - 2.0 * MARGIN;
            (0..n)
                .map(|i| [MARGIN + span * i as f32 / (n - 1) as f32, y])
                .collect()
        }
    }
}

/// `n` positions as a grid with an arrangement as square as possible.
pub fn grid(n: usize) -> Vec<[f32; 2]> {
    if n == 0 {
        return Vec::new();
    }
    let cols = (n as f32).sqrt().ceil() as usize;
    let rows = n.div_ceil(cols);
    let step_x = (1.0 - 2.0 * MARGIN) / cols.max(2).saturating_sub(1) as f32;
    let step_y = (1.0 - 2.0 * MARGIN) / rows.max(2).saturating_sub(1) as f32;
    (0..n)
        .map(|i| {
            let (c, r) = (i % cols, i / cols);
            let x = if cols == 1 {
                0.5
            } else {
                MARGIN + step_x * c as f32
            };
            let y = if rows == 1 {
                0.5
            } else {
                MARGIN + step_y * r as f32
            };
            [x, y]
        })
        .collect()
}

/// Default position for fixtures without a stored position (index in the
/// patch list).
pub fn default_position(index: usize) -> [f32; 2] {
    const PER_ROW: usize = 12;
    let col = (index % PER_ROW) as f32;
    let r = (index / PER_ROW) as f32;
    clamp([MARGIN + col * 0.075, 0.12 + r * 0.16])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapping() {
        assert_eq!(snap([0.26, 0.74], 4), [0.25, 0.75]);
        // unbounded: also outside 0..1
        assert_eq!(snap([1.26, -0.34], 10), [1.3, -0.3]);
    }

    #[test]
    fn row_is_evenly_spread() {
        let r = row(3, 0.3);
        assert_eq!(r.len(), 3);
        assert!((r[0][0] - MARGIN).abs() < 1e-6);
        assert!((r[1][0] - 0.5).abs() < 1e-6);
        assert!((r[2][0] - (1.0 - MARGIN)).abs() < 1e-6);
        assert!(r.iter().all(|p| p[1] == 0.3));
        assert_eq!(row(1, 0.2), vec![[0.5, 0.2]]);
    }

    #[test]
    fn grid_fits_inside() {
        for n in 1..40 {
            let g = grid(n);
            assert_eq!(g.len(), n);
            assert!(g
                .iter()
                .all(|p| (0.0..=1.0).contains(&p[0]) && (0.0..=1.0).contains(&p[1])));
        }
    }
}
