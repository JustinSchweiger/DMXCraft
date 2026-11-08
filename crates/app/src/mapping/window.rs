//! Fenster „Tastatur-Zuordnung“ bzw. „MIDI-Zuordnung“: Voreinstellungen
//! (löschen, importieren, exportieren), Liste aller Zuordnungen und die
//! Einstellungen der gewählten Zuordnung.

use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use dmxcraft_core::mapping::{
    note_name, ButtonMode, Control, Feedback, Input, MapKind, Mapping, MidiMessage, SceneTarget,
};
use dmxcraft_core::{Engine, Show};
use eframe::egui::{self, vec2, Align, Align2, Layout, Sense, Ui};

use super::{color, Mapper};
use crate::i18n::t;
use crate::midi_device::{color32, DeviceModel};
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::dialog::{form, Dialog};
use crate::ui::widgets::menu::dropdown;
use crate::ui::widgets::section::section_header;
use crate::ui::widgets::{Icon, IconButton};

/// Dateiendung exportierter Zuordnungen.
pub const MAP_EXT: &str = "dmxmap";

const ROW_H: f32 = 22.0;
const LIST_H: f32 = 200.0;

/// Zustand des Fensters zwischen den Bildern.
#[derive(Default)]
pub struct WindowState {
    /// Sortierung nach Kategorie absteigend.
    sort_desc: bool,
    /// Bearbeiteter LED-Zustand: an (sonst aus).
    pub(super) edit_on: bool,
    import: Option<(MapKind, Receiver<Option<PathBuf>>)>,
    export: Option<(MapKind, Vec<Mapping>, Receiver<Option<PathBuf>>)>,
}

/// Datei mit exportierten Zuordnungen.
#[derive(serde::Serialize, serde::Deserialize)]
struct MapFile {
    kind: MapKind,
    mappings: Vec<Mapping>,
}

/// Fenster zeigen (nur im Zuordnungsmodus).
pub fn show(ctx: &egui::Context, e: &mut Engine, st: &mut AppState, mapper: &mut Mapper) {
    poll_files(e, st, mapper);
    let Some(kind) = st.mapping else {
        return;
    };
    let title = match kind {
        MapKind::Keyboard => t("Tastatur-Zuordnung"),
        MapKind::Midi => t("MIDI-Zuordnung"),
    };
    let mut open = true;
    let pos = ctx.content_rect().right_top() + vec2(-theme::SP_L, 48.0);
    Dialog::new(title)
        .id(match kind {
            MapKind::Keyboard => "mapping_window_key",
            MapKind::Midi => "mapping_window_midi",
        })
        .width(420.0)
        .top_right(pos)
        .keep_on_escape()
        .show(ctx, &mut open, |ui, _| {
            presets(ui, e, st, mapper, kind);
            ui.add_space(theme::SP_XS);
            table(ui, e, st, mapper, kind);
            ui.add_space(theme::SP_M);
            settings(ui, e, st, mapper, kind);
        });
    if !open {
        st.mapping = None;
        st.map_learn = false;
    }
}

/// Kopf „Voreinstellungen“ mit Löschen, Import, Export und „+“ für globale
/// Befehle.
fn presets(ui: &mut Ui, e: &mut Engine, st: &mut AppState, mapper: &mut Mapper, kind: MapKind) {
    section_header(ui, t("Voreinstellungen"), |ui| {
        let empty = e.show().mappings.list(kind).is_empty();
        if ui
            .add(IconButton::new(Icon::Export).flat().enabled(!empty))
            .on_hover_text(t("Voreinstellung exportieren"))
            .clicked()
        {
            let list = e.show().mappings.list(kind).clone();
            mapper.win.export = Some((kind, list, pick_export(ui.ctx(), kind)));
        }
        if ui
            .add(IconButton::new(Icon::Import).flat())
            .on_hover_text(t("Voreinstellung importieren"))
            .clicked()
        {
            mapper.win.import = Some((kind, pick_import(ui.ctx())));
        }
        if ui
            .add(IconButton::new(Icon::Trash).flat().danger().enabled(!empty))
            .on_hover_text(t("Aktuelle Voreinstellung löschen"))
            .clicked()
        {
            e.show_mut().mappings.list_mut(kind).clear();
            st.map_sel = None;
            st.map_learn = false;
        }
        let add = ui
            .add(IconButton::new(Icon::Plus).flat())
            .on_hover_text(t("Globalen Befehl zuordnen"));
        if add.clicked() {
            egui::Popup::toggle_id(ui.ctx(), add.id);
        }
        dropdown(&add, |ui| {
            let items = [
                (Control::Blackout, t("Blackout")),
                (Control::Master, t("Master")),
                (Control::StopAll, t("Alle Szenen stoppen")),
                (Control::BankStep { delta: 1 }, t("Nächste Bank")),
                (Control::BankStep { delta: -1 }, t("Vorige Bank")),
                (
                    Control::Scene {
                        target: SceneTarget::Selected,
                    },
                    t("Ausgewählte Szene"),
                ),
            ];
            for (c, label) in items {
                if ui.button(label).clicked() {
                    super::pick(ui.ctx(), (c, None));
                    ui.close();
                }
            }
        });
    });
}

/// Liste: Kategorie, Ziel, Eingabe.
fn table(ui: &mut Ui, e: &mut Engine, st: &mut AppState, mapper: &mut Mapper, kind: MapKind) {
    let w = ui.available_width();
    let cols = [w * 0.24, w * 0.46, w * 0.30];
    let show = e.show();
    let mut rows: Vec<(u32, String, String, String)> = show
        .mappings
        .list(kind)
        .iter()
        .map(|m| {
            let (cat, target) = super::describe_mapping(show, m);
            let input = match &m.input {
                Some(i) => i.label(),
                None => "–".to_string(),
            };
            (m.id, cat, target, input)
        })
        .collect();
    rows.sort_by(|a, b| {
        let o = a.1.cmp(&b.1).then_with(|| a.2.cmp(&b.2));
        if mapper.win.sort_desc {
            o.reverse()
        } else {
            o
        }
    });

    // Kopfzeile
    let (head, resp) = ui.allocate_exact_size(vec2(w, ROW_H), Sense::click());
    if resp.clicked()
        && ui
            .input(|i| i.pointer.interact_pos())
            .is_some_and(|p| p.x < head.left() + cols[0])
    {
        mapper.win.sort_desc = !mapper.win.sort_desc;
    }
    let p = ui.painter();
    p.rect_filled(head, theme::R_S, theme::bg());
    let input_head = match kind {
        MapKind::Keyboard => t("Taste"),
        MapKind::Midi => t("MIDI-Eingang"),
    };
    let heads = [t("Kategorie"), t("Ziel"), input_head];
    let mut x = head.left() + theme::SP_S;
    for (i, h) in heads.iter().enumerate() {
        p.text(
            egui::pos2(x, head.center().y),
            Align2::LEFT_CENTER,
            h.to_uppercase(),
            theme::caption(),
            theme::text_dim(),
        );
        if i == 0 {
            // Sortierpfeil
            let w0 = p
                .layout_no_wrap(h.to_uppercase(), theme::caption(), theme::text_dim())
                .size()
                .x;
            let c = egui::pos2(x + w0 + 8.0, head.center().y);
            let d = if mapper.win.sort_desc { -3.0 } else { 3.0 };
            p.add(egui::Shape::convex_polygon(
                vec![c + vec2(-4.0, -d), c + vec2(4.0, -d), c + vec2(0.0, d)],
                theme::text_dim(),
                egui::Stroke::NONE,
            ));
        }
        x += cols[i];
    }

    let mut delete = None;
    egui::ScrollArea::vertical()
        .id_salt(("mapping_list", kind))
        .max_height(LIST_H)
        .min_scrolled_height(LIST_H)
        .auto_shrink([false, false])
        .show(ui, |ui| {
            if rows.is_empty() {
                ui.add_space(theme::SP_M);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new(t(
                            "Noch keine Zuordnungen. Ein markiertes Element anklicken.",
                        ))
                        .color(theme::text_faint()),
                    );
                });
            }
            for (id, cat, target, input) in &rows {
                let (rect, resp) = ui.allocate_exact_size(vec2(w, ROW_H), Sense::click());
                let sel = st.map_sel == Some(*id);
                let p = ui.painter();
                if sel {
                    p.rect_filled(rect, theme::R_S, color(kind).gamma_multiply(0.45));
                } else if resp.hovered() {
                    p.rect_filled(rect, theme::R_S, theme::raised_hover());
                }
                let mut x = rect.left() + theme::SP_S;
                for (i, text) in [cat, target, input].into_iter().enumerate() {
                    let clip = egui::Rect::from_min_max(
                        egui::pos2(x, rect.top()),
                        egui::pos2(x + cols[i] - theme::SP_S, rect.bottom()),
                    );
                    p.with_clip_rect(clip).text(
                        egui::pos2(x, rect.center().y),
                        Align2::LEFT_CENTER,
                        text,
                        theme::body(),
                        if i == 2 && input == "–" {
                            theme::text_faint()
                        } else {
                            theme::text()
                        },
                    );
                    x += cols[i];
                }
                if resp.clicked() {
                    st.map_sel = Some(*id);
                    st.map_learn = false;
                }
                resp.context_menu(|ui| {
                    if ui.button(t("Neu zuweisen")).clicked() {
                        st.map_sel = Some(*id);
                        st.map_learn = true;
                        ui.close();
                    }
                    if ui.button(t("Löschen")).clicked() {
                        delete = Some(*id);
                        ui.close();
                    }
                });
                if sel && st.map_learn {
                    // sichtbar halten, wenn über das Overlay gewählt
                    resp.scroll_to_me(None);
                }
            }
        });
    if !st.map_learn
        && !ui.ctx().egui_wants_keyboard_input()
        && ui.input(|i| i.key_pressed(egui::Key::Delete))
    {
        delete = delete.or(st.map_sel);
    }
    if let Some(id) = delete {
        e.show_mut().mappings.list_mut(kind).retain(|m| m.id != id);
        if st.map_sel == Some(id) {
            st.map_sel = None;
            st.map_learn = false;
        }
    }
}

/// Einstellungen der gewählten Zuordnung.
fn settings(ui: &mut Ui, e: &mut Engine, st: &mut AppState, mapper: &mut Mapper, kind: MapKind) {
    section_header(ui, t("Einstellungen"), |_| {});
    let Some(id) = st.map_sel else {
        hint(ui, t("Ein markiertes Element anklicken, um es zuzuordnen oder seine Zuordnung zu bearbeiten."));
        return;
    };
    let Some(mut m) = e
        .show()
        .mappings
        .list(kind)
        .iter()
        .find(|m| m.id == id)
        .cloned()
    else {
        st.map_sel = None;
        return;
    };
    let before = m.clone();
    let show = e.show();
    let inputs = mapper.midi.input_names.clone();
    let outputs = mapper.midi.output_names.clone();
    let mut learn = st.map_learn;
    let mut remove = false;

    form(ui, &format!("mapping_settings_{kind:?}"), |f| {
        // Eingabe
        f.row(
            match kind {
                MapKind::Keyboard => t("Taste"),
                MapKind::Midi => t("Eingabe"),
            },
            |ui| {
                let text = if learn {
                    t("Warte auf Eingabe …").to_string()
                } else {
                    m.input
                        .as_ref()
                        .map_or_else(|| t("(keine)").to_string(), Input::label)
                };
                let c = if learn { color(kind) } else { theme::text() };
                ui.label(egui::RichText::new(text).color(c).strong());
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    let label = if learn {
                        t("Abbrechen")
                    } else {
                        t("Neu zuweisen")
                    };
                    if ui.button(label).clicked() {
                        learn = !learn;
                    }
                });
            },
        );
        // Ziel (Szenen: welche Szene, Geräte: Auswahl oder einzelner Kanal)
        {
            let options = targets(show, &m);
            if options.len() > 1 {
                f.row(t("Ziel"), |ui| {
                    let current = options
                        .iter()
                        .find(|(c, _)| *c == m.control)
                        .map_or_else(String::new, |o| o.1.clone());
                    combo(ui, ("map_target", id), current, |ui| {
                        for (c, label) in &options {
                            ui.selectable_value(&mut m.control, *c, label);
                        }
                    });
                });
            }
        }
        // Modus
        f.row(t("Tastenmodus"), |ui| {
            let label = |b: ButtonMode| match b {
                ButtonMode::Toggle => t("An / Aus"),
                ButtonMode::Value => t("Wert in %"),
            };
            combo(ui, ("map_mode", id), label(m.mode), |ui| {
                for b in [ButtonMode::Toggle, ButtonMode::Value] {
                    ui.selectable_value(&mut m.mode, b, label(b));
                }
            });
        });
        if matches!(
            m.control,
            Control::Scene { .. } | Control::Blackout | Control::Channel { .. }
        ) && m.mode == ButtonMode::Toggle
        {
            f.row(t("Flash-Taste"), |ui| {
                ui.add(crate::ui::widgets::Toggle::new(&mut m.flash, ""))
                    .on_hover_text(t("Nur aktiv, solange die Taste gedrückt ist"));
            });
        }
        // MIDI-Eingang
        if let Some(Input::Midi {
            device,
            channel,
            message,
            number,
        }) = &mut m.input
        {
            f.row(t("Gerät"), |ui| {
                let shown = if device.is_empty() {
                    t("Alle Geräte").to_string()
                } else {
                    device.clone()
                };
                combo(ui, ("map_in", id), shown, |ui| {
                    ui.selectable_value(device, String::new(), t("Alle Geräte"));
                    for n in &inputs {
                        ui.selectable_value(device, n.clone(), n);
                    }
                });
            });
            f.row(t("Kanal"), |ui| {
                combo(ui, ("map_ch", id), channel.to_string(), |ui| {
                    for c in 1..=16u8 {
                        ui.selectable_value(channel, c, c.to_string());
                    }
                });
            });
            f.row(t("Nachricht"), |ui| {
                let label = |x: MidiMessage| match x {
                    MidiMessage::Note => t("Note"),
                    MidiMessage::Cc => t("Regler (CC)"),
                };
                combo(ui, ("map_msg", id), label(*message), |ui| {
                    for x in [MidiMessage::Note, MidiMessage::Cc] {
                        ui.selectable_value(message, x, label(x));
                    }
                });
            });
            f.row(t("Nummer"), |ui| {
                let note = *message == MidiMessage::Note;
                ui.add(
                    egui::DragValue::new(number)
                        .range(0..=127)
                        .custom_formatter(move |v, _| {
                            if note {
                                format!("{} ({v})", note_name(v as u8))
                            } else {
                                format!("{v}")
                            }
                        }),
                );
            });
        }
    });

    // MIDI-Ausgang (LED-Rückmeldung)
    if kind == MapKind::Midi {
        ui.add_space(theme::SP_S);
        section_header(ui, t("MIDI-Ausgang"), |_| {});
        let models: Vec<(String, DeviceModel)> = mapper
            .devices
            .iter()
            .map(|d| (d.output.clone(), d.model))
            .collect();
        let edit_on = &mut mapper.win.edit_on;
        form(ui, &format!("mapping_out_{id}"), |f| {
            f.row(t("Gerät"), |ui| {
                let mut dev = m
                    .feedback
                    .as_ref()
                    .map(|f| f.device.clone())
                    .unwrap_or_default();
                let shown = if dev.is_empty() {
                    t("Keins").to_string()
                } else {
                    dev.clone()
                };
                combo(ui, ("map_out", id), shown, |ui| {
                    ui.selectable_value(&mut dev, String::new(), t("Keins"));
                    for n in &outputs {
                        ui.selectable_value(&mut dev, n.clone(), n);
                    }
                });
                if dev.is_empty() {
                    m.feedback = None;
                } else {
                    m.feedback.get_or_insert_with(Feedback::default).device = dev;
                }
            });
            let Some(fb) = &mut m.feedback else {
                return;
            };
            f.row(t("Zustand"), |ui| {
                ui.selectable_value(edit_on, false, t("Aus"));
                ui.selectable_value(edit_on, true, t("An"));
            });
            let model = models
                .iter()
                .find(|(o, _)| *o == fb.device)
                .map_or_else(|| DeviceModel::detect(&fb.device), |d| d.1);
            let s = if *edit_on { &mut fb.on } else { &mut fb.off };
            f.row(t("Kanal"), |ui| {
                combo(
                    ui,
                    ("map_fbch", id, *edit_on),
                    channel_label(s.channel, model),
                    |ui| {
                        for c in 1..=16u8 {
                            ui.selectable_value(&mut s.channel, c, channel_label(c, model));
                        }
                    },
                );
            });
            f.row(t("Velocity"), |ui| {
                if let Some(p) = model.palette() {
                    palette_button(ui, ("map_pal", id, *edit_on), p, &mut s.velocity);
                }
                ui.add(egui::DragValue::new(&mut s.velocity).range(0..=127));
            });
        });
    }

    ui.add_space(theme::SP_S);
    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
        if ui
            .add(IconButton::new(Icon::Trash).flat().danger())
            .on_hover_text(t("Zuordnung löschen"))
            .clicked()
        {
            remove = true;
        }
    });

    st.map_learn = learn;
    let maps = &mut e.show_mut().mappings;
    if remove {
        maps.list_mut(kind).retain(|x| x.id != id);
        st.map_sel = None;
        st.map_learn = false;
    } else if m != before {
        if let Some(slot) = maps.find_mut(kind, id) {
            *slot = m;
        }
    }
}

/// Mögliche Ziele einer Zuordnung, ausgehend vom angeklickten Element:
/// bei Szenen die Szene, die Auswahl oder eine Position in der Bank, bei
/// Geräten die Auswahl oder ein einzelner Kanal.
fn targets(show: &Show, m: &Mapping) -> Vec<(Control, String)> {
    let scene = |target| Control::Scene { target };
    let mut out = Vec::new();
    if let Control::Fixture { id } = m.element() {
        out.push((Control::Fixture { id }, t("Auswahl umschalten").to_string()));
        if let Some(ty) = show.fixture(id).and_then(|f| show.type_of(f)) {
            for (i, c) in ty.channels.iter().enumerate() {
                out.push((
                    Control::Channel {
                        channel: dmxcraft_core::ChannelRef {
                            fixture: id,
                            channel: i as u16,
                        },
                    },
                    crate::tr!("Kanal {n}: {name}", n = i + 1, name = c.name),
                ));
            }
        }
    }
    if let Control::Scene {
        target: SceneTarget::Scene { id },
    } = m.element()
    {
        out.push((scene(SceneTarget::Scene { id }), t("Szene").to_string()));
        out.push((
            scene(SceneTarget::Selected),
            t("Ausgewählte Szene").to_string(),
        ));
        if let Some(z) = show.scene(id).and_then(|s| s.zone) {
            let list = super::zone_scenes(show, z);
            if let Some(index) = list.iter().position(|s| *s == id) {
                out.push((
                    scene(SceneTarget::Index { zone: z, index }),
                    crate::tr!("Nr. {n} in dieser Bank", n = index + 1),
                ));
                out.push((
                    scene(SceneTarget::IndexInShownBank { index }),
                    crate::tr!("Nr. {n} in der angezeigten Bank", n = index + 1),
                ));
            }
        }
    }
    out
}

/// Kanal der Rückmeldung, bei bekannten Geräten mit Bedeutung.
fn channel_label(c: u8, model: DeviceModel) -> String {
    match model.channel_meaning(c) {
        Some(m) => format!("{c} – {m}"),
        None => c.to_string(),
    }
}

/// Farbfeld der gewählten Anschlagstärke; Klick öffnet die Palette des Geräts.
fn palette_button(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    palette: &[u32; 128],
    velocity: &mut u8,
) {
    let (rect, resp) = ui.allocate_exact_size(vec2(36.0, 22.0), Sense::click());
    let c = color32(palette[usize::from(*velocity & 0x7f)]);
    ui.painter().rect_filled(rect, theme::R_S, c);
    ui.painter().rect_stroke(
        rect,
        theme::R_S,
        egui::Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
    );
    let resp = resp.on_hover_text(t("Farbe wählen"));
    let pid = resp.id;
    if resp.clicked() {
        egui::Popup::toggle_id(ui.ctx(), pid);
    }
    egui::Popup::menu(&resp)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            const CELL: f32 = 20.0;
            ui.spacing_mut().item_spacing = vec2(2.0, 2.0);
            egui::Grid::new(ui.make_persistent_id(id))
                .spacing(vec2(2.0, 2.0))
                .show(ui, |ui| {
                    for row in 0..8 {
                        for col in 0..16 {
                            let v = (row * 16 + col) as u8;
                            let (r, cell) =
                                ui.allocate_exact_size(vec2(CELL, CELL), Sense::click());
                            ui.painter()
                                .rect_filled(r, 2, color32(palette[usize::from(v)]));
                            if v == *velocity || cell.hovered() {
                                ui.painter().rect_stroke(
                                    r,
                                    2,
                                    egui::Stroke::new(2.0, theme::text()),
                                    egui::StrokeKind::Inside,
                                );
                            }
                            if cell.on_hover_text(v.to_string()).clicked() {
                                *velocity = v;
                                egui::Popup::close_id(ui.ctx(), pid);
                            }
                        }
                        ui.end_row();
                    }
                });
        });
}

fn combo(
    ui: &mut Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    text: impl Into<egui::WidgetText>,
    add: impl FnOnce(&mut Ui),
) {
    egui::ComboBox::from_id_salt(id)
        .selected_text(text)
        .width(ui.available_width())
        .show_ui(ui, add);
}

fn hint(ui: &mut Ui, text: &str) {
    ui.label(egui::RichText::new(text).color(theme::text_faint()));
}

fn pick_import(ctx: &egui::Context) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Voreinstellung importieren"))
            .add_filter(t("DMXCraft-Zuordnung"), &[MAP_EXT])
            .pick_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

fn pick_export(ctx: &egui::Context, kind: MapKind) -> Receiver<Option<PathBuf>> {
    let (tx, rx) = std::sync::mpsc::channel();
    let ctx = ctx.clone();
    let name = match kind {
        MapKind::Keyboard => format!("{}.{MAP_EXT}", t("Tastatur")),
        MapKind::Midi => format!("MIDI.{MAP_EXT}"),
    };
    std::thread::spawn(move || {
        let path = rfd::FileDialog::new()
            .set_title(t("Voreinstellung exportieren"))
            .add_filter(t("DMXCraft-Zuordnung"), &[MAP_EXT])
            .set_file_name(name)
            .save_file();
        let _ = tx.send(path);
        ctx.request_repaint();
    });
    rx
}

/// Ergebnisse der Dateidialoge verarbeiten.
fn poll_files(e: &mut Engine, st: &mut AppState, mapper: &mut Mapper) {
    if let Some((kind, rx)) = &mapper.win.import {
        if let Ok(path) = rx.try_recv() {
            let kind = *kind;
            mapper.win.import = None;
            if let Some(path) = path {
                match read_file(&path) {
                    Ok(list) => {
                        let n = list.len();
                        let maps = &mut e.show_mut().mappings;
                        maps.list_mut(kind).clear();
                        for mut m in list {
                            maps.next_id = maps.next_id.max(1);
                            m.id = maps.next_id;
                            maps.next_id += 1;
                            maps.list_mut(kind).push(m);
                        }
                        st.map_sel = None;
                        st.message = Some((crate::tr!("{n} Zuordnungen importiert", n = n), false));
                    }
                    Err(err) => st.message = Some((err, true)),
                }
            }
        }
    }
    if let Some((kind, list, rx)) = &mapper.win.export {
        if let Ok(path) = rx.try_recv() {
            if let Some(mut path) = path {
                if path.extension().is_none() {
                    path.set_extension(MAP_EXT);
                }
                let file = MapFile {
                    kind: *kind,
                    mappings: list.clone(),
                };
                let res = serde_json::to_string_pretty(&file)
                    .map_err(|e| e.to_string())
                    .and_then(|s| std::fs::write(&path, s).map_err(|e| e.to_string()));
                st.message = Some(match res {
                    Ok(()) => (t("Voreinstellung exportiert").to_string(), false),
                    Err(err) => (err, true),
                });
            }
            mapper.win.export = None;
        }
    }
}

fn read_file(path: &std::path::Path) -> Result<Vec<Mapping>, String> {
    let text = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let f: MapFile = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    Ok(f.mappings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_file_roundtrip() {
        let mut maps = dmxcraft_core::mapping::Mappings::default();
        maps.add(MapKind::Keyboard, Control::Blackout);
        let f = MapFile {
            kind: MapKind::Keyboard,
            mappings: maps.keyboard.clone(),
        };
        let s = serde_json::to_string(&f).unwrap();
        let back: MapFile = serde_json::from_str(&s).unwrap();
        assert_eq!(back.mappings, maps.keyboard);
    }

    #[test]
    fn channel_labels() {
        assert_eq!(channel_label(3, DeviceModel::Generic), "3");
        assert!(channel_label(7, DeviceModel::ApcMiniMk2).starts_with("7 – "));
        assert!(channel_label(16, DeviceModel::ApcMiniMk2).contains("1/2"));
    }
}
