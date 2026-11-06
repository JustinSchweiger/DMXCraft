//! Mit dem Painter gezeichnete Symbole – unabhängig von Schrift-Glyphen.

use eframe::egui::{self, pos2, Color32, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};

use crate::ui::theme;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Play,
    Pause,
    Prev,
    Next,
    Plus,
    LightOff,
    LightOn,
    Center,
    Locked,
    Unlocked,
    ArrangeRow,
    ArrangeGrid,
    AlignH,
    AlignV,
    Reset,
    Menu,
    Minimize,
    Maximize,
    Restore,
    Close,
    FileNew,
    FileOpen,
    Recent,
    Save,
    SaveAs,
    Import,
    Undo,
    Redo,
    Settings,
    Fullscreen,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    Cube,
    Levels,
    Builder,
    Keyboard,
    Midi,
    Touch,
    Update,
    Info,
    Edit,
    Trash,
    Group,
    Copy,
    Paste,
}

impl Icon {
    pub fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let c = rect.center();
        let s = rect.width().min(rect.height()) * 0.5;
        let stroke = Stroke::new(1.6, color);
        match self {
            Icon::Play => {
                let pts = vec![
                    pos2(c.x - s * 0.45, c.y - s * 0.6),
                    pos2(c.x + s * 0.6, c.y),
                    pos2(c.x - s * 0.45, c.y + s * 0.6),
                ];
                painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
            }
            Icon::Pause => {
                for dx in [-0.3, 0.3] {
                    let r = Rect::from_center_size(
                        pos2(c.x + s * dx, c.y),
                        Vec2::new(s * 0.28, s * 1.2),
                    );
                    painter.rect_filled(r, 1.0, color);
                }
            }
            Icon::Prev | Icon::Next => {
                let dir = if self == Icon::Next { 1.0 } else { -1.0 };
                let tip = c.x + dir * s * 0.35;
                let back = c.x - dir * s * 0.45;
                let pts = vec![
                    pos2(back, c.y - s * 0.55),
                    pos2(tip, c.y),
                    pos2(back, c.y + s * 0.55),
                ];
                painter.add(Shape::convex_polygon(pts, color, Stroke::NONE));
                let bar = Rect::from_center_size(
                    pos2(tip + dir * s * 0.12, c.y),
                    Vec2::new(s * 0.18, s * 1.1),
                );
                painter.rect_filled(bar, 1.0, color);
            }
            Icon::LightOff | Icon::LightOn => {
                // Scheinwerfer von vorn: Gehäuse, Linse, bei „an“ mit Strahlen
                let r = s * 0.42;
                let lens = c + Vec2::new(0.0, -s * 0.05);
                painter.circle_stroke(lens, r, stroke);
                if self == Icon::LightOn {
                    painter.circle_filled(lens, r * 0.62, color);
                    for k in 0..8 {
                        let a = std::f32::consts::TAU * k as f32 / 8.0;
                        let dir = Vec2::angled(a);
                        painter.line_segment(
                            [lens + dir * r * 1.3, lens + dir * r * 1.75],
                            Stroke::new(1.3, color),
                        );
                    }
                } else {
                    painter.line_segment(
                        [
                            lens + Vec2::new(-r * 0.7, -r * 0.7),
                            lens + Vec2::new(r * 0.7, r * 0.7),
                        ],
                        stroke,
                    );
                }
            }
            Icon::Center => {
                let r = s * 0.45;
                painter.circle_stroke(c, r, stroke);
                painter.circle_filled(c, s * 0.1, color);
                for dir in [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y] {
                    painter.line_segment([c + dir * r * 0.55, c + dir * r * 1.45], stroke);
                }
            }
            Icon::Locked | Icon::Unlocked => {
                let body = Rect::from_center_size(
                    c + Vec2::new(0.0, s * 0.25),
                    Vec2::new(s * 1.1, s * 0.8),
                );
                painter.rect_filled(body, 1.5, color);
                let r = s * 0.32;
                let lift = if self == Icon::Unlocked {
                    s * 0.25
                } else {
                    0.0
                };
                let shift = if self == Icon::Unlocked { s * 0.3 } else { 0.0 };
                let top = body.top() - lift;
                let n = 12;
                let pts: Vec<_> = (0..=n)
                    .map(|i| {
                        let a = std::f32::consts::PI + std::f32::consts::PI * i as f32 / n as f32;
                        pos2(c.x + shift + a.cos() * r, top - r * 0.6 + a.sin() * r)
                    })
                    .collect();
                let mut legs = vec![pos2(c.x + shift - r, top)];
                legs.extend(pts);
                legs.push(pos2(
                    c.x + shift + r,
                    if self == Icon::Unlocked {
                        top - r * 0.3
                    } else {
                        top
                    },
                ));
                painter.add(Shape::line(legs, Stroke::new(1.6, color)));
                painter.circle_filled(body.center(), s * 0.1, theme::bg());
            }
            Icon::ArrangeRow => {
                for k in -1..=1 {
                    painter.circle_filled(c + Vec2::new(k as f32 * s * 0.6, 0.0), s * 0.2, color);
                }
            }
            Icon::ArrangeGrid => {
                for (dx, dy) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                    painter.circle_filled(
                        c + Vec2::new(dx * s * 0.38, dy * s * 0.38),
                        s * 0.2,
                        color,
                    );
                }
            }
            Icon::AlignH => {
                painter.line_segment(
                    [pos2(c.x - s * 0.8, c.y), pos2(c.x + s * 0.8, c.y)],
                    Stroke::new(1.0, color),
                );
                for (k, dy) in [(-1.0, -0.25), (0.0, 0.2), (1.0, -0.1)] {
                    let r = Rect::from_center_size(
                        c + Vec2::new(k * s * 0.55, dy * s * 0.0),
                        Vec2::new(s * 0.3, s * 0.5 + dy.abs() * s),
                    );
                    painter.rect_filled(r, 1.0, color);
                }
            }
            Icon::AlignV => {
                painter.line_segment(
                    [pos2(c.x, c.y - s * 0.8), pos2(c.x, c.y + s * 0.8)],
                    Stroke::new(1.0, color),
                );
                for (k, dx) in [(-1.0, 0.25), (0.0, -0.2), (1.0, 0.1)] {
                    let r = Rect::from_center_size(
                        c + Vec2::new(0.0, k * s * 0.55),
                        Vec2::new(s * 0.5 + f32::abs(dx) * s, s * 0.3),
                    );
                    painter.rect_filled(r, 1.0, color);
                }
            }
            Icon::Reset => {
                // Kreisbogen mit Pfeilspitze
                let r = s * 0.55;
                let n = 20;
                let (a0, a1) = (-0.6f32, std::f32::consts::TAU - 1.4);
                let pts: Vec<_> = (0..=n)
                    .map(|i| c + Vec2::angled(a0 + (a1 - a0) * i as f32 / n as f32) * r)
                    .collect();
                painter.add(Shape::line(pts, stroke));
                let tip = c + Vec2::angled(a0) * r;
                painter.add(Shape::convex_polygon(
                    vec![
                        tip + Vec2::new(-s * 0.35, -s * 0.05),
                        tip + Vec2::new(s * 0.1, -s * 0.4),
                        tip + Vec2::new(s * 0.15, s * 0.1),
                    ],
                    color,
                    Stroke::NONE,
                ));
            }
            Icon::Menu => {
                for dy in [-0.45, 0.0, 0.45] {
                    let y = c.y + s * dy;
                    painter.line_segment([pos2(c.x - s * 0.6, y), pos2(c.x + s * 0.6, y)], stroke);
                }
            }
            Icon::Minimize => {
                painter.line_segment(
                    [
                        pos2(c.x - s * 0.5, c.y + s * 0.3),
                        pos2(c.x + s * 0.5, c.y + s * 0.3),
                    ],
                    stroke,
                );
            }
            Icon::Maximize => {
                let r = Rect::from_center_size(c, Vec2::splat(s * 1.0));
                painter.rect_stroke(r, 1.0, stroke, egui::StrokeKind::Middle);
            }
            Icon::Restore => {
                let back = Rect::from_center_size(
                    c + Vec2::new(s * 0.18, -s * 0.18),
                    Vec2::splat(s * 0.8),
                );
                let front = Rect::from_center_size(
                    c + Vec2::new(-s * 0.12, s * 0.12),
                    Vec2::splat(s * 0.8),
                );
                painter.rect_stroke(back, 1.0, stroke, egui::StrokeKind::Middle);
                painter.rect_filled(front, 1.0, theme::bg());
                painter.rect_stroke(front, 1.0, stroke, egui::StrokeKind::Middle);
            }
            Icon::Close => {
                let d = s * 0.45;
                painter.line_segment([pos2(c.x - d, c.y - d), pos2(c.x + d, c.y + d)], stroke);
                painter.line_segment([pos2(c.x - d, c.y + d), pos2(c.x + d, c.y - d)], stroke);
            }
            Icon::FileNew | Icon::FileOpen | Icon::Save | Icon::SaveAs | Icon::Import => {
                file_icons(self, painter, c, s, color, stroke);
            }
            Icon::Recent | Icon::Update => {
                let r = s * 0.62;
                painter.circle_stroke(c, r, stroke);
                if self == Icon::Recent {
                    painter.line_segment([c, c + Vec2::new(0.0, -r * 0.65)], stroke);
                    painter.line_segment([c, c + Vec2::new(r * 0.5, r * 0.2)], stroke);
                } else {
                    // Pfeil nach unten in den Kreis
                    painter.line_segment(
                        [c + Vec2::new(0.0, -r * 0.6), c + Vec2::new(0.0, r * 0.45)],
                        stroke,
                    );
                    painter.add(Shape::line(
                        vec![
                            c + Vec2::new(-r * 0.4, r * 0.05),
                            c + Vec2::new(0.0, r * 0.48),
                            c + Vec2::new(r * 0.4, r * 0.05),
                        ],
                        stroke,
                    ));
                }
            }
            Icon::Undo | Icon::Redo => {
                let dir = if self == Icon::Redo { -1.0 } else { 1.0 };
                let r = s * 0.5;
                let cc = c + Vec2::new(0.0, s * 0.12);
                let n = 14;
                let pts: Vec<_> = (0..=n)
                    .map(|i| {
                        let a = -std::f32::consts::FRAC_PI_2
                            + std::f32::consts::PI * i as f32 / n as f32;
                        pos2(cc.x + dir * a.cos() * r, cc.y + a.sin() * r)
                    })
                    .collect();
                painter.add(Shape::line(pts, stroke));
                let start = pos2(cc.x, cc.y - r);
                painter.line_segment([start, pos2(cc.x - dir * r * 0.9, cc.y - r)], stroke);
                let tip = pos2(cc.x - dir * r * 0.95, cc.y - r);
                painter.add(Shape::convex_polygon(
                    vec![
                        tip + Vec2::new(-dir * s * 0.12, 0.0),
                        tip + Vec2::new(dir * s * 0.22, -s * 0.24),
                        tip + Vec2::new(dir * s * 0.22, s * 0.24),
                    ],
                    color,
                    Stroke::NONE,
                ));
            }
            Icon::Settings => {
                // Zahnkranz als Umriss: 8 Zähne, Loch in der Mitte
                let (outer, inner) = (s * 0.78, s * 0.56);
                let teeth = 8;
                let mut pts = Vec::new();
                for k in 0..teeth {
                    let base = std::f32::consts::TAU * k as f32 / teeth as f32;
                    let step = std::f32::consts::TAU / teeth as f32;
                    for (frac, r) in [(0.0, inner), (0.18, outer), (0.5, outer), (0.68, inner)] {
                        pts.push(c + Vec2::angled(base + step * frac) * r);
                    }
                }
                painter.add(Shape::closed_line(pts, Stroke::new(1.5, color)));
                painter.circle_stroke(c, s * 0.22, Stroke::new(1.5, color));
            }
            Icon::Fullscreen => {
                let d = s * 0.6;
                let l = s * 0.35;
                for (sx, sy) in [(-1.0, -1.0), (1.0, -1.0), (-1.0, 1.0), (1.0, 1.0)] {
                    let corner = c + Vec2::new(sx * d, sy * d);
                    painter.add(Shape::line(
                        vec![
                            corner + Vec2::new(0.0, -sy * l),
                            corner,
                            corner + Vec2::new(-sx * l, 0.0),
                        ],
                        stroke,
                    ));
                }
            }
            Icon::ZoomIn | Icon::ZoomOut | Icon::ZoomReset => {
                let lens = c + Vec2::new(-s * 0.12, -s * 0.12);
                let r = s * 0.45;
                painter.circle_stroke(lens, r, stroke);
                painter.line_segment(
                    [lens + Vec2::splat(r * 0.72), c + Vec2::splat(s * 0.68)],
                    Stroke::new(2.2, color),
                );
                let m = r * 0.5;
                if self != Icon::ZoomReset {
                    painter
                        .line_segment([lens - Vec2::new(m, 0.0), lens + Vec2::new(m, 0.0)], stroke);
                }
                if self == Icon::ZoomIn {
                    painter
                        .line_segment([lens - Vec2::new(0.0, m), lens + Vec2::new(0.0, m)], stroke);
                }
            }
            Icon::Cube => {
                let a = s * 0.62;
                let top = c + Vec2::new(0.0, -a);
                let mid = c + Vec2::new(0.0, a * 0.05);
                let left = c + Vec2::new(-a * 0.87, -a * 0.47);
                let right = c + Vec2::new(a * 0.87, -a * 0.47);
                let bl = c + Vec2::new(-a * 0.87, a * 0.52);
                let br = c + Vec2::new(a * 0.87, a * 0.52);
                let bottom = c + Vec2::new(0.0, a * 1.0);
                painter.add(Shape::closed_line(
                    vec![top, right, br, bottom, bl, left],
                    stroke,
                ));
                painter.add(Shape::line(vec![left, mid, right], stroke));
                painter.line_segment([mid, bottom], stroke);
            }
            Icon::Levels => {
                for (k, h) in [(-1.0, 0.5), (0.0, 1.1), (1.0, 0.8)] {
                    let w = s * 0.34;
                    let x = c.x + k * s * 0.5;
                    let bottom = c.y + s * 0.6;
                    let r = Rect::from_min_max(
                        pos2(x - w / 2.0, bottom - s * h),
                        pos2(x + w / 2.0, bottom),
                    );
                    painter.rect_filled(r, 1.0, color);
                }
            }
            Icon::Builder => {
                // Schraubenschlüssel
                let a = c + Vec2::new(-s * 0.45, s * 0.45);
                let b = c + Vec2::new(s * 0.2, -s * 0.2);
                painter.line_segment([a, b], Stroke::new(2.6, color));
                painter.circle_stroke(
                    b + Vec2::new(s * 0.12, -s * 0.12),
                    s * 0.3,
                    Stroke::new(2.0, color),
                );
            }
            Icon::Keyboard => {
                let body = Rect::from_center_size(c, Vec2::new(s * 1.6, s * 1.0));
                painter.rect_stroke(body, 2.0, stroke, egui::StrokeKind::Middle);
                for row in 0..2 {
                    for col in 0..4 {
                        let p = pos2(
                            body.left() + s * 0.32 + col as f32 * s * 0.32,
                            body.top() + s * 0.3 + row as f32 * s * 0.24,
                        );
                        painter.circle_filled(p, s * 0.06, color);
                    }
                }
                painter.line_segment(
                    [
                        pos2(body.left() + s * 0.45, body.bottom() - s * 0.22),
                        pos2(body.right() - s * 0.45, body.bottom() - s * 0.22),
                    ],
                    stroke,
                );
            }
            Icon::Midi => {
                let r = s * 0.62;
                painter.circle_stroke(c, r, stroke);
                for k in 0..5 {
                    let a = std::f32::consts::PI * (0.0 + k as f32 / 4.0);
                    let d = Vec2::angled(std::f32::consts::PI + a);
                    painter.circle_filled(c + d * r * 0.55, s * 0.08, color);
                }
                painter.rect_filled(
                    Rect::from_center_size(
                        c + Vec2::new(0.0, r * 0.72),
                        Vec2::new(s * 0.22, s * 0.12),
                    ),
                    0.0,
                    color,
                );
            }
            Icon::Touch => {
                painter.circle_filled(c + Vec2::new(0.0, -s * 0.15), s * 0.16, color);
                painter.circle_stroke(
                    c + Vec2::new(0.0, -s * 0.15),
                    s * 0.42,
                    Stroke::new(1.2, color),
                );
                painter.line_segment(
                    [c + Vec2::new(0.0, -s * 0.15), c + Vec2::new(0.0, s * 0.7)],
                    Stroke::new(2.0, color),
                );
            }
            Icon::Info => {
                let r = s * 0.62;
                painter.circle_stroke(c, r, stroke);
                painter.circle_filled(c + Vec2::new(0.0, -r * 0.45), s * 0.08, color);
                painter.line_segment(
                    [c + Vec2::new(0.0, -r * 0.1), c + Vec2::new(0.0, r * 0.5)],
                    Stroke::new(1.8, color),
                );
            }
            Icon::Edit => {
                let a = c + Vec2::new(-s * 0.5, s * 0.5);
                let b = c + Vec2::new(s * 0.45, -s * 0.45);
                painter.line_segment(
                    [a + Vec2::new(s * 0.08, -s * 0.08), b],
                    Stroke::new(3.0, color),
                );
                painter.add(Shape::convex_polygon(
                    vec![
                        a,
                        a + Vec2::new(s * 0.28, 0.0),
                        a + Vec2::new(0.0, -s * 0.28),
                    ],
                    color,
                    Stroke::NONE,
                ));
            }
            Icon::Trash => {
                let lid = c.y - s * 0.45;
                painter.line_segment([pos2(c.x - s * 0.6, lid), pos2(c.x + s * 0.6, lid)], stroke);
                painter.line_segment(
                    [
                        pos2(c.x - s * 0.2, lid - s * 0.18),
                        pos2(c.x + s * 0.2, lid - s * 0.18),
                    ],
                    stroke,
                );
                let body = Rect::from_min_max(
                    pos2(c.x - s * 0.45, lid + s * 0.12),
                    pos2(c.x + s * 0.45, c.y + s * 0.65),
                );
                painter.rect_stroke(body, 1.5, stroke, egui::StrokeKind::Middle);
            }
            Icon::Group => {
                for (dx, dy) in [(-0.35, -0.3), (0.35, -0.3), (0.0, 0.35)] {
                    painter.circle_stroke(c + Vec2::new(dx * s, dy * s), s * 0.28, stroke);
                }
            }
            Icon::Copy | Icon::Paste => {
                let back = Rect::from_center_size(
                    c + Vec2::new(s * 0.15, -s * 0.15),
                    Vec2::new(s * 0.85, s * 1.0),
                );
                let front = back.translate(Vec2::new(-s * 0.3, s * 0.3));
                painter.rect_stroke(back, 1.5, stroke, egui::StrokeKind::Middle);
                painter.rect_filled(front, 1.5, theme::raised());
                painter.rect_stroke(front, 1.5, stroke, egui::StrokeKind::Middle);
                if self == Icon::Paste {
                    painter.rect_filled(front.shrink(s * 0.2), 1.0, color);
                }
            }
            Icon::Plus => {
                painter.line_segment(
                    [pos2(c.x - s * 0.55, c.y), pos2(c.x + s * 0.55, c.y)],
                    stroke,
                );
                painter.line_segment(
                    [pos2(c.x, c.y - s * 0.55), pos2(c.x, c.y + s * 0.55)],
                    stroke,
                );
            }
        }
    }
}

/// Datei-Symbole: Blatt mit Eselsohr und je einem Zusatz.
fn file_icons(
    icon: Icon,
    painter: &egui::Painter,
    c: egui::Pos2,
    s: f32,
    color: Color32,
    stroke: Stroke,
) {
    if icon == Icon::FileOpen {
        // Ordner
        let body = Rect::from_center_size(c + Vec2::new(0.0, s * 0.1), Vec2::new(s * 1.5, s * 1.0));
        painter.add(Shape::closed_line(
            vec![
                body.left_bottom(),
                body.left_top(),
                body.left_top() + Vec2::new(s * 0.5, 0.0),
                body.left_top() + Vec2::new(s * 0.65, s * 0.18),
                body.right_top() + Vec2::new(0.0, s * 0.18),
                body.right_bottom(),
            ],
            stroke,
        ));
        return;
    }
    if icon == Icon::Save || icon == Icon::SaveAs {
        // Diskette
        let body = Rect::from_center_size(c, Vec2::splat(s * 1.3));
        painter.rect_stroke(body, 2.0, stroke, egui::StrokeKind::Middle);
        let label = Rect::from_min_max(
            body.left_top() + Vec2::new(s * 0.3, 0.0),
            body.right_top() + Vec2::new(-s * 0.3, s * 0.4),
        );
        painter.rect_filled(label, 0.0, color);
        painter.circle_stroke(c + Vec2::new(0.0, s * 0.25), s * 0.18, stroke);
        if icon == Icon::SaveAs {
            painter.circle_filled(body.right_bottom(), s * 0.3, theme::raised());
            painter.line_segment(
                [
                    body.right_bottom() + Vec2::new(-s * 0.18, s * 0.18),
                    body.right_bottom() + Vec2::new(s * 0.22, -s * 0.22),
                ],
                Stroke::new(2.0, color),
            );
        }
        return;
    }
    let w = s * 1.1;
    let h = s * 1.4;
    let r = Rect::from_center_size(c, Vec2::new(w, h));
    let ear = s * 0.35;
    painter.add(Shape::closed_line(
        vec![
            r.left_top(),
            r.right_top() - Vec2::new(ear, 0.0),
            r.right_top() + Vec2::new(0.0, ear),
            r.right_bottom(),
            r.left_bottom(),
        ],
        stroke,
    ));
    match icon {
        Icon::FileNew => {
            let m = c + Vec2::new(0.0, s * 0.15);
            painter.line_segment(
                [m - Vec2::new(s * 0.28, 0.0), m + Vec2::new(s * 0.28, 0.0)],
                stroke,
            );
            painter.line_segment(
                [m - Vec2::new(0.0, s * 0.28), m + Vec2::new(0.0, s * 0.28)],
                stroke,
            );
        }
        Icon::Import => {
            let m = c + Vec2::new(0.0, s * 0.1);
            painter.line_segment(
                [m - Vec2::new(0.0, s * 0.4), m + Vec2::new(0.0, s * 0.3)],
                stroke,
            );
            painter.add(Shape::line(
                vec![
                    m + Vec2::new(-s * 0.28, 0.02),
                    m + Vec2::new(0.0, s * 0.32),
                    m + Vec2::new(s * 0.28, 0.02),
                ],
                stroke,
            ));
        }
        _ => {}
    }
}

/// Kleiner quadratischer Knopf mit Symbol.
pub struct IconButton {
    icon: Icon,
    size: f32,
    enabled: bool,
    active: bool,
    flat: bool,
    danger: bool,
}

impl IconButton {
    pub fn new(icon: Icon) -> Self {
        Self {
            icon,
            size: 24.0,
            enabled: true,
            active: false,
            flat: false,
            danger: false,
        }
    }
    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }
    pub fn active(mut self, on: bool) -> Self {
        self.active = on;
        self
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    /// Ohne Hintergrund (nur beim Überfahren hinterlegt).
    pub fn flat(mut self) -> Self {
        self.flat = true;
        self
    }
    /// Beim Überfahren rot (Schließen-Knopf).
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(self.size), sense);
        let hovered = resp.hovered() && self.enabled;
        let bg = if self.active {
            theme::accent_dim()
        } else if hovered && self.danger {
            theme::DANGER
        } else if hovered {
            theme::raised_hover()
        } else if self.flat {
            egui::Color32::TRANSPARENT
        } else {
            theme::raised()
        };
        let p = ui.painter();
        p.rect_filled(rect, theme::R_S as f32, bg);
        let color = if !self.enabled {
            theme::text_faint()
        } else if self.active {
            theme::accent()
        } else {
            theme::text()
        };
        self.icon.paint(p, rect.shrink(6.0), color);
        resp
    }
}
