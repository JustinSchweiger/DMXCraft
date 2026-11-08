//! Seitenleisten im Bau-Modus: links Raum, Objektbaum und Geräte, rechts
//! die Eigenschaften der Auswahl (angedockt).

use dmxcraft_core::venue::{Material, Object, Placement, Shape, Transform, TrussProfile, Venue};
use dmxcraft_core::FixtureId;
use eframe::egui::{self, Color32};
use glam::Vec3;

use super::{
    aim_selected, apply_placement, renumber, rig, targets, vec3_row, DialogKind, Pick, VizView,
};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{fold_section, Icon, IconButton, Segmented};
use crate::views::ViewCtx;

/// Reiter der linken Leiste.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OutlineTab {
    #[default]
    Objects,
    Fixtures,
}

impl VizView {
    pub(super) fn outline_panel(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let n_fx = cx.engine.show().fixtures.len();
        let objects = t("Objekte");
        let fixtures = tr!("Geräte ({n})", n = n_fx);
        Segmented::new(&[
            (OutlineTab::Objects, objects),
            (OutlineTab::Fixtures, fixtures.as_str()),
        ])
        .show(ui, &mut self.outline);
        ui.add_space(theme::SP_S);
        if self.outline == OutlineTab::Fixtures {
            ui.add(
                egui::TextEdit::singleline(&mut self.fixture_filter)
                    .hint_text(t("Suchen"))
                    .desired_width(f32::INFINITY),
            );
            ui.label(
                egui::RichText::new(t("Klick wählt aus, Strg: hinzufügen, Umschalt: Bereich"))
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
        fold_section(ui, "viz_room", t("Raum"), false, |ui| {
            ui.horizontal(|ui| {
                ui.label(t("Name"));
                ui.text_edit_singleline(&mut v.name);
            });
            ui.horizontal(|ui| {
                ui.label(t("Größe (m)"));
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
            if ui
                .button(t("Leeren Raum mit dieser Größe anlegen"))
                .clicked()
            {
                let (size, name, env) = (v.size, v.name.clone(), v.environment);
                *v = Venue::room(name, size);
                v.environment = env;
                self.selected = None;
            }
        });
        fold_section(ui, "viz_add", t("Hinzufügen"), true, |ui| {
            ui.horizontal_wrapped(|ui| {
                let at = self.camera.map_or(Vec3::ZERO, |c| c.target);
                let items: [(&str, Shape, [f32; 3]); 6] = [
                    (
                        t("Quader"),
                        Shape::Box {
                            size: [1.0, 1.0, 1.0],
                        },
                        [at.x, 0.5, at.z],
                    ),
                    (
                        t("Podest"),
                        Shape::Box {
                            size: [2.0, 0.4, 1.0],
                        },
                        [at.x, 0.2, at.z],
                    ),
                    (
                        t("Fläche"),
                        Shape::Plane { size: [2.0, 2.0] },
                        [at.x, 0.01, at.z],
                    ),
                    (
                        t("Zylinder"),
                        Shape::Cylinder {
                            radius: 0.2,
                            height: 2.0,
                        },
                        [at.x, 1.0, at.z],
                    ),
                    (
                        t("Traverse 2 m"),
                        Shape::Truss {
                            length: 2.0,
                            profile: TrussProfile::Triangle,
                            width: 0.29,
                        },
                        [at.x, 4.0, at.z],
                    ),
                    (
                        t("Traverse 3 m"),
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
                if ui.button(t("Gruppe")).clicked() {
                    let o = v.object(
                        t("Gruppe"),
                        Shape::Group,
                        Transform::default(),
                        Material::default(),
                    );
                    self.selected = Some(Pick::Object(o.id));
                    v.objects.push(o);
                }
            });
            let loading = self.model_job.is_some();
            let label = if loading {
                t("Modell wird geladen …")
            } else {
                t("3D-Modell importieren (glTF/GLB) …")
            };
            if ui.add_enabled(!loading, egui::Button::new(label)).clicked() {
                self.open_dialog(ui.ctx(), DialogKind::ImportModel);
            }
        });
        let reveal = self.reveal.take().map(|id| (id, v.ancestors(id)));
        fold_section(ui, "viz_objects", t("Objekte"), true, |ui| {
            let mut act = TreeAction::None;
            object_tree(ui, &v.objects, 0, self.selected, reveal.as_ref(), &mut act);
            match act {
                TreeAction::None => {}
                TreeAction::Select(id) => self.selected = Some(Pick::Object(id)),
                TreeAction::ToggleHidden(id) => {
                    if let Some(o) = v.find_mut(id) {
                        o.hidden = !o.hidden;
                    }
                }
            }
        });
        self.missing_section(ui, cx);
    }

    fn fixtures_tab(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let show = cx.engine.show();
        let filter = self.fixture_filter.to_lowercase();
        let mut list: Vec<(FixtureId, String, u8, u16, bool)> = show
            .fixtures
            .iter()
            .filter(|f| filter.is_empty() || f.name.to_lowercase().contains(&filter))
            .map(|f| {
                (
                    f.id,
                    f.name.clone(),
                    f.universe,
                    f.address,
                    f.place3d.is_none(),
                )
            })
            .collect();
        list.sort_by_key(|f| (f.2, f.3));
        let mods = ui.input(|i| i.modifiers);
        let mut clicked = None;
        for (i, (id, name, uni, addr, auto)) in list.iter().enumerate() {
            let sel = cx.state.selected.contains(id);
            let r = ui
                .horizontal(|ui| {
                    let r = ui.selectable_label(sel, name);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if *auto {
                            ui.label(
                                egui::RichText::new(t("auto"))
                                    .small()
                                    .color(theme::text_faint()),
                            )
                            .on_hover_text(t("Lage aus dem Bühnenplan abgeleitet"));
                        }
                        ui.label(
                            egui::RichText::new(format!("{}·{}", uni + 1, addr))
                                .small()
                                .color(theme::text_dim()),
                        );
                    });
                    r
                })
                .inner;
            if r.clicked() {
                clicked = Some((i, *id));
            }
            if self.reveal_fixture == Some(*id) {
                r.scroll_to_me(Some(egui::Align::Center));
                self.reveal_fixture = None;
            }
        }
        if list.is_empty() {
            ui.label(egui::RichText::new(t("Keine Geräte")).color(theme::text_faint()));
        }
        if let Some((i, id)) = clicked {
            let anchor = self
                .fixture_anchor
                .and_then(|a| list.iter().position(|f| f.0 == a));
            if mods.shift {
                if let Some(a) = anchor {
                    let (lo, hi) = (a.min(i), a.max(i));
                    if !mods.command {
                        cx.state.selected.clear();
                    }
                    cx.state.selected.extend(list[lo..=hi].iter().map(|f| f.0));
                } else {
                    cx.state.selected = [id].into();
                }
            } else if mods.command {
                if !cx.state.selected.remove(&id) {
                    cx.state.selected.insert(id);
                }
                self.fixture_anchor = Some(id);
            } else {
                cx.state.selected = [id].into();
                self.fixture_anchor = Some(id);
            }
            self.selected = Some(Pick::Fixture(id));
        }
    }

    fn missing_section(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        if self.missing.is_empty() {
            return;
        }
        let title = tr!("Ohne Gegenstück im Patch ({n})", n = self.missing.len());
        fold_section(ui, "viz_missing", &title, true, |ui| {
            ui.label(egui::RichText::new(t("Diese Geräte aus dem Import haben im Patch kein Gerät mit gleichem Universum und gleicher Adresse. Vorgeschlagen ist ein passender Typ aus der Bibliothek (kursiv: nur ähnlich, bitte prüfen).")).small().color(theme::text_faint()));
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
                            "{m} {t} ({mode} · {n} Kanäle)",
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
                    .add_enabled(n > 0, egui::Button::new(tr!("{n} Geräte anlegen", n = n)))
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
                        cx.state.info(tr!("{n} Geräte angelegt", n = made));
                    } else {
                        cx.state.error(tr!(
                            "{n} Geräte angelegt, nicht möglich: {f}",
                            n = made,
                            f = failed.join(", ")
                        ));
                    }
                }
                if ui.button(t("Liste ausblenden")).clicked() {
                    self.missing.clear();
                }
            });
        });
    }

    /// Rechte Leiste: Eigenschaften der Auswahl.
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
                            "Objekt oder Gerät anklicken (im Bild oder links in der Liste), um es zu bearbeiten.",
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
        let mut remove = false;
        let mut duplicate = false;
        let Some(o) = v.find_mut(id) else {
            self.selected = None;
            return;
        };
        fold_section(ui, "viz_p_general", t("Allgemein"), true, |ui| {
            ui.horizontal(|ui| {
                ui.label(t("Name"));
                ui.text_edit_singleline(&mut o.name);
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut o.locked, t("Gesperrt"));
                ui.checkbox(&mut o.hidden, t("Ausgeblendet"));
            });
        });
        let enabled = !o.locked;
        ui.add_enabled_ui(enabled, |ui| {
            fold_section(ui, "viz_p_place", t("Lage"), true, |ui| {
                vec3_row(ui, t("Position (m)"), &mut o.transform.position, 0.01);
                vec3_row(ui, t("Drehung (°)"), &mut o.transform.rotation, 0.5);
                vec3_row(ui, t("Skalierung"), &mut o.transform.scale, 0.01);
                ui.horizontal(|ui| {
                    if ui.button(t("Drehung zurücksetzen")).clicked() {
                        o.transform.rotation = [0.0; 3];
                    }
                    if ui.button(t("Auf den Boden")).clicked() {
                        o.transform.position[1] = match &o.shape {
                            Shape::Box { size } => size[1] * 0.5 * o.transform.scale[1],
                            Shape::Cylinder { height, .. } => height * 0.5 * o.transform.scale[1],
                            _ => 0.0,
                        };
                    }
                });
            });
            if !matches!(o.shape, Shape::Group) {
                fold_section(ui, "viz_p_shape", t("Form"), true, |ui| {
                    match &mut o.shape {
                        Shape::Box { size } => vec3_row(ui, t("Maße (m)"), size, 0.01),
                        Shape::Plane { size } => {
                            ui.horizontal(|ui| {
                                ui.label(t("Maße (m)"));
                                for s in size.iter_mut() {
                                    ui.add(egui::DragValue::new(s).speed(0.01).range(0.01..=500.0));
                                }
                            });
                        }
                        Shape::Cylinder { radius, height } => {
                            ui.horizontal(|ui| {
                                ui.label(t("Radius / Höhe (m)"));
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
                                ui.label(t("Länge (m)"));
                                ui.add(egui::DragValue::new(length).speed(0.05).range(0.25..=30.0));
                                let mut sq = *profile == TrussProfile::Square;
                                ui.checkbox(&mut sq, t("Vierpunkt"));
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
                                        "Modell {n} · {k} Dreiecke · {x} × {y} × {z} m",
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
                            ui.label(t("Farbe"));
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
                    ui.add(
                        egui::Slider::new(&mut o.material.opacity, 0.05..=1.0).text(t("Deckkraft")),
                    );
                    ui.add(egui::Slider::new(&mut o.material.gloss, 0.0..=1.0).text(t("Glanz")));
                    ui.add(
                        egui::Slider::new(&mut o.material.emissive, 0.0..=1.0).text(t("Leuchten")),
                    );
                });
            }
        });
        ui.horizontal(|ui| {
            duplicate = ui
                .add_enabled(enabled, egui::Button::new(t("Duplizieren")))
                .clicked();
            remove = ui
                .add_enabled(enabled, egui::Button::new(t("Löschen")))
                .clicked();
        });
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
        fold_section(ui, "viz_f_place", t("Lage"), true, |ui| {
            vec3_row(ui, t("Position (m)"), &mut p.position, 0.01);
            vec3_row(ui, t("Drehung (°)"), &mut p.rotation, 0.5);
            ui.label(
                egui::RichText::new(t(
                    "Drehung 0/0/0 strahlt senkrecht nach unten. Im Bild drehen die Ringe um die Achsen des Raums.",
                ))
                .small()
                .color(theme::text_faint()),
            );
        });
        if p != before {
            apply_placement(cx, Pick::Fixture(id), p);
        }
        let title = if n > 1 {
            tr!("Ausrichten ({n} Geräte)", n = n)
        } else {
            t("Ausrichten").to_string()
        };
        fold_section(ui, "viz_f_aim", &title, true, |ui| {
            ui.horizontal_wrapped(|ui| {
                let dirs: [(&str, [f32; 3]); 4] = [
                    (t("Unten"), [0.0, -1.0, 0.0]),
                    (t("Oben"), [0.0, 1.0, 0.0]),
                    (t("Vorne"), [0.0, 0.0, 1.0]),
                    (t("Hinten"), [0.0, 0.0, -1.0]),
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
            vec3_row(ui, t("Zielpunkt (m)"), &mut self.aim, 0.05);
            ui.horizontal_wrapped(|ui| {
                if ui.button(t("Auf Zielpunkt richten")).clicked() {
                    aim_selected(cx, self.selected, self.aim);
                }
                if ui
                    .selectable_label(self.aim_pick, t("Zielpunkt im Bild wählen"))
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
                .add_enabled(
                    has_truss,
                    egui::Button::new(t("An nächste Traverse hängen")),
                )
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
                egui::Button::new(t("Automatisch (aus dem Bühnenplan)")),
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

enum TreeAction {
    None,
    Select(u32),
    ToggleHidden(u32),
}

/// Objektbaum; Gruppen lassen sich auf- und zuklappen.
fn object_tree(
    ui: &mut egui::Ui,
    list: &[Object],
    depth: usize,
    sel: Option<Pick>,
    reveal: Option<&(u32, Vec<u32>)>,
    act: &mut TreeAction,
) {
    for o in list {
        let id = ui.make_persistent_id(("viz_tree", o.id));
        let has_children = !o.children.is_empty();
        let mut state =
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
        if reveal.is_some_and(|r| r.1.contains(&o.id)) {
            state.set_open(true);
        }
        let open = state.is_open();
        let row = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.add_space(depth as f32 * 12.0);
            if has_children {
                let icon = if open {
                    Icon::ChevronDown
                } else {
                    Icon::ChevronRight
                };
                if ui.add(IconButton::new(icon).size(18.0).flat()).clicked() {
                    state.toggle(ui);
                }
            } else {
                ui.add_space(18.0 + 2.0);
            }
            if ui
                .add(
                    IconButton::new(if o.hidden { Icon::EyeOff } else { Icon::Eye })
                        .size(18.0)
                        .flat(),
                )
                .clicked()
            {
                *act = TreeAction::ToggleHidden(o.id);
            }
            let active = sel == Some(Pick::Object(o.id));
            let mut text = egui::RichText::new(&o.name);
            if o.hidden {
                text = text.color(theme::text_faint());
            }
            if o.locked {
                text = text.italics();
            }
            let r = ui.selectable_label(active, text);
            if has_children {
                ui.label(
                    egui::RichText::new(o.children.len().to_string())
                        .small()
                        .color(theme::text_faint()),
                );
            }
            if r.clicked() {
                *act = TreeAction::Select(o.id);
            }
            if r.double_clicked() && has_children {
                state.toggle(ui);
            }
            r
        });
        if reveal.is_some_and(|r| r.0 == o.id) {
            row.inner.scroll_to_me(Some(egui::Align::Center));
        }
        state.store(ui.ctx());
        if has_children && state.is_open() {
            object_tree(ui, &o.children, depth + 1, sel, reveal, act);
        }
    }
}
