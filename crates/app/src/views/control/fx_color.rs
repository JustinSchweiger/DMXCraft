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
pub(super) const LABEL_W: f32 = 112.0;
/// Breite der Wertanzeige vor einem Regler.
const VALUE_W: f32 = 38.0;
pub(super) const ROW_H: f32 = 24.0;

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
            let res = Dialog::new(t("Save palette"))
                .id("palette_save")
                .cancel(t("Cancel"))
                .primary(t("Save"))
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
            let mut list = self.user().clone();
            let before = list.clone();
            let mut delete = None;
            let mut rename = self.rename.clone();
            let mut start_rename = None;
            let mut commit_rename = false;
            let res = Dialog::new(t("Manage palettes"))
                .id("palette_manage")
                .resizable([520.0, 460.0])
                .secondary(t("Restore default palettes"))
                .cancel(t("Close"))
                .show(ctx, &mut open, |ui, _| {
                    ui.label(
                        egui::RichText::new(t(
                            "Double-click a name to rename it. Click a swatch to change its color, right-click to remove it.",
                        ))
                        .color(theme::text_dim()),
                    );
                    ui.add_space(theme::SP_S);
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .show(ui, |ui| {
                            for (i, p) in list.iter_mut().enumerate() {
                                ui.horizontal(|ui| {
                                    ui.set_min_height(30.0);
                                    let (nr, _) = ui.allocate_exact_size(
                                        Vec2::new(150.0, 26.0),
                                        Sense::hover(),
                                    );
                                    let mut child = ui.new_child(
                                        egui::UiBuilder::new()
                                            .max_rect(nr)
                                            .layout(egui::Layout::left_to_right(egui::Align::Center)),
                                    );
                                    match rename.as_mut().filter(|(r, _)| *r == i) {
                                        Some((_, name)) => {
                                            let r = child.add(text_field(name).desired_width(140.0));
                                            r.request_focus();
                                            if r.lost_focus() {
                                                commit_rename = true;
                                            }
                                        }
                                        None => {
                                            if child
                                                .add(
                                                    egui::Label::new(&p.name)
                                                        .truncate()
                                                        .sense(Sense::click()),
                                                )
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
                                                .on_hover_text(t("Delete palette"))
                                                .clicked()
                                            {
                                                delete = Some(i);
                                            }
                                            ui.with_layout(
                                                egui::Layout::left_to_right(egui::Align::Center),
                                                |ui| {
                                                    edit_colors(
                                                        ui,
                                                        ("manage", i),
                                                        &mut p.colors,
                                                        true,
                                                        18.0,
                                                    );
                                                },
                                            );
                                        },
                                    );
                                });
                            }
                        });
                });
            if let Some(i) = start_rename {
                rename = Some((i, list[i].name.clone()));
            }
            if commit_rename {
                if let Some((i, name)) = rename.take() {
                    if let Some(p) = list.get_mut(i).filter(|_| !name.trim().is_empty()) {
                        p.name = name.trim().to_string();
                    }
                }
            }
            if let Some(i) = delete {
                list.remove(i);
                rename = None;
            }
            if res.clicked(0) {
                // fehlende mitgelieferte Paletten wieder anhängen
                for b in palettes::builtin() {
                    if !list.iter().any(|p| p.name == b.name) {
                        list.push(b);
                    }
                }
            }
            self.rename = rename;
            if list != before {
                *self.user() = list;
                self.persist();
            }
            if !open || res.clicked(1) {
                self.manage = false;
                self.rename = None;
            }
        }
    }
}

/// Farbfelder bearbeiten: Klick = Farbwähler, Rechtsklick = entfernen,
/// „+“ = hinzufügen. `true`, wenn sich etwas geändert hat.
pub(super) fn edit_colors(
    ui: &mut egui::Ui,
    salt: impl std::hash::Hash + Copy + std::fmt::Debug,
    colors: &mut Vec<[u8; 3]>,
    enabled: bool,
    size: f32,
) -> bool {
    let before = colors.clone();
    ui.spacing_mut().item_spacing.x = 3.0;
    let n = colors.len();
    let mut remove = None;
    for (i, c) in colors.iter_mut().enumerate() {
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
                if resp.hovered() && enabled {
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
            t("Right-click: remove")
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
        colors.remove(i);
    }
    if ui
        .add_enabled(
            enabled && colors.len() < 16,
            egui::Button::new(egui::RichText::new("+").color(theme::text_dim()))
                .frame(false)
                .min_size(Vec2::splat(size)),
        )
        .on_hover_text(t("Add color"))
        .clicked()
    {
        let last = colors.last().copied().unwrap_or([255, 255, 255]);
        colors.push(last);
    }
    *colors != before
}

/// Zeigt die Einstellungen; `true`, wenn sich etwas geändert hat.
#[allow(clippy::too_many_arguments)]
pub fn color_fx_ui(
    ui: &mut egui::Ui,
    salt: u32,
    fx: &mut ColorFx,
    preview: &[[u8; 3]],
    accent: Color32,
    enabled: bool,
    store: &mut PaletteStore,
    values: bool,
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
                        let keep = fx.palette.clone();
                        fx.set_preset(p);
                        if values {
                            // VALUE FX behält seine Grauwerte
                            fx.palette = keep;
                        }
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
    // Zeile mit Paletten-Menü, darunter die Farbfelder über die volle Breite
    label_row(ui, if values { t("Values") } else { t("Palette") }, |ui| {
        palette_editor(ui, &mut fx.palette, enabled, store);
    });
    ui.horizontal_wrapped(|ui| {
        edit_colors(ui, ("fx", salt), &mut fx.palette, enabled, 18.0);
    });
    ui.add_space(theme::SP_XS);
    if !values {
        toggle_row(ui, t("Grayscale"), &mut fx.grayscale, enabled);
    }
    label_row(ui, t("Transform"), |ui| {
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
            s(ui, "width", t("Color width"), &mut fx.width, 0.0, 100.0, 1);
            s(ui, "angle", t("Angle"), &mut fx.angle, 0.0, 360.0, 0);
            s(ui, "blend", t("Blend"), &mut fx.blend, 0.0, 100.0, 1);
        }
        ColorPreset::Burst => {
            s(ui, "width", t("Color width"), &mut fx.width, 0.0, 500.0, 0);
            s(ui, "blend", t("Blend"), &mut fx.blend, 0.0, 100.0, 1);
        }
        ColorPreset::Plasma => {
            s(
                ui,
                "sx",
                t("Size X axis"),
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
                t("Size Y axis"),
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
                t("Speed X axis"),
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
                t("Speed Y axis"),
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
            s(ui, "size", t("Size"), &mut fx.size, 1.0, 32.0, 0);
            toggle_row(ui, t("One direction only"), &mut fx.one_direction, enabled);
            toggle_row(ui, t("Crossfade"), &mut fx.crossfade, enabled);
            toggle_row(ui, t("Fade outward"), &mut fx.fade_outward, enabled);
            slider_row(
                ui,
                id("trail"),
                t("Trail"),
                &mut fx.trail,
                (0.0, 100.0),
                0,
                enabled,
                accent,
            );
        }
        ColorPreset::Sweep => {
            toggle_row(ui, t("Alternate direction"), &mut fx.alternate, enabled);
        }
        ColorPreset::Sparkle => {
            s(
                ui,
                "count",
                t("Sparkle count"),
                &mut fx.sparkle_count,
                1.0,
                50.0,
                0,
            );
            s(
                ui,
                "life",
                t("Sparkle lifetime"),
                &mut fx.sparkle_life,
                0.0,
                1.0,
                1,
            );
            s(
                ui,
                "sw",
                t("Sparkle width"),
                &mut fx.sparkle_width,
                1.0,
                16.0,
                0,
            );
        }
        ColorPreset::RandomFill => {
            s(ui, "dot", t("Dot width"), &mut fx.dot_width, 1.0, 16.0, 0);
        }
        ColorPreset::Perlin => {
            s(ui, "oct", t("Octaves"), &mut fx.octaves, 1.0, 8.0, 0);
            s(ui, "zoom", t("Zoom"), &mut fx.zoom, 0.0, 100.0, 0);
            s(ui, "dir", t("Direction"), &mut fx.direction, 0.0, 7.0, 0);
            s(ui, "tempo", t("Tempo"), &mut fx.tempo, 0.0, 10.0, 0);
            s(ui, "amp", t("Amplitude"), &mut fx.amplitude, 0.0, 100.0, 0);
        }
    }
    *fx != before
}

/// Zeile: Beschriftung links, Inhalt rechts daneben.
pub(super) fn label_row(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
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

pub(super) fn toggle_row(ui: &mut egui::Ui, label: &str, v: &mut bool, enabled: bool) {
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
pub(super) fn slider_row(
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
pub(super) fn palette_editor(
    ui: &mut egui::Ui,
    palette: &mut Vec<[u8; 3]>,
    enabled: bool,
    store: &mut PaletteStore,
) {
    let small = |ui: &mut egui::Ui, text: &str, tip: &str| {
        ui.add_enabled(
            enabled,
            egui::Button::new(egui::RichText::new(text).color(theme::text_dim()))
                .frame(false)
                .min_size(Vec2::new(18.0, 18.0)),
        )
        .on_hover_text(tip)
    };
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let more = small(ui, "•••", t("Palettes"));
        crate::ui::widgets::menu::dropdown(&more, |ui| {
            use crate::ui::widgets::menu::{self, MenuItem};
            menu::begin(ui);
            ui.set_max_width(220.0);
            let mut pick = None;
            let user = store.user().clone();
            for p in user.iter() {
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
            if menu::item(ui, MenuItem::new(t("New"))) {
                pick = Some(vec![[0, 0, 0], [255, 255, 255]]);
            }
            if menu::item(ui, MenuItem::new(t("Save…"))) {
                store.save = Some((String::new(), palette.clone()));
            }
            if menu::item(ui, MenuItem::new(t("Manage…"))) {
                store.manage = true;
            }
            if let Some(colors) = pick {
                *palette = colors;
            }
        });
    });
}

/// Farbstreifen mit gleich breiten Feldern.
pub(super) fn paint_strip(p: &egui::Painter, r: Rect, colors: &[[u8; 3]]) {
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
