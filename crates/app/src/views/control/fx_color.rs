//! Einstellungen eines COLOR FX in der Seitenleiste „Effekte“: Vorlage,
//! Vorschau der Beams, Farbpalette (mit gespeicherten Paletten), Graustufen,
//! Umformung und die Regler der jeweiligen Vorlage.

use crate::i18n::t;
use dmxcraft_core::fx::{ColorFx, ColorPreset, Transform};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::palettes::{self, NamedPalette};
use crate::ui::theme;
use crate::ui::widgets::dialog::text_field;
use crate::ui::widgets::{Dialog, Toggle};

/// Breite der Beschriftungsspalte.
const LABEL_W: f32 = 112.0;
/// Breite der Wertanzeige vor einem Regler.
const VALUE_W: f32 = 38.0;
const ROW_H: f32 = 24.0;

/// Eigene Paletten und die zugehörigen Fenster.
#[derive(Default)]
pub struct PaletteStore {
    user: Option<Vec<NamedPalette>>,
    /// „Speichern…“ offen: Name und zu speichernde Farben.
    save: Option<(String, Vec<[u8; 3]>)>,
    /// „Verwalten…“ offen.
    manage: bool,
    /// Umbenennen in „Verwalten…“: Index und neuer Name.
    rename: Option<(usize, String)>,
    pub error: Option<String>,
}

impl PaletteStore {
    fn user(&mut self) -> &mut Vec<NamedPalette> {
        self.user.get_or_insert_with(palettes::load_user)
    }

    fn persist(&mut self) {
        if let Err(e) = palettes::save_user(self.user()) {
            self.error = Some(e);
        }
    }

    /// Fenster „Palette speichern“ und „Paletten verwalten“.
    pub fn windows(&mut self, ctx: &egui::Context) {
        if let Some((name, colors)) = self.save.as_mut() {
            let mut open = true;
            let mut name_now = name.clone();
            let res = Dialog::new(t("Palette speichern"))
                .id("palette_save")
                .cancel(t("Abbrechen"))
                .primary(t("Speichern"))
                .show(ctx, &mut open, |ui, first| {
                    ui.horizontal(|ui| {
                        ui.add_sized(
                            [crate::ui::widgets::dialog::LABEL_W, 28.0],
                            egui::Label::new(t("Name")),
                        );
                        let r = ui.add(text_field(&mut name_now).desired_width(220.0));
                        if first {
                            r.request_focus();
                        }
                    });
                });
            *name = name_now;
            let colors = colors.clone();
            if res.clicked(1) && !name.trim().is_empty() {
                let name = name.trim().to_string();
                let list = self.user();
                match list.iter_mut().find(|p| p.name == name) {
                    Some(p) => p.colors = colors,
                    None => list.push(NamedPalette { name, colors }),
                }
                self.persist();
                self.save = None;
            } else if !open || res.clicked(0) {
                self.save = None;
            }
        }
        if self.manage {
            let mut open = true;
            let mut delete = None;
            let mut start_rename = None;
            let mut rename = self.rename.clone();
            let mut commit_rename = false;
            let list = self.user().clone();
            Dialog::new(t("Paletten verwalten"))
                .id("palette_manage")
                .width(380.0)
                .cancel(t("Schließen"))
                .show(ctx, &mut open, |ui, _| {
                    if list.is_empty() {
                        ui.label(
                            egui::RichText::new(t("Noch keine eigenen Paletten gespeichert."))
                                .color(theme::text_dim()),
                        );
                    }
                    for (i, p) in list.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.set_min_height(30.0);
                            match rename.as_mut().filter(|(r, _)| *r == i) {
                                Some((_, name)) => {
                                    let r = ui.add(text_field(name).desired_width(160.0));
                                    r.request_focus();
                                    if r.lost_focus() {
                                        commit_rename = true;
                                    }
                                }
                                None => {
                                    if ui
                                        .add(egui::Label::new(&p.name).sense(Sense::click()))
                                        .on_hover_text(t("Doppelklick zum Umbenennen"))
                                        .double_clicked()
                                    {
                                        start_rename = Some(i);
                                    }
                                }
                            }
                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    if ui
                                        .add(
                                            crate::ui::widgets::IconButton::new(
                                                crate::ui::widgets::Icon::Trash,
                                            )
                                            .flat(),
                                        )
                                        .on_hover_text(t("Palette löschen"))
                                        .clicked()
                                    {
                                        delete = Some(i);
                                    }
                                    strip(ui, &p.colors, Vec2::new(120.0, 14.0));
                                },
                            );
                        });
                    }
                });
            if let Some(i) = start_rename {
                rename = Some((i, list[i].name.clone()));
            }
            if commit_rename {
                if let Some((i, name)) = rename.take() {
                    if !name.trim().is_empty() {
                        if let Some(p) = self.user().get_mut(i) {
                            p.name = name.trim().to_string();
                        }
                        self.persist();
                    }
                }
            }
            self.rename = rename;
            if let Some(i) = delete {
                self.user().remove(i);
                self.persist();
            }
            if !open {
                self.manage = false;
                self.rename = None;
            }
        }
    }
}

/// Zeigt die Einstellungen; `true`, wenn sich etwas geändert hat.
pub fn color_fx_ui(
    ui: &mut egui::Ui,
    salt: u32,
    fx: &mut ColorFx,
    preview: &[[u8; 3]],
    accent: Color32,
    enabled: bool,
    store: &mut PaletteStore,
) -> bool {
    let before = fx.clone();
    let id = |s: &str| egui::Id::unique(("color_fx", salt, s.to_owned()));
    ui.spacing_mut().item_spacing.y = 2.0;

    // Vorlage
    ui.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt(id("preset"))
            .selected_text(fx.preset.label())
            .width(ui.available_width() - 4.0)
            .show_ui(ui, |ui| {
                for p in ColorPreset::ALL {
                    if ui.selectable_label(fx.preset == p, p.label()).clicked() && fx.preset != p {
                        fx.set_preset(p);
                    }
                }
            });
    });
    ui.add_space(theme::SP_XS);
    // Vorschau: aktuelle Farbe jedes Beams
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 22.0), Sense::hover());
    paint_strip(ui.painter(), r, preview);
    ui.add_space(theme::SP_XS);

    // Farbpalette
    label_row(ui, t("Farbpalette"), |ui| {
        palette_editor(ui, salt, fx, enabled, store);
    });
    toggle_row(ui, t("Graustufen"), &mut fx.grayscale, enabled);
    label_row(ui, t("Transformieren"), |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            egui::ComboBox::from_id_salt(id("transform"))
                .selected_text(t(fx.transform.label()))
                .width(ui.available_width() - 4.0)
                .show_ui(ui, |ui| {
                    for tr in Transform::ALL {
                        ui.selectable_value(&mut fx.transform, tr, t(tr.label()));
                    }
                });
        });
    });

    // Regler der Vorlage
    let s =
        |ui: &mut egui::Ui, key: &str, label: &str, v: &mut f32, min: f32, max: f32, dec: usize| {
            slider_row(ui, id(key), label, v, (min, max), dec, enabled, accent);
        };
    match fx.preset {
        ColorPreset::Rainbow => {
            s(ui, "width", t("Farbenbreite"), &mut fx.width, 0.0, 100.0, 1);
            s(ui, "angle", t("Winkel"), &mut fx.angle, 0.0, 360.0, 0);
            s(ui, "blend", t("Verlauf"), &mut fx.blend, 0.0, 100.0, 1);
        }
        ColorPreset::Burst => {
            s(ui, "width", t("Farbenbreite"), &mut fx.width, 0.0, 100.0, 0);
            s(ui, "blend", t("Verlauf"), &mut fx.blend, 0.0, 100.0, 1);
        }
        ColorPreset::Plasma => {
            s(
                ui,
                "sx",
                t("Größe X-Achse"),
                &mut fx.plasma_size[0],
                0.0,
                10.0,
                0,
            );
            s(
                ui,
                "px",
                t("Parameter"),
                &mut fx.plasma_param[0],
                0.0,
                10.0,
                0,
            );
            s(
                ui,
                "sy",
                t("Größe Y-Achse"),
                &mut fx.plasma_size[1],
                0.0,
                10.0,
                0,
            );
            s(
                ui,
                "py",
                t("Parameter"),
                &mut fx.plasma_param[1],
                0.0,
                10.0,
                0,
            );
            s(
                ui,
                "vx",
                t("Speed X-Achse"),
                &mut fx.plasma_speed[0],
                -5.0,
                5.0,
                0,
            );
            s(
                ui,
                "vpx",
                t("Parameter"),
                &mut fx.plasma_speed_param[0],
                -5.0,
                5.0,
                0,
            );
            s(
                ui,
                "vy",
                t("Speed Y-Achse"),
                &mut fx.plasma_speed[1],
                -5.0,
                5.0,
                0,
            );
            s(
                ui,
                "vpy",
                t("Parameter"),
                &mut fx.plasma_speed_param[1],
                -5.0,
                5.0,
                0,
            );
        }
        ColorPreset::KnightRider => {
            s(ui, "size", t("Größe"), &mut fx.size, 1.0, 32.0, 0);
            toggle_row(ui, t("Nur eine Richtung"), &mut fx.one_direction, enabled);
            toggle_row(ui, t("Überblendung"), &mut fx.crossfade, enabled);
            toggle_row(ui, t("Verlauf nach außen"), &mut fx.fade_outward, enabled);
            slider_row(
                ui,
                id("trail"),
                t("Nachleuchten"),
                &mut fx.trail,
                (0.0, 100.0),
                0,
                enabled,
                accent,
            );
        }
        ColorPreset::Sweep => {
            slider_row(
                ui,
                id("angle"),
                t("Winkel"),
                &mut fx.angle,
                (0.0, 360.0),
                0,
                enabled,
                accent,
            );
            toggle_row(ui, t("Richtung ändern"), &mut fx.alternate, enabled);
        }
        ColorPreset::Sparkle => {
            s(
                ui,
                "count",
                t("Glitzeranzahl"),
                &mut fx.sparkle_count,
                1.0,
                50.0,
                0,
            );
            s(
                ui,
                "life",
                t("Glitzer-Lebenszeit"),
                &mut fx.sparkle_life,
                0.0,
                1.0,
                1,
            );
            s(
                ui,
                "sw",
                t("Glitzerbreite"),
                &mut fx.sparkle_width,
                1.0,
                16.0,
                0,
            );
        }
        ColorPreset::RandomFill => {
            s(ui, "dot", t("Punktbreite"), &mut fx.dot_width, 1.0, 16.0, 0);
        }
        ColorPreset::Perlin => {
            s(ui, "oct", t("Oktaven"), &mut fx.octaves, 1.0, 8.0, 0);
            s(ui, "zoom", t("Zoom"), &mut fx.zoom, 0.0, 100.0, 0);
            s(ui, "dir", t("Richtung"), &mut fx.direction, 0.0, 7.0, 0);
            s(ui, "tempo", t("Tempo"), &mut fx.tempo, 0.0, 10.0, 0);
            s(ui, "amp", t("Amplitude"), &mut fx.amplitude, 0.0, 100.0, 0);
        }
    }
    *fx != before
}

/// Zeile: Beschriftung links, Inhalt rechts daneben.
fn label_row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_min_height(ROW_H);
        let (r, _) = ui.allocate_exact_size(Vec2::new(LABEL_W, ROW_H), Sense::hover());
        ui.painter().text(
            pos2(r.left(), r.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::body(),
            theme::text_dim(),
        );
        add(ui);
    });
}

fn toggle_row(ui: &mut egui::Ui, label: &str, v: &mut bool, enabled: bool) {
    ui.horizontal(|ui| {
        ui.set_min_height(ROW_H);
        ui.label(egui::RichText::new(label).color(theme::text_dim()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add(Toggle::new(v, "").enabled(enabled));
        });
    });
}

/// Regler mit Wertanzeige; ganzzahlig bei `dec == 0`.
#[allow(clippy::too_many_arguments)]
fn slider_row(
    ui: &mut egui::Ui,
    id: egui::Id,
    label: &str,
    v: &mut f32,
    (min, max): (f32, f32),
    dec: usize,
    enabled: bool,
    accent: Color32,
) {
    label_row(ui, label, |ui| {
        let (vr, _) = ui.allocate_exact_size(Vec2::new(VALUE_W, ROW_H), Sense::hover());
        ui.painter().text(
            pos2(vr.left(), vr.center().y),
            egui::Align2::LEFT_CENTER,
            format!("{:.*}", dec, *v),
            theme::caption(),
            if enabled {
                theme::text()
            } else {
                theme::text_faint()
            },
        );
        let w = (ui.available_width() - 6.0).max(40.0);
        let (rect, _) = ui.allocate_exact_size(Vec2::new(w, ROW_H), Sense::hover());
        let sense = if enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        };
        let resp = ui.interact(rect, id, sense);
        let track = Rect::from_center_size(rect.center(), Vec2::new(rect.width() - 10.0, 3.0));
        if resp.dragged() || resp.clicked() {
            if let Some(p) = resp.interact_pointer_pos() {
                let f = ((p.x - track.left()) / track.width()).clamp(0.0, 1.0);
                let mut nv = min + f * (max - min);
                let k = 10f32.powi(dec as i32);
                nv = (nv * k).round() / k;
                *v = nv;
            }
        }
        if resp.double_clicked() {
            // Doppelklick: Mitte bzw. Minimum
            *v = if min < 0.0 { 0.0 } else { min };
        }
        let f = ((*v - min) / (max - min)).clamp(0.0, 1.0);
        let p = ui.painter();
        p.rect_filled(track, 1.5, theme::bg());
        let filled = Rect::from_min_max(
            track.min,
            pos2(track.left() + track.width() * f, track.bottom()),
        );
        p.rect_filled(
            filled,
            1.5,
            if enabled {
                accent.gamma_multiply(0.7)
            } else {
                theme::border()
            },
        );
        let hx = track.left() + track.width() * f;
        let handle = Rect::from_center_size(pos2(hx, rect.center().y), Vec2::new(8.0, 15.0));
        let hot = resp.hovered() || resp.dragged();
        p.rect_filled(handle, 2.0, theme::raised_hover());
        p.rect_stroke(
            handle,
            2.0,
            Stroke::new(
                1.0,
                if hot && enabled {
                    theme::text()
                } else {
                    theme::text_dim()
                },
            ),
            StrokeKind::Inside,
        );
    });
}

/// Farbfelder der Palette: Klick = Farbe wählen, Rechtsklick = entfernen,
/// „+“ = Farbe hinzufügen, „…“ = gespeicherte Paletten.
fn palette_editor(
    ui: &mut egui::Ui,
    salt: u32,
    fx: &mut ColorFx,
    enabled: bool,
    store: &mut PaletteStore,
) {
    ui.spacing_mut().item_spacing.x = 2.0;
    let mut remove = None;
    let n = fx.palette.len();
    let avail = ui.available_width() - 44.0;
    let size = (avail / (n.max(1) as f32) - 2.0).clamp(8.0, 16.0);
    for (i, c) in fx.palette.iter_mut().enumerate() {
        let (r, resp) = ui.allocate_exact_size(
            Vec2::splat(size),
            if enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );
        let p = ui.painter();
        p.rect_filled(r, 2.0, theme::rgb(*c));
        p.rect_stroke(
            r,
            2.0,
            Stroke::new(
                1.0,
                if resp.hovered() {
                    Color32::WHITE
                } else {
                    theme::border()
                },
            ),
            StrokeKind::Inside,
        );
        let resp = resp.on_hover_text(format!(
            "#{:02X}{:02X}{:02X} · {}",
            c[0],
            c[1],
            c[2],
            t("Rechtsklick: entfernen")
        ));
        crate::ui::widgets::menu::dropdown(&resp, |ui| {
            ui.set_min_width(212.0);
            crate::ui::widgets::color_picker::color_picker(
                ui,
                ui.make_persistent_id(("palette_color", salt, i)),
                c,
            );
        });
        if resp.secondary_clicked() && n > 1 {
            remove = Some(i);
        }
    }
    if let Some(i) = remove {
        fx.palette.remove(i);
    }
    let small = |ui: &mut egui::Ui, text: &str, tip: &str| {
        ui.add_enabled(
            enabled,
            egui::Button::new(egui::RichText::new(text).color(theme::text_dim()))
                .frame(false)
                .min_size(Vec2::new(16.0, 16.0)),
        )
        .on_hover_text(tip)
    };
    if small(ui, "+", t("Farbe hinzufügen")).clicked() && fx.palette.len() < 16 {
        let last = fx.palette.last().copied().unwrap_or([255, 255, 255]);
        fx.palette.push(last);
    }
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let more = small(ui, "•••", t("Paletten"));
        crate::ui::widgets::menu::dropdown(&more, |ui| {
            use crate::ui::widgets::menu::{self, MenuItem};
            menu::begin(ui);
            ui.set_max_width(220.0);
            let mut pick = None;
            let user = store.user().clone();
            for p in palettes::builtin().iter().chain(user.iter()) {
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(220.0, 34.0), Sense::click());
                let resp = resp.on_hover_cursor(egui::CursorIcon::PointingHand);
                let r = rect.shrink2(Vec2::new(8.0, 3.0));
                let painter = ui.painter();
                if resp.hovered() {
                    painter.rect_filled(rect, 2.0, theme::raised_hover());
                }
                painter.text(
                    r.left_top(),
                    egui::Align2::LEFT_TOP,
                    &p.name,
                    theme::caption(),
                    theme::text(),
                );
                let sr = Rect::from_min_max(pos2(r.left(), r.bottom() - 10.0), r.max);
                paint_strip(painter, sr, &p.colors);
                if resp.clicked() {
                    pick = Some(p.colors.clone());
                    ui.close();
                }
            }
            menu::separator(ui);
            if menu::item(ui, MenuItem::new(t("Neu"))) {
                pick = Some(vec![[0, 0, 0], [255, 255, 255]]);
            }
            if menu::item(ui, MenuItem::new(t("Speichern…"))) {
                store.save = Some((String::new(), fx.palette.clone()));
            }
            if menu::item(ui, MenuItem::new(t("Verwalten…"))) {
                store.manage = true;
            }
            if let Some(colors) = pick {
                fx.palette = colors;
            }
        });
    });
}

/// Farbstreifen mit gleich breiten Feldern.
fn paint_strip(p: &egui::Painter, r: Rect, colors: &[[u8; 3]]) {
    p.rect_filled(r, 2.0, theme::bg());
    if colors.is_empty() {
        return;
    }
    let w = r.width() / colors.len() as f32;
    for (i, c) in colors.iter().enumerate() {
        let cell = Rect::from_min_size(
            pos2(r.left() + i as f32 * w, r.top()),
            Vec2::new(w + 0.5, r.height()),
        );
        p.rect_filled(cell, 0.0, theme::rgb(*c));
    }
    p.rect_stroke(
        r,
        2.0,
        Stroke::new(1.0, theme::border()),
        StrokeKind::Inside,
    );
}

fn strip(ui: &mut egui::Ui, colors: &[[u8; 3]], size: Vec2) {
    let (r, _) = ui.allocate_exact_size(size, Sense::hover());
    paint_strip(ui.painter(), r, colors);
}
