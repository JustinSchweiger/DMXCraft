//! Eigener Farbwähler im App-Stil: Feld für Sättigung/Helligkeit, Leiste für
//! den Farbton und ein Hex-Feld (#RRGGBB).
//!
//! Der Farbton wird pro Wähler gemerkt, damit er bei Grau/Schwarz (dort ist
//! er aus RGB nicht ablesbar) nicht verloren geht.

use eframe::egui::{
    self, ecolor::Hsva, epaint::Mesh, pos2, vec2, Color32, Rect, Sense, Shape, Stroke, StrokeKind,
    Ui, Vec2,
};

use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::dialog::text_field;
use crate::ui::widgets::Icon;

const W: f32 = 200.0;
const SV_H: f32 = 120.0;
const HUE_H: f32 = 14.0;

fn hsv_color(h: f32, s: f32, v: f32) -> Color32 {
    Hsva::new(h, s, v, 1.0).into()
}

/// Zeigt den Farbwähler; `true`, wenn `rgb` geändert wurde.
pub fn color_picker(ui: &mut Ui, id: egui::Id, rgb: &mut [u8; 3]) -> bool {
    let mut changed = false;
    let stored: Option<[f32; 3]> = ui.data(|d| d.get_temp(id));
    let from_rgb = Hsva::from_srgb(*rgb);
    // gemerkten Farbton behalten, solange die Farbe dazu passt
    let mut hsv = match stored {
        Some([h, s, v]) if Hsva::new(h, s, v, 1.0).to_srgb() == *rgb => [h, s, v],
        Some([h, _, _]) if from_rgb.s < 0.01 || from_rgb.v < 0.01 => [h, from_rgb.s, from_rgb.v],
        _ => [from_rgb.h, from_rgb.s, from_rgb.v],
    };

    // ----- Sättigung (x) / Helligkeit (y) -----
    let (sv_rect, sv) = ui.allocate_exact_size(vec2(W, SV_H), Sense::click_and_drag());
    let mut mesh = Mesh::default();
    const N: u32 = 12;
    for yi in 0..=N {
        for xi in 0..=N {
            let (s, v) = (xi as f32 / N as f32, 1.0 - yi as f32 / N as f32);
            mesh.colored_vertex(
                pos2(
                    sv_rect.left() + sv_rect.width() * xi as f32 / N as f32,
                    sv_rect.top() + sv_rect.height() * yi as f32 / N as f32,
                ),
                hsv_color(hsv[0], s, v),
            );
        }
    }
    for yi in 0..N {
        for xi in 0..N {
            let i = yi * (N + 1) + xi;
            mesh.add_triangle(i, i + 1, i + N + 1);
            mesh.add_triangle(i + 1, i + N + 2, i + N + 1);
        }
    }
    let p = ui.painter();
    p.add(Shape::mesh(mesh));
    p.rect_stroke(
        sv_rect,
        3.0,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );
    if let Some(pos) = sv
        .interact_pointer_pos()
        .filter(|_| sv.dragged() || sv.clicked())
    {
        hsv[1] = ((pos.x - sv_rect.left()) / sv_rect.width()).clamp(0.0, 1.0);
        hsv[2] = 1.0 - ((pos.y - sv_rect.top()) / sv_rect.height()).clamp(0.0, 1.0);
        changed = true;
    }
    let knob = pos2(
        sv_rect.left() + hsv[1] * sv_rect.width(),
        sv_rect.top() + (1.0 - hsv[2]) * sv_rect.height(),
    );
    p.circle_stroke(knob, 6.0, Stroke::new(2.0, Color32::WHITE));
    p.circle_stroke(knob, 7.0, Stroke::new(1.0, Color32::BLACK));

    ui.add_space(theme::SP_XS);

    // ----- Farbton -----
    let (hue_rect, hue) = ui.allocate_exact_size(vec2(W, HUE_H), Sense::click_and_drag());
    let mut mesh = Mesh::default();
    const HN: u32 = 24;
    for i in 0..=HN {
        let f = i as f32 / HN as f32;
        let x = hue_rect.left() + hue_rect.width() * f;
        let c = hsv_color(f, 1.0, 1.0);
        mesh.colored_vertex(pos2(x, hue_rect.top()), c);
        mesh.colored_vertex(pos2(x, hue_rect.bottom()), c);
    }
    for i in 0..HN {
        let k = i * 2;
        mesh.add_triangle(k, k + 1, k + 2);
        mesh.add_triangle(k + 1, k + 3, k + 2);
    }
    let p = ui.painter();
    p.add(Shape::mesh(mesh));
    p.rect_stroke(
        hue_rect,
        3.0,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );
    if let Some(pos) = hue
        .interact_pointer_pos()
        .filter(|_| hue.dragged() || hue.clicked())
    {
        hsv[0] = ((pos.x - hue_rect.left()) / hue_rect.width()).clamp(0.0, 0.999);
        changed = true;
    }
    let hx = hue_rect.left() + hsv[0] * hue_rect.width();
    let marker = Rect::from_center_size(pos2(hx, hue_rect.center().y), vec2(5.0, HUE_H + 4.0));
    p.rect_stroke(
        marker,
        2.0,
        Stroke::new(2.0, Color32::WHITE),
        StrokeKind::Middle,
    );

    ui.add_space(theme::SP_XS);

    // ----- Vorschau + Hex -----
    ui.horizontal(|ui| {
        let (prev, _) = ui.allocate_exact_size(vec2(28.0, 24.0), Sense::hover());
        ui.painter()
            .rect_filled(prev, 3.0, hsv_color(hsv[0], hsv[1], hsv[2]));
        ui.painter().rect_stroke(
            prev,
            3.0,
            Stroke::new(1.0, theme::border()),
            StrokeKind::Inside,
        );
        let hex_id = id.with("hex");
        let current = format!("#{:02X}{:02X}{:02X}", rgb[0], rgb[1], rgb[2]);
        let mut text: String = ui
            .data(|d| d.get_temp(hex_id))
            .unwrap_or_else(|| current.clone());
        let r = ui.add(
            text_field(&mut text)
                .desired_width(W - 28.0 - ui.spacing().item_spacing.x)
                .font(egui::TextStyle::Monospace),
        );
        if r.changed() {
            if let Some(c) = parse_hex(&text) {
                let h = Hsva::from_srgb(c);
                hsv = [if h.s > 0.01 { h.h } else { hsv[0] }, h.s, h.v];
                changed = true;
            }
        }
        // Während der Eingabe den Text behalten, sonst die aktuelle Farbe zeigen
        if r.has_focus() {
            ui.data_mut(|d| d.insert_temp(hex_id, text));
        } else {
            ui.data_mut(|d| d.remove::<String>(hex_id));
        }
    });

    if changed {
        *rgb = Hsva::new(hsv[0], hsv[1], hsv[2], 1.0).to_srgb();
    }
    ui.data_mut(|d| d.insert_temp(id, hsv));
    changed
}

/// „#RRGGBB“, „RRGGBB“ oder „#RGB“.
fn parse_hex(s: &str) -> Option<[u8; 3]> {
    let h = s.trim().trim_start_matches('#');
    let full: String = match h.len() {
        3 => h.chars().flat_map(|c| [c, c]).collect(),
        6 => h.to_string(),
        _ => return None,
    };
    let v = u32::from_str_radix(&full, 16).ok()?;
    Some([(v >> 16) as u8, (v >> 8) as u8, v as u8])
}

/// Farbvorgaben (Quadrate in Bank- und Szenenmenü).
pub const PRESET_COLORS: [[u8; 3]; 12] = [
    [0x19, 0xe3, 0xf0],
    [0x55, 0xa8, 0xff],
    [0xa0, 0x84, 0xff],
    [0xf4, 0x5d, 0xb8],
    [0xff, 0x3b, 0x4e],
    [0xff, 0x80, 0x4d],
    [0xf8, 0xb3, 0x3a],
    [0xff, 0xe1, 0x4d],
    [0x8d, 0xe0, 0x4c],
    [0x2e, 0xcc, 0x71],
    [0xd8, 0xde, 0xe6],
    [0x8a, 0x93, 0x9e],
];

/// Ein Feld der Farbauswahl.
#[derive(Clone, Copy, PartialEq)]
enum Swatch {
    /// Farbe nach Position (keine eigene Farbe).
    Auto,
    Color([u8; 3]),
    /// Eigene Farbe über den Farbwähler.
    Custom,
}

/// Farbfelder in zwei Reihen für Menüs; `Some(neu)` bei Auswahl, `Some(None)`
/// = Automatisch. „+“ klappt den Farbwähler auf.
pub fn color_swatches(
    ui: &mut Ui,
    id: egui::Id,
    current: Option<[u8; 3]>,
    auto_tip: &str,
) -> Option<Option<[u8; 3]>> {
    const SQ: f32 = 22.0;
    const PER_ROW: usize = 7;
    let mut picked = None;
    let picker_id = id.with("open");
    let mut picker_open = ui.data(|d| d.get_temp::<bool>(picker_id)).unwrap_or(false);
    let custom = current.filter(|c| !PRESET_COLORS.contains(c));
    let fields: Vec<Swatch> = std::iter::once(Swatch::Auto)
        .chain(PRESET_COLORS.iter().copied().map(Swatch::Color))
        .chain(std::iter::once(Swatch::Custom))
        .collect();
    for row in fields.chunks(PER_ROW) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            ui.add_space(theme::SP_S);
            for field in row {
                let (rect, resp) = ui.allocate_exact_size(Vec2::splat(SQ), Sense::click());
                let p = ui.painter();
                let active = match field {
                    Swatch::Auto => current.is_none(),
                    Swatch::Color(c) => current == Some(*c),
                    Swatch::Custom => custom.is_some(),
                };
                match field {
                    Swatch::Color(c) => {
                        p.rect_filled(rect.shrink(2.0), 3.0, theme::rgb(*c));
                    }
                    Swatch::Auto => {
                        // durchgestrichenes Feld
                        p.rect_stroke(
                            rect.shrink(2.5),
                            3.0,
                            Stroke::new(1.0, theme::text_dim()),
                            StrokeKind::Inside,
                        );
                        p.line_segment(
                            [rect.shrink(5.0).left_bottom(), rect.shrink(5.0).right_top()],
                            Stroke::new(1.2, theme::text_dim()),
                        );
                    }
                    Swatch::Custom => {
                        if let Some(c) = custom {
                            p.rect_filled(rect.shrink(2.0), 3.0, theme::rgb(c));
                        } else {
                            p.rect_stroke(
                                rect.shrink(2.5),
                                3.0,
                                Stroke::new(1.0, theme::text_dim()),
                                StrokeKind::Inside,
                            );
                        }
                        let fg = if custom.is_some() {
                            theme::bg()
                        } else {
                            theme::text()
                        };
                        Icon::Plus.paint(p, rect.shrink(6.0), fg);
                    }
                }
                if active {
                    p.rect_stroke(
                        rect,
                        4.0,
                        Stroke::new(2.0, theme::text()),
                        StrokeKind::Inside,
                    );
                } else if resp.hovered() {
                    p.rect_stroke(
                        rect,
                        4.0,
                        Stroke::new(1.0, theme::text_dim()),
                        StrokeKind::Inside,
                    );
                }
                let resp = match field {
                    Swatch::Auto => resp.on_hover_text(auto_tip),
                    Swatch::Custom => resp.on_hover_text(t("Custom color")),
                    Swatch::Color(_) => resp,
                };
                if resp.clicked() {
                    match field {
                        Swatch::Auto => picked = Some(None),
                        Swatch::Color(c) => picked = Some(Some(*c)),
                        Swatch::Custom => picker_open = !picker_open,
                    }
                }
            }
        });
    }
    if picker_open {
        let mut rgb = current.unwrap_or([0x19, 0xe3, 0xf0]);
        ui.horizontal(|ui| {
            ui.add_space(theme::SP_S);
            ui.vertical(|ui| {
                if color_picker(ui, id.with("picker"), &mut rgb) {
                    picked = Some(Some(rgb));
                }
            });
        });
    }
    ui.data_mut(|d| d.insert_temp(picker_id, picker_open));
    picked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_parsing() {
        assert_eq!(parse_hex("#19E3F0"), Some([0x19, 0xe3, 0xf0]));
        assert_eq!(parse_hex("fff"), Some([255, 255, 255]));
        assert_eq!(parse_hex("#12"), None);
        assert_eq!(parse_hex("zzzzzz"), None);
    }
}
