//! Right-click menu for fixtures – shared by the stage (CONTROL), the stage
//! plan editor and the DMX patch. The menu only reads the show and returns a
//! [`FixtureAction`] that is then executed with [`apply`].

use crate::i18n::t;
use std::collections::BTreeSet;

use dmxcraft_core::{Engine, FixtureId, GroupId, PlanShape, Show};
use eframe::egui;

use crate::state::{AppState, Mode};
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::stage_canvas::fixture_pos;
use crate::ui::widgets::stage_canvas::paint_shape_icon;
use crate::ui::widgets::Icon;

/// Size steps for menus and toolbar.
pub const SIZES: [(&str, f32); 5] = [
    ("Small", 0.7),
    ("Normal", 1.0),
    ("Large", 1.5),
    ("Very large", 2.2),
    ("Huge", 3.2),
];

/// Where the menu was opened (determines the offered entries).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MenuPlace {
    Stage,
    StageEditor,
    Patch,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FixtureAction {
    AddToGroup(GroupId),
    RemoveFromGroup(GroupId),
    NewGroup,
    SelectSameType,
    SelectAll,
    SelectNone,
    InvertOrder,
    CopyLayout,
    PasteLayout,
    MirrorH,
    MirrorV,
    ResetPosition,
    /// `None` = automatic by fixture type.
    SetShape(Option<PlanShape>),
    SetSize(f32),
    /// Set the rotation (degrees).
    SetRotation(f32),
    /// Rotate further by an angle (degrees).
    RotateBy(f32),
    NextFreeAddress,
    ShowInSetup,
    Rename,
    Cut,
    Copy,
    Paste,
    Duplicate,
    Delete,
}

/// Draws the menu entries. `target` is the fixture under the mouse.
pub fn menu(
    ui: &mut egui::Ui,
    show: &Show,
    selected: &BTreeSet<FixtureId>,
    target: Option<FixtureId>,
    place: MenuPlace,
    has_layout_clipboard: bool,
    has_fixture_clipboard: bool,
) -> Option<FixtureAction> {
    let mut out = None;
    let mut act = |ui: &mut egui::Ui, item: MenuItem<'_>, a: FixtureAction| {
        if menu::item(ui, item) {
            out = Some(a);
        }
    };
    menu::begin(ui);
    let n = selected.len();
    let some = n > 0;

    if let Some(f) = target.and_then(|t| show.fixture(t)) {
        let title = if n > 1 {
            tr!("{n} fixtures selected", n = n)
        } else {
            format!("{} · U{}/{}", f.name, f.universe + 1, f.address)
        };
        menu::title(ui, &title);
    }

    // Groups
    menu::sub(
        ui,
        MenuItem::new(t("Add to group"))
            .icon(Icon::Group)
            .enabled(some && !show.groups.is_empty()),
        |ui| {
            for g in &show.groups {
                act(ui, MenuItem::new(&g.name), FixtureAction::AddToGroup(g.id));
            }
        },
    );
    let containing: Vec<_> = show
        .groups
        .iter()
        .filter(|g| g.fixtures.iter().any(|f| selected.contains(f)))
        .collect();
    menu::sub(
        ui,
        MenuItem::new(t("Remove from group")).enabled(!containing.is_empty()),
        |ui| {
            for g in &containing {
                act(
                    ui,
                    MenuItem::new(&g.name),
                    FixtureAction::RemoveFromGroup(g.id),
                );
            }
        },
    );
    act(
        ui,
        MenuItem::new(t("New group from selection"))
            .icon(Icon::Plus)
            .enabled(some),
        FixtureAction::NewGroup,
    );
    menu::separator(ui);

    // Selection
    act(
        ui,
        MenuItem::new(t("Select same fixture type")).enabled(some),
        FixtureAction::SelectSameType,
    );
    act(ui, MenuItem::new(t("Select all")), FixtureAction::SelectAll);
    act(
        ui,
        MenuItem::new(t("Clear selection")).enabled(some),
        FixtureAction::SelectNone,
    );
    act(
        ui,
        MenuItem::new(t("Reverse order")).enabled(n > 1),
        FixtureAction::InvertOrder,
    );

    // Display
    if place != MenuPlace::Patch {
        menu::separator(ui);
        let current_shape = (n == 1)
            .then(|| selected.iter().next().map(|id| show.plan_shape(*id)))
            .flatten();
        menu::sub(ui, MenuItem::new(t("Shape")).enabled(some), |ui| {
            act(
                ui,
                MenuItem::new(t("Automatic (by type)"))
                    .icon_fn(|p, r, c| paint_shape_icon(p, None, r, c)),
                FixtureAction::SetShape(None),
            );
            menu::separator(ui);
            for shape in PlanShape::ALL {
                act(
                    ui,
                    MenuItem::new(t(shape.label()))
                        .icon_fn(move |p, r, c| paint_shape_icon(p, Some(shape), r, c))
                        .checked(current_shape == Some(shape)),
                    FixtureAction::SetShape(Some(shape)),
                );
            }
        });
        let current_size = (n == 1)
            .then(|| selected.iter().next().map(|id| show.plan_size(*id)))
            .flatten();
        menu::sub(ui, MenuItem::new(t("Size")).enabled(some), |ui| {
            for (label, f) in SIZES {
                act(
                    ui,
                    MenuItem::new(t(label))
                        .checked(current_size.is_some_and(|c| (c - f).abs() < 0.01)),
                    FixtureAction::SetSize(f),
                );
            }
        });
        let current_rot = (n == 1)
            .then(|| selected.iter().next().map(|id| show.plan_rotation(*id)))
            .flatten();
        menu::sub(
            ui,
            MenuItem::new(t("Rotate")).icon(Icon::Reset).enabled(some),
            |ui| {
                for deg in [0.0f32, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0] {
                    act(
                        ui,
                        MenuItem::new(&format!("{deg:.0}°"))
                            .checked(current_rot.is_some_and(|c| (c - deg).abs() < 0.01)),
                        FixtureAction::SetRotation(deg),
                    );
                }
                menu::separator(ui);
                for (label, d) in [
                    (t("15° left"), -15.0),
                    (t("5° left"), -5.0),
                    (t("5° right"), 5.0),
                    (t("15° right"), 15.0),
                ] {
                    act(ui, MenuItem::new(label), FixtureAction::RotateBy(d));
                }
            },
        );
    }

    // Arrangement
    if place == MenuPlace::StageEditor {
        menu::separator(ui);
        act(
            ui,
            MenuItem::new(t("Copy layout"))
                .icon(Icon::Copy)
                .enabled(some),
            FixtureAction::CopyLayout,
        );
        act(
            ui,
            MenuItem::new(t("Paste layout"))
                .icon(Icon::Paste)
                .enabled(some && has_layout_clipboard),
            FixtureAction::PasteLayout,
        );
        act(
            ui,
            MenuItem::new(t("Mirror horizontally"))
                .icon(Icon::AlignV)
                .enabled(n > 1),
            FixtureAction::MirrorH,
        );
        act(
            ui,
            MenuItem::new(t("Mirror vertically"))
                .icon(Icon::AlignH)
                .enabled(n > 1),
            FixtureAction::MirrorV,
        );
        act(
            ui,
            MenuItem::new(t("Reset position"))
                .icon(Icon::Reset)
                .enabled(some),
            FixtureAction::ResetPosition,
        );
    }
    if place == MenuPlace::Patch {
        menu::separator(ui);
        act(
            ui,
            MenuItem::new(t("Move to next free address"))
                .icon(Icon::Next)
                .enabled(some),
            FixtureAction::NextFreeAddress,
        );
    }

    if place != MenuPlace::Stage {
        let k = |c: &str| crate::titlebar::keys::clip(c);
        let (kx, kc, kv, kd) = (k("X"), k("C"), k("V"), k("D"));
        menu::separator(ui);
        act(
            ui,
            MenuItem::new(t("Cut"))
                .icon(Icon::Scissors)
                .shortcut(&kx)
                .enabled(some),
            FixtureAction::Cut,
        );
        act(
            ui,
            MenuItem::new(t("Copy"))
                .icon(Icon::Copy)
                .shortcut(&kc)
                .enabled(some),
            FixtureAction::Copy,
        );
        act(
            ui,
            MenuItem::new(t("Paste"))
                .icon(Icon::Paste)
                .shortcut(&kv)
                .enabled(has_fixture_clipboard),
            FixtureAction::Paste,
        );
        act(
            ui,
            MenuItem::new(t("Duplicate"))
                .icon(Icon::Copy)
                .shortcut(&kd)
                .enabled(some),
            FixtureAction::Duplicate,
        );
    }
    menu::separator(ui);
    if place == MenuPlace::Stage {
        act(
            ui,
            MenuItem::new(t("Edit in setup"))
                .icon(Icon::Edit)
                .enabled(some),
            FixtureAction::ShowInSetup,
        );
    } else {
        act(
            ui,
            MenuItem::new(t("Rename")).icon(Icon::Edit).enabled(n == 1),
            FixtureAction::Rename,
        );
    }
    if place != MenuPlace::Stage {
        act(
            ui,
            MenuItem::new(t("Delete"))
                .icon(Icon::Trash)
                .shortcut(crate::titlebar::keys::delete())
                .danger()
                .enabled(some),
            FixtureAction::Delete,
        );
    }
    out
}

/// Executes a menu action.
pub fn apply(e: &mut Engine, st: &mut AppState, action: FixtureAction) {
    let ids: Vec<FixtureId> = e
        .show()
        .fixtures
        .iter()
        .map(|f| f.id)
        .filter(|id| st.selected.contains(id))
        .collect();
    match action {
        FixtureAction::AddToGroup(g) => {
            if let Some(group) = e.show_mut().groups.iter_mut().find(|x| x.id == g) {
                for id in &ids {
                    if !group.fixtures.contains(id) {
                        group.fixtures.push(*id);
                    }
                }
                st.info(tr!(
                    "{n} fixture(s) added to “{v}”",
                    n = ids.len(),
                    v = group.name
                ));
            }
        }
        FixtureAction::RemoveFromGroup(g) => {
            if let Some(group) = e.show_mut().groups.iter_mut().find(|x| x.id == g) {
                group.fixtures.retain(|f| !ids.contains(f));
            }
        }
        FixtureAction::NewGroup => {
            let name = tr!("Group {n}", n = e.show().groups.len() + 1);
            e.show_mut().add_group(name.clone(), ids);
            st.info(tr!(
                "Created “{name}” – double-click the group to rename it",
                name = name
            ));
        }
        FixtureAction::SelectSameType => {
            let types: Vec<String> = ids
                .iter()
                .filter_map(|id| e.show().fixture(*id).map(|f| f.type_key.clone()))
                .collect();
            st.selected = e
                .show()
                .fixtures
                .iter()
                .filter(|f| types.contains(&f.type_key))
                .map(|f| f.id)
                .collect();
        }
        FixtureAction::SelectAll => st.selected = e.show().fixtures.iter().map(|f| f.id).collect(),
        FixtureAction::SelectNone => st.selected.clear(),
        FixtureAction::InvertOrder => {
            // Reverse the order in the patch (determines chaser/effect order).
            let show = e.show_mut();
            let slots: Vec<usize> = show
                .fixtures
                .iter()
                .enumerate()
                .filter(|(_, f)| ids.contains(&f.id))
                .map(|(i, _)| i)
                .collect();
            let mut picked: Vec<_> = slots.iter().map(|i| show.fixtures[*i].clone()).collect();
            picked.reverse();
            for (slot, f) in slots.into_iter().zip(picked) {
                show.fixtures[slot] = f;
            }
        }
        FixtureAction::CopyLayout => {
            let pos: Vec<[f32; 2]> = ids.iter().map(|id| fixture_pos(e.show(), *id)).collect();
            if let Some(first) = pos.first().copied() {
                st.layout_clipboard = Some(
                    pos.iter()
                        .map(|p| [p[0] - first[0], p[1] - first[1]])
                        .collect(),
                );
                st.info(tr!("Layout of {n} fixture(s) copied", n = pos.len()));
            }
        }
        FixtureAction::PasteLayout => {
            let Some(clip) = st.layout_clipboard.clone() else {
                return;
            };
            let Some(anchor) = ids.first().map(|id| fixture_pos(e.show(), *id)) else {
                return;
            };
            for (id, rel) in ids.iter().zip(clip.iter()) {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_pos = Some([anchor[0] + rel[0], anchor[1] + rel[1]]);
                }
            }
        }
        FixtureAction::MirrorH | FixtureAction::MirrorV => {
            let axis = usize::from(action == FixtureAction::MirrorV);
            let pos: Vec<[f32; 2]> = ids.iter().map(|id| fixture_pos(e.show(), *id)).collect();
            let (min, max) = pos.iter().fold((f32::MAX, f32::MIN), |(a, b), p| {
                (a.min(p[axis]), b.max(p[axis]))
            });
            for (id, mut p) in ids.iter().zip(pos) {
                p[axis] = min + max - p[axis];
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_pos = Some(p);
                }
            }
        }
        FixtureAction::ResetPosition => {
            for id in &ids {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_pos = None;
                }
            }
        }
        FixtureAction::SetShape(shape) => {
            for id in &ids {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_shape = shape;
                }
            }
        }
        FixtureAction::SetSize(size) => {
            for id in &ids {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_size = (size != 1.0).then_some(size);
                }
            }
        }
        FixtureAction::SetRotation(deg) => {
            for id in &ids {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    f.plan_rotation =
                        (deg.rem_euclid(360.0) != 0.0).then_some(deg.rem_euclid(360.0));
                }
            }
        }
        FixtureAction::RotateBy(d) => {
            for id in &ids {
                if let Some(f) = e.show_mut().fixture_mut(*id) {
                    let deg = (f.plan_rotation.unwrap_or(0.0) + d).rem_euclid(360.0);
                    f.plan_rotation = (deg != 0.0).then_some(deg);
                }
            }
        }
        FixtureAction::NextFreeAddress => {
            for id in ids {
                let Some((u, ch)) = e
                    .show()
                    .fixture(id)
                    .map(|f| (f.universe, e.show().fixture_channels(id)))
                else {
                    continue;
                };
                // temporarily take it out of the patch so its own position does not get
                // in the way
                let free = {
                    let mut tmp = e.show().clone();
                    tmp.fixtures.retain(|f| f.id != id);
                    tmp.next_free_address(u, ch)
                };
                match free {
                    Some(a) => {
                        if let Err(err) = e.show_mut().repatch_fixture(id, u, a) {
                            st.error(err.to_string());
                        }
                    }
                    None => st.error(t("No free range in this universe")),
                }
            }
        }
        FixtureAction::ShowInSetup => {
            st.mode = Mode::Setup;
            st.monitor_focus = ids.first().copied();
        }
        FixtureAction::Rename => {
            st.mode = Mode::Setup;
            st.rename_request = ids.first().copied();
        }
        FixtureAction::Copy | FixtureAction::Cut => {
            st.fixture_clipboard = ids
                .iter()
                .filter_map(|id| e.show().fixture(*id).cloned())
                .collect();
            if action == FixtureAction::Cut {
                apply(e, st, FixtureAction::Delete);
            } else {
                st.info(tr!("{n} fixture(s) copied", n = ids.len()));
            }
        }
        FixtureAction::Paste | FixtureAction::Duplicate => {
            let src: Vec<dmxcraft_core::Fixture> = if action == FixtureAction::Paste {
                st.fixture_clipboard.clone()
            } else {
                ids.iter()
                    .filter_map(|id| e.show().fixture(*id).cloned())
                    .collect()
            };
            let Some(first) = src.first() else {
                return;
            };
            // Paste starting at the clicked cell, otherwise after the original
            let (u, a) = match (action == FixtureAction::Paste, st.patch_cursor) {
                (true, Some(c)) => c,
                _ => (first.universe, first.address),
            };
            let new = e.show_mut().paste_fixtures(&src, u, a);
            if new.len() < src.len() {
                st.error(t("Not all fixtures fit into the patch"));
            }
            if !new.is_empty() {
                st.selected = new.into_iter().collect();
            }
        }
        FixtureAction::Delete => {
            let n = ids.len();
            for id in ids {
                e.show_mut().remove_fixture(id);
            }
            e.sync();
            st.selected.clear();
            st.info(tr!("{n} fixture(s) deleted", n = n));
        }
    }
}
