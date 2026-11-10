//! Interface icons from the Phosphor icon font (MIT, see
//! assets/fonts/Phosphor-LICENSE.txt) – a uniform style for all buttons.
//! Only “FX” (text) and “play backwards” are drawn by hand.

use eframe::egui::{self, pos2, Color32, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, Widget};

use crate::ui::theme;

/// Font families of the icons (registered in `theme::install_fonts`).
pub const ICON_FAMILY: &str = "phosphor";
pub const ICON_FILL_FAMILY: &str = "phosphor-fill";

/// Not every icon is in use yet – the list is the stock.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Play,
    Pause,
    Prev,
    Next,
    Plus,
    LightOff,
    LightOn,
    Center,
    Locked,
    Unlocked,
    ArrangeRow,
    ArrangeGrid,
    AlignH,
    AlignV,
    Reset,
    Menu,
    Minimize,
    Maximize,
    Restore,
    Close,
    FileNew,
    FileOpen,
    Recent,
    Save,
    SaveAs,
    Import,
    Export,
    Undo,
    Redo,
    Settings,
    Fullscreen,
    ZoomIn,
    ZoomOut,
    ZoomReset,
    /// Magnifier without a sign (mouse pointer in the time ruler).
    Magnifier,
    /// AI assistant (MCP).
    Robot,
    Cube,
    Levels,
    Builder,
    Keyboard,
    Midi,
    Touch,
    Update,
    Info,
    Edit,
    Trash,
    Group,
    Copy,
    Paste,
    ZoomFit,
    Music,
    ChevronLeft,
    ChevronRight,
    ChevronDown,
    ChevronUp,
    Remote,
    SpeakerHigh,
    Camera,
    SpeakerLow,
    SpeakerNone,
    SpeakerMute,
    EyeOff,
    Repeat,
    RepeatOnce,
    Sliders,
    Lightning,
    Gear,
    ListChecks,
    Magnet,
    GridFour,
    TextAa,
    Cursor,
    ArrowsH,
    Stack,
    Eye,
    FollowRight,
    Scissors,
    Waveform,
    Clock,
    MusicNote,
    /// Play backwards (triangle pointing left).
    PlayBack,
    /// Stop (filled square, drawn by hand).
    Stop,
    /// Lettering “FX” (effects).
    Fx,
}

impl Icon {
    /// Glyph and whether the filled variant is used.
    fn glyph(self) -> Option<(&'static str, bool)> {
        match self {
            Icon::Play => Some(("\u{E3D0}", true)),
            Icon::Pause => Some(("\u{E39E}", true)),
            Icon::Prev => Some(("\u{E5A4}", true)),
            Icon::Next => Some(("\u{E5A6}", true)),
            Icon::Plus => Some(("\u{E3D4}", false)),
            Icon::LightOff => Some(("\u{E2DC}", false)),
            Icon::LightOn => Some(("\u{E472}", false)),
            Icon::Center => Some(("\u{E1D6}", false)),
            Icon::Locked => Some(("\u{E2FA}", false)),
            Icon::Unlocked => Some(("\u{E306}", false)),
            Icon::ArrangeRow => Some(("\u{E5A2}", false)),
            Icon::ArrangeGrid => Some(("\u{E464}", false)),
            Icon::AlignH => Some(("\u{E50A}", false)),
            Icon::AlignV => Some(("\u{E50C}", false)),
            Icon::Reset => Some(("\u{E038}", false)),
            Icon::Menu => Some(("\u{E2F0}", false)),
            Icon::Minimize => Some(("\u{E32A}", false)),
            Icon::Maximize => Some(("\u{E45E}", false)),
            Icon::Restore => Some(("\u{E1CC}", false)),
            Icon::Close => Some(("\u{E4F6}", false)),
            Icon::FileNew => Some(("\u{E236}", false)),
            Icon::FileOpen => Some(("\u{E256}", false)),
            Icon::Recent => Some(("\u{E1A0}", false)),
            Icon::Save => Some(("\u{E248}", false)),
            Icon::SaveAs => Some(("\u{EAF4}", false)),
            Icon::Import => Some(("\u{E20C}", false)),
            Icon::Export => Some(("\u{E4C0}", false)),
            Icon::Undo => Some(("\u{E08A}", false)),
            Icon::Redo => Some(("\u{E08C}", false)),
            Icon::Settings => Some(("\u{E272}", false)),
            Icon::Fullscreen => Some(("\u{E1D0}", false)),
            Icon::ZoomIn => Some(("\u{E310}", false)),
            Icon::ZoomOut => Some(("\u{E30E}", false)),
            Icon::ZoomReset | Icon::Magnifier => Some(("\u{E30C}", false)),
            Icon::Robot => Some(("\u{E762}", false)),
            Icon::Cube => Some(("\u{E1DA}", false)),
            Icon::Levels => Some(("\u{E150}", false)),
            Icon::Builder => Some(("\u{E5D4}", false)),
            Icon::Keyboard => Some(("\u{E2D8}", false)),
            Icon::Midi => Some(("\u{E9C8}", false)),
            Icon::Touch => Some(("\u{EC90}", false)),
            Icon::Update => Some(("\u{E030}", false)),
            Icon::Info => Some(("\u{E2CE}", false)),
            Icon::Edit => Some(("\u{E3B4}", false)),
            Icon::Trash => Some(("\u{E4A6}", false)),
            Icon::Group => Some(("\u{E68E}", false)),
            Icon::Copy => Some(("\u{E1CA}", false)),
            Icon::Paste => Some(("\u{E196}", false)),
            Icon::ZoomFit => Some(("\u{E626}", false)),
            Icon::Music => Some(("\u{E340}", false)),
            Icon::ChevronLeft => Some(("\u{E138}", false)),
            Icon::ChevronRight => Some(("\u{E13A}", false)),
            Icon::ChevronDown => Some(("\u{E136}", false)),
            Icon::ChevronUp => Some(("\u{E13C}", false)),
            Icon::Remote => Some(("\u{E1E0}", false)),
            Icon::SpeakerHigh => Some(("\u{E44A}", false)),
            Icon::Camera => Some(("\u{E10E}", false)),
            Icon::SpeakerLow => Some(("\u{E44C}", false)),
            Icon::SpeakerNone => Some(("\u{E44E}", false)),
            Icon::SpeakerMute => Some(("\u{E45C}", false)),
            Icon::EyeOff => Some(("\u{E224}", false)),
            Icon::Repeat => Some(("\u{E3F6}", false)),
            Icon::RepeatOnce => Some(("\u{E3F8}", false)),
            Icon::Sliders => Some(("\u{E434}", false)),
            Icon::Lightning => Some(("\u{E2DE}", true)),
            Icon::Gear => Some(("\u{E87C}", false)),
            Icon::ListChecks => Some(("\u{EADC}", false)),
            Icon::Magnet => Some(("\u{E680}", false)),
            Icon::GridFour => Some(("\u{E296}", false)),
            Icon::TextAa => Some(("\u{E6EE}", false)),
            Icon::Cursor => Some(("\u{E1DC}", false)),
            Icon::ArrowsH => Some(("\u{EB06}", false)),
            Icon::Stack => Some(("\u{E466}", false)),
            Icon::Eye => Some(("\u{E220}", false)),
            Icon::FollowRight => Some(("\u{E064}", false)),
            Icon::Scissors => Some(("\u{EAE0}", false)),
            Icon::Waveform => Some(("\u{E802}", false)),
            Icon::Clock => Some(("\u{E19A}", false)),
            Icon::MusicNote => Some(("\u{E33C}", false)),
            Icon::PlayBack | Icon::Stop | Icon::Fx => None,
        }
    }

    pub fn paint(self, painter: &egui::Painter, rect: Rect, color: Color32) {
        let size = rect.width().min(rect.height());
        let c = rect.center();
        if let Some((g, fill)) = self.glyph() {
            let family = if fill { ICON_FILL_FAMILY } else { ICON_FAMILY };
            let font = egui::FontId::new(size * 1.05, egui::FontFamily::Name(family.into()));
            painter.text(c, egui::Align2::CENTER_CENTER, g, font, color);
            return;
        }
        match self {
            Icon::PlayBack => {
                let s = size * 0.5;
                let pts = vec![
                    pos2(c.x + s * 0.55, c.y - s * 0.7),
                    pos2(c.x + s * 0.55, c.y + s * 0.7),
                    pos2(c.x - s * 0.65, c.y),
                ];
                painter.add(Shape::convex_polygon(pts, color, Stroke::new(1.5, color)));
            }
            Icon::Stop => {
                let s = size * 0.3;
                painter.rect_filled(Rect::from_center_size(c, Vec2::splat(s * 2.0)), 2.0, color);
            }
            _ => {
                painter.text(
                    c,
                    egui::Align2::CENTER_CENTER,
                    "FX",
                    egui::FontId::new(
                        size * 0.62,
                        egui::FontFamily::Name(theme::HEADING_FAMILY.into()),
                    ),
                    color,
                );
            }
        }
    }
}

/// Small square button with an icon.
pub struct IconButton {
    icon: Icon,
    size: f32,
    enabled: bool,
    active: bool,
    flat: bool,
    danger: bool,
    /// Small label below the icon (e.g. “ALL”).
    caption: Option<String>,
    /// Color for the active state instead of the accent color.
    tint: Option<egui::Color32>,
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
            caption: None,
            tint: None,
        }
    }
    /// Color of the active state (e.g. scene color).
    pub fn tint(mut self, c: egui::Color32) -> Self {
        self.tint = Some(c);
        self
    }
    /// Small label below the icon.
    pub fn caption(mut self, text: impl Into<String>) -> Self {
        self.caption = Some(text.into());
        self
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
    /// Without background (highlighted only on hover).
    pub fn flat(mut self) -> Self {
        self.flat = true;
        self
    }
    /// Red on hover (close button).
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
            self.tint
                .map_or_else(theme::accent_dim, |c| c.gamma_multiply(0.3))
        } else if hovered && self.danger {
            theme::DANGER
        } else if hovered {
            theme::raised_hover()
        } else if self.flat {
            egui::Color32::TRANSPARENT
        } else {
            theme::raised()
        };
        let p = ui.painter();
        p.rect_filled(rect, theme::R_S as f32, bg);
        let color = if !self.enabled {
            theme::text_faint()
        } else if self.active {
            self.tint.unwrap_or_else(theme::accent)
        } else {
            theme::text()
        };
        match &self.caption {
            Some(cap) => {
                let icon = Rect::from_min_max(
                    rect.min + Vec2::new(rect.width() * 0.22, rect.height() * 0.1),
                    rect.max - Vec2::new(rect.width() * 0.22, rect.height() * 0.4),
                );
                self.icon.paint(p, icon, color);
                p.text(
                    pos2(rect.center().x, rect.bottom() - rect.height() * 0.08),
                    egui::Align2::CENTER_BOTTOM,
                    cap,
                    egui::FontId::proportional((rect.height() * 0.27).max(8.0)),
                    color,
                );
            }
            None => self.icon.paint(p, rect.shrink(6.0), color),
        }
        resp
    }
}

/// Warning sign: filled circle with an exclamation mark (bar and dot).
pub fn paint_alert(p: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let c = rect.center();
    let r = rect.width().min(rect.height()) / 2.0;
    p.circle_filled(c, r, color);
    let ink = egui::Color32::WHITE;
    p.line_segment(
        [
            egui::pos2(c.x, c.y - r * 0.55),
            egui::pos2(c.x, c.y + r * 0.15),
        ],
        egui::Stroke::new((r * 0.24).max(1.5), ink),
    );
    p.circle_filled(egui::pos2(c.x, c.y + r * 0.5), (r * 0.14).max(1.0), ink);
}
