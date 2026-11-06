//! Eigenes DmxCraft-Erscheinungsbild (Farben aus dem App-Logo).

use dmxcraft_core::{Show, ZoneId};
use eframe::egui::{self, Color32, CornerRadius, Stroke, Visuals};

pub const BG: Color32 = Color32::from_rgb(0x12, 0x16, 0x1d);
pub const PANEL: Color32 = Color32::from_rgb(0x19, 0x1f, 0x28);
pub const CARD: Color32 = Color32::from_rgb(0x22, 0x2a, 0x36);
pub const CARD_HOVER: Color32 = Color32::from_rgb(0x2a, 0x34, 0x42);
pub const ACCENT: Color32 = Color32::from_rgb(0x19, 0xe3, 0xf0);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x0e, 0x6f, 0x78);
pub const TEXT_WEAK: Color32 = Color32::from_rgb(0x8a, 0x96, 0xa8);
pub const DANGER: Color32 = Color32::from_rgb(0xd9, 0x3b, 0x4a);
pub const OK: Color32 = Color32::from_rgb(0x4c, 0xd1, 0x7f);

/// Standardfarben für Zonen ohne eigene Farbe.
pub const ZONE_PALETTE: [[u8; 3]; 8] = [
    [0x19, 0xe3, 0xf0],
    [0xf0, 0x4f, 0xb4],
    [0xf5, 0xb0, 0x2e],
    [0x8b, 0xe0, 0x45],
    [0x9b, 0x7b, 0xff],
    [0xff, 0x7a, 0x45],
    [0x4a, 0xa8, 0xff],
    [0xe8, 0xe8, 0xe8],
];

pub fn apply(ctx: &egui::Context) {
    let mut v = Visuals::dark();
    v.panel_fill = PANEL;
    v.window_fill = PANEL;
    v.extreme_bg_color = BG;
    v.faint_bg_color = Color32::from_rgb(0x1d, 0x24, 0x2e);
    v.selection.bg_fill = ACCENT_DIM;
    v.selection.stroke = Stroke::new(1.0, ACCENT);
    v.hyperlink_color = ACCENT;
    let r = CornerRadius::same(4);
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = r;
    }
    v.widgets.inactive.weak_bg_fill = CARD;
    v.widgets.inactive.bg_fill = CARD;
    v.widgets.hovered.weak_bg_fill = CARD_HOVER;
    v.widgets.hovered.bg_fill = CARD_HOVER;
    v.widgets.active.weak_bg_fill = ACCENT_DIM;
    v.widgets.active.bg_fill = ACCENT_DIM;
    ctx.set_visuals(v);
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

/// Farbe einer Zone (eigene Farbe oder aus der Palette nach Position).
pub fn zone_color(show: &Show, zone: Option<ZoneId>) -> Color32 {
    let Some(z) = zone else {
        return TEXT_WEAK;
    };
    let idx = show.zones.iter().position(|x| x.id == z).unwrap_or(0);
    let c = show
        .zone(z)
        .and_then(|z| z.color)
        .unwrap_or(ZONE_PALETTE[idx % ZONE_PALETTE.len()]);
    rgb(c)
}
