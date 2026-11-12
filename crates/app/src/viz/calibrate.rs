//! Calibrating a moving head (build mode, one moving head selected): pick a
//! mark in the 3D view that you can also find in the real room (stage
//! corner, edge of a table …), steer the real head with pan/tilt until its
//! beam hits the mark, save the point. From 3–4 points DMXCraft computes
//! where the head hangs and how it is turned, so the 3D venue matches the
//! room.

use dmxcraft_core::calibrate::{CalPoint, Calibration};
use dmxcraft_core::{ChannelKind, ChannelRef, Engine, FixtureId};
use eframe::egui::{self, vec2, Stroke};
use glam::{Mat4, Vec3};

use super::props::button;
use super::{apply_placement, project, Pick, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{fold_section, Toggle};
use crate::views::ViewCtx;

/// State of a calibration (not saved).
pub struct Calibrate {
    pub fixture: Option<FixtureId>,
    pub points: Vec<CalPoint>,
    /// Mark picked in the view, waiting for the beam.
    pub mark: Option<[f32; 3]>,
    /// The next click in the view sets the mark.
    pub picking: bool,
    /// Pan/tilt (programmer values) the head gets live.
    pub pan: u16,
    pub tilt: u16,
    /// Light the head (dimmer, white, shutter open) while calibrating.
    pub light: bool,
    /// Also compute the position (otherwise only the rotation).
    pub with_position: bool,
    pub result: Option<Calibration>,
    /// Live channels set for the head (to release them).
    live: Vec<ChannelRef>,
}

impl Default for Calibrate {
    fn default() -> Self {
        Self {
            fixture: None,
            points: Vec::new(),
            mark: None,
            picking: false,
            pan: 32768,
            tilt: 32768,
            light: true,
            with_position: true,
            result: None,
            live: Vec::new(),
        }
    }
}

impl Calibrate {
    pub fn active(&self) -> bool {
        self.fixture.is_some()
    }

    /// Send pan/tilt (and the light) of the head as live values.
    fn drive(&mut self, e: &mut Engine) {
        let Some(id) = self.fixture else {
            return;
        };
        let show = e.show();
        let Some(ty) = show.fixture(id).and_then(|f| show.type_of(f)).cloned() else {
            return;
        };
        let mut set = Vec::new();
        for (i, def) in ty.channels.iter().enumerate() {
            let v = match def.kind {
                ChannelKind::Pan => Some((self.pan >> 8) as u8),
                ChannelKind::PanFine => Some((self.pan & 0xFF) as u8),
                ChannelKind::Tilt => Some((self.tilt >> 8) as u8),
                ChannelKind::TiltFine => Some((self.tilt & 0xFF) as u8),
                _ if !self.light => None,
                ChannelKind::Dimmer => Some(255),
                ChannelKind::Red | ChannelKind::Green | ChannelKind::Blue | ChannelKind::White => {
                    Some(255)
                }
                ChannelKind::Strobe => Some(def.open_value()),
                // narrow beam: easier to aim exactly
                ChannelKind::Zoom => Some(0),
                _ => None,
            };
            if let Some(v) = v {
                set.push((
                    ChannelRef {
                        fixture: id,
                        channel: i as u16,
                    },
                    v,
                ));
            }
        }
        if ty.needs_virtual_dimmer() && self.light {
            set.push((ChannelRef::virtual_dimmer(id), 255));
        }
        for ch in std::mem::take(&mut self.live) {
            if !set.iter().any(|(c, _)| *c == ch) {
                e.clear_live_channel(ch);
            }
        }
        for (ch, v) in &set {
            e.set_live(*ch, *v);
        }
        self.live = set.into_iter().map(|(c, _)| c).collect();
    }

    /// End: the head goes back to the scenes.
    pub fn stop(&mut self, e: &mut Engine) {
        for ch in std::mem::take(&mut self.live) {
            e.clear_live_channel(ch);
        }
        *self = Self {
            light: self.light,
            with_position: self.with_position,
            ..Self::default()
        };
    }

    /// The mark was picked: point the head roughly at it (with the current
    /// placement), the rest is fine-tuning.
    pub fn set_mark(&mut self, e: &mut Engine, mark: [f32; 3]) {
        self.mark = Some(mark);
        self.picking = false;
        if let Some(id) = self.fixture {
            if let Some((p, t)) =
                e.show()
                    .aim_moving_head_near(id, mark, Some((self.pan, self.tilt)))
            {
                self.pan = p;
                self.tilt = t;
            }
        }
        self.drive(e);
    }

    /// Marks and the beam of the head in the view.
    pub fn paint(&self, painter: &egui::Painter, vp: Mat4, rect: egui::Rect, e: &Engine) {
        if !self.active() {
            return;
        }
        let cross = |p: Vec3, col: egui::Color32, label: String| {
            if let Some(c) = project(vp, rect, p) {
                let r = 7.0;
                painter.circle_stroke(c, r, Stroke::new(2.0, col));
                painter.line_segment(
                    [c - vec2(r + 4.0, 0.0), c + vec2(r + 4.0, 0.0)],
                    Stroke::new(1.5, col),
                );
                painter.line_segment(
                    [c - vec2(0.0, r + 4.0), c + vec2(0.0, r + 4.0)],
                    Stroke::new(1.5, col),
                );
                painter.text(
                    c + vec2(r + 5.0, -r - 2.0),
                    egui::Align2::LEFT_BOTTOM,
                    label,
                    theme::caption(),
                    col,
                );
            }
        };
        for (i, p) in self.points.iter().enumerate() {
            cross(Vec3::from(p.target), theme::OK, format!("{}", i + 1));
        }
        if let Some(m) = self.mark {
            cross(Vec3::from(m), theme::accent(), t("Mark").to_string());
        }
        if self.picking {
            painter.text(
                rect.center_top() + vec2(0.0, 12.0),
                egui::Align2::CENTER_TOP,
                t("Click a mark you can also find in the room"),
                theme::body(),
                theme::accent(),
            );
        }
        let _ = e;
    }
}

impl VizView {
    /// Section in the properties of a single selected moving head.
    pub(super) fn calibrate_section(
        &mut self,
        ui: &mut egui::Ui,
        cx: &mut ViewCtx<'_>,
        id: FixtureId,
    ) {
        fold_section(ui, "viz_f_calibrate", t("Calibrate"), false, |ui| {
            let cal = &mut self.calibrate;
            if cal.fixture != Some(id) {
                ui.label(
                    egui::RichText::new(t(
                        "Point the real head at 3–4 marks you can find in the room and in the 3D view – DMXCraft then computes where the head hangs and how it is turned.",
                    ))
                    .color(theme::text_dim()),
                );
                ui.add_space(theme::SP_XS);
                if button(ui, t("Start calibration"), true).clicked() {
                    cal.stop(cx.engine);
                    cal.fixture = Some(id);
                    let p = cx.engine.show().placement3d(id);
                    // start straight down from the current placement
                    let below = [p.position[0], 0.0, p.position[2]];
                    if let Some((pa, ti)) = cx.engine.show().aim_moving_head(id, below) {
                        cal.pan = pa;
                        cal.tilt = ti;
                    }
                    cal.picking = true;
                    cal.drive(cx.engine);
                }
                return;
            }
            // 1. mark
            ui.horizontal(|ui| {
                let label = match cal.mark {
                    Some(m) => format!("{} {:.2} / {:.2} / {:.2} m", t("Mark"), m[0], m[1], m[2]),
                    None => t("No mark yet").to_string(),
                };
                ui.label(label);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if button(ui, t("Pick mark"), true).clicked() {
                        cal.picking = !cal.picking;
                    }
                });
            });
            // 2. beam onto the mark
            let mut changed = false;
            for (label, v) in [(t("Pan"), &mut cal.pan), (t("Tilt"), &mut cal.tilt)] {
                ui.horizontal(|ui| {
                    ui.add_sized(vec2(36.0, 20.0), egui::Label::new(label));
                    for (txt, d) in [("−−", -256i32), ("−", -16)] {
                        if ui.small_button(txt).clicked() {
                            *v = (i32::from(*v) + d).clamp(0, 65535) as u16;
                            changed = true;
                        }
                    }
                    let w = (ui.available_width() - 70.0).max(60.0);
                    ui.spacing_mut().slider_width = w;
                    changed |= ui
                        .add(egui::Slider::new(v, 0..=65535).show_value(false))
                        .changed();
                    for (txt, d) in [("+", 16i32), ("++", 256)] {
                        if ui.small_button(txt).clicked() {
                            *v = (i32::from(*v) + d).clamp(0, 65535) as u16;
                            changed = true;
                        }
                    }
                });
            }
            ui.horizontal(|ui| {
                changed |= ui
                    .add(Toggle::new(&mut cal.light, t("Light the head")))
                    .changed();
            });
            if changed {
                cal.drive(cx.engine);
            }
            ui.add_space(theme::SP_XS);
            if button(
                ui,
                t("Beam is on the mark – save point"),
                cal.mark.is_some(),
            )
            .clicked()
            {
                if let Some(m) = cal.mark.take() {
                    cal.points.push(CalPoint {
                        target: m,
                        pan: cal.pan,
                        tilt: cal.tilt,
                    });
                    cal.result = None;
                    cal.picking = true;
                }
            }
            // 3. points and result
            ui.add_space(theme::SP_S);
            let mut remove = None;
            for (i, p) in cal.points.iter().enumerate() {
                ui.horizontal(|ui| {
                    let err = cal
                        .result
                        .as_ref()
                        .and_then(|r| r.errors.get(i))
                        .map(|e| format!(" · {e:.1}°"))
                        .unwrap_or_default();
                    ui.label(
                        egui::RichText::new(format!(
                            "{}  {:.2} / {:.2} / {:.2} m{err}",
                            i + 1,
                            p.target[0],
                            p.target[1],
                            p.target[2]
                        ))
                        .font(theme::caption()),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("×").clicked() {
                            remove = Some(i);
                        }
                    });
                });
            }
            if let Some(i) = remove {
                cal.points.remove(i);
                cal.result = None;
            }
            ui.horizontal(|ui| {
                ui.add(Toggle::new(&mut cal.with_position, t("Also position")))
                    .on_hover_text(t(
                        "Off: only the rotation (position as measured) – needs 2 points instead of 3",
                    ));
            });
            let need = if cal.with_position { 3 } else { 2 };
            ui.horizontal_wrapped(|ui| {
                if button(ui, t("Compute"), cal.points.len() >= need).clicked() {
                    cal.result =
                        cx.engine
                            .show()
                            .calibrate_fixture(id, &cal.points, cal.with_position);
                    if cal.result.is_none() {
                        cx.state.error(t("The points do not fit a placement"));
                    }
                }
                if let Some(r) = cal.result.clone() {
                    if button(ui, t("Apply"), true).clicked() {
                        apply_placement(cx, Pick::Fixture(id), r.placement);
                        cx.state.info(tr!(
                            "Placement corrected (deviation {e}°)",
                            e = format!("{:.1}", r.rms)
                        ));
                    }
                }
                if button(ui, t("End"), true).clicked() {
                    cal.stop(cx.engine);
                }
            });
            if cal.points.len() < need {
                ui.label(
                    egui::RichText::new(tr!("{n} more points needed", n = need - cal.points.len()))
                        .font(theme::caption())
                        .color(theme::text_dim()),
                );
            }
            if let Some(r) = &cal.result {
                let p = r.placement;
                let col = if r.rms < 1.0 {
                    theme::OK
                } else if r.rms < 3.0 {
                    theme::WARN
                } else {
                    theme::DANGER
                };
                ui.label(
                    egui::RichText::new(tr!("Deviation {e}°", e = format!("{:.2}", r.rms)))
                        .color(col),
                );
                ui.label(
                    egui::RichText::new(format!(
                        "{} {:.2} / {:.2} / {:.2} m · {} {:.1} / {:.1} / {:.1}°",
                        t("Position"),
                        p.position[0],
                        p.position[1],
                        p.position[2],
                        t("Rotation"),
                        p.rotation[0],
                        p.rotation[1],
                        p.rotation[2]
                    ))
                    .font(theme::caption())
                    .color(theme::text_dim()),
                );
            }
        });
    }
}
