//! Kanal-Fader des Programmers.
//!
//! Aufbau von oben nach unten: Statuspunkt (leuchtet in der Farbe des
//! Betriebsmodus, wenn der Kanal einen Programmer-Wert ausgibt), Kanalnummer,
//! Symbol der Kanalart, Wert (Klick = eintippen, „OFF“ = nicht gesetzt) und
//! die Schiene mit Griff.
//!
//! Bedienung: Punkt anklicken = Kanal speichern/freigeben (auch mit Wert 0),
//! Ziehen/Klicken = setzen, Mausrad = ±2 (Umschalt ±10),
//! Doppelklick = 0 ↔ voll, Rechtsklick = Menü (Freigeben, 0, Voll).

use crate::i18n::t;
use dmxcraft_core::ChannelKind;
use eframe::egui::{self, pos2, Color32, Rect, Sense, Shape, Stroke, StrokeKind, Ui, Vec2};

use crate::ui::theme;

/// Standardbreite einer Fader-Spalte (schmal, z. B. für „Alle Kanäle“).
pub const FADER_W: f32 = 44.0;
const DOT_H: f32 = 20.0;
const NUM_H: f32 = 18.0;
const SYMBOL_H: f32 = 30.0;
const VALUE_H: f32 = 24.0;
const HANDLE: Vec2 = Vec2::new(24.0, 34.0);
/// Schritt je Mausrad-Raste.
const WHEEL_STEP: u8 = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FaderEvent {
    None,
    Changed(u8),
    Release,
    /// FX-Symbol am Fader angeklickt (Kanal dem Effekt zuordnen/entfernen).
    ToggleFx,
}

pub struct Fader {
    value: u8,
    number: u16,
    kind: ChannelKind,
    hint: Option<String>,
    active: bool,
    /// Farbe des Statuspunkts (Betriebsmodus).
    dot: Color32,
    color: Color32,
    height: Option<f32>,
    width: f32,
    /// FX-Symbol zeigen: `Some(zugeordnet)`.
    fx: Option<bool>,
}

impl Fader {
    pub fn new(value: u8, number: u16, kind: ChannelKind) -> Self {
        Self {
            value,
            number,
            kind,
            hint: None,
            active: false,
            dot: theme::OK,
            color: theme::accent(),
            height: None,
            width: FADER_W,
            fx: None,
        }
    }

    /// Wert liegt im Programmer (Punkt leuchtet, Griff hell).
    pub fn active(mut self, on: bool) -> Self {
        self.active = on;
        self
    }

    /// Farbe des Statuspunkts.
    pub fn dot(mut self, c: Color32) -> Self {
        self.dot = c;
        self
    }

    /// Farbe des Symbols (Farbkanäle).
    pub fn color(mut self, c: Color32) -> Self {
        self.color = c;
        self
    }

    /// Gesamthöhe; Standard: die gesamte verfügbare Höhe.
    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    /// Spaltenbreite.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w.max(FADER_W);
        self
    }

    /// Kleines FX-Symbol am Kanalsymbol (rot = vom Effekt bewegt).
    pub fn fx(mut self, assigned: Option<bool>) -> Self {
        self.fx = assigned;
        self
    }

    pub fn hint(mut self, text: impl Into<String>) -> Self {
        self.hint = Some(text.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> FaderEvent {
        let head = DOT_H + NUM_H + SYMBOL_H + VALUE_H;
        let h = self
            .height
            .unwrap_or_else(|| ui.available_height())
            .max(head + HANDLE.y + 40.0);
        let (outer, _) = ui.allocate_exact_size(Vec2::new(self.width, h), Sense::hover());
        let id = ui.make_persistent_id(("fader", outer.min.x as i32, self.number));
        let mut event = FaderEvent::None;
        let mut value = self.value;

        let p = ui.painter().clone();
        // Spalte
        p.rect_filled(outer, 0.0, theme::raised());
        p.line_segment(
            [outer.right_top(), outer.right_bottom()],
            Stroke::new(1.0, theme::bg()),
        );

        // Statuspunkt – Klick: Kanal speichern (mit aktuellem Wert) bzw. freigeben
        let dot_c = pos2(outer.center().x, outer.top() + DOT_H * 0.55);
        let dot_rect = Rect::from_center_size(dot_c, Vec2::new(self.width.min(28.0), DOT_H));
        let dot_resp = ui.interact(dot_rect, id.with("dot"), Sense::click());
        if dot_resp.clicked() {
            event = if self.active {
                FaderEvent::Release
            } else {
                FaderEvent::Changed(value)
            };
        }
        if dot_resp.hovered() {
            p.circle_filled(dot_c, 9.0, theme::raised_hover());
        }
        if self.active {
            p.circle_filled(dot_c, 5.0, self.dot);
            p.circle_filled(dot_c, 8.0, self.dot.gamma_multiply(0.18));
        } else {
            p.circle_filled(dot_c, 5.0, theme::bg());
            p.circle_stroke(dot_c, 5.0, Stroke::new(1.0, theme::border()));
        }
        // Kanalnummer
        let mut y = outer.top() + DOT_H;
        p.text(
            pos2(outer.center().x, y + NUM_H * 0.5),
            egui::Align2::CENTER_CENTER,
            self.number.to_string(),
            theme::heading_font(),
            theme::text(),
        );
        y += NUM_H;
        // Symbol
        let sym = Rect::from_center_size(
            pos2(outer.center().x, y + SYMBOL_H * 0.5),
            Vec2::splat(24.0),
        );
        paint_symbol(&p, sym, self.kind, self.color);
        // Kanalname nur beim Überfahren des Symbols
        if let Some(h) = &self.hint {
            ui.interact(sym, id.with("symbol"), Sense::hover())
                .on_hover_text(h);
        }
        if let Some(on) = self.fx {
            let badge = Rect::from_min_size(
                pos2(sym.right() - 6.0, sym.top() - 4.0),
                Vec2::new(16.0, 11.0),
            );
            let r = ui
                .interact(badge.expand(2.0), id.with("fx"), Sense::click())
                .on_hover_text(if on {
                    t("Vom Effekt entfernen")
                } else {
                    t("Dem Effekt zuordnen")
                });
            if r.clicked() {
                event = FaderEvent::ToggleFx;
            }
            let fill = if on {
                theme::DANGER
            } else if r.hovered() {
                theme::raised_hover()
            } else {
                theme::bg()
            };
            p.rect_filled(badge, 2.0, fill);
            p.rect_stroke(
                badge,
                2.0,
                Stroke::new(1.0, if on { theme::DANGER } else { theme::border() }),
                StrokeKind::Inside,
            );
            p.text(
                badge.center(),
                egui::Align2::CENTER_CENTER,
                "FX",
                egui::FontId::new(8.0, egui::FontFamily::Name(theme::HEADING_FAMILY.into())),
                if on {
                    Color32::WHITE
                } else {
                    theme::text_dim()
                },
            );
        }
        y += SYMBOL_H;

        // Wert – Klick zum Eintippen
        let value_rect = Rect::from_min_size(
            pos2(outer.left() + 3.0, y + 2.0),
            Vec2::new(self.width - 6.0, VALUE_H - 4.0),
        );
        let edit_id = id.with("edit");
        let editing: Option<String> = ui.data(|d| d.get_temp(edit_id));
        if let Some(mut text) = editing {
            let r = ui.put(
                value_rect,
                egui::TextEdit::singleline(&mut text)
                    .font(theme::caption())
                    .align(egui::Align2::CENTER_CENTER)
                    .margin(egui::Margin::symmetric(2, 1)),
            );
            if !r.has_focus() && !r.lost_focus() {
                r.request_focus();
                crate::ui::widgets::dialog::select_all(ui.ctx(), r.id, &text);
            }
            let (enter, esc) = ui.input(|i| {
                (
                    i.key_pressed(egui::Key::Enter),
                    i.key_pressed(egui::Key::Escape),
                )
            });
            if esc {
                ui.data_mut(|d| d.remove::<String>(edit_id));
            } else if enter || r.lost_focus() {
                ui.data_mut(|d| d.remove::<String>(edit_id));
                if let Ok(v) = text.trim().parse::<u16>() {
                    value = v.min(255) as u8;
                    event = FaderEvent::Changed(value);
                }
            } else {
                ui.data_mut(|d| d.insert_temp(edit_id, text));
            }
        } else {
            // Klick = eintippen, Ziehen nach oben/unten = Wert ändern
            let r = ui.interact(value_rect, id.with("value"), Sense::click_and_drag());
            if r.hovered() || r.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
            }
            let acc_id = id.with("value_drag");
            if r.drag_started() {
                ui.data_mut(|d| d.insert_temp(acc_id, 0.0f32));
            }
            let delta = crate::ui::widgets::drag::endless_delta(ui, &r);
            if r.dragged() {
                let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
                acc -= delta.y / 2.0; // 1 Schritt je 2 Punkte
                let steps = acc.trunc();
                if steps != 0.0 {
                    acc -= steps;
                    value = (f32::from(value) + steps).clamp(0.0, 255.0) as u8;
                    event = FaderEvent::Changed(value);
                }
                ui.data_mut(|d| d.insert_temp(acc_id, acc));
            }
            if r.hovered() || r.dragged() {
                p.rect_filled(value_rect, theme::R_S, theme::raised_hover());
            }
            let (txt, color) = if self.active {
                (value.to_string(), theme::text())
            } else {
                ("OFF".to_string(), theme::text_faint())
            };
            p.text(
                value_rect.center(),
                egui::Align2::CENTER_CENTER,
                txt,
                theme::caption(),
                color,
            );
            if r.clicked() {
                ui.data_mut(|d| d.insert_temp(edit_id, value.to_string()));
            }
        }
        y += VALUE_H;

        // Schiene
        let track_area = Rect::from_min_max(pos2(outer.left(), y), outer.max - Vec2::new(0.0, 6.0));
        let travel = Rect::from_min_max(
            pos2(track_area.left(), track_area.top() + HANDLE.y * 0.5),
            pos2(track_area.right(), track_area.bottom() - HANDLE.y * 0.5),
        );
        let resp = ui.interact(track_area, id, Sense::click_and_drag());
        if resp.double_clicked() {
            value = if value > 0 { 0 } else { 255 };
            event = FaderEvent::Changed(value);
        } else if resp.dragged() || resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let tv = 1.0 - ((pos.y - travel.top()) / travel.height()).clamp(0.0, 1.0);
                let v = (tv * 255.0).round() as u8;
                if v != value || !self.active {
                    value = v;
                    event = FaderEvent::Changed(v);
                }
            }
        }
        if ui.rect_contains_pointer(outer) {
            // je Raste ein Schritt; Touchpad-Pixel werden gesammelt
            let acc_id = id.with("wheel");
            let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
            let shift = ui.input(|i| i.modifiers.shift);
            let mut notches = 0i32;
            // Ereignisse nur einmal je Frame auswerten (egui kann einen Frame
            // in mehreren Durchläufen zeichnen)
            let frame_id = id.with("wheel_frame");
            let nr = ui.ctx().cumulative_frame_nr();
            let fresh = ui.data(|d| d.get_temp::<u64>(frame_id)) != Some(nr);
            ui.data_mut(|d| d.insert_temp(frame_id, nr));
            ui.input(|i| {
                for ev in i.events.iter().filter(|_| fresh) {
                    if let egui::Event::MouseWheel { unit, delta, .. } = ev {
                        let d = if delta.y != 0.0 { delta.y } else { delta.x };
                        match unit {
                            egui::MouseWheelUnit::Point => acc += d / 40.0,
                            _ => acc += d.signum(),
                        }
                    }
                }
            });
            while acc.abs() >= 1.0 {
                notches += acc.signum() as i32;
                acc -= acc.signum();
            }
            ui.data_mut(|d| d.insert_temp(acc_id, acc));
            if notches != 0 {
                let step = i32::from(if shift { 10 } else { WHEEL_STEP });
                value = (i32::from(value) + notches * step).clamp(0, 255) as u8;
                event = FaderEvent::Changed(value);
            }
            // nicht zusätzlich die Fader-Leiste scrollen
            ui.input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
        }
        crate::ui::widgets::menu::context(&resp, |ui| {
            use crate::ui::widgets::menu::{self, MenuItem};
            menu::begin(ui);
            if menu::item(ui, MenuItem::new(t("Freigeben")).icon(super::Icon::Reset)) {
                event = FaderEvent::Release;
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Aus (0)")).icon(super::Icon::LightOff)) {
                event = FaderEvent::Changed(0);
            }
            for (label, v) in [("25 %", 64u8), ("50 %", 128), ("75 %", 191)] {
                if menu::item(ui, MenuItem::new(label)) {
                    event = FaderEvent::Changed(v);
                }
            }
            if menu::item(
                ui,
                MenuItem::new(t("Voll (255)")).icon(super::Icon::LightOn),
            ) {
                event = FaderEvent::Changed(255);
            }
        });

        // Schlitz
        let slot = Rect::from_center_size(
            pos2(travel.center().x, track_area.center().y),
            Vec2::new(6.0, track_area.height() - 6.0),
        );
        p.rect_filled(slot, 3.0, theme::bg());
        // Griff
        let tv = f32::from(value) / 255.0;
        let hy = travel.bottom() - travel.height() * tv;
        let handle = Rect::from_center_size(pos2(travel.center().x, hy), HANDLE);
        let hot = resp.hovered() || resp.dragged();
        let (fill, edge, line) = if self.active {
            (
                theme::raised_hover(),
                if hot {
                    theme::text()
                } else {
                    theme::text_dim()
                },
                theme::text(),
            )
        } else {
            (
                theme::raised(),
                if hot {
                    theme::text_dim()
                } else {
                    theme::border()
                },
                theme::text_faint(),
            )
        };
        p.rect_filled(handle, theme::R_S, fill);
        p.rect_stroke(
            handle,
            theme::R_S,
            Stroke::new(1.5, edge),
            StrokeKind::Inside,
        );
        p.line_segment(
            [
                pos2(handle.left() + 7.0, handle.center().y),
                pos2(handle.right() - 7.0, handle.center().y),
            ],
            Stroke::new(2.5, line),
        );
        event
    }
}

/// Farbe einer Kanalart (Pegel im Fader, Farbpunkt im Symbol).
pub fn kind_color(k: ChannelKind) -> Color32 {
    match k {
        ChannelKind::Red => Color32::from_rgb(0xff, 0x45, 0x45),
        ChannelKind::Green => Color32::from_rgb(0x3d, 0xe0, 0x6a),
        ChannelKind::Blue => Color32::from_rgb(0x4a, 0x7d, 0xff),
        ChannelKind::White => Color32::from_rgb(0xf2, 0xf2, 0xf2),
        ChannelKind::Amber => Color32::from_rgb(0xff, 0xa8, 0x2e),
        ChannelKind::Uv => Color32::from_rgb(0xa2, 0x5c, 0xff),
        _ => theme::accent(),
    }
}

/// Symbol der Kanalart (eigene, schlichte Zeichnungen).
pub fn paint_symbol(p: &egui::Painter, rect: Rect, kind: ChannelKind, color: Color32) {
    use ChannelKind as K;
    let c = rect.center();
    let r = rect.width() * 0.5;
    let fg = theme::text();
    match kind {
        K::Red | K::Green | K::Blue | K::White | K::Amber | K::Uv => {
            p.circle_filled(c, r * 0.8, color);
            p.circle_stroke(c, r * 0.8, Stroke::new(1.0, Color32::from_black_alpha(90)));
        }
        K::ColorWheel => {
            let cols = [
                Color32::from_rgb(0xff, 0x40, 0x40),
                Color32::from_rgb(0xff, 0xd0, 0x30),
                Color32::from_rgb(0x40, 0xd0, 0x60),
                Color32::from_rgb(0x40, 0x80, 0xff),
            ];
            for (i, col) in cols.iter().enumerate() {
                let a0 = std::f32::consts::FRAC_PI_2 * i as f32;
                let mut pts = vec![c];
                for k in 0..=8 {
                    let a = a0 + std::f32::consts::FRAC_PI_2 * k as f32 / 8.0;
                    pts.push(c + Vec2::angled(a) * r * 0.8);
                }
                p.add(Shape::convex_polygon(pts, *col, Stroke::NONE));
            }
        }
        _ => {
            // Kachel mit Zeichen
            p.rect_filled(rect, theme::R_S, theme::bg());
            p.rect_stroke(
                rect,
                theme::R_S,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let s = Stroke::new(1.6, fg);
            match kind {
                K::Dimmer => super::Icon::LightOn.paint(p, rect.shrink(4.0), color),
                K::Pan | K::PanFine => {
                    let w = r * 0.6;
                    p.line_segment([c - Vec2::new(w, 0.0), c + Vec2::new(w, 0.0)], s);
                    for d in [-1.0, 1.0] {
                        let tip = c + Vec2::new(d * w, 0.0);
                        p.line_segment([tip, tip + Vec2::new(-d * 4.0, -4.0)], s);
                        p.line_segment([tip, tip + Vec2::new(-d * 4.0, 4.0)], s);
                    }
                    if kind == K::PanFine {
                        fine_mark(p, rect);
                    }
                }
                K::Tilt | K::TiltFine => {
                    let w = r * 0.6;
                    p.line_segment([c - Vec2::new(0.0, w), c + Vec2::new(0.0, w)], s);
                    for d in [-1.0, 1.0] {
                        let tip = c + Vec2::new(0.0, d * w);
                        p.line_segment([tip, tip + Vec2::new(-4.0, -d * 4.0)], s);
                        p.line_segment([tip, tip + Vec2::new(4.0, -d * 4.0)], s);
                    }
                    if kind == K::TiltFine {
                        fine_mark(p, rect);
                    }
                }
                K::Strobe => {
                    let pts = [
                        c + Vec2::new(2.0, -r * 0.7),
                        c + Vec2::new(-r * 0.45, 1.0),
                        c + Vec2::new(-0.5, 1.0),
                        c + Vec2::new(-2.0, r * 0.7),
                        c + Vec2::new(r * 0.45, -1.0),
                        c + Vec2::new(0.5, -1.0),
                    ];
                    p.add(Shape::closed_line(
                        pts.to_vec(),
                        Stroke::new(1.3, theme::WARN),
                    ));
                }
                K::Gobo => {
                    p.circle_stroke(c, r * 0.65, s);
                    for k in 0..3 {
                        let a = std::f32::consts::TAU * k as f32 / 3.0 - 1.57;
                        p.circle_filled(c + Vec2::angled(a) * r * 0.32, 2.0, fg);
                    }
                }
                K::Speed => {
                    let n = 12;
                    let pts: Vec<_> = (0..=n)
                        .map(|i| {
                            let a = std::f32::consts::PI * (1.0 + i as f32 / n as f32);
                            c + Vec2::new(0.0, 3.0) + Vec2::angled(a) * r * 0.6
                        })
                        .collect();
                    p.add(Shape::line(pts, s));
                    p.line_segment(
                        [c + Vec2::new(0.0, 3.0), c + Vec2::new(r * 0.35, -r * 0.25)],
                        s,
                    );
                }
                K::Laser => {
                    // Quelle links, Strahlen fächern nach rechts auf
                    let src = c + Vec2::new(-r * 0.6, r * 0.35);
                    let cols = [
                        Color32::from_rgb(0xff, 0x40, 0x40),
                        Color32::from_rgb(0x40, 0xe0, 0x60),
                        Color32::from_rgb(0x50, 0x90, 0xff),
                    ];
                    for (k, col) in cols.iter().enumerate() {
                        let end = c + Vec2::new(r * 0.65, -r * 0.65 + k as f32 * r * 0.5);
                        p.line_segment([src, end], Stroke::new(1.3, *col));
                        p.circle_filled(end, 1.6, *col);
                    }
                    p.circle_filled(src, 2.2, fg);
                }
                K::Fog => {
                    // Wolke aus drei Kreisen
                    let base = c + Vec2::new(0.0, r * 0.15);
                    for (dx, dy, rr) in [(-0.35, 0.05, 0.3), (0.0, -0.2, 0.36), (0.35, 0.05, 0.28)]
                    {
                        p.circle_filled(
                            base + Vec2::new(dx * r, dy * r),
                            rr * r,
                            theme::text_dim(),
                        );
                    }
                    p.line_segment(
                        [
                            base + Vec2::new(-r * 0.62, r * 0.33),
                            base + Vec2::new(r * 0.6, r * 0.33),
                        ],
                        Stroke::new(1.2, theme::text_dim()),
                    );
                }
                K::Fan => {
                    // drei Flügel
                    for k in 0..3 {
                        let a = std::f32::consts::TAU * k as f32 / 3.0 - 1.2;
                        let tip = c + Vec2::angled(a) * r * 0.7;
                        let side = c + Vec2::angled(a + 0.7) * r * 0.45;
                        p.add(Shape::convex_polygon(vec![c, side, tip], fg, Stroke::NONE));
                    }
                    p.circle_filled(c, 2.0, theme::bg());
                }
                K::Zoom => {
                    // Lichtkegel mit Pfeilen nach außen
                    let src = c + Vec2::new(-r * 0.6, 0.0);
                    for d in [-1.0, 1.0] {
                        let end = c + Vec2::new(r * 0.6, d * r * 0.6);
                        p.line_segment([src, end], s);
                        p.line_segment([end, end + Vec2::new(-4.0, 0.0)], s);
                        p.line_segment([end, end + Vec2::new(0.0, -d * 4.0)], s);
                    }
                }
                K::Focus => {
                    p.circle_stroke(c, r * 0.5, s);
                    for d in [Vec2::X, -Vec2::X, Vec2::Y, -Vec2::Y] {
                        p.line_segment([c + d * r * 0.5, c + d * r * 0.78], s);
                    }
                    p.circle_filled(c, 1.8, fg);
                }
                K::Program => {
                    // Abspielen-Dreieck mit Funken
                    let w = r * 0.5;
                    p.add(Shape::convex_polygon(
                        vec![
                            c + Vec2::new(-w * 0.7, -w),
                            c + Vec2::new(w, 0.0),
                            c + Vec2::new(-w * 0.7, w),
                        ],
                        theme::accent(),
                        Stroke::NONE,
                    ));
                    p.circle_filled(c + Vec2::new(r * 0.6, -r * 0.6), 1.5, fg);
                    p.circle_filled(c + Vec2::new(r * 0.68, r * 0.45), 1.2, fg);
                }
                K::Control => {
                    // drei Regler
                    for k in 0..3 {
                        let x = c.x + (k as f32 - 1.0) * r * 0.5;
                        p.line_segment(
                            [pos2(x, c.y - r * 0.6), pos2(x, c.y + r * 0.6)],
                            Stroke::new(1.2, theme::text_dim()),
                        );
                        let y = c.y + [0.3, -0.35, 0.1][k] * r;
                        p.rect_filled(
                            Rect::from_center_size(pos2(x, y), Vec2::new(5.0, 3.0)),
                            1.0,
                            fg,
                        );
                    }
                }
                K::NoFunction => {
                    p.circle_stroke(c, r * 0.55, Stroke::new(1.3, theme::text_faint()));
                    p.line_segment(
                        [
                            c + Vec2::angled(-0.8) * r * 0.55,
                            c + Vec2::angled(2.35) * r * 0.55,
                        ],
                        Stroke::new(1.3, theme::text_faint()),
                    );
                }
                _ => {
                    // Generisch: Wertebalken
                    let w = r * 1.2;
                    let bar = Rect::from_center_size(c, Vec2::new(w, 6.0));
                    p.rect_stroke(bar, 2.0, Stroke::new(1.2, fg), StrokeKind::Inside);
                    p.rect_filled(
                        Rect::from_min_size(bar.min, Vec2::new(w * 0.55, 6.0)),
                        2.0,
                        fg,
                    );
                }
            }
        }
    }
}

fn fine_mark(p: &egui::Painter, rect: Rect) {
    p.text(
        rect.right_bottom() - Vec2::new(3.0, 1.0),
        egui::Align2::RIGHT_BOTTOM,
        "F",
        egui::FontId::proportional(8.0),
        theme::text_dim(),
    );
}
