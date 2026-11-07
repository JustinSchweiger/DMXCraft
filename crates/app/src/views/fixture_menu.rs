//! Rechtsklick-Menü für Geräte – gemeinsam für Bühne (STEUERN),
//! Bühnenplan-Editor und DMX-Patch. Das Menü liest die Show nur und liefert
//! eine [`FixtureAction`], die anschließend mit [`apply`] ausgeführt wird.

use crate::i18n::t;
use std::collections::BTreeSet;

use dmxcraft_core::{Engine, FixtureId, GroupId, PlanShape, Show};
use eframe::egui;

use crate::state::{AppState, Mode};
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::stage_canvas::fixture_pos;
use crate::ui::widgets::stage_canvas::paint_shape_icon;
use crate::ui::widgets::Icon;

/// Größenstufen für Menüs und Werkzeugleiste.
pub const SIZES: [(&str, f32); 5] = [
    ("Klein", 0.7),
    ("Normal", 1.0),
    ("Groß", 1.5),
    ("Sehr groß", 2.2),
    ("Riesig", 3.2),
];

/// Wo das Menü geöffnet wurde (bestimmt die angebotenen Einträge).
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
    /// `None` = automatisch nach Gerätetyp.
    SetShape(Option<PlanShape>),
    SetSize(f32),
    NextFreeAddress,
    ShowInSetup,
    Rename,
    Delete,
}

/// Zeichnet die Menüeinträge. `target` ist das Gerät unter der Maus.
pub fn menu(
    ui: &mut egui::Ui,
    show: &Show,
    selected: &BTreeSet<FixtureId>,
    target: Option<FixtureId>,
    place: MenuPlace,
    has_layout_clipboard: bool,
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
            tr!("{n} Geräte ausgewählt", n = n)
        } else {
            format!("{} · U{}/{}", f.name, f.universe + 1, f.address)
        };
        menu::title(ui, &title);
    }

    // Gruppen
    menu::sub(
        ui,
        MenuItem::new(t("Zu Gruppe hinzufügen"))
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
        MenuItem::new(t("Aus Gruppe entfernen")).enabled(!containing.is_empty()),
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
        MenuItem::new(t("Neue Gruppe aus Auswahl"))
            .icon(Icon::Plus)
            .enabled(some),
        FixtureAction::NewGroup,
    );
    menu::separator(ui);

    // Auswahl
    act(
        ui,
        MenuItem::new(t("Gleichen Gerätetyp auswählen")).enabled(some),
        FixtureAction::SelectSameType,
    );
    act(
        ui,
        MenuItem::new(t("Alle auswählen")),
        FixtureAction::SelectAll,
    );
    act(
        ui,
        MenuItem::new(t("Auswahl aufheben")).enabled(some),
        FixtureAction::SelectNone,
    );
    act(
        ui,
        MenuItem::new(t("Reihenfolge umkehren")).enabled(n > 1),
        FixtureAction::InvertOrder,
    );

    // Darstellung
    if place != MenuPlace::Patch {
        menu::separator(ui);
        let current_shape = (n == 1)
            .then(|| selected.iter().next().map(|id| show.plan_shape(*id)))
            .flatten();
        menu::sub(ui, MenuItem::new(t("Form")).enabled(some), |ui| {
            act(
                ui,
                MenuItem::new(t("Automatisch (nach Typ)"))
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
        menu::sub(ui, MenuItem::new(t("Größe")).enabled(some), |ui| {
            for (label, f) in SIZES {
                act(
                    ui,
                    MenuItem::new(t(label))
                        .checked(current_size.is_some_and(|c| (c - f).abs() < 0.01)),
                    FixtureAction::SetSize(f),
                );
            }
        });
    }

    // Anordnung
    if place == MenuPlace::StageEditor {
        menu::separator(ui);
        act(
            ui,
            MenuItem::new(t("Layout kopieren"))
                .icon(Icon::Copy)
                .enabled(some),
            FixtureAction::CopyLayout,
        );
        act(
            ui,
            MenuItem::new(t("Layout einfügen"))
                .icon(Icon::Paste)
                .enabled(some && has_layout_clipboard),
            FixtureAction::PasteLayout,
        );
        act(
            ui,
            MenuItem::new(t("Waagerecht spiegeln"))
                .icon(Icon::AlignV)
                .enabled(n > 1),
            FixtureAction::MirrorH,
        );
        act(
            ui,
            MenuItem::new(t("Senkrecht spiegeln"))
                .icon(Icon::AlignH)
                .enabled(n > 1),
            FixtureAction::MirrorV,
        );
        act(
            ui,
            MenuItem::new(t("Position zurücksetzen"))
                .icon(Icon::Reset)
                .enabled(some),
            FixtureAction::ResetPosition,
        );
    }
    if place == MenuPlace::Patch {
        menu::separator(ui);
        act(
            ui,
            MenuItem::new(t("Auf nächste freie Adresse"))
                .icon(Icon::Next)
                .enabled(some),
            FixtureAction::NextFreeAddress,
        );
    }

    menu::separator(ui);
    if place == MenuPlace::Stage {
        act(
            ui,
            MenuItem::new(t("Unter Einrichten bearbeiten"))
                .icon(Icon::Edit)
                .enabled(some),
            FixtureAction::ShowInSetup,
        );
    } else {
        act(
            ui,
            MenuItem::new(t("Umbenennen"))
                .icon(Icon::Edit)
                .enabled(n == 1),
            FixtureAction::Rename,
        );
    }
    if place != MenuPlace::Stage {
        act(
            ui,
            MenuItem::new(t("Löschen"))
                .icon(Icon::Trash)
                .shortcut(crate::titlebar::keys::delete())
                .danger()
                .enabled(some),
            FixtureAction::Delete,
        );
    }
    out
}

/// Führt eine Menüaktion aus.
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
                    "{n} Gerät(e) zu „{v}“ hinzugefügt",
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
            let name = tr!("Gruppe {n}", n = e.show().groups.len() + 1);
            e.show_mut().add_group(name.clone(), ids);
            st.info(tr!(
                "„{name}“ angelegt – umbenennen unter EINRICHTEN → Zonen & Gruppen",
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
            // Reihenfolge im Patch umkehren (bestimmt Lauflicht-/Effektreihenfolge).
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
                st.info(tr!("Layout von {n} Gerät(en) kopiert", n = pos.len()));
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
        FixtureAction::NextFreeAddress => {
            for id in ids {
                let Some((u, ch)) = e
                    .show()
                    .fixture(id)
                    .map(|f| (f.universe, e.show().fixture_channels(id)))
                else {
                    continue;
                };
                // vorübergehend aus dem Patch nehmen, damit die eigene Lage nicht stört
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
                    None => st.error(t("Kein freier Bereich in diesem Universum")),
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
        FixtureAction::Delete => {
            let n = ids.len();
            for id in ids {
                e.show_mut().remove_fixture(id);
            }
            e.sync();
            st.selected.clear();
            st.info(tr!("{n} Gerät(e) gelöscht", n = n));
        }
    }
}
