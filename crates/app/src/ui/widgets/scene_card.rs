//! Szenenkarte für das Szenen-Raster.
//!
//! Aufbau: links der Hauptbereich (Name in der Szenenfarbe, darunter der Typ
//! in Großbuchstaben), rechts ein schmaler Streifen. Ein Klick auf den
//! Hauptbereich startet/stoppt die Szene, ein Klick auf den Streifen
//! wählt sie aus (der Streifen leuchtet dann in der Szenenfarbe). Am Streifen
//! lässt sich die Szene auch in eine andere Bank ziehen.

use eframe::egui::{
    self, pos2, Color32, CornerRadius, Rect, Response, Sense, Stroke, StrokeKind, Ui, Vec2,
};

use crate::ui::theme;
use crate::ui::widgets::Icon;

/// Höhe einer Karte.
pub const CARD_H: f32 = 54.0;
/// Höhe einer Karte in der Kompaktansicht (nur Name).
pub const COMPACT_H: f32 = 32.0;
/// Breite des Markier-Streifens rechts.
const STRIP_W: f32 = 22.0;

pub struct SceneCard<'a> {
    pub name: &'a str,
    pub kind: String,
    pub color: Color32,
    pub active: bool,
    pub paused: bool,
    /// Gesamtfortschritt 0..=1 (nur wenn aktiv).
    pub progress: f32,
    pub width: f32,
    /// Markiert (Streifen, Strg/Umschalt, „Alle Szenen auswählen“).
    pub selected: bool,
    /// Ablauf mit mehreren Schritten (Symbol rechts unten).
    pub sequence: bool,
    /// Ein-/Ausblenden läuft: Rampe in der Fortschrittsleiste.
    pub fade: Option<dmxcraft_core::FadeState>,
    /// Flash-Szene (kleiner Blitz auf der Karte).
    pub flash: bool,
    /// SUPER SCENE: Notensymbol.
    pub super_scene: bool,
    /// Kompaktansicht: niedrig, nur der Name (ohne Typ).
    pub compact: bool,
}

/// Antworten der beiden Bereiche einer Karte.
pub struct SceneCardResponse {
    /// Hauptbereich: starten/stoppen.
    pub play: Response,
    /// Streifen: markieren.
    pub select: Response,
    /// Rechteck des Hauptbereichs (z. B. für das Umbenennen-Feld).
    pub main_rect: Rect,
}

impl SceneCardResponse {
    /// Beide Bereiche zusammen (für das Kontextmenü).
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
        // Streifen: markiert = Szenenfarbe, sonst Grau
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
            // Hover nur am Rand – die Fläche bleibt (Streifen daneben ist grau)
            Stroke::new(1.5, self.color.gamma_multiply(0.75))
        } else {
            Stroke::new(1.0, theme::border())
        };
        p.rect_stroke(rect, r, outline, StrokeKind::Inside);

        let inner = main_rect.shrink2(Vec2::new(10.0, 8.0));
        // rechts Platz für die Symbole (Zustand, Flash, Note)
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
        // Zustand rechts oben im Hauptbereich (kompakt: mittig): läuft / Pause / Ablauf
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
        // Flash: kleiner Blitz links neben dem Zustandssymbol
        if self.flash {
            let r =
                Rect::from_center_size(pos2(main_rect.right() - 32.0, icon_y), Vec2::splat(13.0));
            Icon::Lightning.paint(p, r, if self.active { self.color } else { theme::WARN });
        }
        // SUPER SCENE: Musiknote (enthält Audio und Szenen)
        if self.super_scene {
            let x = if self.flash { 50.0 } else { 32.0 };
            let r = Rect::from_center_size(pos2(main_rect.right() - x, icon_y), Vec2::splat(13.0));
            Icon::Music.paint(p, r, self.color);
        }
        // Ein-/Ausblenden: Rampe in der Leiste (steigt beim Ein-, fällt beim
        // Ausblenden), gefüllt bis zum aktuellen Stand
        if let Some(fade) = self.fade {
            let bar = Rect::from_min_max(
                pos2(main_rect.left() + 8.0, main_rect.bottom() - 11.0),
                pos2(main_rect.right() - 8.0, main_rect.bottom() - 2.0),
            );
            let (b, h) = (bar.bottom(), bar.height());
            let ramp = |x: f32| -> f32 {
                // Höhe der Rampe an Stelle x (0..1)
                // nie ganz flach, damit der Anfang sichtbar bleibt
                0.2 + 0.8
                    * match fade {
                        dmxcraft_core::FadeState::In(_) => x,
                        dmxcraft_core::FadeState::Out(_) => 1.0 - x,
                    }
            };
            let pt = |x: f32| pos2(bar.left() + bar.width() * x, b - h * ramp(x));
            // Umriss der ganzen Rampe
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
            // Fortschritt unten im Hauptbereich
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
