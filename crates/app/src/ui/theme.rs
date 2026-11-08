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
    /// Helles Theme (egui-Grundstil hell, farbige Schrift wird abgedunkelt).
    pub light: bool,
    /// Hintergrund der Arbeitsflächen (Bühnenansicht, Effekt-Felder).
    pub canvas: Color32,
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
    light: false,
    canvas: c(0x0b, 0x0e, 0x12),
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
    light: false,
    canvas: c(0x17, 0x17, 0x18),
};

/// Graphit mit kräftigem Blau als Akzent.
pub const GRAPHITE: Palette = Palette {
    bg: c(0x1b, 0x1d, 0x20),
    surface: c(0x23, 0x26, 0x2a),
    raised: c(0x2d, 0x31, 0x36),
    raised_hover: c(0x38, 0x3d, 0x43),
    border: c(0x40, 0x45, 0x4c),
    accent: c(0x3d, 0xa5, 0xf0),
    accent_hover: c(0x7c, 0xc2, 0xf5),
    accent_dim: c(0x15, 0x4a, 0x73),
    text: c(0xea, 0xec, 0xee),
    text_dim: c(0xa4, 0xaa, 0xb2),
    text_faint: c(0x6a, 0x71, 0x7a),
    fixture_bar: c(0x45, 0x4b, 0x53),
    light: false,
    canvas: c(0x1b, 0x1d, 0x20),
};

/// Tiefschwarz (OLED-Bildschirme, dunkle Säle).
pub const BLACK: Palette = Palette {
    bg: c(0x00, 0x00, 0x00),
    surface: c(0x08, 0x08, 0x09),
    raised: c(0x13, 0x14, 0x16),
    raised_hover: c(0x1e, 0x20, 0x23),
    border: c(0x24, 0x26, 0x2a),
    accent: c(0x19, 0xe3, 0xf0),
    accent_hover: c(0x6c, 0xee, 0xf6),
    accent_dim: c(0x0a, 0x4d, 0x53),
    text: c(0xe8, 0xea, 0xec),
    text_dim: c(0x92, 0x97, 0x9e),
    text_faint: c(0x50, 0x54, 0x5a),
    fixture_bar: c(0x22, 0x25, 0x2a),
    light: false,
    canvas: c(0x00, 0x00, 0x00),
};

/// Nachtmodus: gedämpftes Rot statt Weiß – blendet am dunklen Pult nicht und
/// schont die Nachtsicht.
pub const NIGHT: Palette = Palette {
    bg: c(0x0a, 0x05, 0x05),
    surface: c(0x12, 0x09, 0x09),
    raised: c(0x1c, 0x0f, 0x0f),
    raised_hover: c(0x28, 0x14, 0x14),
    border: c(0x33, 0x18, 0x18),
    accent: c(0xe0, 0x3a, 0x2e),
    accent_hover: c(0xf0, 0x60, 0x55),
    accent_dim: c(0x5a, 0x16, 0x12),
    text: c(0xe8, 0x9a, 0x90),
    text_dim: c(0xa8, 0x62, 0x5a),
    text_faint: c(0x66, 0x36, 0x32),
    fixture_bar: c(0x30, 0x16, 0x16),
    light: false,
    canvas: c(0x0a, 0x05, 0x05),
};

/// Violett mit Magenta-Akzent.
pub const VIOLET: Palette = Palette {
    bg: c(0x0f, 0x0b, 0x16),
    surface: c(0x16, 0x11, 0x20),
    raised: c(0x20, 0x19, 0x2d),
    raised_hover: c(0x2b, 0x22, 0x3b),
    border: c(0x33, 0x29, 0x45),
    accent: c(0xd0, 0x6b, 0xff),
    accent_hover: c(0xe0, 0x9c, 0xff),
    accent_dim: c(0x52, 0x26, 0x6e),
    text: c(0xec, 0xe6, 0xf4),
    text_dim: c(0xa8, 0x9c, 0xbb),
    text_faint: c(0x65, 0x5a, 0x78),
    fixture_bar: c(0x31, 0x27, 0x44),
    light: false,
    canvas: c(0x0f, 0x0b, 0x16),
};

/// Hoher Kontrast: Schwarz, Weiß und Gelb, kräftige Ränder.
pub const CONTRAST: Palette = Palette {
    bg: c(0x00, 0x00, 0x00),
    surface: c(0x00, 0x00, 0x00),
    raised: c(0x14, 0x14, 0x14),
    raised_hover: c(0x26, 0x26, 0x26),
    border: c(0x9a, 0x9a, 0x9a),
    accent: c(0xff, 0xd8, 0x00),
    accent_hover: c(0xff, 0xe8, 0x66),
    accent_dim: c(0x5c, 0x4e, 0x00),
    text: c(0xff, 0xff, 0xff),
    text_dim: c(0xdc, 0xdc, 0xdc),
    text_faint: c(0xa0, 0xa0, 0xa0),
    fixture_bar: c(0x30, 0x30, 0x30),
    light: false,
    canvas: c(0x00, 0x00, 0x00),
};

/// Hell (neutral), Akzent Petrol (Cyan wäre auf Weiß zu blass).
pub const LIGHT: Palette = Palette {
    bg: c(0xe9, 0xec, 0xef),
    surface: c(0xf4, 0xf6, 0xf8),
    raised: c(0xff, 0xff, 0xff),
    raised_hover: c(0xe6, 0xf2, 0xf4),
    border: c(0xcf, 0xd5, 0xdb),
    accent: c(0x0a, 0x93, 0xa3),
    accent_hover: c(0x06, 0x7a, 0x88),
    accent_dim: c(0xc6, 0xeb, 0xef),
    text: c(0x1c, 0x22, 0x28),
    text_dim: c(0x55, 0x5f, 0x69),
    text_faint: c(0x93, 0x9b, 0xa3),
    fixture_bar: c(0xd6, 0xdc, 0xe2),
    light: true,
    canvas: c(0xdf, 0xe3, 0xe7),
};

/// Daslight: Farben der Daslight-Oberfläche (aus dem Profile Builder
/// gemessen): Hellgrau, weiße Karten, Blau als Akzent.
pub const DASLIGHT: Palette = Palette {
    bg: c(0xf6, 0xf6, 0xf6),
    surface: c(0xf6, 0xf6, 0xf6),
    raised: c(0xff, 0xff, 0xff),
    raised_hover: c(0xea, 0xf4, 0xfc),
    border: c(0xd0, 0xd0, 0xd0),
    accent: c(0x15, 0x92, 0xe6),
    accent_hover: c(0x0f, 0x7c, 0xc6),
    accent_dim: c(0xc9, 0xe5, 0xf9),
    text: c(0x22, 0x22, 0x22),
    text_dim: c(0x55, 0x55, 0x55),
    text_faint: c(0x96, 0x96, 0x96),
    fixture_bar: c(0xde, 0xde, 0xde),
    light: true,
    canvas: c(0xe7, 0xe7, 0xe7),
};

/// Wählbare Themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Navy,
    Gray,
    Graphite,
    Black,
    Violet,
    Night,
    Contrast,
    Light,
    Daslight,
}

impl Theme {
    pub const ALL: [Theme; 9] = [
        Theme::Navy,
        Theme::Gray,
        Theme::Graphite,
        Theme::Black,
        Theme::Violet,
        Theme::Night,
        Theme::Contrast,
        Theme::Light,
        Theme::Daslight,
    ];

    /// Schlüssel für die Einstellungen.
    pub fn key(self) -> &'static str {
        match self {
            Theme::Navy => "navy",
            Theme::Gray => "gray",
            Theme::Graphite => "graphite",
            Theme::Black => "black",
            Theme::Violet => "violet",
            Theme::Night => "night",
            Theme::Contrast => "contrast",
            Theme::Light => "light",
            Theme::Daslight => "daslight",
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
            Theme::Graphite => "Graphit",
            Theme::Black => "Schwarz (OLED)",
            Theme::Violet => "Violett",
            Theme::Night => "Nachtmodus (rot)",
            Theme::Contrast => "Hoher Kontrast",
            Theme::Light => "Hell",
            Theme::Daslight => "Daslight (hell)",
        }
    }

    fn palette(self) -> &'static Palette {
        match self {
            Theme::Navy => &NAVY,
            Theme::Gray => &GRAY,
            Theme::Graphite => &GRAPHITE,
            Theme::Black => &BLACK,
            Theme::Violet => &VIOLET,
            Theme::Night => &NIGHT,
            Theme::Contrast => &CONTRAST,
            Theme::Light => &LIGHT,
            Theme::Daslight => &DASLIGHT,
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
/// Ist das aktive Theme hell?
pub fn is_light() -> bool {
    pal().light
}
/// Hintergrund der Arbeitsflächen (Bühnenansicht, Effekt-Felder).
pub fn canvas() -> Color32 {
    pal().canvas
}
/// Rasterlinien auf der Arbeitsfläche.
pub fn canvas_line() -> Color32 {
    if is_light() {
        c(0xb4, 0xb9, 0xbf)
    } else {
        border()
    }
}
/// Linien/Bahnen auf der Arbeitsfläche.
pub fn canvas_dim() -> Color32 {
    if is_light() {
        c(0x5d, 0x63, 0x6a)
    } else {
        text_dim()
    }
}
/// Hilfslinien auf der Arbeitsfläche.
pub fn canvas_faint() -> Color32 {
    if is_light() {
        c(0x95, 0x9b, 0xa2)
    } else {
        text_faint()
    }
}
/// Griffe und Markierungen auf der Arbeitsfläche (hell auf dunkel, dunkel
/// auf hell).
pub fn handle() -> Color32 {
    if is_light() {
        c(0x2a, 0x2e, 0x33)
    } else {
        Color32::WHITE
    }
}
/// Farbe einer Lampe auf der Arbeitsfläche. `col` ist die Lichtfarbe schon
/// mit der Helligkeit verrechnet (aus = schwarz). Auf dunklem Grund passt
/// das so; auf hellem Grund wäre „aus“ ein schwarzer Fleck – dort geht es
/// von einem hellen Grau (aus) zur vollen Farbe (an).
pub fn lamp(col: Color32) -> Color32 {
    if !is_light() {
        return col;
    }
    let (r, g, b) = (f32::from(col.r()), f32::from(col.g()), f32::from(col.b()));
    let l = r.max(g).max(b) / 255.0;
    let full = |x: f32| if l > 0.0 { x / l } else { 0.0 };
    let off = [0xc9 as f32, 0xcd as f32, 0xd2 as f32];
    let mix = |o: f32, x: f32| (o * (1.0 - l) + full(x) * l).clamp(0.0, 255.0) as u8;
    Color32::from_rgb(mix(off[0], r), mix(off[1], g), mix(off[2], b))
}
/// Farbige Schrift (Zonen-/Szenenfarbe) lesbar machen: auf hellen Themes
/// abgedunkelt, sonst unverändert.
pub fn ink(c: Color32) -> Color32 {
    if !is_light() {
        return c;
    }
    // so weit abdunkeln, dass die Helligkeit für dunkle Schrift reicht
    let lum = 0.2126 * f32::from(c.r()) + 0.7152 * f32::from(c.g()) + 0.0722 * f32::from(c.b());
    let target = 105.0;
    let k = if lum > target { target / lum } else { 1.0 };
    Color32::from_rgb(
        (f32::from(c.r()) * k) as u8,
        (f32::from(c.g()) * k) as u8,
        (f32::from(c.b()) * k) as u8,
    )
}

pub const DANGER: Color32 = Color32::from_rgb(0xef, 0x44, 0x55);
pub const OK: Color32 = Color32::from_rgb(0x34, 0xd3, 0x99);
pub const WARN: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
/// Frequenzbänder der Audio-Wellenform (Low, Mid, High).
pub const BAND_LOW: Color32 = Color32::from_rgb(0x2f, 0x80, 0xff);
pub const BAND_MID: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const BAND_HIGH: Color32 = Color32::from_rgb(0xee, 0xf2, 0xff);
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
    let light = is_light();
    ctx.set_theme(if light {
        egui::Theme::Light
    } else {
        egui::Theme::Dark
    });
    install_fonts(ctx);
    let mut v = if light {
        Visuals::light()
    } else {
        Visuals::dark()
    };
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
    for w in [
        &mut v.widgets.noninteractive,
        &mut v.widgets.inactive,
        &mut v.widgets.hovered,
        &mut v.widgets.active,
        &mut v.widgets.open,
    ] {
        w.fg_stroke.color = text();
    }
    v.widgets.noninteractive.fg_stroke.color = text_dim();
    if light {
        // weichere Schatten auf hellem Grund
        v.window_shadow.color = Color32::from_black_alpha(40);
        v.popup_shadow.color = Color32::from_black_alpha(40);
    }
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
