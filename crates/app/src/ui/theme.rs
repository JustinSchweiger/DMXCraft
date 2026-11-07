//! Design-Tokens von DMXCraft (siehe docs/UI-GUIDELINES.md).
//! Farben, Abstände, Radien und Schriftgrößen nur von hier verwenden.

use dmxcraft_core::{Show, ZoneId};
use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, Visuals};

// ---------- Farben ----------
//
// Die Flächen- und Textfarben hängen vom gewählten Theme ab und werden über
// Funktionen gelesen (`theme::bg()` …). Signalfarben (Gefahr, OK, Warnung)
// sind in allen Themes gleich.

/// Ein Farbsatz für Flächen und Text.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    pub bg: Color32,
    pub surface: Color32,
    pub raised: Color32,
    pub raised_hover: Color32,
    pub border: Color32,
    pub accent: Color32,
    pub accent_hover: Color32,
    pub accent_dim: Color32,
    pub text: Color32,
    pub text_dim: Color32,
    pub text_faint: Color32,
    /// Geräte-Balken im DMX-Patch.
    pub fixture_bar: Color32,
    /// Füllung ausgeschalteter Geräte im Bühnenplan.
    pub fixture_off: Color32,
}

const fn c(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Dunkelblau mit Cyan – die Farben des Logos (Standard).
pub const NAVY: Palette = Palette {
    bg: c(0x0b, 0x0e, 0x12),
    surface: c(0x12, 0x16, 0x1c),
    raised: c(0x1a, 0x20, 0x28),
    raised_hover: c(0x23, 0x2b, 0x35),
    border: c(0x28, 0x30, 0x3a),
    accent: c(0x19, 0xe3, 0xf0),
    accent_hover: c(0x6c, 0xee, 0xf6),
    accent_dim: c(0x0c, 0x5c, 0x64),
    text: c(0xe6, 0xec, 0xf2),
    text_dim: c(0x95, 0xa2, 0xb0),
    text_faint: c(0x55, 0x60, 0x6c),
    fixture_bar: c(0x2b, 0x35, 0x42),
    fixture_off: c(0x1e, 0x25, 0x2e),
};

/// Neutrales Grau ohne Farbstich, Akzent bleibt Cyan.
pub const GRAY: Palette = Palette {
    bg: c(0x17, 0x17, 0x18),
    surface: c(0x20, 0x20, 0x21),
    raised: c(0x2b, 0x2b, 0x2c),
    raised_hover: c(0x36, 0x36, 0x38),
    border: c(0x3b, 0x3b, 0x3d),
    accent: c(0x19, 0xe3, 0xf0),
    accent_hover: c(0x6c, 0xee, 0xf6),
    accent_dim: c(0x11, 0x5a, 0x61),
    text: c(0xec, 0xec, 0xec),
    text_dim: c(0xa8, 0xa8, 0xaa),
    text_faint: c(0x6c, 0x6c, 0x70),
    fixture_bar: c(0x42, 0x42, 0x45),
    fixture_off: c(0x2e, 0x2e, 0x30),
};

/// Wählbare Themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Navy,
    Gray,
}

impl Theme {
    pub const ALL: [Theme; 2] = [Theme::Navy, Theme::Gray];

    /// Schlüssel für die Einstellungen.
    pub fn key(self) -> &'static str {
        match self {
            Theme::Navy => "navy",
            Theme::Gray => "gray",
        }
    }

    pub fn from_key(key: &str) -> Theme {
        Theme::ALL
            .into_iter()
            .find(|t| t.key() == key)
            .unwrap_or(Theme::Navy)
    }

    /// Anzeigename (deutsche Quelle, übersetzt über `i18n::t`).
    pub fn label(self) -> &'static str {
        match self {
            Theme::Navy => "Dunkelblau",
            Theme::Gray => "Grau",
        }
    }

    fn palette(self) -> &'static Palette {
        match self {
            Theme::Navy => &NAVY,
            Theme::Gray => &GRAY,
        }
    }
}

static ACTIVE: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

/// Aktives Theme setzen; danach `apply` aufrufen, damit egui es übernimmt.
pub fn set_theme(t: Theme) {
    let i = Theme::ALL.iter().position(|x| *x == t).unwrap_or(0) as u8;
    ACTIVE.store(i, std::sync::atomic::Ordering::Relaxed);
}

pub fn active() -> Theme {
    Theme::ALL[usize::from(ACTIVE.load(std::sync::atomic::Ordering::Relaxed)) % Theme::ALL.len()]
}

fn pal() -> &'static Palette {
    active().palette()
}

pub fn bg() -> Color32 {
    pal().bg
}
pub fn surface() -> Color32 {
    pal().surface
}
pub fn raised() -> Color32 {
    pal().raised
}
pub fn raised_hover() -> Color32 {
    pal().raised_hover
}
pub fn border() -> Color32 {
    pal().border
}
pub fn accent() -> Color32 {
    pal().accent
}
pub fn accent_hover() -> Color32 {
    pal().accent_hover
}
pub fn accent_dim() -> Color32 {
    pal().accent_dim
}
pub fn text() -> Color32 {
    pal().text
}
pub fn text_dim() -> Color32 {
    pal().text_dim
}
pub fn text_faint() -> Color32 {
    pal().text_faint
}
pub fn fixture_bar() -> Color32 {
    pal().fixture_bar
}
pub fn fixture_off() -> Color32 {
    pal().fixture_off
}

pub const DANGER: Color32 = Color32::from_rgb(0xef, 0x44, 0x55);
pub const OK: Color32 = Color32::from_rgb(0x34, 0xd3, 0x99);
pub const WARN: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
/// Gehäusefarbe der DIP-Schalter-Anzeige.
pub const DIP_BODY: Color32 = Color32::from_rgb(0x1c, 0x3f, 0x8a);

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

/// Schriftfamilie für Bereichsüberschriften (Noto Sans SemiBold, OFL).
pub const HEADING_FAMILY: &str = "heading";
/// Größe der Bereichsüberschriften.
pub const FS_HEADING: f32 = 12.0;

pub fn heading_font() -> FontId {
    FontId::new(FS_HEADING, egui::FontFamily::Name(HEADING_FAMILY.into()))
}

/// Text einer Bereichsüberschrift – überall gleich: Großbuchstaben, halbfett.
pub fn heading_text(title: &str) -> egui::RichText {
    egui::RichText::new(title.to_uppercase())
        .font(heading_font())
        .color(text())
}

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        HEADING_FAMILY.into(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSans-SemiBold.ttf"
        ))),
    );
    let mut family = vec![HEADING_FAMILY.to_string()];
    // Fallback für Zeichen, die die Überschriftenschrift nicht hat
    family.extend(
        fonts
            .families
            .get(&egui::FontFamily::Proportional)
            .cloned()
            .unwrap_or_default(),
    );
    fonts
        .families
        .insert(egui::FontFamily::Name(HEADING_FAMILY.into()), family);
    // Symbole (Phosphor, MIT)
    use crate::ui::widgets::icons::{ICON_FAMILY, ICON_FILL_FAMILY};
    for (name, bytes) in [
        (
            ICON_FAMILY,
            &include_bytes!("../../assets/fonts/Phosphor.ttf")[..],
        ),
        (
            ICON_FILL_FAMILY,
            &include_bytes!("../../assets/fonts/Phosphor-Fill.ttf")[..],
        ),
    ] {
        fonts.font_data.insert(
            name.into(),
            std::sync::Arc::new(egui::FontData::from_static(bytes)),
        );
        fonts
            .families
            .insert(egui::FontFamily::Name(name.into()), vec![name.to_string()]);
    }
    ctx.set_fonts(fonts);
}

pub fn apply(ctx: &egui::Context) {
    ctx.set_theme(egui::Theme::Dark);
    install_fonts(ctx);
    let mut v = Visuals::dark();
    v.panel_fill = surface();
    // Menüs, Aufklapplisten und Tooltips schweben eine Stufe heller;
    // Dialoge setzen ihren Rahmen selbst (surface()).
    v.window_fill = raised();
    v.window_stroke = Stroke::new(1.0, border());
    v.extreme_bg_color = bg();
    v.faint_bg_color = raised();
    v.override_text_color = Some(text());
    v.selection.bg_fill = accent_dim();
    v.selection.stroke = Stroke::new(1.0, accent());
    v.hyperlink_color = accent();
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, border());
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.corner_radius = CornerRadius::same(R_S);
    }
    v.widgets.inactive.weak_bg_fill = raised();
    v.widgets.inactive.bg_fill = raised();
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, border());
    v.widgets.hovered.weak_bg_fill = raised_hover();
    v.widgets.hovered.bg_fill = raised_hover();
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, accent_dim());
    v.widgets.active.weak_bg_fill = accent_dim();
    v.widgets.active.bg_fill = accent_dim();
    ctx.set_visuals(v);

    ctx.global_style_mut(|s| {
        crate::ui::widgets::menu::style(s);
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
        return text_dim();
    };
    let idx = show.zones.iter().position(|x| x.id == z).unwrap_or(0);
    let c = show
        .zone(z)
        .and_then(|z| z.color)
        .unwrap_or(ZONE_PALETTE[idx % ZONE_PALETTE.len()]);
    rgb(c)
}
