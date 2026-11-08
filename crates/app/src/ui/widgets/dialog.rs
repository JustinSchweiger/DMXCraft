//! Einheitliches Dialogfenster: Kopf mit Titel und Schließen-Symbol,
//! Inhalt, Fußzeile mit den Aktionsknöpfen unten rechts.
//!
//! Alle Fenster der App laufen hierüber (siehe docs/UI-GUIDELINES.md, 4.).

use eframe::egui::{
    self, vec2, Align, Align2, CornerRadius, Frame, Id, Layout, Rect, Response, Sense, Stroke,
    TextBuffer, Ui, UiBuilder, Vec2, Widget,
};

use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton};

const HEADER_H: f32 = 36.0;
const FOOTER_H: f32 = 48.0;
/// Höhe von Eingabefeldern in Formularen.
pub const INPUT_H: f32 = 28.0;
/// Breite der Beschriftungsspalte in Formularen.
pub const LABEL_W: f32 = 120.0;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Cancel,
    Secondary,
    Primary,
}

struct Button {
    label: String,
    kind: Kind,
    enabled: bool,
}

pub struct Dialog<'a> {
    title: &'a str,
    id: Id,
    width: f32,
    size: Option<Vec2>,
    /// Feste Inhaltshöhe (nicht veränderbare Dialoge), z. B. bei Reitern.
    height: Option<f32>,
    /// Inhalt ohne Innenabstand (eigene Aufteilung wie Reiterleiste).
    flush: bool,
    buttons: Vec<Button>,
    top_right: Option<egui::Pos2>,
    escape: bool,
}

pub struct DialogResult {
    /// Index des geklickten Knopfs (in der Reihenfolge des Hinzufügens).
    /// Primär-Knöpfe werden auch durch Enter ausgelöst.
    pub clicked: Option<usize>,
}

impl DialogResult {
    pub fn clicked(&self, index: usize) -> bool {
        self.clicked == Some(index)
    }
}

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            id: Id::unique(("dialog", title)),
            width: 380.0,
            size: None,
            height: None,
            flush: false,
            buttons: Vec::new(),
            top_right: None,
            escape: true,
        }
    }

    pub fn id(mut self, id: &str) -> Self {
        self.id = Id::unique(("dialog", id));
        self
    }

    /// Feste Inhaltsbreite (nicht veränderbare Dialoge).
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Feste Inhaltshöhe – das Fenster springt nicht, wenn sich der Inhalt ändert.
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    /// Inhalt ohne Innenabstand; `width`/`height` gelten dann für die ganze Fläche.
    pub fn flush(mut self) -> Self {
        self.flush = true;
        self
    }

    /// Größenveränderbar mit Startgröße.
    pub fn resizable(mut self, size: impl Into<Vec2>) -> Self {
        self.size = Some(size.into());
        self
    }

    /// Schließt den Dialog ohne weitere Aktion (z. B. „Abbrechen“).
    pub fn cancel(self, label: &str) -> Self {
        self.push(label, Kind::Cancel, true)
    }

    /// Weitere Aktion ohne Hervorhebung (z. B. „Nicht speichern“).
    pub fn secondary(self, label: &str) -> Self {
        self.push(label, Kind::Secondary, true)
    }

    /// Hauptaktion, ganz rechts, auch per Enter.
    pub fn primary(self, label: &str) -> Self {
        self.push(label, Kind::Primary, true)
    }

    fn push(mut self, label: &str, kind: Kind, enabled: bool) -> Self {
        self.buttons.push(Button {
            label: label.to_owned(),
            kind,
            enabled,
        });
        self
    }

    /// Zeigt den Dialog. `body` bekommt `true` in den ersten Frames nach dem
    /// Öffnen (zum Fokussieren des ersten Felds – nur dann, sonst geht Tab nicht).
    /// Startposition (rechte obere Ecke) statt Bildschirmmitte.
    pub fn top_right(mut self, pos: egui::Pos2) -> Self {
        self.top_right = Some(pos);
        self
    }

    /// Esc schließt den Dialog nicht (z. B. wenn Esc eine Eingabe abbricht).
    pub fn keep_on_escape(mut self) -> Self {
        self.escape = false;
        self
    }

    pub fn show(
        self,
        ctx: &egui::Context,
        open: &mut bool,
        body: impl FnOnce(&mut Ui, bool),
    ) -> DialogResult {
        let frame_nr = ctx.cumulative_frame_nr();
        // (Öffnungs-Frame, letzter Frame, Fokus vor dem Öffnen):
        // neu geöffnet, wenn ein Frame fehlt.
        let key = self.id.with("frames");
        let last: Option<(u64, u64, Option<Id>)> = ctx.data(|d| d.get_temp(key));
        let focused = ctx.memory(|m| m.focused());
        let (opened, before) = match last {
            Some((o, l, b)) if l + 1 >= frame_nr => (o, b),
            _ => (frame_nr, focused),
        };
        ctx.data_mut(|d| d.insert_temp(key, (opened, frame_nr, before)));
        // egui misst neue Fenster zuerst nur aus und ignoriert dabei Fokus-
        // anfragen. Daher anfragen, bis sich der Fokus geändert hat.
        let first = frame_nr - opened <= 30 && focused == before;

        let mut clicked = None;
        let mut close = self.escape && ctx.input(|i| i.key_pressed(egui::Key::Escape));
        let enter = frame_nr > opened && ctx.input(|i| i.key_pressed(egui::Key::Enter));

        let frame = Frame::new()
            .fill(theme::surface())
            .stroke(Stroke::new(1.0, theme::border()))
            .corner_radius(CornerRadius::same(theme::R_M))
            .shadow(ctx.global_style().visuals.window_shadow);
        let mut window = egui::Window::new(self.title)
            .id(self.id)
            .title_bar(false)
            .collapsible(false)
            .frame(frame)
            .pivot(if self.top_right.is_some() {
                Align2::RIGHT_TOP
            } else {
                Align2::CENTER_CENTER
            })
            .default_pos(self.top_right.unwrap_or(ctx.content_rect().center()));
        window = match self.size {
            Some(s) => window
                .resizable(true)
                .default_size(s)
                .min_size(vec2(320.0, 200.0)),
            None => window.resizable(false),
        };

        let title = self.title;
        let width = self.width;
        let resizable = self.size.is_some();
        let buttons = &self.buttons;
        let fixed_height = self.height;
        let flush = self.flush;
        window.show(ctx, |ui| {
            let spacing = ui.spacing().item_spacing;
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            let w = if resizable {
                ui.available_width()
            } else if flush {
                width
            } else {
                width + 2.0 * theme::SP_L
            };
            ui.set_width(w);

            // Kopf
            let (head, _) = ui.allocate_exact_size(vec2(w, HEADER_H), Sense::hover());
            let r = theme::R_M;
            ui.painter().rect_filled(
                head,
                CornerRadius {
                    nw: r,
                    ne: r,
                    sw: 0,
                    se: 0,
                },
                theme::bg(),
            );
            ui.painter().hline(
                head.x_range(),
                head.bottom() - 0.5,
                Stroke::new(1.0, theme::border()),
            );
            ui.painter().text(
                head.center(),
                Align2::CENTER_CENTER,
                title.to_uppercase(),
                theme::heading_font(),
                theme::text(),
            );
            let close_rect = Rect::from_center_size(
                head.right_center() - vec2(HEADER_H / 2.0, 0.0),
                Vec2::splat(24.0),
            );
            if ui
                .put(close_rect, IconButton::new(Icon::Close).flat().danger())
                .on_hover_text(t("Schließen"))
                .clicked()
            {
                close = true;
            }

            // Inhalt
            let footer_h = if buttons.is_empty() { 0.0 } else { FOOTER_H };
            let body_rect = if resizable {
                let h = (ui.available_height() - footer_h).max(60.0);
                Some(Rect::from_min_size(ui.cursor().min, vec2(w, h)))
            } else {
                None
            };
            let margin = if flush {
                0.0
            } else if resizable {
                theme::SP_S
            } else {
                theme::SP_L
            };
            match body_rect {
                Some(rect) => {
                    ui.scope_builder(UiBuilder::new().max_rect(rect), |ui| {
                        ui.set_min_size(rect.size());
                        ui.spacing_mut().item_spacing = spacing;
                        Frame::new().inner_margin(margin).show(ui, |ui| {
                            ui.set_min_size(rect.size() - Vec2::splat(2.0 * margin));
                            body(ui, first)
                        });
                    });
                }
                None => {
                    Frame::new().inner_margin(margin).show(ui, |ui| {
                        ui.spacing_mut().item_spacing = spacing;
                        ui.set_width(width);
                        if let Some(h) = fixed_height {
                            ui.set_height(h);
                        }
                        body(ui, first)
                    });
                }
            }

            // Fußzeile
            if !buttons.is_empty() {
                let (foot, _) = ui.allocate_exact_size(vec2(w, FOOTER_H), Sense::hover());
                ui.painter().rect_filled(
                    foot,
                    CornerRadius {
                        nw: 0,
                        ne: 0,
                        sw: r,
                        se: r,
                    },
                    theme::bg(),
                );
                ui.painter().hline(
                    foot.x_range(),
                    foot.top() + 0.5,
                    Stroke::new(1.0, theme::border()),
                );
                let mut row = ui.new_child(
                    UiBuilder::new()
                        .max_rect(foot.shrink2(vec2(theme::SP_M, 0.0)))
                        .layout(Layout::right_to_left(Align::Center)),
                );
                row.spacing_mut().item_spacing = vec2(theme::SP_S, 0.0);
                for (i, b) in buttons.iter().enumerate().rev() {
                    let resp = row.add(FooterButton {
                        label: &b.label,
                        primary: b.kind == Kind::Primary,
                        enabled: b.enabled,
                    });
                    let hit = resp.clicked() || (enter && b.kind == Kind::Primary && b.enabled);
                    if hit {
                        if b.kind == Kind::Cancel {
                            close = true;
                        } else {
                            clicked = Some(i);
                        }
                    }
                }
            }
        });
        if close {
            *open = false;
        }
        DialogResult { clicked }
    }
}

/// Knopf der Fußzeile: feste Mindestbreite, Überschrift-Schrift.
/// Auch für eigene Fenster mit Dialog-Fußzeile (Profile Builder).
pub struct FooterButton<'a> {
    label: &'a str,
    primary: bool,
    enabled: bool,
}

impl<'a> FooterButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            primary: false,
            enabled: true,
        }
    }

    pub fn primary(mut self, primary: bool) -> Self {
        self.primary = primary;
        self
    }

    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

impl Widget for FooterButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let galley = ui.painter().layout_no_wrap(
            self.label.to_uppercase(),
            theme::heading_font(),
            theme::text(),
        );
        let w = (galley.size().x + 2.0 * theme::SP_L).max(96.0);
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let (rect, resp) = ui.allocate_exact_size(vec2(w, 30.0), sense);
        let hovered = resp.hovered() && self.enabled;
        let (fill, stroke, text) = match (self.primary, self.enabled, hovered) {
            (_, false, _) => (theme::raised(), theme::border(), theme::text_faint()),
            (true, true, false) => (theme::accent(), theme::accent(), theme::bg()),
            (true, true, true) => (theme::accent_hover(), theme::accent_hover(), theme::bg()),
            (false, true, false) => (theme::raised(), theme::border(), theme::text()),
            (false, true, true) => (theme::raised_hover(), theme::accent_dim(), theme::text()),
        };
        let p = ui.painter();
        p.rect(
            rect,
            theme::R_S as f32,
            fill,
            Stroke::new(1.0, stroke),
            egui::StrokeKind::Inside,
        );
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.label.to_uppercase(),
            theme::heading_font(),
            text,
        );
        resp
    }
}

/// Einzeiliges Eingabefeld im App-Stil: Text vertikal mittig, feste Höhe.
pub fn text_field<'t>(text: &'t mut dyn TextBuffer) -> egui::TextEdit<'t> {
    egui::TextEdit::singleline(text)
        .align(Align2::LEFT_CENTER)
        .min_size(vec2(0.0, INPUT_H))
        .margin(vec2(theme::SP_S, 0.0))
        .background_color(theme::bg())
}

/// Formular: Beschriftung links, Bedienelement rechts, Zeilen bündig.
pub fn form(ui: &mut Ui, id: &str, rows: impl FnOnce(&mut Form<'_>)) {
    egui::Grid::new(id)
        .num_columns(2)
        .min_col_width(LABEL_W)
        .min_row_height(INPUT_H)
        .spacing(vec2(theme::SP_M, theme::SP_S))
        .show(ui, |ui| rows(&mut Form { ui }));
}

pub struct Form<'u> {
    ui: &'u mut Ui,
}

impl Form<'_> {
    /// Eine Zeile; das Bedienelement füllt die restliche Breite.
    pub fn row<R>(&mut self, label: &str, add: impl FnOnce(&mut Ui) -> R) -> R {
        self.ui
            .label(egui::RichText::new(label).color(theme::text_dim()));
        let r = self
            .ui
            .with_layout(Layout::left_to_right(Align::Center), |ui| add(ui))
            .inner;
        self.ui.end_row();
        r
    }

    /// Zeile ohne Beschriftung (z. B. Fehlermeldung unter den Feldern).
    pub fn note(&mut self, add: impl FnOnce(&mut Ui)) {
        self.ui.label("");
        self.ui
            .with_layout(Layout::left_to_right(Align::Center), add);
        self.ui.end_row();
    }
}

/// Ganzen Text eines Eingabefelds markieren, damit man sofort tippen kann.
pub fn select_all(ctx: &egui::Context, id: egui::Id, text: &str) {
    let mut st = egui::TextEdit::load_state(ctx, id).unwrap_or_default();
    st.cursor.set_char_range(Some(egui::text::CCursorRange::two(
        egui::text::CCursor::new(0),
        egui::text::CCursor::new(text.chars().count()),
    )));
    st.store(ctx, id);
}
