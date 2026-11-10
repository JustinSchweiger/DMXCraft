//! Scene card for the scene grid.
//!
//! Layout: on the left the main area (name in the scene color, below it the
//! type in upper case), on the right a narrow strip. A click on the main
//! area starts/stops the scene, a click on the strip selects it (the strip
//! then lights up in the scene color). The strip also lets you drag the
//! scene into another bank.

use eframe::egui::{
    self, pos2, Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::ui::theme;
use crate::ui::widgets::Icon;

/// Height of a card.
pub const CARD_H: f32 = 54.0;
/// Height of a card in the compact view (name only).
pub const COMPACT_H: f32 = 32.0;
/// Width of the selection strip on the right.
const STRIP_W: f32 = 22.0;

pub struct SceneCard<'a> {
    pub name: &'a str,
    pub kind: String,
    pub color: Color32,
    pub active: bool,
    pub paused: bool,
    /// Overall progress 0..=1 (only when active).
    pub progress: f32,
    pub width: f32,
    /// Selected (strip, Ctrl/Shift, “Select all scenes”).
    pub selected: bool,
    /// Sequence with several steps (icon bottom right).
    pub sequence: bool,
    /// Fade in/out running: ramp in the progress bar.
    pub fade: Option<dmxcraft_core::FadeState>,
    /// Flash scene (small lightning bolt on the card).
    pub flash: bool,
    /// SUPER SCENE: note icon.
    pub super_scene: bool,
    /// Compact view: low, name only (without type).
    pub compact: bool,
    /// SUPER SCENE with a missing audio file: red exclamation mark.
    pub missing: bool,
}

/// Responses of the two areas of a card.
pub struct SceneCardResponse {
    /// Main area: start/stop.
    pub play: Response,
    /// Strip: select.
    pub select: Response,
    /// Rectangle of the main area (e.g. for the rename field).
    pub main_rect: Rect,
}

impl SceneCardResponse {
    /// Both areas together (for the context menu).
    pub fn whole(&self) -> Response {
        self.play.clone() | self.select.clone()
    }
}

impl SceneCard<'_> {
    pub fn show(self, ui: &mut Ui) -> SceneCardResponse {
        let h = if self.compact { COMPACT_H } else { CARD_H };
        let (rect, _) = ui.allocate_exact_size(Vec2::new(self.width, h), Sense::hover());
        let main_rect = Rect::from_min_max(rect.min, pos2(rect.right() - STRIP_W, rect.bottom()));
        let strip = Rect::from_min_max(pos2(main_rect.right(), rect.top()), rect.max);
        let id = ui.next_auto_id();
        let play = ui.interact(main_rect, id.with("play"), Sense::click());
        let select = ui.interact(strip, id.with("select"), Sense::click_and_drag());

        let p = ui.painter();
        let r = theme::R_S;
        let fill = if self.active {
            self.color.gamma_multiply(0.18)
        } else {
            theme::raised()
        };
        p.rect_filled(
            main_rect,
            CornerRadius {
                nw: r,
                sw: r,
                ne: 0,
                se: 0,
            },
            fill,
        );
        // Strip: selected = scene color, otherwise gray
        let strip_fill = if self.selected {
            self.color
        } else if select.hovered() || select.dragged() {
            theme::text_faint()
        } else {
            theme::raised_hover()
        };
        p.rect_filled(
            strip,
            CornerRadius {
                nw: 0,
                sw: 0,
                ne: r,
                se: r,
            },
            strip_fill,
        );
        let outline = if self.active {
            Stroke::new(1.5, self.color)
        } else if play.hovered() || select.hovered() {
            // Hover only at the border – the area stays (strip next to it is gray)
            Stroke::new(1.5, self.color.gamma_multiply(0.75))
        } else {
            Stroke::new(1.0, theme::border())
        };
        p.rect_stroke(rect, r, outline, StrokeKind::Inside);

        let inner = main_rect.shrink2(Vec2::new(10.0, 8.0));
        // room on the right for the icons (state, flash, note)
        let icons = 1 + usize::from(self.flash) + usize::from(self.super_scene);
        let name_room = main_rect.width() - 12.0 - 18.0 * icons as f32;
        if self.compact {
            p.text(
                pos2(inner.left(), main_rect.center().y - 1.0),
                egui::Align2::LEFT_CENTER,
                clip(self.name, (name_room / 7.2) as usize),
                theme::body(),
                theme::ink(self.color),
            );
        } else {
            p.text(
                inner.left_top(),
                egui::Align2::LEFT_TOP,
                clip(self.name, ((main_rect.width() - 30.0) / 7.2) as usize),
                theme::body(),
                theme::ink(self.color),
            );
            p.text(
                inner.left_top() + Vec2::new(0.0, 20.0),
                egui::Align2::LEFT_TOP,
                self.kind.to_uppercase(),
                theme::heading_font(),
                if self.active {
                    theme::text()
                } else {
                    theme::text_dim()
                },
            );
        }
        // State top right in the main area (compact: centered): playing / paused /
        // sequence
        let icon_y = if self.compact {
            main_rect.center().y - 1.0
        } else {
            main_rect.top() + 13.0
        };
        let icon_rect =
            Rect::from_center_size(pos2(main_rect.right() - 13.0, icon_y), Vec2::splat(14.0));
        let icon = match (self.active, self.paused, self.sequence) {
            (true, true, _) => Some(Icon::Pause),
            (true, false, _) => Some(Icon::Play),
            (false, _, true) => Some(Icon::Reset),
            _ => None,
        };
        if let Some(i) = icon {
            i.paint(
                p,
                icon_rect,
                if self.active {
                    self.color
                } else {
                    theme::text_faint()
                },
            );
        }
        // Flash: small lightning bolt left of the state icon
        if self.flash {
            let r =
                Rect::from_center_size(pos2(main_rect.right() - 32.0, icon_y), Vec2::splat(13.0));
            Icon::Lightning.paint(p, r, if self.active { self.color } else { theme::WARN });
        }
        // SUPER SCENE: music note (contains audio and scenes)
        if self.super_scene {
            let x = if self.flash { 50.0 } else { 32.0 };
            let r = Rect::from_center_size(pos2(main_rect.right() - x, icon_y), Vec2::splat(13.0));
            if self.missing {
                crate::ui::widgets::icons::paint_alert(p, r.expand(1.0), theme::DANGER);
            } else {
                Icon::Music.paint(p, r, self.color);
            }
        }
        // Fade in/out: ramp in the bar (rising when fading in, falling when fading
        // out), filled up to the current state
        if let Some(fade) = self.fade {
            let bar = Rect::from_min_max(
                pos2(main_rect.left() + 8.0, main_rect.bottom() - 11.0),
                pos2(main_rect.right() - 8.0, main_rect.bottom() - 2.0),
            );
            let (b, h) = (bar.bottom(), bar.height());
            let ramp = |x: f32| -> f32 {
                // Height of the ramp at position x (0..1)
                // never completely flat so the beginning stays visible
                0.2 + 0.8
                    * match fade {
                        dmxcraft_core::FadeState::In(_) => x,
                        dmxcraft_core::FadeState::Out(_) => 1.0 - x,
                    }
            };
            let pt = |x: f32| pos2(bar.left() + bar.width() * x, b - h * ramp(x));
            // Outline of the whole ramp
            p.add(egui::Shape::convex_polygon(
                vec![pos2(bar.left(), b), pt(0.0), pt(1.0), pos2(bar.right(), b)],
                theme::bg(),
                Stroke::NONE,
            ));
            let done = match fade {
                dmxcraft_core::FadeState::In(l) => l,
                dmxcraft_core::FadeState::Out(l) => 1.0 - l,
            }
            .clamp(0.0, 1.0);
            if done > 0.0 {
                p.add(egui::Shape::convex_polygon(
                    vec![
                        pos2(bar.left(), b),
                        pt(0.0),
                        pt(done),
                        pos2(bar.left() + bar.width() * done, b),
                    ],
                    self.color,
                    Stroke::NONE,
                ));
            }
        } else if self.active {
            // Progress at the bottom of the main area
            let bar = Rect::from_min_max(
                pos2(main_rect.left() + 8.0, main_rect.bottom() - 4.0),
                pos2(main_rect.right() - 8.0, main_rect.bottom() - 2.0),
            );
            p.rect_filled(bar, 1.0, theme::bg());
            let w = bar.width() * self.progress.clamp(0.0, 1.0);
            let done = Rect::from_min_size(bar.min, Vec2::new(w, bar.height()));
            p.rect_filled(
                done,
                1.0,
                if self.paused {
                    theme::text_dim()
                } else {
                    self.color
                },
            );
        }
        SceneCardResponse {
            play,
            select,
            main_rect,
        }
    }
}

fn clip(s: &str, max: usize) -> String {
    let max = max.max(4);
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}...", s.chars().take(max - 3).collect::<String>())
    }
}
