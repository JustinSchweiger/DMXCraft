//! Sidebars in build mode: room, object tree and fixtures on the left, the
//! properties of the selection on the right (docked).

use dmxcraft_core::venue::{Material, Placement, Shape, Transform, TrussProfile, Venue};
use dmxcraft_core::FixtureId;
use eframe::egui::{self, Color32};
use glam::Vec3;

use super::{
    aim_selected, apply_placement, renumber, rig, targets, vec3_row, DialogKind, Pick, VizView,
};
use super::{library, textures};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{fold_section, Segmented};
use crate::views::ViewCtx;

/// Choice in the image menu.
enum TexAction {
    /// Image (0 = none), optional tile size
    Set(u32, Option<f32>),
    Builtin(usize, f32),
    Load,
}

/// Put an image on objects (the color is set to white so the image shows).
pub(super) fn apply_texture(
    v: &mut Venue,
    ids: impl IntoIterator<Item = u32>,
    tex: u32,
    tile: Option<f32>,
) {
    // Folder: on everything inside
    fn set(o: &mut dmxcraft_core::venue::Object, tex: u32, tile: Option<f32>) {
        if !matches!(
            o.shape,
            Shape::Group | Shape::Model { .. } | Shape::Truss { .. }
        ) {
            if o.material.texture == 0 && tex != 0 {
                o.material.color = [255, 255, 255];
            }
            o.material.texture = tex;
            if let Some(t) = tile {
                o.material.tile = t;
            }
        }
        for c in &mut o.children {
            set(c, tex, tile);
        }
    }
    for id in ids {
        if let Some(o) = v.find_mut(id) {
            set(o, tex, tile);
        }
    }
    v.prune_models();
}

/// Tabs of the left bar.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutlineTab {
    #[default]
    Objects,
    Fixtures,
}

impl VizView {
    pub(super) fn outline_panel(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let n_fx = cx.engine.show().fixtures.len();
        let objects = t("Objects");
        let fixtures = tr!("Fixtures ({n})", n = n_fx);
        Segmented::new(&[
            (OutlineTab::Objects, objects),
            (OutlineTab::Fixtures, fixtures.as_str()),
        ])
        .show(ui, &mut self.outline);
        ui.add_space(theme::SP_S);
        if self.outline == OutlineTab::Fixtures {
            self.fixture_list_header(ui, cx);
            ui.add(
                egui::TextEdit::singleline(&mut self.fixture_filter)
                    .hint_text(t("Search"))
                    .desired_width(f32::INFINITY),
            );
            ui.label(
                egui::RichText::new(t("Click selects, Ctrl: add, Shift: range, drag: into a folder, right-click: menu"))
                    .small()
                    .color(theme::text_faint()),
            );
            ui.add_space(theme::SP_XS);
        }
        egui::ScrollArea::vertical()
            .id_salt("viz_outline")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.outline {
                OutlineTab::Objects => self.objects_tab(ui, cx),
                OutlineTab::Fixtures => self.fixtures_tab(ui, cx),
            });
    }

    fn objects_tab(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let v = &mut cx.engine.show_mut().venue;
        fold_section(ui, "viz_room", t("Room"), false, |ui| {
            ui.horizontal(|ui| {
                ui.label(t("Name"));
                ui.text_edit_singleline(&mut v.name);
            });
            ui.horizontal(|ui| {
                ui.label(t("Size (m)"));
                for (i, c) in ["B", "H", "T"].iter().enumerate() {
                    ui.add(
                        egui::DragValue::new(&mut v.size[i])
                            .range(1.0..=200.0)
                            .speed(0.05)
                            .prefix(format!("{c} "))
                            .max_decimals(2),
                    );
                }
            });
            if ui.button(t("Create an empty room of this size")).clicked() {
                let (size, name, env) = (v.size, v.name.clone(), v.environment);
                *v = Venue::room(name, size);
                v.environment = env;
                self.selected = None;
            }
        });
        fold_section(ui, "viz_add", t("Add"), true, |ui| {
            ui.horizontal_wrapped(|ui| {
                let at = self.camera.map_or(Vec3::ZERO, |c| c.target);
                let items: [(&str, Shape, [f32; 3]); 6] = [
                    (
                        t("Box"),
                        Shape::Box {
                            size: [1.0, 1.0, 1.0],
                        },
                        [at.x, 0.5, at.z],
                    ),
                    (
                        t("Riser"),
                        Shape::Box {
                            size: [2.0, 0.4, 1.0],
                        },
                        [at.x, 0.2, at.z],
                    ),
                    (
                        t("Plane"),
                        Shape::Plane { size: [2.0, 2.0] },
                        [at.x, 0.01, at.z],
                    ),
                    (
                        t("Cylinder"),
                        Shape::Cylinder {
                            radius: 0.2,
                            height: 2.0,
                        },
                        [at.x, 1.0, at.z],
                    ),
                    (
                        t("Truss 2 m"),
                        Shape::Truss {
                            length: 2.0,
                            profile: TrussProfile::Triangle,
                            width: 0.29,
                        },
                        [at.x, 4.0, at.z],
                    ),
                    (
                        t("Truss 3 m"),
                        Shape::Truss {
                            length: 3.0,
                            profile: TrussProfile::Square,
                            width: 0.29,
                        },
                        [at.x, 4.0, at.z],
                    ),
                ];
                for (label, shape, pos) in items {
                    if ui.button(label).clicked() {
                        let material = if matches!(shape, Shape::Truss { .. }) {
                            Material {
                                color: [190, 190, 195],
                                gloss: 0.6,
                                ..Material::default()
                            }
                        } else {
                            Material::default()
                        };
                        let o = v.object(
                            label,
                            shape,
                            Transform {
                                position: pos,
                                ..Transform::default()
                            },
                            material,
                        );
                        self.selected = Some(Pick::Object(o.id));
                        v.objects.push(o);
                    }
                }
            });
            let loading = self.model_job.is_some();
            let label = if loading {
                t("Loading model …")
            } else {
                t("Import 3D model (glTF/GLB) …")
            };
            if ui.add_enabled(!loading, egui::Button::new(label)).clicked() {
                self.open_dialog(ui.ctx(), DialogKind::ImportModel);
            }
        });
        fold_section(ui, "viz_library", t("Library"), false, |ui| {
            self.library_ui(ui, cx);
        });
        fold_section(ui, "viz_objects", t("Objects"), true, |ui| {
            self.object_tree(ui, cx);
        });
        self.missing_section(ui, cx);
    }

    fn library_ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let (builtin, user) = self
            .library
            .get_or_insert_with(|| (library::builtin(), library::load_user()));
        let at = self.camera.map_or(Vec3::ZERO, |c| c.target);
        let mut insert: Option<library::Prefab> = None;
        ui.label(
            egui::RichText::new(t("Built-in"))
                .small()
                .color(theme::text_dim()),
        );
        ui.horizontal_wrapped(|ui| {
            for p in builtin.iter() {
                if ui.button(&p.name).clicked() {
                    insert = Some(p.clone());
                }
            }
        });
        ui.add_space(theme::SP_XS);
        ui.label(
            egui::RichText::new(t("Custom"))
                .small()
                .color(theme::text_dim()),
        );
        let mut delete = None;
        if user.is_empty() {
            ui.label(
                egui::RichText::new(t("None yet – select an object and save it below."))
                    .small()
                    .color(theme::text_faint()),
            );
        }
        ui.horizontal_wrapped(|ui| {
            for (i, u) in user.iter().enumerate() {
                let r = ui.button(&u.prefab.name);
                if r.clicked() {
                    insert = Some(u.prefab.clone());
                }
                r.context_menu(|ui| {
                    if ui.button(t("Remove from library")).clicked() {
                        delete = Some(i);
                        ui.close();
                    }
                });
            }
        });
        if let Some(i) = delete {
            let u = user.remove(i);
            if let Err(e) = std::fs::remove_file(&u.path) {
                cx.state.error(e.to_string());
            }
        }
        let sel = match self.selected {
            Some(Pick::Object(id)) => Some(id),
            _ => None,
        };
        if ui
            .add_enabled(
                sel.is_some(),
                egui::Button::new(t("Save selection as own object")),
            )
            .on_hover_text(t(
                "Saves the selected object or folder with its contents to the library",
            ))
            .clicked()
        {
            if let Some(p) =
                sel.and_then(|id| library::Prefab::from_venue(&cx.engine.show().venue, id))
            {
                match library::save_user(&p) {
                    Ok(_) => {
                        cx.state.info(tr!("Saved to library: {n}", n = p.name));
                        self.library = None;
                    }
                    Err(e) => cx.state.error(e),
                }
            }
        }
        if let Some(p) = insert {
            let v = &mut cx.engine.show_mut().venue;
            let id = p.insert(
                v,
                (at.x * 20.0).round() / 20.0,
                (at.z * 20.0).round() / 20.0,
            );
            self.sel_objects = [id].into();
            self.selected = Some(Pick::Object(id));
            cx.state.selected.clear();
        }
    }

    fn fixtures_tab(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        self.fixture_list(ui, cx);
    }

    fn missing_section(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        if self.missing.is_empty() {
            return;
        }
        let title = tr!("Not in the patch ({n})", n = self.missing.len());
        fold_section(ui, "viz_missing", &title, true, |ui| {
            ui.label(egui::RichText::new(t("These imported fixtures have no fixture with the same universe and address in the patch. A matching library type is suggested (italic: only similar, please check).")).small().color(theme::text_faint()));
            for m in &mut self.missing {
                let f = &m.fixture;
                ui.horizontal(|ui| {
                    ui.checkbox(&mut m.create, "");
                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(format!(
                                "{} · U{}/{}",
                                f.name,
                                f.universe + 1,
                                f.address
                            ))
                            .small(),
                        );
                        let ty = tr!(
                            "{m} {t} ({mode} · {n} channels)",
                            m = m.proposal.manufacturer,
                            t = m.proposal.model,
                            mode = m.proposal.mode,
                            n = m.proposal.channels.len()
                        );
                        let mut txt = egui::RichText::new(ty).small().color(theme::text_dim());
                        if !m.exact {
                            txt = txt.italics();
                        }
                        ui.label(txt).on_hover_text(&f.library_name);
                    });
                });
            }
            let n = self.missing.iter().filter(|m| m.create).count();
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add_enabled(n > 0, egui::Button::new(tr!("Create {n} fixtures", n = n)))
                    .clicked()
                {
                    let show = cx.engine.show_mut();
                    let mut failed = Vec::new();
                    let mut made = 0;
                    for m in self.missing.drain(..) {
                        if !m.create {
                            continue;
                        }
                        show.upsert_fixture_type(m.proposal.clone());
                        let f = &m.fixture;
                        match show.add_fixture(
                            f.name.clone(),
                            &m.proposal.key,
                            f.universe,
                            f.address,
                        ) {
                            Ok(id) => {
                                if let Some(x) = show.fixture_mut(id) {
                                    x.place3d = Some(f.placement);
                                }
                                made += 1;
                            }
                            Err(e) => failed.push(format!("{}: {e}", f.name)),
                        }
                    }
                    dmxcraft_ofl::fill_physical(show);
                    if failed.is_empty() {
                        cx.state.info(tr!("{n} fixtures created", n = made));
                    } else {
                        cx.state.error(tr!(
                            "{n} fixtures created, not possible: {f}",
                            n = made,
                            f = failed.join(", ")
                        ));
                    }
                }
                if ui.button(t("Hide list")).clicked() {
                    self.missing.clear();
                }
            });
        });
    }

    /// Right bar: properties of the selection.
    pub(super) fn props_panel(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        egui::ScrollArea::vertical()
            .id_salt("viz_props")
            .auto_shrink([false, false])
            .show(ui, |ui| match self.selected {
                Some(Pick::Object(id)) => self.object_props(ui, cx, id),
                Some(Pick::Fixture(id)) => self.fixture_props(ui, cx, id),
                None => {
                    ui.add_space(theme::SP_M);
                    ui.label(
                        egui::RichText::new(t(
                            "Click an object or fixture (in the view or in the list on the left) to edit it.",
                        ))
                        .color(theme::text_faint()),
                    );
                }
            });
    }

    fn object_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: u32) {
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
        let mut remove = false;
        let mut duplicate = false;
        let Some(o) = v.find_mut(id) else {
            self.selected = None;
            return;
        };
        fold_section(ui, "viz_p_general", t("General"), true, |ui| {
            ui.horizontal(|ui| {
                ui.label(t("Name"));
                ui.text_edit_singleline(&mut o.name);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut o.locked, t("Locked"));
                ui.checkbox(&mut o.hidden, t("Hidden"));
            });
        });
        let enabled = !o.locked;
        ui.add_enabled_ui(enabled, |ui| {
            fold_section(ui, "viz_p_place", t("Placement"), true, |ui| {
                vec3_row(ui, t("Position (m)"), &mut o.transform.position, 0.01);
                vec3_row(ui, t("Rotation (°)"), &mut o.transform.rotation, 0.5);
                vec3_row(ui, t("Scale"), &mut o.transform.scale, 0.01);
                ui.horizontal(|ui| {
                    if ui.button(t("Reset rotation")).clicked() {
                        o.transform.rotation = [0.0; 3];
                    }
                    if ui.button(t("Onto the floor")).clicked() {
                        o.transform.position[1] = match &o.shape {
                            Shape::Box { size } => size[1] * 0.5 * o.transform.scale[1],
                            Shape::Cylinder { height, .. } => height * 0.5 * o.transform.scale[1],
                            _ => 0.0,
                        };
                    }
                });
            });
            if !matches!(o.shape, Shape::Group) {
                fold_section(ui, "viz_p_shape", t("Shape"), true, |ui| {
                    match &mut o.shape {
                        Shape::Box { size } => vec3_row(ui, t("Dimensions (m)"), size, 0.01),
                        Shape::Plane { size } => {
                            ui.horizontal(|ui| {
                                ui.label(t("Dimensions (m)"));
                                for s in size.iter_mut() {
                                    ui.add(egui::DragValue::new(s).speed(0.01).range(0.01..=500.0));
                                }
                            });
                        }
                        Shape::Cylinder { radius, height } => {
                            ui.horizontal(|ui| {
                                ui.label(t("Radius / height (m)"));
                                ui.add(
                                    egui::DragValue::new(radius)
                                        .speed(0.005)
                                        .range(0.005..=50.0),
                                );
                                ui.add(
                                    egui::DragValue::new(height).speed(0.01).range(0.01..=100.0),
                                );
                            });
                        }
                        Shape::Truss {
                            length, profile, ..
                        } => {
                            ui.horizontal(|ui| {
                                ui.label(t("Length (m)"));
                                ui.add(egui::DragValue::new(length).speed(0.05).range(0.25..=30.0));
                                let mut sq = *profile == TrussProfile::Square;
                                ui.checkbox(&mut sq, t("Square truss"));
                                *profile = if sq {
                                    TrussProfile::Square
                                } else {
                                    TrussProfile::Triangle
                                };
                            });
                        }
                        Shape::Model { .. } => {
                            if let Some((name, tris, size)) = &model_info {
                                ui.label(
                                    egui::RichText::new(tr!(
                                        "Model {n} · {k} triangles · {x} × {y} × {z} m",
                                        n = name,
                                        k = tris,
                                        x = format!("{:.2}", size[0]),
                                        y = format!("{:.2}", size[1]),
                                        z = format!("{:.2}", size[2])
                                    ))
                                    .small()
                                    .color(theme::text_dim()),
                                );
                            }
                        }
                        Shape::Group => {}
                    }
                });
                fold_section(ui, "viz_p_material", t("Material"), false, |ui| {
                    if !matches!(o.shape, Shape::Model { .. }) {
                        ui.horizontal(|ui| {
                            ui.label(t("Color"));
                            let mut c = Color32::from_rgb(
                                o.material.color[0],
                                o.material.color[1],
                                o.material.color[2],
                            );
                            if ui.color_edit_button_srgba(&mut c).changed() {
                                o.material.color = [c.r(), c.g(), c.b()];
                            }
                        });
                    }
                    if !matches!(o.shape, Shape::Model { .. } | Shape::Truss { .. }) {
                        ui.horizontal(|ui| {
                            ui.label(t("Image"));
                            let current = tex_list
                                .iter()
                                .find(|x| x.0 == o.material.texture)
                                .map_or(t("None").to_string(), |x| x.1.clone());
                            egui::ComboBox::from_id_salt("viz_tex")
                                .selected_text(current)
                                .width(170.0)
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
                                        textures::builtin().iter().enumerate()
                                    {
                                        if !tex_list.iter().any(|x| x.1 == *name)
                                            && ui.button(tr!("+ {n}", n = name)).clicked()
                                        {
                                            tex_action = Some(TexAction::Builtin(i, *tile));
                                        }
                                    }
                                    if ui.button(t("Load image (PNG/JPEG) …")).clicked() {
                                        tex_action = Some(TexAction::Load);
                                    }
                                });
                        });
                        if o.material.texture != 0 {
                            ui.horizontal(|ui| {
                                ui.label(t("Tile size (m)"));
                                ui.add(
                                    egui::DragValue::new(&mut o.material.tile)
                                        .range(0.05..=100.0)
                                        .speed(0.02)
                                        .max_decimals(2),
                                );
                            });
                        }
                    }
                    ui.add(
                        egui::Slider::new(&mut o.material.opacity, 0.05..=1.0).text(t("Opacity")),
                    );
                    ui.add(egui::Slider::new(&mut o.material.gloss, 0.0..=1.0).text(t("Gloss")));
                    ui.add(egui::Slider::new(&mut o.material.emissive, 0.0..=1.0).text(t("Glow")));
                });
            }
        });
        ui.horizontal(|ui| {
            duplicate = ui
                .add_enabled(enabled, egui::Button::new(t("Duplicate")))
                .clicked();
            remove = ui
                .add_enabled(enabled, egui::Button::new(t("Delete")))
                .clicked();
        });
        if let Some(a) = tex_action {
            let (tex, tile) = match a {
                TexAction::Set(k, tile) => (Some(k), tile),
                TexAction::Builtin(i, tile) => {
                    let (name, _, f) = textures::builtin()[i];
                    (Some(v.add_texture(textures::make(name, f))), Some(tile))
                }
                TexAction::Load => {
                    self.open_dialog(ui.ctx(), DialogKind::LoadTexture);
                    (None, None)
                }
            };
            if let Some(tex) = tex {
                apply_texture(v, self.sel_objects.iter().copied().chain([id]), tex, tile);
            }
        }
        if duplicate {
            if let Some(mut copy) = v.find(id).cloned() {
                renumber(v, &mut copy);
                copy.transform.position[0] += 0.5;
                self.selected = Some(Pick::Object(copy.id));
                v.objects.push(copy);
            }
        }
        if remove {
            v.remove(id);
            v.prune_models();
            self.selected = None;
        }
    }

    fn fixture_props(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>, id: FixtureId) {
        let show = cx.engine.show();
        let Some(f) = show.fixture(id) else {
            self.selected = None;
            return;
        };
        ui.label(theme::heading_text(&f.name));
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
        ui.add_space(theme::SP_S);
        let auto = f.place3d.is_none();
        let mut p: Placement = rig::placement(show, id);
        let before = p;
        let n = targets(cx, self.selected).len();
        fold_section(ui, "viz_f_place", t("Placement"), true, |ui| {
            vec3_row(ui, t("Position (m)"), &mut p.position, 0.01);
            vec3_row(ui, t("Rotation (°)"), &mut p.rotation, 0.5);
            ui.label(
                egui::RichText::new(t(
                    "Rotation 0/0/0 points straight down. In the view the rings rotate around the room axes.",
                ))
                .small()
                .color(theme::text_faint()),
            );
        });
        if p != before {
            apply_placement(cx, Pick::Fixture(id), p);
        }
        let title = if n > 1 {
            tr!("Aim ({n} fixtures)", n = n)
        } else {
            t("Aim").to_string()
        };
        fold_section(ui, "viz_f_aim", &title, true, |ui| {
            ui.horizontal_wrapped(|ui| {
                let dirs: [(&str, [f32; 3]); 4] = [
                    (t("Down"), [0.0, -1.0, 0.0]),
                    (t("Up"), [0.0, 1.0, 0.0]),
                    (t("Front"), [0.0, 0.0, 1.0]),
                    (t("Back"), [0.0, 0.0, -1.0]),
                ];
                for (label, d) in dirs {
                    if ui.button(label).clicked() {
                        for fid in targets(cx, self.selected) {
                            let mut p = rig::placement(cx.engine.show(), fid);
                            if d[1] < -0.5 {
                                p.rotation = [0.0; 3];
                            } else {
                                p.aim_at([
                                    p.position[0] + d[0],
                                    p.position[1] + d[1],
                                    p.position[2] + d[2],
                                ]);
                            }
                            apply_placement(cx, Pick::Fixture(fid), p);
                        }
                    }
                }
            });
            vec3_row(ui, t("Target point (m)"), &mut self.aim, 0.05);
            ui.horizontal_wrapped(|ui| {
                if ui.button(t("Aim at target")).clicked() {
                    aim_selected(cx, self.selected, self.aim);
                }
                if ui
                    .selectable_label(self.aim_pick, t("Pick target in view"))
                    .clicked()
                {
                    self.aim_pick = !self.aim_pick;
                }
            });
            let has_truss = cx
                .engine
                .show()
                .venue
                .flatten()
                .iter()
                .any(|(o, _)| matches!(o.shape, Shape::Truss { .. }));
            if ui
                .add_enabled(has_truss, egui::Button::new(t("Hang on nearest truss")))
                .clicked()
            {
                for fid in targets(cx, self.selected) {
                    let mut p = rig::placement(cx.engine.show(), fid);
                    if let Some(h) = cx.engine.show().venue.nearest_truss(p.position) {
                        p.position = h.point;
                        apply_placement(cx, Pick::Fixture(fid), p);
                    }
                }
            }
        });
        if ui
            .add_enabled(
                !auto,
                egui::Button::new(t("Automatic (from the stage plan)")),
            )
            .clicked()
        {
            for fid in targets(cx, self.selected) {
                if let Some(fm) = cx.engine.show_mut().fixture_mut(fid) {
                    fm.place3d = None;
                }
            }
        }
    }
}
