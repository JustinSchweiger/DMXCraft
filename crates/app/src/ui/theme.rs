//! Design tokens of DMXCraft (see docs/UI-GUIDELINES.md).
//! Use colors, spacing, radii and font sizes only from here.

use dmxcraft_core::{Show, ZoneId};
use eframe::egui::{self, Color32, CornerRadius, FontId, Stroke, Visuals};

// ---------- Colors ----------
//
// Surface and text colors depend on the chosen theme and are read through
// functions (`theme::bg()` …). Signal colors (danger, OK, warning) are the
// same in all themes.

/// A color set for surfaces and text.
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
    /// Fixture bars in the DMX patch.
    pub fixture_bar: Color32,
    /// Light theme (egui base style light, colored text is darkened).
    pub light: bool,
    /// Background of the work areas (stage view, effect fields).
    pub canvas: Color32,
}

const fn c(r: u8, g: u8, b: u8) -> Color32 {
    Color32::from_rgb(r, g, b)
}

/// Midnight: deep navy with cyan – the colors of the logo (default).
pub const MIDNIGHT: Palette = Palette {
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

/// Studio: neutral charcoal without a color cast, warm amber accent like
/// the indicator lamps of a mixing desk.
pub const STUDIO: Palette = Palette {
    bg: c(0x16, 0x16, 0x17),
    surface: c(0x1f, 0x1f, 0x20),
    raised: c(0x2a, 0x2a, 0x2b),
    raised_hover: c(0x35, 0x35, 0x37),
    border: c(0x3a, 0x3a, 0x3c),
    accent: c(0xf5, 0xa5, 0x24),
    accent_hover: c(0xf8, 0xc0, 0x66),
    accent_dim: c(0x5e, 0x41, 0x12),
    text: c(0xec, 0xec, 0xec),
    text_dim: c(0xa8, 0xa8, 0xaa),
    text_faint: c(0x6c, 0x6c, 0x70),
    fixture_bar: c(0x42, 0x42, 0x45),
    light: false,
    canvas: c(0x16, 0x16, 0x17),
};

/// Slate: cool blue-gray with a royal blue accent, calm for long sessions.
pub const SLATE: Palette = Palette {
    bg: c(0x1a, 0x1e, 0x25),
    surface: c(0x22, 0x27, 0x30),
    raised: c(0x2c, 0x32, 0x3d),
    raised_hover: c(0x37, 0x3e, 0x4b),
    border: c(0x3f, 0x47, 0x55),
    accent: c(0x5b, 0x8d, 0xff),
    accent_hover: c(0x8d, 0xb0, 0xff),
    accent_dim: c(0x22, 0x3a, 0x70),
    text: c(0xe8, 0xec, 0xf3),
    text_dim: c(0xa0, 0xa9, 0xb8),
    text_faint: c(0x66, 0x6f, 0x7e),
    fixture_bar: c(0x45, 0x4e, 0x5d),
    light: false,
    canvas: c(0x16, 0x1a, 0x20),
};

/// Pitch black: pure black for OLED screens and dark venues, lime accent that
/// stays visible at low screen brightness.
pub const PITCH_BLACK: Palette = Palette {
    bg: c(0x00, 0x00, 0x00),
    surface: c(0x08, 0x08, 0x09),
    raised: c(0x13, 0x14, 0x16),
    raised_hover: c(0x1e, 0x20, 0x23),
    border: c(0x24, 0x26, 0x2a),
    accent: c(0xb6, 0xf0, 0x3c),
    accent_hover: c(0xcd, 0xf6, 0x7c),
    accent_dim: c(0x3a, 0x4f, 0x10),
    text: c(0xe8, 0xea, 0xec),
    text_dim: c(0x92, 0x97, 0x9e),
    text_faint: c(0x50, 0x54, 0x5a),
    fixture_bar: c(0x22, 0x25, 0x2a),
    light: false,
    canvas: c(0x00, 0x00, 0x00),
};

/// Ultraviolet: dark violet with a magenta accent – club and party look.
pub const ULTRAVIOLET: Palette = Palette {
    bg: c(0x0f, 0x0b, 0x16),
    surface: c(0x16, 0x11, 0x20),
    raised: c(0x20, 0x19, 0x2d),
    raised_hover: c(0x2b, 0x22, 0x3b),
    border: c(0x33, 0x29, 0x45),
    accent: c(0xe0, 0x5c, 0xf5),
    accent_hover: c(0xec, 0x93, 0xf9),
    accent_dim: c(0x56, 0x22, 0x66),
    text: c(0xec, 0xe6, 0xf4),
    text_dim: c(0xa8, 0x9c, 0xbb),
    text_faint: c(0x65, 0x5a, 0x78),
    fixture_bar: c(0x31, 0x27, 0x44),
    light: false,
    canvas: c(0x0f, 0x0b, 0x16),
};

/// Ember: warm dark brown with an orange accent, like tungsten light.
pub const EMBER: Palette = Palette {
    bg: c(0x14, 0x10, 0x0d),
    surface: c(0x1c, 0x17, 0x13),
    raised: c(0x27, 0x20, 0x1a),
    raised_hover: c(0x33, 0x2a, 0x22),
    border: c(0x3d, 0x32, 0x28),
    accent: c(0xff, 0x7a, 0x33),
    accent_hover: c(0xff, 0xa2, 0x70),
    accent_dim: c(0x62, 0x2e, 0x12),
    text: c(0xf2, 0xea, 0xe2),
    text_dim: c(0xb3, 0xa5, 0x97),
    text_faint: c(0x73, 0x66, 0x5a),
    fixture_bar: c(0x3e, 0x33, 0x29),
    light: false,
    canvas: c(0x12, 0x0e, 0x0b),
};

/// Night mode: muted red instead of white – does not dazzle at a dark desk
/// and preserves night vision.
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

/// High contrast: black, white and yellow, strong borders.
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

/// Daylight: light and neutral for bright rooms, petrol accent (cyan would
/// be too pale on white).
pub const DAYLIGHT: Palette = Palette {
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

/// Paper: warm off-white with an ink blue accent, easy to read outdoors.
pub const PAPER: Palette = Palette {
    bg: c(0xf3, 0xf0, 0xea),
    surface: c(0xf8, 0xf6, 0xf1),
    raised: c(0xff, 0xff, 0xfc),
    raised_hover: c(0xec, 0xf0, 0xf8),
    border: c(0xd8, 0xd2, 0xc6),
    accent: c(0x2f, 0x5b, 0xd8),
    accent_hover: c(0x24, 0x48, 0xb0),
    accent_dim: c(0xd3, 0xdd, 0xf7),
    text: c(0x26, 0x23, 0x1e),
    text_dim: c(0x5e, 0x58, 0x4f),
    text_faint: c(0x9d, 0x96, 0x8b),
    fixture_bar: c(0xe2, 0xdc, 0xd0),
    light: true,
    canvas: c(0xe9, 0xe5, 0xdc),
};

/// Selectable themes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Theme {
    Midnight,
    Studio,
    Slate,
    PitchBlack,
    Ultraviolet,
    Ember,
    Night,
    Contrast,
    Daylight,
    Paper,
}

impl Theme {
    pub const ALL: [Theme; 10] = [
        Theme::Midnight,
        Theme::Studio,
        Theme::Slate,
        Theme::PitchBlack,
        Theme::Ultraviolet,
        Theme::Ember,
        Theme::Night,
        Theme::Contrast,
        Theme::Daylight,
        Theme::Paper,
    ];

    /// Key for the settings (the old keys stay valid for existing settings).
    pub fn key(self) -> &'static str {
        match self {
            Theme::Midnight => "navy",
            Theme::Studio => "gray",
            Theme::Slate => "graphite",
            Theme::PitchBlack => "black",
            Theme::Ultraviolet => "violet",
            Theme::Ember => "ember",
            Theme::Night => "night",
            Theme::Contrast => "contrast",
            Theme::Daylight => "light",
            Theme::Paper => "paper",
        }
    }

    pub fn from_key(key: &str) -> Theme {
        if key == "daslight" {
            return Theme::Paper;
        }
        Theme::ALL
            .into_iter()
            .find(|t| t.key() == key)
            .unwrap_or(Theme::Midnight)
    }

    /// Display name (English source, translated via `i18n::t`).
    pub fn label(self) -> &'static str {
        match self {
            Theme::Midnight => "Midnight",
            Theme::Studio => "Studio",
            Theme::Slate => "Slate",
            Theme::PitchBlack => "Pitch black",
            Theme::Ultraviolet => "Ultraviolet",
            Theme::Ember => "Ember",
            Theme::Night => "Night (red)",
            Theme::Contrast => "High contrast",
            Theme::Daylight => "Daylight",
            Theme::Paper => "Paper",
        }
    }

    /// What the theme is for (English source, translated via `i18n::t`).
    pub fn description(self) -> &'static str {
        match self {
            Theme::Midnight => "Deep navy with cyan – the DMXCraft colors",
            Theme::Studio => "Neutral charcoal with an amber accent",
            Theme::Slate => "Cool blue-gray, calm for long sessions",
            Theme::PitchBlack => "Pure black for OLED screens and dark venues",
            Theme::Ultraviolet => "Dark violet with magenta – club look",
            Theme::Ember => "Warm brown with orange, like tungsten light",
            Theme::Night => "Red only – keeps your night vision at a dark desk",
            Theme::Contrast => "Black, white and yellow for best readability",
            Theme::Daylight => "Light and neutral for bright rooms",
            Theme::Paper => "Warm off-white with ink blue, readable outdoors",
        }
    }

    pub fn palette(self) -> &'static Palette {
        match self {
            Theme::Midnight => &MIDNIGHT,
            Theme::Studio => &STUDIO,
            Theme::Slate => &SLATE,
            Theme::PitchBlack => &PITCH_BLACK,
            Theme::Ultraviolet => &ULTRAVIOLET,
            Theme::Ember => &EMBER,
            Theme::Night => &NIGHT,
            Theme::Contrast => &CONTRAST,
            Theme::Daylight => &DAYLIGHT,
            Theme::Paper => &PAPER,
        }
    }
}

/// Own colors on top of the theme (from the settings).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Overrides {
    /// Accent color (selection, active elements).
    pub accent: Option<[u8; 3]>,
    /// Background of the work areas (stage view, effect fields, timeline).
    pub canvas: Option<[u8; 3]>,
}

struct Active {
    theme: Theme,
    overrides: Overrides,
    palette: Palette,
}

static ACTIVE: std::sync::RwLock<Active> = std::sync::RwLock::new(Active {
    theme: Theme::Midnight,
    overrides: Overrides {
        accent: None,
        canvas: None,
    },
    palette: MIDNIGHT,
});

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let m = |x: u8, y: u8| (f32::from(x) * (1.0 - t) + f32::from(y) * t).round() as u8;
    Color32::from_rgb(m(a.r(), b.r()), m(a.g(), b.g()), m(a.b(), b.b()))
}

/// Theme palette with the own colors applied; hover and dim variants of the
/// accent are derived from it.
pub fn resolve(t: Theme, o: Overrides) -> Palette {
    let mut p = *t.palette();
    if let Some(a) = o.accent {
        let a = rgb(a);
        p.accent = a;
        if p.light {
            p.accent_hover = mix(a, Color32::BLACK, 0.18);
            p.accent_dim = mix(p.raised, a, 0.22);
        } else {
            p.accent_hover = mix(a, Color32::WHITE, 0.35);
            p.accent_dim = mix(p.bg, a, 0.38);
        }
    }
    if let Some(cv) = o.canvas {
        p.canvas = rgb(cv);
    }
    p
}

/// Set the active theme; call `apply` afterwards so egui takes it over.
pub fn set_theme(t: Theme) {
    let mut a = ACTIVE.write().unwrap_or_else(|e| e.into_inner());
    a.theme = t;
    a.palette = resolve(t, a.overrides);
}

/// Set the own colors on top of the theme; call `apply` afterwards.
pub fn set_overrides(o: Overrides) {
    let mut a = ACTIVE.write().unwrap_or_else(|e| e.into_inner());
    a.overrides = o;
    a.palette = resolve(a.theme, o);
}

fn pal() -> Palette {
    ACTIVE.read().unwrap_or_else(|e| e.into_inner()).palette
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
/// Is the active theme light?
pub fn is_light() -> bool {
    pal().light
}
/// Background of the work areas (stage view, effect fields).
pub fn canvas() -> Color32 {
    pal().canvas
}
/// Is the work-area background light? Usually follows the theme, but an own
/// canvas color can differ from it.
pub fn canvas_is_light() -> bool {
    let c = canvas();
    0.2126 * f32::from(c.r()) + 0.7152 * f32::from(c.g()) + 0.0722 * f32::from(c.b()) > 140.0
}
/// Work-area color derived from the canvas when it does not match the theme.
fn canvas_tone(t: f32) -> Option<Color32> {
    (canvas_is_light() != is_light()).then(|| {
        let to = if canvas_is_light() {
            Color32::BLACK
        } else {
            Color32::WHITE
        };
        mix(canvas(), to, t)
    })
}
/// Grid lines on the work area.
pub fn canvas_line() -> Color32 {
    if let Some(c) = canvas_tone(0.18) {
        c
    } else if is_light() {
        c(0xb4, 0xb9, 0xbf)
    } else {
        border()
    }
}
/// Lines/tracks on the work area.
pub fn canvas_dim() -> Color32 {
    if let Some(c) = canvas_tone(0.6) {
        c
    } else if is_light() {
        c(0x5d, 0x63, 0x6a)
    } else {
        text_dim()
    }
}
/// Guide lines on the work area.
pub fn canvas_faint() -> Color32 {
    if let Some(c) = canvas_tone(0.35) {
        c
    } else if is_light() {
        c(0x95, 0x9b, 0xa2)
    } else {
        text_faint()
    }
}
/// Handles and markers on the work area (light on dark, dark on light).
pub fn handle() -> Color32 {
    if canvas_is_light() {
        c(0x2a, 0x2e, 0x33)
    } else {
        Color32::WHITE
    }
}
/// Color of a lamp on the work area. `col` is the light color already
/// multiplied by the brightness (off = black). On a dark background that
/// works as is; on a light background “off” would be a black blot – there
/// it goes from a light gray (off) to the full color (on).
pub fn lamp(col: Color32) -> Color32 {
    if !canvas_is_light() {
        return col;
    }
    let (r, g, b) = (f32::from(col.r()), f32::from(col.g()), f32::from(col.b()));
    let l = r.max(g).max(b) / 255.0;
    let full = |x: f32| if l > 0.0 { x / l } else { 0.0 };
    let off = [0xc9 as f32, 0xcd as f32, 0xd2 as f32];
    let mix = |o: f32, x: f32| (o * (1.0 - l) + full(x) * l).clamp(0.0, 255.0) as u8;
    Color32::from_rgb(mix(off[0], r), mix(off[1], g), mix(off[2], b))
}
/// Make colored text (zone/scene color) readable: darkened on light themes,
/// otherwise unchanged.
pub fn ink(c: Color32) -> Color32 {
    if !is_light() {
        return c;
    }
    // darken until the brightness is enough for dark text
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
/// Frequency bands of the audio waveform (low, mid, high).
pub const BAND_LOW: Color32 = Color32::from_rgb(0x2f, 0x80, 0xff);
pub const BAND_MID: Color32 = Color32::from_rgb(0xf5, 0xa5, 0x24);
pub const BAND_HIGH: Color32 = Color32::from_rgb(0xee, 0xf2, 0xff);
/// Housing color of the DIP switch display.
pub const DIP_BODY: Color32 = Color32::from_rgb(0x1c, 0x3f, 0x8a);

/// Default colors for zones without their own color.
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

// ---------- Spacing, radii, fonts ----------
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

/// Font family for section headings (Noto Sans SemiBold, OFL).
pub const HEADING_FAMILY: &str = "heading";
/// Size of the section headings.
pub const FS_HEADING: f32 = 12.0;

pub fn heading_font() -> FontId {
    FontId::new(FS_HEADING, egui::FontFamily::Name(HEADING_FAMILY.into()))
}

/// Text of a section heading – the same everywhere: upper case, semibold.
pub fn heading_text(title: &str) -> egui::RichText {
    egui::RichText::new(title.to_uppercase())
        .font(heading_font())
        .color(text())
}

/// Fallback font for symbols (subset of DejaVu Sans).
const SYMBOLS_FONT: &str = "dejavu-symbols";

fn install_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        HEADING_FAMILY.into(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/NotoSans-SemiBold.ttf"
        ))),
    );
    // Symbols the default fonts lack (arrows, check marks, stars, math …):
    // the last fallback, so emoji keep their emoji glyphs
    fonts.font_data.insert(
        SYMBOLS_FONT.into(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/DejaVuSans-Symbols.ttf"
        ))),
    );
    for fam in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
        fonts
            .families
            .entry(fam)
            .or_default()
            .push(SYMBOLS_FONT.to_string());
    }
    let mut family = vec![HEADING_FAMILY.to_string()];
    // Fallback for characters the heading font does not have
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
    // Icons (Phosphor, MIT)
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
    // Menus, dropdowns and tooltips float one step lighter; dialogs set their
    // own frame (surface()).
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
        // softer shadows on a light background
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

/// Color of a zone (own color or from the palette by position).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn themes_are_distinct() {
        for (i, a) in Theme::ALL.iter().enumerate() {
            assert_eq!(Theme::from_key(a.key()), *a);
            for b in &Theme::ALL[i + 1..] {
                assert_ne!(a.key(), b.key());
                assert_ne!(a.palette().accent, b.palette().accent, "{a:?} / {b:?}");
            }
        }
        // old settings keep working
        assert_eq!(Theme::from_key("daslight"), Theme::Paper);
        assert_eq!(Theme::from_key("unknown"), Theme::Midnight);
    }

    #[test]
    fn overrides_apply() {
        let o = Overrides {
            accent: Some([0xff, 0x00, 0x80]),
            canvas: Some([0x10, 0x20, 0x30]),
        };
        let p = resolve(Theme::Paper, o);
        assert_eq!(p.accent, c(0xff, 0x00, 0x80));
        assert_eq!(p.canvas, c(0x10, 0x20, 0x30));
        assert_ne!(p.accent_dim, PAPER.accent_dim);
        assert_eq!(p.bg, PAPER.bg);
        let d = resolve(Theme::Studio, Overrides::default());
        assert_eq!(d.accent, STUDIO.accent);
    }
}
