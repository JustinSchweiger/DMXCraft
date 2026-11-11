//! Sidebars in build mode: room, object tree and fixtures on the left, the
//! properties of the selection on the right (docked).

use dmxcraft_core::venue::{Material, Shape, Transform, TrussProfile, Venue};
use eframe::egui;
use glam::Vec3;

use super::library;
use super::props::{button, vec3_field};
use super::{DialogKind, Pick, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{fold_section, form, text_field, Icon, Segmented, Toggle};
use crate::views::ViewCtx;

/// Choice in the image menu.
pub(super) enum TexAction {
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
                text_field(&mut self.fixture_filter)
                    .hint_text(t("Search"))
                    .desired_width(f32::INFINITY),
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
            form(ui, "viz_room_form", |f| {
                f.row(t("Name"), |ui| {
                    ui.add(text_field(&mut v.name).desired_width(f32::INFINITY));
                });
                f.row(t("Size (m)"), |ui| {
                    vec3_field(ui, &mut v.size, 0.05, 1.0..=200.0, 0.0);
                });
            });
            ui.add_space(theme::SP_XS);
            if button(ui, t("Create an empty room of this size"), true).clicked() {
                let (size, name, env) = (v.size, v.name.clone(), v.environment);
                *v = Venue::room(name, size);
                v.environment = env;
                self.selected = None;
            }
        });
        fold_section(ui, "viz_add", t("Add"), true, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(theme::SP_XS, theme::SP_XS);
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
                    if button(ui, label, true).clicked() {
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
            ui.add_space(theme::SP_XS);
            if button(ui, label, !loading).clicked() {
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
        ui.label(theme::heading_text(t("Built-in")));
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(theme::SP_XS, theme::SP_XS);
            for p in builtin.iter() {
                if button(ui, &p.name, true).clicked() {
                    insert = Some(p.clone());
                }
            }
        });
        let mut delete = None;
        if !user.is_empty() {
            ui.add_space(theme::SP_S);
            ui.label(theme::heading_text(t("Custom")));
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(theme::SP_XS, theme::SP_XS);
                for (i, u) in user.iter().enumerate() {
                    let r = button(ui, &u.prefab.name, true);
                    if r.clicked() {
                        insert = Some(u.prefab.clone());
                    }
                    menu::context(&r, |ui| {
                        menu::begin(ui);
                        if menu::item(
                            ui,
                            MenuItem::new(t("Remove from library"))
                                .icon(Icon::Trash)
                                .danger(),
                        ) {
                            delete = Some(i);
                        }
                    });
                }
            });
        }
        if let Some(i) = delete {
            let u = user.remove(i);
            if let Err(e) = std::fs::remove_file(&u.path) {
                cx.state.error(e.to_string());
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
                    ui.add(Toggle::new(&mut m.create, ""));
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
                ui.spacing_mut().item_spacing = egui::vec2(theme::SP_XS, theme::SP_XS);
                if button(ui, &tr!("Create {n} fixtures", n = n), n > 0).clicked() {
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
                if button(ui, t("Hide list"), true).clicked() {
                    self.missing.clear();
                }
            });
        });
    }
}
