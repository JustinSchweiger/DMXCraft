//! Design-Tokens von DmxCraft (siehe docs/UI-GUIDELINES.md).
//! Farben, Abstände, Radien und Schriftgrößen nur von hier verwenden.

use dmxcraft_core::{Show, ZoneId};
use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, Visuals};

// ---------- Farben ----------
pub const BG: Color32 = Color32::from_rgb(0x0b, 0x0e, 0x12);
pub const SURFACE: Color32 = Color32::from_rgb(0x12, 0x16, 0x1c);
pub const RAISED: Color32 = Color32::from_rgb(0x1a, 0x20, 0x28);
pub const RAISED_HOVER: Color32 = Color32::from_rgb(0x23, 0x2b, 0x35);
pub const BORDER: Color32 = Color32::from_rgb(0x28, 0x30, 0x3a);
pub const ACCENT: Color32 = Color32::from_rgb(0x19, 0xe3, 0xf0);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x0c, 0x5c, 0x64);
pub const TEXT: Color32 = Color32::from_rgb(0xe6, 0xec, 0xf2);
pub const TEXT_DIM: Color32 = Color32::from_rgb(0x95, 0xa2, 0xb0);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x55, 0x60, 0x6c);
pub const DANGER: Color32 = Color32::from_rgb(0xef, 0x44, 0x55);
pub const OK: Color32 = Color32::from_rgb(0x34, 0xd3, 0x99);
/// Gehäusefarbe der DIP-Schalter-Anzeige.
pub const DIP_BODY: Color32 = Color32::from_rgb(0x1c, 0x3f, 0x8a);
/// Geräte-Balken im DMX-Patch.
pub const FIXTURE_BAR: Color32 = Color32::from_rgb(0x2b, 0x35, 0x42);
/// Füllung ausgeschalteter Geräte im Bühnenplan.
pub const FIXTURE_OFF: Color32 = Color32::from_rgb(0x1e, 0x25, 0x2e);
pub const WARN: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);

/// Standardfarben für Zonen ohne eigene Farbe.
pub const ZONE_PALETTE: [[u8; 3]; 8] = [
    [0x19, 0xe3, 0xf0],
    [0xf4, 0x5d, 0xb8],
    [0xf8, 0xb3, 0x3a],
    [0x8d, 0xe0, 0x4c],
    [0xa0, 0x84, 0xff],
    [0xff, 0x80, 0x4d],
    [0x55, 0xa8, 0xff],
    [0xd8, 0xde, 0xe6],
];

// ---------- Abstände, Radien, Schrift ----------
pub const SP_XS: f32 = 4.0;
pub const SP_S: f32 = 8.0;
pub const SP_M: f32 = 12.0;
pub const SP_L: f32 = 16.0;
pub const R_S: u8 = 4;
pub const R_M: u8 = 8;
pub const FS_CAPTION: f32 = 10.5;
pub const FS_BODY: f32 = 13.0;
pub const FS_TITLE: f32 = 15.0;

pub fn caption() -> FontId {
    FontId::proportional(FS_CAPTION)
}

pub fn body() -> FontId {
    FontId::proportional(FS_BODY)
}

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    let mut v = Visuals::dark();
    v.panel_fill = SURFACE;
    v.window_fill = SURFACE;
    v.window_stroke = Stroke::new(1.0, BORDER);
    v.extreme_bg_color = BG;
    v.faint_bg_color = RAISED;
    v.override_text_color = Some(TEXT);
    v.selection.bg_fill = ACCENT_DIM;
    v.selection.stroke = Stroke::new(1.0, ACCENT);
    v.hyperlink_color = ACCENT;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER);
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = CornerRadius::same(R_S);
    }
    v.widgets.inactive.weak_bg_fill = RAISED;
    v.widgets.inactive.bg_fill = RAISED;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER);
    v.widgets.hovered.weak_bg_fill = RAISED_HOVER;
    v.widgets.hovered.bg_fill = RAISED_HOVER;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT_DIM);
    v.widgets.active.weak_bg_fill = ACCENT_DIM;
    v.widgets.active.bg_fill = ACCENT_DIM;
    ctx.set_visuals(v);

    ctx.global_style_mut(|s| {
        s.spacing.item_spacing = egui::vec2(SP_S, SP_XS + 2.0);
        s.spacing.button_padding = egui::vec2(SP_S, SP_XS);
        s.spacing.interact_size.y = 24.0;
        s.text_styles
            .insert(egui::TextStyle::Body, FontId::proportional(FS_BODY));
        s.text_styles
            .insert(egui::TextStyle::Button, FontId::proportional(FS_BODY));
        s.text_styles.insert(
            egui::TextStyle::Heading,
            FontId::proportional(FS_TITLE + 2.0),
        );
        s.text_styles
            .insert(egui::TextStyle::Small, FontId::proportional(FS_CAPTION));
    });
}

pub fn rgb(c: [u8; 3]) -> Color32 {
    Color32::from_rgb(c[0], c[1], c[2])
}

/// Farbe einer Zone (eigene Farbe oder aus der Palette nach Position).
pub fn zone_color(show: &Show, zone: Option<ZoneId>) -> Color32 {
    let Some(z) = zone else {
        return TEXT_DIM;
    };
    let idx = show.zones.iter().position(|x| x.id == z).unwrap_or(0);
    let c = show
        .zone(z)
        .and_then(|z| z.color)
        .unwrap_or(ZONE_PALETTE[idx % ZONE_PALETTE.len()]);
    rgb(c)
}
