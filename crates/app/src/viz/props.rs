//! Right bar in build mode: properties of the selection. Changes apply to
//! every selected object or fixture: dragged positions move all of them by
//! the same amount, typed values and all other fields are set on all.

use std::ops::RangeInclusive;

use dmxcraft_core::venue::{Object, Placement, Shape, TrussProfile, Venue};
use dmxcraft_core::{ChannelKind, ChannelRef, FixtureId};
use eframe::egui::{self, vec2, Align2, Color32, Sense, Stroke, StrokeKind};

use super::panels::{apply_texture, TexAction};
use super::{apply_placement, renumber, rig, targets, DialogKind, Pick, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::dialog::INPUT_H;
use crate::ui::widgets::{fold_section, form, menu, text_field, Icon, IconButton, Segmented};
use crate::views::quick::Layer;
use crate::views::ViewCtx;

/// Space for an icon button at the end of a row (keeps the fields aligned).
const RESERVE: f32 = INPUT_H + theme::SP_XS;

/// What changed in a vector field.
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct VecEdit {
    pub changed: [bool; 3],
    /// changed by dragging (→ move all by the same amount)
    pub dragged: bool,
}

impl VecEdit {
    fn any(&self) -> bool {
        self.changed.iter().any(|c| *c)
    }
}

/// Three number fields X/Y/Z filling the row (minus `reserve` on the right).
pub(super) fn vec3_field(
    ui: &mut egui::Ui,
    v: &mut [f32; 3],
    speed: f64,
    range: RangeInclusive<f32>,
    reserve: f32,
) -> VecEdit {
    let mut e = VecEdit::default();
    ui.spacing_mut().item_spacing.x = theme::SP_XS;
    let w = ((ui.available_width() - reserve - 2.0 * theme::SP_XS) / 3.0).max(48.0);
    for (i, c) in v.iter_mut().enumerate() {
        if *c == 0.0 {
            *c = 0.0; // no “-0.0”
        }
        let r = ui.add_sized(
            vec2(w, INPUT_H),
            egui::DragValue::new(c)
                .speed(speed)
                .range(range.clone())
                .max_decimals(3)
                .prefix(["X  ", "Y  ", "Z  "][i]),
        );
        if r.changed() {
            e.changed[i] = true;
            e.dragged |= r.dragged();
        }
    }
    e
}

/// Number field over the full width.
fn number(ui: &mut egui::Ui, v: &mut f32, speed: f64, range: RangeInclusive<f32>, suffix: &str) {
    ui.add_sized(
        vec2(ui.available_width() - RESERVE, INPUT_H),
        egui::DragValue::new(v)
            .speed(speed)
            .range(range)
            .max_decimals(2)
            .suffix(suffix.to_string()),
    );
}

/// Share 0–1 as percent over the full width.
fn percent(ui: &mut egui::Ui, v: &mut f32, min: f32) {
    let mut p = (*v * 100.0).round();
    let r = ui.add_sized(
        vec2(ui.available_width() - RESERVE, INPUT_H),
        egui::DragValue::new(&mut p)
            .speed(0.5)
            .range(min * 100.0..=100.0)
            .max_decimals(0)
            .suffix(" %"),
    );
    if r.changed() {
        *v = p / 100.0;
    }
}

/// Text button in app style (like the dialog buttons, row height).
pub(super) fn button(ui: &mut egui::Ui, label: &str, enabled: bool) -> egui::Response {
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_string(), theme::body(), theme::text());
    let w = galley.size().x + 2.0 * theme::SP_M;
    let sense = if enabled {
        Sense::click()
    } else {
        Sense::hover()
    };
    let (rect, resp) = ui.allocate_exact_size(vec2(w, INPUT_H), sense);
    let hovered = resp.hovered() && enabled;
    let (fill, stroke, text) = match (enabled, hovered) {
        (false, _) => (theme::raised(), theme::border(), theme::text_faint()),
        (true, false) => (theme::raised(), theme::border(), theme::text()),
        (true, true) => (theme::raised_hover(), theme::accent_dim(), theme::text()),
    };
    let p = ui.painter();
    p.rect(
        rect,
        theme::R_S as f32,
        fill,
        Stroke::new(1.0, stroke),
        StrokeKind::Inside,
    );
    p.text(
        rect.center(),
        Align2::CENTER_CENTER,
        label,
        theme::body(),
        text,
    );
    resp
}

/// Color field: swatch over the full width, click opens the color picker.
fn color_field(ui: &mut egui::Ui, id: egui::Id, rgb: &mut [u8; 3]) -> bool {
    let (rect, resp) = ui.allocate_exact_size(
        vec2(ui.available_width() - RESERVE, INPUT_H),
        Sense::click(),
    );
    let p = ui.painter();
    p.rect(
        rect,
        theme::R_S as f32,
        Color32::from_rgb(rgb[0], rgb[1], rgb[2]),
        Stroke::new(
            1.0,
            if resp.hovered() {
                theme::accent_dim()
            } else {
                theme::border()
            },
        ),
        StrokeKind::Inside,
    );
    if resp.clicked() {
        egui::Popup::toggle_id(ui.ctx(), resp.id);
    }
    let mut changed = false;
    menu::dropdown(&resp, |ui| {
        ui.set_min_width(212.0);
        changed = crate::ui::widgets::color_picker::color_picker(ui, id, rgb);
    });
    changed
}

/// Lock (or unlock) an object with everything inside it.
pub(super) fn set_locked(o: &mut Object, on: bool) {
    o.locked = on;
    for c in &mut o.children {
        set_locked(c, on);
    }
}

/// Transfer the changes of the edited object (`before` → `after`) to `o`.
fn transfer(before: &Object, after: &Object, pos: VecEdit, o: &mut Object) {
    if after.locked != before.locked {
        set_locked(o, after.locked);
    }
    if after.hidden != before.hidden {
        o.hidden = after.hidden;
    }
    if o.locked {
        return;
    }
    let (b, a) = (&before.transform, &after.transform);
    for i in 0..3 {
        if pos.changed[i] {
            if pos.dragged {
                o.transform.position[i] += a.position[i] - b.position[i];
            } else {
                o.transform.position[i] = a.position[i];
            }
        }
        if a.rotation[i] != b.rotation[i] {
            o.transform.rotation[i] = a.rotation[i];
        }
        if a.scale[i] != b.scale[i] {
            o.transform.scale[i] = a.scale[i];
        }
    }
    match (&before.shape, &after.shape, &mut o.shape) {
        (Shape::Box { size: b }, Shape::Box { size: a }, Shape::Box { size: s }) => {
            for i in 0..3 {
                if a[i] != b[i] {
                    s[i] = a[i];
                }
            }
        }
        (Shape::Plane { size: b }, Shape::Plane { size: a }, Shape::Plane { size: s }) => {
            for i in 0..2 {
                if a[i] != b[i] {
                    s[i] = a[i];
                }
            }
        }
        (
            Shape::Cylinder {
                radius: br,
                height: bh,
            },
            Shape::Cylinder {
                radius: ar,
                height: ah,
            },
            Shape::Cylinder { radius, height },
        ) => {
            if ar != br {
                *radius = *ar;
            }
            if ah != bh {
                *height = *ah;
            }
        }
        (
            Shape::Truss {
                length: bl,
                profile: bp,
                ..
            },
            Shape::Truss {
                length: al,
                profile: ap,
                ..
            },
            Shape::Truss {
                length, profile, ..
            },
        ) => {
            if al != bl {
                *length = *al;
            }
            if ap != bp {
                *profile = *ap;
            }
        }
        _ => {}
    }
    let (b, a, m) = (&before.material, &after.material, &mut o.material);
    if a.color != b.color {
        m.color = a.color;
    }
    if a.tile != b.tile {
        m.tile = a.tile;
    }
    if a.opacity != b.opacity {
        m.opacity = a.opacity;
    }
    if a.gloss != b.gloss {
        m.gloss = a.gloss;
    }
    if a.emissive != b.emissive {
        m.emissive = a.emissive;
    }
}

/// Height of the center so the object stands on the floor.
fn floor_height(o: &Object) -> f32 {
    match &o.shape {
        Shape::Box { size } => size[1] * 0.5 * o.transform.scale[1],
        Shape::Cylinder { height, .. } => height * 0.5 * o.transform.scale[1],
        _ => 0.0,
    }
}

/// Selected objects: the edited one first.
fn selected_objects(view: &VizView, id: u32) -> Vec<u32> {
    let mut ids = vec![id];
    ids.extend(view.sel_objects.iter().copied().filter(|x| *x != id));
    ids
}

/// Point moving heads at `target` (programmer pan/tilt), turn other fixtures
/// with their housing. Returns the number of moving heads out of reach.
pub(super) fn aim_fixtures(cx: &mut ViewCtx<'_>, ids: &[FixtureId], target: [f32; 3]) -> usize {
    let layer = Layer::of(cx.state.programmer_mode);
    let mut missed = 0;
    for &id in ids {
        let show = cx.engine.show();
        let Some(ty) = show.fixture(id).and_then(|f| show.type_of(f)).cloned() else {
            continue;
        };
        if ty.has_kind(ChannelKind::Pan) && ty.has_kind(ChannelKind::Tilt) {
            let Some((pan, tilt)) = show.aim_moving_head(id, target) else {
                missed += 1;
                continue;
            };
            for (i, def) in ty.channels.iter().enumerate() {
                let v = match def.kind {
                    ChannelKind::Pan => (pan >> 8) as u8,
                    ChannelKind::PanFine => (pan & 0xFF) as u8,
                    ChannelKind::Tilt => (tilt >> 8) as u8,
                    ChannelKind::TiltFine => (tilt & 0xFF) as u8,
                    _ => continue,
                };
                let ch = ChannelRef {
                    fixture: id,
                    channel: i as u16,
                };
                layer.set(cx.engine, ch, v);
            }
        } else {
            let mut p = rig::placement(show, id);
            p.aim_at(target);
            apply_placement(cx, Pick::Fixture(id), p);
        }
    }
    missed
}

impl VizView {
    /// Right bar: properties of the selection.
    pub(super) fn props_panel(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        egui::ScrollArea::vertical()
            .id_salt("viz_props")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.selected {
                Some(Pick::Object(id)) => self.object_props(ui, cx, id),
                Some(Pick::Fixture(id)) => self.fixture_props(ui, cx, id),
                None => {}
            });
    }

    fn object_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: u32) {
        let ids = selected_objects(self, id);
        let n = ids.len();
        let v = &mut cx.engine.show_mut().venue;
        let model_info = match v.find(id).map(|o| &o.shape) {
            Some(Shape::Model { model }) => v
                .models
                .get(model)
                .map(|m| (m.name.clone(), m.triangles(), m.size)),
            _ => None,
        };
        let tex_list: Vec<(u32, String)> = v
            .textures
            .iter()
            .map(|(k, t)| (*k, t.name.clone()))
            .collect();
        let mut tex_action: Option<TexAction> = None;
        let (mut remove, mut duplicate, mut reset_rot, mut to_floor) = (false, false, false, false);
        let Some(o) = v.find_mut(id) else {
            self.selected = None;
            return;
        };
        let before = o.clone();
        let mut pos = VecEdit::default();

        // Header: name (or count) and the actions
        ui.horizontal(|ui| {
            ui.set_min_height(INPUT_H);
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            let actions = 4.0 * (INPUT_H + theme::SP_XS);
            if n > 1 {
                ui.add_sized(
                    vec2(ui.available_width() - actions, INPUT_H),
                    egui::Label::new(theme::heading_text(&tr!("{n} objects", n = n))),
                );
            } else {
                ui.add(text_field(&mut o.name).desired_width(ui.available_width() - actions));
            }
            let lock_icon = if o.locked {
                Icon::Locked
            } else {
                Icon::Unlocked
            };
            if ui
                .add(IconButton::new(lock_icon).size(INPUT_H).active(o.locked))
                .on_hover_text(if o.locked { t("Unlock") } else { t("Lock") })
                .clicked()
            {
                let on = !o.locked;
                set_locked(o, on);
            }
            let eye = if o.hidden { Icon::EyeOff } else { Icon::Eye };
            if ui
                .add(IconButton::new(eye).size(INPUT_H).active(o.hidden))
                .on_hover_text(if o.hidden { t("Show") } else { t("Hide") })
                .clicked()
            {
                o.hidden = !o.hidden;
            }
            duplicate = ui
                .add(IconButton::new(Icon::Copy).size(INPUT_H).enabled(!o.locked))
                .on_hover_text(t("Duplicate"))
                .clicked();
            remove = ui
                .add(
                    IconButton::new(Icon::Trash)
                        .size(INPUT_H)
                        .danger()
                        .enabled(!o.locked),
                )
                .on_hover_text(t("Delete"))
                .clicked();
        });
        ui.add_space(theme::SP_S);

        let enabled = !o.locked;
        ui.add_enabled_ui(enabled, |ui| {
            fold_section(ui, "viz_p_place", t("Placement"), true, |ui| {
                form(ui, "viz_p_place_form", |f| {
                    f.row(t("Position (m)"), |ui| {
                        pos = vec3_field(
                            ui,
                            &mut o.transform.position,
                            0.01,
                            -500.0..=500.0,
                            RESERVE,
                        );
                        to_floor = ui
                            .add(IconButton::new(Icon::AlignV).size(INPUT_H))
                            .on_hover_text(t("Onto the floor"))
                            .clicked();
                    });
                    f.row(t("Rotation (°)"), |ui| {
                        vec3_field(ui, &mut o.transform.rotation, 0.5, -360.0..=360.0, RESERVE);
                        reset_rot = ui
                            .add(IconButton::new(Icon::Reset).size(INPUT_H))
                            .on_hover_text(t("Reset rotation"))
                            .clicked();
                    });
                    f.row(t("Scale"), |ui| {
                        vec3_field(ui, &mut o.transform.scale, 0.01, 0.001..=1000.0, RESERVE);
                    });
                });
            });
            if matches!(o.shape, Shape::Group) {
                return;
            }
            fold_section(ui, "viz_p_shape", t("Shape"), true, |ui| {
                form(ui, "viz_p_shape_form", |f| match &mut o.shape {
                    Shape::Box { size } => {
                        f.row(t("Dimensions (m)"), |ui| {
                            vec3_field(ui, size, 0.01, 0.01..=500.0, RESERVE);
                        });
                    }
                    Shape::Plane { size } => {
                        f.row(t("Width (m)"), |ui| {
                            number(ui, &mut size[0], 0.01, 0.01..=500.0, " m")
                        });
                        f.row(t("Depth (m)"), |ui| {
                            number(ui, &mut size[1], 0.01, 0.01..=500.0, " m")
                        });
                    }
                    Shape::Cylinder { radius, height } => {
                        f.row(t("Radius"), |ui| {
                            number(ui, radius, 0.005, 0.005..=50.0, " m")
                        });
                        f.row(t("Height"), |ui| {
                            number(ui, height, 0.01, 0.01..=100.0, " m")
                        });
                    }
                    Shape::Truss {
                        length, profile, ..
                    } => {
                        f.row(t("Length"), |ui| {
                            number(ui, length, 0.05, 0.25..=30.0, " m")
                        });
                        f.row(t("Profile"), |ui| {
                            Segmented::new(&[
                                (TrussProfile::Triangle, t("Triangle")),
                                (TrussProfile::Square, t("Square")),
                            ])
                            .show(ui, profile);
                        });
                    }
                    Shape::Model { .. } => {
                        if let Some((name, tris, size)) = &model_info {
                            f.row(t("Model"), |ui| {
                                ui.label(name.as_str());
                            });
                            f.row(t("Triangles"), |ui| {
                                ui.label(tris.to_string());
                            });
                            f.row(t("Size (m)"), |ui| {
                                ui.label(format!(
                                    "{:.2} × {:.2} × {:.2}",
                                    size[0], size[1], size[2]
                                ));
                            });
                        }
                    }
                    Shape::Group => {}
                });
            });
            fold_section(ui, "viz_p_material", t("Material"), true, |ui| {
                form(ui, "viz_p_material_form", |f| {
                    let is_model = matches!(o.shape, Shape::Model { .. });
                    if !is_model {
                        f.row(t("Color"), |ui| {
                            color_field(
                                ui,
                                ui.make_persistent_id(("viz_color", id)),
                                &mut o.material.color,
                            );
                        });
                    }
                    if !matches!(o.shape, Shape::Model { .. } | Shape::Truss { .. }) {
                        f.row(t("Image"), |ui| {
                            let current = tex_list
                                .iter()
                                .find(|x| x.0 == o.material.texture)
                                .map_or(t("None").to_string(), |x| x.1.clone());
                            ui.spacing_mut().interact_size.y = INPUT_H;
                            egui::ComboBox::from_id_salt("viz_tex")
                                .selected_text(current)
                                .width(ui.available_width() - RESERVE)
                                .height(420.0)
                                .show_ui(ui, |ui| {
                                    if ui
                                        .selectable_label(o.material.texture == 0, t("None"))
                                        .clicked()
                                    {
                                        tex_action = Some(TexAction::Set(0, None));
                                    }
                                    for (k, name) in &tex_list {
                                        if ui
                                            .selectable_label(o.material.texture == *k, name)
                                            .clicked()
                                        {
                                            tex_action = Some(TexAction::Set(*k, None));
                                        }
                                    }
                                    ui.separator();
                                    for (i, (name, tile, _)) in
                                        super::textures::builtin().iter().enumerate()
                                    {
                                        if !tex_list.iter().any(|x| x.1 == *name)
                                            && ui.selectable_label(false, *name).clicked()
                                        {
                                            tex_action = Some(TexAction::Builtin(i, *tile));
                                        }
                                    }
                                    ui.separator();
                                    if ui
                                        .selectable_label(false, t("Load image (PNG/JPEG) …"))
                                        .clicked()
                                    {
                                        tex_action = Some(TexAction::Load);
                                    }
                                });
                        });
                        if o.material.texture != 0 {
                            f.row(t("Tile size"), |ui| {
                                number(ui, &mut o.material.tile, 0.02, 0.05..=100.0, " m")
                            });
                        }
                    }
                    f.row(t("Opacity"), |ui| {
                        percent(ui, &mut o.material.opacity, 0.05)
                    });
                    f.row(t("Gloss"), |ui| percent(ui, &mut o.material.gloss, 0.0));
                    f.row(t("Glow"), |ui| percent(ui, &mut o.material.emissive, 0.0));
                });
            });
        });

        // Transfer to the other selected objects
        let after = o.clone();
        if n > 1 && (after != before || pos.any()) {
            for other in &ids[1..] {
                if let Some(x) = v.find_mut(*other) {
                    transfer(&before, &after, pos, x);
                }
            }
        }
        for oid in &ids {
            let Some(x) = v.find_mut(*oid) else { continue };
            if x.locked {
                continue;
            }
            if reset_rot {
                x.transform.rotation = [0.0; 3];
            }
            if to_floor {
                x.transform.position[1] = floor_height(x);
            }
        }
        if let Some(a) = tex_action {
            let (tex, tile) = match a {
                TexAction::Set(k, tile) => (Some(k), tile),
                TexAction::Builtin(i, tile) => {
                    let (name, _, f) = super::textures::builtin()[i];
                    (
                        Some(v.add_texture(super::textures::make(name, f))),
                        Some(tile),
                    )
                }
                TexAction::Load => {
                    self.open_dialog(ui.ctx(), DialogKind::LoadTexture);
                    (None, None)
                }
            };
            if let Some(tex) = tex {
                apply_texture(v, ids.iter().copied(), tex, tile);
            }
        }
        if duplicate {
            let mut new = Vec::new();
            for oid in &ids {
                if let Some(copy) = duplicate_object(v, *oid) {
                    new.push(copy);
                }
            }
            self.sel_objects = new.iter().copied().collect();
            self.selected = new.first().map(|c| Pick::Object(*c));
        }
        if remove {
            for oid in &ids {
                if v.find(*oid).is_some_and(|x| !x.locked) {
                    v.remove(*oid);
                }
            }
            v.prune_models();
            self.sel_objects.clear();
            self.selected = None;
        }
    }

    fn fixture_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: FixtureId) {
        let show = cx.engine.show();
        let Some(f) = show.fixture(id) else {
            self.selected = None;
            return;
        };
        let ids = {
            let mut ids = vec![id];
            ids.extend(targets(cx, self.selected).into_iter().filter(|x| *x != id));
            ids
        };
        let n = ids.len();
        let title = if n > 1 {
            tr!("{n} fixtures", n = n)
        } else {
            f.name.clone()
        };
        ui.label(theme::heading_text(&title));
        if n == 1 {
            let type_name = show
                .type_of(f)
                .map(|t| format!("{} {}", t.manufacturer, t.model))
                .unwrap_or_default();
            ui.label(
                egui::RichText::new(format!(
                    "U{} · {} · {}",
                    f.universe + 1,
                    f.address,
                    type_name
                ))
                .small()
                .color(theme::text_dim()),
            );
        }
        ui.add_space(theme::SP_S);
        let any_manual = ids
            .iter()
            .any(|x| show.fixture(*x).is_some_and(|f| f.place3d.is_some()));
        let mut p: Placement = rig::placement(show, id);
        let before = p;
        let (mut pos, mut rot) = (VecEdit::default(), VecEdit::default());
        let mut hang = false;
        let mut automatic = false;
        fold_section(ui, "viz_f_place", t("Placement"), true, |ui| {
            form(ui, "viz_f_place_form", |f| {
                f.row(t("Position (m)"), |ui| {
                    pos = vec3_field(ui, &mut p.position, 0.01, -500.0..=500.0, RESERVE);
                });
                f.row(t("Rotation (°)"), |ui| {
                    rot = vec3_field(ui, &mut p.rotation, 0.5, -360.0..=360.0, RESERVE);
                });
            });
            ui.add_space(theme::SP_XS);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(theme::SP_XS, theme::SP_XS);
                let has_truss = cx
                    .engine
                    .show()
                    .venue
                    .flatten()
                    .iter()
                    .any(|(o, _)| matches!(o.shape, Shape::Truss { .. }));
                hang = button(ui, t("Hang on nearest truss"), has_truss).clicked();
                automatic = button(ui, t("From the stage plan"), any_manual)
                    .on_hover_text(t(
                        "Placement automatic again: position from the stage plan, below the ceiling",
                    ))
                    .clicked();
            });
        });
        if pos.any() || rot.any() {
            for fid in &ids {
                let mut q = if *fid == id {
                    p
                } else {
                    rig::placement(cx.engine.show(), *fid)
                };
                if *fid != id {
                    for i in 0..3 {
                        if pos.changed[i] {
                            if pos.dragged {
                                q.position[i] += p.position[i] - before.position[i];
                            } else {
                                q.position[i] = p.position[i];
                            }
                        }
                        if rot.changed[i] {
                            q.rotation[i] = p.rotation[i];
                        }
                    }
                }
                apply_placement(cx, Pick::Fixture(*fid), q);
            }
        }
        if hang {
            for fid in &ids {
                let mut q = rig::placement(cx.engine.show(), *fid);
                if let Some(h) = cx.engine.show().venue.nearest_truss(q.position) {
                    q.position = h.point;
                    apply_placement(cx, Pick::Fixture(*fid), q);
                }
            }
        }
        if automatic {
            for fid in &ids {
                if let Some(fm) = cx.engine.show_mut().fixture_mut(*fid) {
                    fm.place3d = None;
                }
            }
        }

        // Aim: moving heads by pan/tilt (programmer), others with the housing
        fold_section(ui, "viz_f_aim", t("Aim"), true, |ui| {
            let mut aim = false;
            form(ui, "viz_f_aim_form", |f| {
                f.row(t("Target (m)"), |ui| {
                    vec3_field(ui, &mut self.aim, 0.05, -500.0..=500.0, RESERVE);
                    if ui
                        .add(
                            IconButton::new(Icon::Cursor)
                                .size(INPUT_H)
                                .active(self.aim_pick),
                        )
                        .on_hover_text(t("Pick the target in the view"))
                        .clicked()
                    {
                        self.aim_pick = !self.aim_pick;
                    }
                });
            });
            ui.add_space(theme::SP_XS);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = vec2(theme::SP_XS, theme::SP_XS);
                aim = button(ui, t("Aim at target"), true)
                    .on_hover_text(t(
                        "Moving heads turn with pan/tilt (programmer), other fixtures with their housing",
                    ))
                    .clicked();
                let dirs: [(&str, [f32; 3]); 4] = [
                    (t("Down"), [0.0, -1.0, 0.0]),
                    (t("Up"), [0.0, 1.0, 0.0]),
                    (t("Front"), [0.0, 0.0, 1.0]),
                    (t("Back"), [0.0, 0.0, -1.0]),
                ];
                for (label, d) in dirs {
                    if button(ui, label, true)
                        .on_hover_text(t("Turn the housing"))
                        .clicked()
                    {
                        for fid in &ids {
                            let mut q = rig::placement(cx.engine.show(), *fid);
                            if d[1] < -0.5 {
                                q.rotation = [0.0; 3];
                            } else {
                                q.aim_at([
                                    q.position[0] + d[0],
                                    q.position[1] + d[1],
                                    q.position[2] + d[2],
                                ]);
                            }
                            apply_placement(cx, Pick::Fixture(*fid), q);
                        }
                    }
                }
            });
            if aim {
                self.aim_now(cx, &ids);
            }
        });
    }

    /// Aim the selected fixtures at the target point.
    pub(super) fn aim_now(&mut self, cx: &mut ViewCtx<'_>, ids: &[FixtureId]) {
        let missed = aim_fixtures(cx, ids, self.aim);
        if missed > 0 {
            cx.state.error(tr!(
                "{n} moving heads cannot reach the target (pan/tilt range or limits)",
                n = missed
            ));
        }
    }
}

/// Copy of an object right behind it in its folder (offset by 0.5 m);
/// returns the id of the copy.
pub(super) fn duplicate_object(v: &mut Venue, id: u32) -> Option<u32> {
    let mut copy = v.find(id).filter(|x| !x.locked).cloned()?;
    renumber(v, &mut copy);
    copy.transform.position[0] += 0.5;
    let new = copy.id;
    let parent = v.parent_of(id);
    let list = v.list_mut(parent)?;
    let at = list
        .iter()
        .position(|x| x.id == id)
        .map_or(list.len(), |i| i + 1);
    list.insert(at, copy);
    Some(new)
}
