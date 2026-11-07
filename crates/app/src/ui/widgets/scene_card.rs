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
        let (rect, _) = ui.allocate_exact_size(Vec2::new(self.width, CARD_H), Sense::hover());
        let main_rect = Rect::from_min_max(rect.min, pos2(rect.right() - STRIP_W, rect.bottom()));
        let strip = Rect::from_min_max(pos2(main_rect.right(), rect.top()), rect.max);
        let id = ui.next_auto_id();
        let play = ui.interact(main_rect, id.with("play"), Sense::click());
        let select = ui.interact(strip, id.with("select"), Sense::click_and_drag());

        let p = ui.painter();
        let r = theme::R_S;
        let fill = if self.active {
            self.color.gamma_multiply(0.18)
        } else if play.hovered() {
            theme::raised_hover()
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
        } else {
            Stroke::new(1.0, theme::border())
        };
        p.rect_stroke(rect, r, outline, StrokeKind::Inside);

        let inner = main_rect.shrink2(Vec2::new(10.0, 8.0));
        p.text(
            inner.left_top(),
            egui::Align2::LEFT_TOP,
            clip(self.name, ((main_rect.width() - 30.0) / 7.2) as usize),
            theme::body(),
            self.color,
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
        // Zustand rechts unten im Hauptbereich: läuft / Pause / Ablauf
        let icon_rect = Rect::from_center_size(
            pos2(main_rect.right() - 13.0, main_rect.top() + 13.0),
            Vec2::splat(14.0),
        );
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
        // Fortschritt unten im Hauptbereich
        if self.active {
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
