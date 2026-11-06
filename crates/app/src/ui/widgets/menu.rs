//! Menüs im App-Stil: Hauptmenü und alle Kontextmenüs.
//!
//! Ein Eintrag: Symbolspalte links, Text, Tastenkürzel rechts (gedämpft),
//! bei Untermenüs ein Pfeil. Überfahren = hinterlegt, ohne Rahmen.

use eframe::egui::{self, pos2, vec2, Align2, Color32, Painter, Rect, Response, Sense, Stroke, Ui};

use crate::ui::theme;
use crate::ui::widgets::Icon;

/// Höhe eines Menüeintrags.
pub const ITEM_H: f32 = 28.0;
/// Mindestbreite eines Menüs.
pub const MIN_W: f32 = 230.0;
const ICON_SLOT: f32 = 28.0;

type IconFn<'a> = Box<dyn FnOnce(&Painter, Rect, Color32) + 'a>;

pub struct MenuItem<'a> {
    label: &'a str,
    icon: Option<IconFn<'a>>,
    shortcut: Option<&'a str>,
    enabled: bool,
    checked: bool,
    danger: bool,
    arrow: bool,
    slot: bool,
    tooltip: Option<&'a str>,
}

impl<'a> MenuItem<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            icon: None,
            shortcut: None,
            enabled: true,
            checked: false,
            danger: false,
            arrow: false,
            slot: true,
            tooltip: None,
        }
    }

    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(Box::new(move |p, r, c| icon.paint(p, r, c)));
        self
    }

    /// Eigenes Symbol (z. B. Gerätform).
    pub fn icon_fn(mut self, f: impl FnOnce(&Painter, Rect, Color32) + 'a) -> Self {
        self.icon = Some(Box::new(f));
        self
    }

    pub fn shortcut(mut self, s: &'a str) -> Self {
        self.shortcut = Some(s);
        self
    }

    pub fn enabled(mut self, on: bool) -> Self {
        self.enabled = on;
        self
    }

    /// Aktiver Zustand: Häkchen in der Symbolspalte (wenn kein Symbol) und
    /// Text in `ACCENT`.
    pub fn checked(mut self, on: bool) -> Self {
        self.checked = on;
        self
    }

    /// Ohne Symbolspalte (oberste Menüebene).
    pub fn plain(mut self) -> Self {
        self.slot = false;
        self
    }

    /// Zerstörende Aktion (Löschen): beim Überfahren rot.
    pub fn danger(mut self) -> Self {
        self.danger = true;
        self
    }

    /// Hinweis für noch nicht verfügbare oder erklärungsbedürftige Einträge.
    pub fn tooltip(mut self, t: &'a str) -> Self {
        self.tooltip = Some(t);
        self
    }

    fn paint(self, ui: &mut Ui) -> Response {
        let font = theme::body();
        let label_w = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), font.clone(), theme::text())
            .size()
            .x;
        let short_w = self.shortcut.map_or(0.0, |s| {
            ui.painter()
                .layout_no_wrap(s.to_owned(), theme::caption(), theme::text())
                .size()
                .x
                + theme::SP_L
        });
        let slot = if self.slot { ICON_SLOT } else { theme::SP_XS };
        let natural = slot + label_w + short_w + 20.0 + theme::SP_S * 2.0;
        let sense = if self.enabled {
            Sense::click()
        } else {
            Sense::hover()
        };
        let resp = ui.allocate_response(vec2(natural.max(MIN_W), ITEM_H), sense);
        let rect = resp.rect;
        let hovered = self.enabled && (resp.hovered() || resp.has_focus());
        let p = ui.painter();
        if hovered {
            let fill = if self.danger {
                theme::DANGER
            } else {
                theme::raised_hover()
            };
            p.rect_filled(rect, theme::R_S as f32, fill);
        }
        let color = if !self.enabled {
            theme::text_faint()
        } else if hovered && self.danger {
            theme::text()
        } else if self.danger {
            theme::DANGER
        } else if self.checked {
            theme::accent()
        } else {
            theme::text()
        };
        let icon_rect = Rect::from_center_size(
            pos2(
                rect.left() + theme::SP_XS + ICON_SLOT / 2.0,
                rect.center().y,
            ),
            egui::Vec2::splat(18.0),
        );
        match self.icon {
            Some(f) => f(p, icon_rect, color),
            None if self.checked => {
                let c = icon_rect.center();
                let s = icon_rect.width() * 0.5;
                p.add(egui::Shape::line(
                    vec![
                        pos2(c.x - s * 0.5, c.y),
                        pos2(c.x - s * 0.12, c.y + s * 0.4),
                        pos2(c.x + s * 0.55, c.y - s * 0.45),
                    ],
                    Stroke::new(1.8, color),
                ));
            }
            None => {}
        }
        p.text(
            pos2(
                rect.left() + theme::SP_XS + slot + theme::SP_XS,
                rect.center().y,
            ),
            Align2::LEFT_CENTER,
            self.label,
            font,
            color,
        );
        let mut right = rect.right() - theme::SP_S;
        if self.arrow {
            let c = pos2(right - 4.0, rect.center().y);
            p.add(egui::Shape::line(
                vec![
                    pos2(c.x - 2.5, c.y - 4.5),
                    pos2(c.x + 2.0, c.y),
                    pos2(c.x - 2.5, c.y + 4.5),
                ],
                Stroke::new(
                    1.5,
                    if self.enabled {
                        theme::text_dim()
                    } else {
                        theme::text_faint()
                    },
                ),
            ));
            right -= 16.0;
        }
        if let Some(s) = self.shortcut {
            p.text(
                pos2(right, rect.center().y),
                Align2::RIGHT_CENTER,
                s,
                theme::caption(),
                if self.enabled {
                    theme::text_dim()
                } else {
                    theme::text_faint()
                },
            );
        }
        match self.tooltip {
            Some(t) => resp.on_hover_text(t),
            None => resp,
        }
    }
}

impl egui::Widget for MenuItem<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.paint(ui)
    }
}

/// Am Anfang jedes Menüs: Mindestbreite, Einträge ohne Lücke.
pub fn begin(ui: &mut Ui) {
    ui.set_min_width(MIN_W);
    ui.spacing_mut().item_spacing.y = 1.0;
}

/// Eintrag, der beim Klick das Menü schließt. Gibt `true` bei Klick zurück.
pub fn item(ui: &mut Ui, item: MenuItem<'_>) -> bool {
    let clicked = ui.add(item).clicked();
    if clicked {
        ui.close();
    }
    clicked
}

/// Eintrag mit Untermenü (öffnet beim Überfahren).
pub fn sub(ui: &mut Ui, mut item: MenuItem<'_>, content: impl FnOnce(&mut Ui)) {
    item.arrow = true;
    let enabled = item.enabled;
    let resp = ui.add(item);
    if enabled {
        egui::containers::menu::SubMenu::new().show(ui, &resp, |ui| {
            begin(ui);
            content(ui);
        });
    }
}

/// Kontextmenü (Rechtsklick) im App-Stil. Klicks auf deaktivierte Einträge
/// oder Trenner lassen das Menü offen; Einträge schließen es selbst.
pub fn context(resp: &Response, content: impl FnOnce(&mut Ui)) {
    egui::Popup::context_menu(resp)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(content);
}

/// Aufklappmenü an einem Knopf (z. B. Hauptmenü), gleiches Schließverhalten.
pub fn dropdown(resp: &Response, content: impl FnOnce(&mut Ui)) {
    egui::Popup::menu(resp)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(content);
}

/// Trennlinie zwischen Gruppen.
pub fn separator(ui: &mut Ui) {
    // Breite 0: das gestreckte Layout füllt die Menübreite, ohne sie zu vergrößern.
    let rect = ui.allocate_response(vec2(0.0, 9.0), Sense::hover()).rect;
    ui.painter().hline(
        (rect.left() + theme::SP_S)..=(rect.right() - theme::SP_S),
        rect.center().y,
        Stroke::new(1.0, theme::border()),
    );
}

/// Kopfzeile eines Kontextmenüs (z. B. Gerätename), nicht anklickbar.
pub fn title(ui: &mut Ui, text: &str) {
    let rect = ui.allocate_response(vec2(0.0, 26.0), Sense::hover()).rect;
    ui.painter().text(
        pos2(rect.left() + theme::SP_S, rect.center().y),
        Align2::LEFT_CENTER,
        text.to_uppercase(),
        theme::heading_font(),
        theme::text_dim(),
    );
}

/// Grundeinstellungen für Menüs (Rahmen, Abstände) – in `theme::apply`.
pub fn style(style: &mut egui::Style) {
    style.spacing.menu_margin = egui::Margin::same(theme::SP_XS as i8);
    style.visuals.menu_corner_radius = egui::CornerRadius::same(theme::R_M);
    style.visuals.popup_shadow = egui::epaint::Shadow {
        offset: [0, 6],
        blur: 18,
        spread: 0,
        color: Color32::from_black_alpha(140),
    };
}
