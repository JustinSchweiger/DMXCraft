//! Rechtsklick-Menü für Geräte – gemeinsam für Bühne (STEUERN),
//! Bühnenplan-Editor und DMX-Patch. Das Menü liest die Show nur und liefert
//! eine [`FixtureAction`], die anschließend mit [`apply`] ausgeführt wird.

use std::collections::BTreeSet;

use dmxcraft_core::{plan, Engine, FixtureId, GroupId, PlanShape, Show};
use eframe::egui;

use crate::state::{AppState, Mode};
use crate::ui::widgets::stage_canvas::fixture_pos;

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
    let mut item = |ui: &mut egui::Ui, enabled: bool, label: &str, a: FixtureAction| {
        if ui.add_enabled(enabled, egui::Button::new(label)).clicked() {
            out = Some(a);
            ui.close();
        }
    };
    let n = selected.len();
    let some = n > 0;

    if let Some(f) = target.and_then(|t| show.fixture(t)) {
        let title = if n > 1 {
            format!("{n} Geräte ausgewählt")
        } else {
            format!("{} · U{}/{}", f.name, f.universe + 1, f.address)
        };
        ui.label(egui::RichText::new(title).strong());
        ui.separator();
    }

    // Gruppen
    ui.add_enabled_ui(some && !show.groups.is_empty(), |ui| {
        ui.menu_button("Zu Gruppe hinzufügen", |ui| {
            for g in &show.groups {
                item(ui, true, &g.name, FixtureAction::AddToGroup(g.id));
            }
        });
    });
    let containing: Vec<_> = show
        .groups
        .iter()
        .filter(|g| g.fixtures.iter().any(|f| selected.contains(f)))
        .collect();
    ui.add_enabled_ui(!containing.is_empty(), |ui| {
        ui.menu_button("Aus Gruppe entfernen", |ui| {
            for g in &containing {
                item(ui, true, &g.name, FixtureAction::RemoveFromGroup(g.id));
            }
        });
    });
    item(ui, some, "Neue Gruppe aus Auswahl", FixtureAction::NewGroup);
    ui.separator();

    // Auswahl
    item(
        ui,
        some,
        "Gleichen Gerätetyp auswählen",
        FixtureAction::SelectSameType,
    );
    item(ui, true, "Alle auswählen", FixtureAction::SelectAll);
    item(ui, some, "Auswahl aufheben", FixtureAction::SelectNone);
    item(
        ui,
        n > 1,
        "Reihenfolge umkehren",
        FixtureAction::InvertOrder,
    );

    // Darstellung
    if place != MenuPlace::Patch {
        ui.separator();
        ui.add_enabled_ui(some, |ui| {
            ui.menu_button("Form", |ui| {
                item(
                    ui,
                    true,
                    "Automatisch (nach Typ)",
                    FixtureAction::SetShape(None),
                );
                ui.separator();
                for s in PlanShape::ALL {
                    item(ui, true, s.label(), FixtureAction::SetShape(Some(s)));
                }
            });
            ui.menu_button("Größe", |ui| {
                for (label, f) in SIZES {
                    item(ui, true, label, FixtureAction::SetSize(f));
                }
            });
        });
    }

    // Anordnung
    if place == MenuPlace::StageEditor {
        ui.separator();
        item(ui, some, "Layout kopieren", FixtureAction::CopyLayout);
        item(
            ui,
            some && has_layout_clipboard,
            "Layout einfügen",
            FixtureAction::PasteLayout,
        );
        item(ui, n > 1, "Waagerecht spiegeln", FixtureAction::MirrorH);
        item(ui, n > 1, "Senkrecht spiegeln", FixtureAction::MirrorV);
        item(
            ui,
            some,
            "Position zurücksetzen",
            FixtureAction::ResetPosition,
        );
    }
    if place == MenuPlace::Patch {
        ui.separator();
        item(
            ui,
            some,
            "Auf nächste freie Adresse",
            FixtureAction::NextFreeAddress,
        );
    }

    ui.separator();
    if place == MenuPlace::Stage {
        item(
            ui,
            some,
            "Unter Einrichten bearbeiten",
            FixtureAction::ShowInSetup,
        );
    } else {
        item(ui, n == 1, "Umbenennen", FixtureAction::Rename);
    }
    if place != MenuPlace::Stage {
        ui.add_enabled_ui(some, |ui| {
            ui.menu_button("Löschen", |ui| {
                item(
                    ui,
                    true,
                    &format!("{n} Gerät(e) wirklich löschen"),
                    FixtureAction::Delete,
                );
            });
        });
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
                st.info(format!(
                    "{} Gerät(e) zu „{}“ hinzugefügt",
                    ids.len(),
                    group.name
                ));
            }
        }
        FixtureAction::RemoveFromGroup(g) => {
            if let Some(group) = e.show_mut().groups.iter_mut().find(|x| x.id == g) {
                group.fixtures.retain(|f| !ids.contains(f));
            }
        }
        FixtureAction::NewGroup => {
            let name = format!("Gruppe {}", e.show().groups.len() + 1);
            e.show_mut().add_group(name.clone(), ids);
            st.info(format!(
                "„{name}“ angelegt – umbenennen unter EINRICHTEN → Zonen & Gruppen"
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
                st.info(format!("Layout von {} Gerät(en) kopiert", pos.len()));
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
                    f.plan_pos = Some(plan::clamp([anchor[0] + rel[0], anchor[1] + rel[1]]));
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
                    None => st.error("Kein freier Bereich in diesem Universum"),
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
            st.info(format!("{n} Gerät(e) gelöscht"));
        }
    }
}
