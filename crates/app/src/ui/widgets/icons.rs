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
