//! Linke Leiste: Objektbaum (Gruppen als Ordner) und Geräteliste mit
//! Ordnern – auswählen, umbenennen, per Ziehen einsortieren, sortieren.

use std::collections::BTreeSet;

use dmxcraft_core::venue::{Material, Object, Shape, Transform, Venue};
use dmxcraft_core::FixtureId;
use eframe::egui;
use glam::{Mat4, Vec3};

use super::{deg_quat, renumber, Pick, VizView};
use crate::i18n::t;
use crate::ui::theme;
use crate::ui::widgets::{Icon, IconButton};
use crate::views::ViewCtx;

/// Was gerade umbenannt wird.
#[derive(Clone, Debug, PartialEq)]
pub enum Renaming {
    Object(u32, String),
    Folder(u32, String),
}

/// Gezogene Objekte bzw. Geräte.
struct DragObjects(Vec<u32>);
struct DragFixtures(Vec<FixtureId>);

/// Abschnitt der Geräteliste: Ordner (ID, Name) bzw. „ohne Ordner“, Zeilen.
type Section = (Option<(u32, String)>, Vec<usize>);

/// Sortierung der Geräteliste.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum FixtureSort {
    #[default]
    Address,
    Name,
}

enum TreeAction {
    Select(u32),
    ToggleHidden(u32),
    ToggleLock(u32),
    /// Objekte in eine Gruppe (bzw. oberste Ebene) legen, optional vor ein Objekt.
    Drop {
        ids: Vec<u32>,
        parent: Option<u32>,
        before: Option<u32>,
    },
    StartRename(u32),
    NewFolder(Option<u32>),
    GroupSelection(Option<u32>),
    Sort(Option<u32>),
    Duplicate(u32),
    Delete(u32),
}

struct Tree<'a> {
    sel: &'a BTreeSet<u32>,
    reveal: Option<&'a (u32, Vec<u32>)>,
    renaming: &'a mut Option<Renaming>,
    /// Feld nur beim Start fokussieren
    focus: &'a mut bool,
    acts: Vec<TreeAction>,
    /// Umbenennen abgeschlossen: (ID, neuer Name)
    renamed: Option<(u32, String)>,
}

/// Auswählbare Zeile, die sich ziehen lässt (Inhalt `payload` beim Ziehen).
fn drag_row<P: std::any::Any + Send + Sync>(
    ui: &mut egui::Ui,
    active: bool,
    text: egui::RichText,
    ghost: &str,
    payload: P,
) -> egui::Response {
    let r = ui.add(egui::Button::selectable(active, text).sense(egui::Sense::click_and_drag()));
    if r.drag_started() {
        egui::DragAndDrop::set_payload(ui.ctx(), payload);
    }
    if r.dragged() {
        if let Some(p) = ui.ctx().pointer_interact_pos() {
            let painter = ui.ctx().layer_painter(egui::LayerId::new(
                egui::Order::Tooltip,
                egui::Id::unique("viz_drag_ghost"),
            ));
            let g = painter.layout_no_wrap(ghost.to_string(), theme::body(), theme::text());
            let rect = egui::Rect::from_min_size(p + egui::vec2(12.0, 4.0), g.size()).expand(4.0);
            painter.rect_filled(rect, theme::R_S, theme::raised());
            painter.galley(rect.min + egui::vec2(4.0, 4.0), g, theme::text());
        }
    }
    r
}

/// Breite Markierung, wo beim Loslassen eingefügt wird.
fn drop_marker(ui: &egui::Ui, rect: egui::Rect, into: bool) {
    let p = ui.painter();
    if into {
        p.rect_stroke(
            rect.expand(1.0),
            theme::R_S,
            egui::Stroke::new(1.5, theme::accent()),
            egui::StrokeKind::Outside,
        );
    } else {
        p.hline(
            rect.x_range(),
            rect.top(),
            egui::Stroke::new(2.0, theme::accent()),
        );
    }
}

impl Tree<'_> {
    fn list(&mut self, ui: &mut egui::Ui, list: &[Object], depth: usize, parent: Option<u32>) {
        for o in list {
            self.row(ui, o, depth, parent);
        }
    }

    fn row(&mut self, ui: &mut egui::Ui, o: &Object, depth: usize, parent: Option<u32>) {
        let is_group = matches!(o.shape, Shape::Group);
        let id = ui.make_persistent_id(("viz_tree", o.id));
        let mut state =
            egui::collapsing_header::CollapsingState::load_with_default_open(ui.ctx(), id, false);
        if self.reveal.is_some_and(|r| r.1.contains(&o.id)) {
            state.set_open(true);
        }
        let open = state.is_open();
        let active = self.sel.contains(&o.id);
        let row = ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            ui.add_space(depth as f32 * 12.0);
            if is_group {
                let icon = if open {
                    Icon::ChevronDown
                } else {
                    Icon::ChevronRight
                };
                if ui.add(IconButton::new(icon).size(18.0).flat()).clicked() {
                    state.toggle(ui);
                }
            } else {
                ui.add_space(20.0);
            }
            if ui
                .add(
                    IconButton::new(if o.hidden { Icon::EyeOff } else { Icon::Eye })
                        .size(18.0)
                        .flat(),
                )
                .clicked()
            {
                self.acts.push(TreeAction::ToggleHidden(o.id));
            }
            if let Some(Renaming::Object(rid, text)) = self.renaming.as_mut() {
                if *rid == o.id {
                    let r = ui.add(egui::TextEdit::singleline(text).desired_width(150.0));
                    if std::mem::take(self.focus) {
                        r.request_focus();
                    } else if r.lost_focus() || !r.has_focus() {
                        self.renamed = Some((o.id, text.clone()));
                    }
                    return None;
                }
            }
            // Ziehen: alle ausgewählten, wenn dieses dazugehört
            let payload = if active {
                DragObjects(self.sel.iter().copied().collect())
            } else {
                DragObjects(vec![o.id])
            };
            let mut text = egui::RichText::new(&o.name);
            if o.hidden {
                text = text.color(theme::text_faint());
            }
            if o.locked {
                text = text.italics();
            }
            let label = drag_row(ui, active, text, &o.name, payload);
            if is_group {
                ui.label(
                    egui::RichText::new(o.children.len().to_string())
                        .small()
                        .color(theme::text_faint()),
                );
            }
            Some(label)
        });
        let rect = row.response.rect;
        if let Some(label) = row.inner {
            if label.clicked() {
                self.acts.push(TreeAction::Select(o.id));
            }
            if label.double_clicked() {
                if is_group {
                    state.toggle(ui);
                } else {
                    self.acts.push(TreeAction::StartRename(o.id));
                }
            }
            label.context_menu(|ui| {
                ui.set_min_width(200.0);
                if ui.button(t("Rename")).clicked() {
                    self.acts.push(TreeAction::StartRename(o.id));
                    ui.close();
                }
                let here = if is_group { Some(o.id) } else { parent };
                if ui.button(t("New folder")).clicked() {
                    self.acts.push(TreeAction::NewFolder(here));
                    ui.close();
                }
                if ui.button(t("Selection into new folder")).clicked() {
                    if !active {
                        self.acts.push(TreeAction::Select(o.id));
                    }
                    self.acts.push(TreeAction::GroupSelection(parent));
                    ui.close();
                }
                if is_group && ui.button(t("Sort contents by name")).clicked() {
                    self.acts.push(TreeAction::Sort(Some(o.id)));
                    ui.close();
                }
                ui.separator();
                let lock = if o.locked { t("Unlock") } else { t("Lock") };
                if ui.button(lock).clicked() {
                    self.acts.push(TreeAction::ToggleLock(o.id));
                    ui.close();
                }
                if ui.button(t("Duplicate")).clicked() {
                    self.acts.push(TreeAction::Duplicate(o.id));
                    ui.close();
                }
                if ui.button(t("Delete")).clicked() {
                    self.acts.push(TreeAction::Delete(o.id));
                    ui.close();
                }
            });
        }
        // Ablegen: auf eine Gruppe → hinein, sonst davor
        let target = ui.interact(rect, id.with("drop"), egui::Sense::hover());
        if target.dnd_hover_payload::<DragObjects>().is_some() {
            drop_marker(ui, rect, is_group);
        }
        if let Some(p) = target.dnd_release_payload::<DragObjects>() {
            let ids = p.0.clone();
            self.acts.push(if is_group {
                TreeAction::Drop {
                    ids,
                    parent: Some(o.id),
                    before: None,
                }
            } else {
                TreeAction::Drop {
                    ids,
                    parent,
                    before: Some(o.id),
                }
            });
        }
        if self.reveal.is_some_and(|r| r.0 == o.id) {
            ui.scroll_to_rect(rect, Some(egui::Align::Center));
        }
        state.store(ui.ctx());
        if is_group && state.is_open() {
            self.list(ui, &o.children, depth + 1, Some(o.id));
        }
    }
}

/// Objekte in eine andere Gruppe legen; die Lage im Raum bleibt erhalten.
pub fn move_objects(v: &mut Venue, ids: &[u32], parent: Option<u32>, before: Option<u32>) {
    for &id in ids {
        // nicht in sich selbst oder eigene Untergruppen
        if Some(id) == parent || parent.is_some_and(|p| v.ancestors(p).contains(&id)) {
            continue;
        }
        if before == Some(id) {
            continue;
        }
        let Some(o) = v.find(id) else {
            continue;
        };
        let world = Mat4::from_cols_array_2d(&v.parent_matrix(id))
            * Mat4::from_cols_array_2d(&o.transform.matrix());
        let Some(mut o) = v.remove(id) else {
            continue;
        };
        let p = match parent {
            Some(p) => {
                let Some(g) = v.find(p) else {
                    v.objects.push(o);
                    continue;
                };
                Mat4::from_cols_array_2d(&v.parent_matrix(p))
                    * Mat4::from_cols_array_2d(&g.transform.matrix())
            }
            None => Mat4::IDENTITY,
        };
        let (scale, rot, pos) = (p.inverse() * world).to_scale_rotation_translation();
        let r3 = |v: Vec3| v.to_array().map(|x| (x * 10000.0).round() / 10000.0);
        o.transform = Transform {
            position: r3(pos),
            rotation: deg_quat(rot),
            scale: r3(scale),
        };
        let Some(list) = v.list_mut(parent) else {
            v.objects.push(o);
            continue;
        };
        let at = before
            .and_then(|b| list.iter().position(|x| x.id == b))
            .unwrap_or(list.len());
        list.insert(at, o);
    }
}

impl VizView {
    /// Objektbaum mit Ordnern (Gruppen).
    pub(super) fn object_tree(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        ui.horizontal(|ui| {
            if ui
                .add(IconButton::new(Icon::Plus).size(22.0))
                .on_hover_text(t("New folder"))
                .clicked()
            {
                let v = &mut cx.engine.show_mut().venue;
                let o = v.object(
                    t("Folder"),
                    Shape::Group,
                    Transform::default(),
                    Material::default(),
                );
                let id = o.id;
                v.objects.push(o);
                self.renaming = Some(Renaming::Object(id, t("Folder").to_string()));
                self.rename_focus = true;
            }
            if ui
                .button(t("A–Z"))
                .on_hover_text(t("Sort top level by name"))
                .clicked()
            {
                cx.engine.show_mut().venue.sort_by_name(None);
            }
            if self.sel_objects.len() > 1 {
                ui.label(
                    egui::RichText::new(crate::tr!("{n} selected", n = self.sel_objects.len()))
                        .small()
                        .color(theme::text_dim()),
                );
            }
        });
        let reveal = self
            .reveal
            .take()
            .map(|id| (id, cx.engine.show().venue.ancestors(id)));
        let sel = self.sel_objects.clone();
        let mut tree = Tree {
            sel: &sel,
            reveal: reveal.as_ref(),
            renaming: &mut self.renaming,
            focus: &mut self.rename_focus,
            acts: Vec::new(),
            renamed: None,
        };
        let objects = std::mem::take(&mut cx.engine.show_mut().venue.objects);
        tree.list(ui, &objects, 0, None);
        cx.engine.show_mut().venue.objects = objects;
        // freie Fläche darunter: auf oberste Ebene ablegen
        let (rect, resp) =
            ui.allocate_exact_size(egui::vec2(ui.available_width(), 40.0), egui::Sense::hover());
        if resp.dnd_hover_payload::<DragObjects>().is_some() {
            drop_marker(ui, rect, true);
        }
        if let Some(p) = resp.dnd_release_payload::<DragObjects>() {
            tree.acts.push(TreeAction::Drop {
                ids: p.0.clone(),
                parent: None,
                before: None,
            });
        }
        let Tree { acts, renamed, .. } = tree;
        if let Some((id, name)) = renamed {
            if let Some(o) = cx.engine.show_mut().venue.find_mut(id) {
                if !name.trim().is_empty() {
                    o.name = name.trim().to_string();
                }
            }
            self.renaming = None;
        }
        let add = ui.input(|i| i.modifiers.command || i.modifiers.shift);
        for a in acts {
            let v = &mut cx.engine.show_mut().venue;
            match a {
                TreeAction::Select(id) => self.select_object_in(&mut cx.state.selected, id, add),
                TreeAction::ToggleHidden(id) => {
                    if let Some(o) = v.find_mut(id) {
                        o.hidden = !o.hidden;
                    }
                }
                TreeAction::ToggleLock(id) => {
                    if let Some(o) = v.find_mut(id) {
                        o.locked = !o.locked;
                    }
                }
                TreeAction::Drop {
                    ids,
                    parent,
                    before,
                } => move_objects(v, &ids, parent, before),
                TreeAction::StartRename(id) => {
                    if let Some(o) = v.find(id) {
                        self.renaming = Some(Renaming::Object(id, o.name.clone()));
                        self.rename_focus = true;
                    }
                }
                TreeAction::NewFolder(parent) => {
                    let o = v.object(
                        t("Folder"),
                        Shape::Group,
                        Transform::default(),
                        Material::default(),
                    );
                    let id = o.id;
                    if let Some(list) = v.list_mut(parent) {
                        list.push(o);
                    }
                    self.renaming = Some(Renaming::Object(id, t("Folder").to_string()));
                    self.rename_focus = true;
                }
                TreeAction::GroupSelection(parent) => {
                    let o = v.object(
                        t("Folder"),
                        Shape::Group,
                        Transform::default(),
                        Material::default(),
                    );
                    let gid = o.id;
                    if let Some(list) = v.list_mut(parent) {
                        list.push(o);
                    }
                    let ids: Vec<u32> = self.sel_objects.iter().copied().collect();
                    move_objects(v, &ids, Some(gid), None);
                    self.reveal = Some(gid);
                    self.renaming = Some(Renaming::Object(gid, t("Folder").to_string()));
                    self.rename_focus = true;
                }
                TreeAction::Sort(parent) => v.sort_by_name(parent),
                TreeAction::Duplicate(id) => {
                    if let Some(mut copy) = v.find(id).cloned() {
                        renumber(v, &mut copy);
                        let parent = v.parent_of(id);
                        if let Some(list) = v.list_mut(parent) {
                            let at = list
                                .iter()
                                .position(|x| x.id == id)
                                .map_or(list.len(), |i| i + 1);
                            self.selected = Some(Pick::Object(copy.id));
                            self.sel_objects = [copy.id].into();
                            list.insert(at, copy);
                        }
                    }
                }
                TreeAction::Delete(id) => {
                    v.remove(id);
                    v.prune_models();
                    self.sel_objects.remove(&id);
                    if self.selected == Some(Pick::Object(id)) {
                        self.selected = None;
                    }
                }
            }
        }
    }

    /// Geräteliste mit Ordnern.
    pub(super) fn fixture_list(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let show = cx.engine.show();
        let filter = self.fixture_filter.to_lowercase();
        let mut all: Vec<(FixtureId, String, u8, u16, bool)> = show
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
        match self.fixture_sort {
            FixtureSort::Address => all.sort_by_key(|f| (f.2, f.3)),
            FixtureSort::Name => all.sort_by(|a, b| natural(&a.1, &b.1)),
        }
        let folders: Vec<(u32, String, Vec<FixtureId>)> = show
            .venue
            .fixture_folders
            .iter()
            .map(|d| (d.id, d.name.clone(), d.fixtures.clone()))
            .collect();
        // Reihenfolge der Anzeige (für Umschalt-Bereich): Ordner, dann ohne Ordner
        let mut order: Vec<FixtureId> = Vec::new();
        let mut sections: Vec<Section> = Vec::new();
        for (fid, name, ids) in &folders {
            let rows: Vec<usize> = (0..all.len())
                .filter(|i| ids.contains(&all[*i].0))
                .collect();
            sections.push((Some((*fid, name.clone())), rows));
        }
        let loose: Vec<usize> = (0..all.len())
            .filter(|i| !folders.iter().any(|d| d.2.contains(&all[*i].0)))
            .collect();
        sections.push((None, loose));
        let mods = ui.input(|i| i.modifiers);
        let mut clicked: Option<FixtureId> = None;
        let mut moves: Vec<(Vec<FixtureId>, Option<u32>)> = Vec::new();
        let mut new_folder_with: Option<Vec<FixtureId>> = None;
        let mut delete_folder = None;
        let mut select_folder: Option<Vec<FixtureId>> = None;
        let mut renamed: Option<(u32, String)> = None;
        let selected = cx.state.selected.clone();
        for (folder, rows) in &sections {
            let fid = folder.as_ref().map(|f| f.0);
            let ids_here: Vec<FixtureId> = rows.iter().map(|i| all[*i].0).collect();
            // Kopf des Ordners (Ablageziel)
            let head_id = ui.make_persistent_id(("viz_fx_folder", fid));
            let mut state = egui::collapsing_header::CollapsingState::load_with_default_open(
                ui.ctx(),
                head_id,
                true,
            );
            let open = state.is_open();
            let head = ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                let icon = if open {
                    Icon::ChevronDown
                } else {
                    Icon::ChevronRight
                };
                if ui.add(IconButton::new(icon).size(18.0).flat()).clicked() {
                    state.toggle(ui);
                }
                match (folder, self.renaming.as_mut()) {
                    (Some((id, _)), Some(Renaming::Folder(rid, text))) if rid == id => {
                        let r = ui.add(egui::TextEdit::singleline(text).desired_width(150.0));
                        if std::mem::take(&mut self.rename_focus) {
                            r.request_focus();
                        } else if r.lost_focus() || !r.has_focus() {
                            renamed = Some((*id, text.clone()));
                        }
                        None
                    }
                    _ => {
                        let title = match folder {
                            Some((_, n)) => n.clone(),
                            None => t("No folder").to_string(),
                        };
                        let r = ui.add(
                            egui::Label::new(theme::heading_text(&title))
                                .sense(egui::Sense::click()),
                        );
                        ui.label(
                            egui::RichText::new(rows.len().to_string())
                                .small()
                                .color(theme::text_faint()),
                        );
                        Some(r)
                    }
                }
            });
            if let Some(r) = head.inner {
                if r.clicked() {
                    state.toggle(ui);
                }
                r.context_menu(|ui| {
                    ui.set_min_width(180.0);
                    if ui.button(t("Select all")).clicked() {
                        select_folder = Some(ids_here.clone());
                        ui.close();
                    }
                    if let Some((id, name)) = folder {
                        if ui.button(t("Rename")).clicked() {
                            self.renaming = Some(Renaming::Folder(*id, name.clone()));
                            self.rename_focus = true;
                            ui.close();
                        }
                        if ui.button(t("Dissolve folder")).clicked() {
                            delete_folder = Some(*id);
                            ui.close();
                        }
                    }
                });
            }
            let target = ui.interact(
                head.response.rect,
                head_id.with("drop"),
                egui::Sense::hover(),
            );
            if target.dnd_hover_payload::<DragFixtures>().is_some() {
                drop_marker(ui, head.response.rect, true);
            }
            if let Some(p) = target.dnd_release_payload::<DragFixtures>() {
                moves.push((p.0.clone(), fid));
            }
            state.store(ui.ctx());
            if !state.is_open() {
                continue;
            }
            for &i in rows {
                let (id, name, uni, addr, auto) = &all[i];
                order.push(*id);
                let sel = selected.contains(id);
                let payload = if sel {
                    DragFixtures(selected.iter().copied().collect())
                } else {
                    DragFixtures(vec![*id])
                };
                let row = ui.horizontal(|ui| {
                    ui.add_space(14.0);
                    let r = drag_row(ui, sel, egui::RichText::new(name), name, payload);
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if *auto {
                            ui.label(
                                egui::RichText::new(t("auto"))
                                    .small()
                                    .color(theme::text_faint()),
                            )
                            .on_hover_text(t("Placement derived from the stage plan"));
                        }
                        ui.label(
                            egui::RichText::new(format!("{}·{}", uni + 1, addr))
                                .small()
                                .color(theme::text_dim()),
                        );
                    });
                    r
                });
                let r = row.inner;
                if r.clicked() {
                    clicked = Some(*id);
                }
                r.context_menu(|ui| {
                    ui.set_min_width(200.0);
                    let ids: Vec<FixtureId> = if sel {
                        selected.iter().copied().collect()
                    } else {
                        vec![*id]
                    };
                    if ui.button(t("New folder with selection")).clicked() {
                        new_folder_with = Some(ids.clone());
                        ui.close();
                    }
                    ui.menu_button(t("Move to"), |ui| {
                        for (fid, fname, _) in &folders {
                            if ui.button(fname).clicked() {
                                moves.push((ids.clone(), Some(*fid)));
                                ui.close();
                            }
                        }
                        if ui.button(t("No folder")).clicked() {
                            moves.push((ids.clone(), None));
                            ui.close();
                        }
                    });
                });
                // Ablegen auf ein Gerät: in dessen Ordner
                let target = ui.interact(
                    row.response.rect,
                    ui.make_persistent_id(("viz_fx_drop", id.0)),
                    egui::Sense::hover(),
                );
                if target.dnd_hover_payload::<DragFixtures>().is_some() {
                    drop_marker(ui, row.response.rect, false);
                }
                if let Some(p) = target.dnd_release_payload::<DragFixtures>() {
                    moves.push((p.0.clone(), fid));
                }
                if self.reveal_fixture == Some(*id) {
                    ui.scroll_to_rect(row.response.rect, Some(egui::Align::Center));
                    self.reveal_fixture = None;
                }
            }
            ui.add_space(theme::SP_XS);
        }
        if all.is_empty() {
            ui.label(egui::RichText::new(t("No fixtures")).color(theme::text_faint()));
        }
        // ----- anwenden -----
        let v = &mut cx.engine.show_mut().venue;
        for (ids, folder) in moves {
            v.move_fixtures(&ids, folder);
        }
        if let Some(ids) = new_folder_with {
            let id = v.add_fixture_folder(t("Folder"));
            v.move_fixtures(&ids, Some(id));
            self.renaming = Some(Renaming::Folder(id, t("Folder").to_string()));
            self.rename_focus = true;
        }
        if let Some(id) = delete_folder {
            v.fixture_folders.retain(|d| d.id != id);
        }
        if let Some((id, name)) = renamed {
            if let Some(d) = v.fixture_folders.iter_mut().find(|d| d.id == id) {
                if !name.trim().is_empty() {
                    d.name = name.trim().to_string();
                }
            }
            self.renaming = None;
        }
        if let Some(ids) = select_folder {
            cx.state.selected = ids.iter().copied().collect();
            self.selected = ids.first().map(|id| Pick::Fixture(*id));
            self.sel_objects.clear();
        }
        if let Some(id) = clicked {
            let anchor = self
                .fixture_anchor
                .and_then(|a| order.iter().position(|f| *f == a));
            let i = order.iter().position(|f| *f == id).unwrap_or(0);
            if mods.shift {
                if let Some(a) = anchor {
                    let (lo, hi) = (a.min(i), a.max(i));
                    if !mods.command {
                        cx.state.selected.clear();
                    }
                    cx.state.selected.extend(order[lo..=hi].iter().copied());
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
                self.sel_objects.clear();
            }
            self.selected = Some(Pick::Fixture(id));
        }
    }

    /// Kopfzeile der Geräteliste: neuer Ordner, Sortierung.
    pub(super) fn fixture_list_header(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        ui.horizontal(|ui| {
            if ui
                .add(IconButton::new(Icon::Plus).size(22.0))
                .on_hover_text(t("New folder (with the selection)"))
                .clicked()
            {
                let ids: Vec<FixtureId> = cx.state.selected.iter().copied().collect();
                let v = &mut cx.engine.show_mut().venue;
                let id = v.add_fixture_folder(t("Folder"));
                v.move_fixtures(&ids, Some(id));
                self.renaming = Some(Renaming::Folder(id, t("Folder").to_string()));
                self.rename_focus = true;
            }
            egui::ComboBox::from_id_salt("viz_fx_sort")
                .selected_text(match self.fixture_sort {
                    FixtureSort::Address => t("By address"),
                    FixtureSort::Name => t("By name"),
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.fixture_sort,
                        FixtureSort::Address,
                        t("By address"),
                    );
                    ui.selectable_value(&mut self.fixture_sort, FixtureSort::Name, t("By name"));
                });
            let n = cx.state.selected.len();
            if n > 1 {
                ui.label(
                    egui::RichText::new(crate::tr!("{n} selected", n = n))
                        .small()
                        .color(theme::text_dim()),
                );
            }
        });
    }
}

/// Namen mit Zahlen natürlich vergleichen („Par 2“ vor „Par 10“).
fn natural(a: &str, b: &str) -> std::cmp::Ordering {
    let key = |s: &str| -> Vec<(String, u64)> {
        let mut out = Vec::new();
        let mut text = String::new();
        let mut num = String::new();
        for c in s.to_lowercase().chars() {
            if c.is_ascii_digit() {
                num.push(c);
            } else {
                if !num.is_empty() {
                    out.push((std::mem::take(&mut text), num.parse().unwrap_or(0)));
                    num.clear();
                }
                text.push(c);
            }
        }
        out.push((text, num.parse().unwrap_or(0)));
        out
    };
    key(a).cmp(&key(b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn natural_order() {
        let mut v = vec!["Par 10", "Par 2", "Bar", "par 1"];
        v.sort_by(|a, b| natural(a, b));
        assert_eq!(v, ["Bar", "par 1", "Par 2", "Par 10"]);
    }

    #[test]
    fn moving_into_rotated_group_keeps_world_place() {
        let mut v = Venue::room("t", [10.0, 6.0, 10.0]);
        let g = v.object(
            "G",
            Shape::Group,
            Transform {
                position: [2.0, 1.0, 0.0],
                rotation: [0.0, 90.0, 0.0],
                scale: [1.0; 3],
            },
            Material::default(),
        );
        let gid = g.id;
        v.objects.push(g);
        let b = v.object(
            "B",
            Shape::Box { size: [1.0; 3] },
            Transform {
                position: [3.0, 1.0, 0.0],
                ..Transform::default()
            },
            Material::default(),
        );
        let bid = b.id;
        v.objects.push(b);
        move_objects(&mut v, &[bid], Some(gid), None);
        assert_eq!(v.parent_of(bid), Some(gid));
        let (_, m) = v.flatten().into_iter().find(|(o, _)| o.id == bid).unwrap();
        let p = Vec3::new(m[3][0], m[3][1], m[3][2]);
        assert!(p.distance(Vec3::new(3.0, 1.0, 0.0)) < 1e-3, "{p:?}");
        // nicht in sich selbst
        move_objects(&mut v, &[gid], Some(bid), None);
        assert_eq!(v.parent_of(gid), None);
        // zurück auf die oberste Ebene
        move_objects(&mut v, &[bid], None, None);
        assert_eq!(v.parent_of(bid), None);
    }
}
