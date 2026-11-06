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
                painter.circle_filled(body.center(), s * 0.1, theme::BG);
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
                painter.rect_filled(front, 1.0, theme::BG);
                painter.rect_stroke(front, 1.0, stroke, egui::StrokeKind::Middle);
            }
            Icon::Close => {
                let d = s * 0.45;
                painter.line_segment([pos2(c.x - d, c.y - d), pos2(c.x + d, c.y + d)], stroke);
                painter.line_segment([pos2(c.x - d, c.y + d), pos2(c.x + d, c.y - d)], stroke);
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
            theme::ACCENT_DIM
        } else if hovered && self.danger {
            theme::DANGER
        } else if hovered {
            theme::RAISED_HOVER
        } else if self.flat {
            egui::Color32::TRANSPARENT
        } else {
            theme::RAISED
        };
        let p = ui.painter();
        p.rect_filled(rect, theme::R_S as f32, bg);
        let color = if !self.enabled {
            theme::TEXT_FAINT
        } else if self.active {
            theme::ACCENT
        } else {
            theme::TEXT
        };
        self.icon.paint(p, rect.shrink(6.0), color);
        resp
    }
}
