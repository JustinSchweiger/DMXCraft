//! STEUERN → Programmer: ausgewählte Geräte live steuern und als Szene/Schritt
//! speichern.

use dmxcraft_core::{ChannelKind, ChannelRef, ChannelValue, DmxFrame, Engine, Step, ZoneId};
use eframe::egui::{self, Color32};

use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::{section_header, Fader, FaderEvent, Segmented, Swatch, XyPad};
use crate::views::{View, ViewCtx};

/// Kanal-Kategorien wie auf einem Lichtpult.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Category {
    #[default]
    Dimmer,
    Color,
    Position,
    Gobo,
    Beam,
    Other,
    All,
}

impl Category {
    const ITEMS: [(Category, &'static str); 7] = [
        (Category::Dimmer, "Dimmer"),
        (Category::Color, "Farbe"),
        (Category::Position, "Position"),
        (Category::Gobo, "Gobo"),
        (Category::Beam, "Strahl"),
        (Category::Other, "Sonstiges"),
        (Category::All, "Alle Kanäle"),
    ];

    pub fn contains(self, k: ChannelKind) -> bool {
        use ChannelKind as K;
        match self {
            Category::Dimmer => matches!(k, K::Dimmer),
            Category::Color => matches!(
                k,
                K::Red | K::Green | K::Blue | K::White | K::Amber | K::Uv | K::ColorWheel
            ),
            Category::Position => matches!(k, K::Pan | K::PanFine | K::Tilt | K::TiltFine),
            Category::Gobo => matches!(k, K::Gobo),
            Category::Beam => matches!(k, K::Strobe | K::Speed),
            Category::Other => matches!(k, K::Generic),
            Category::All => true,
        }
    }
}

/// Farbe des Pegels im Fader je Kanalart.
fn kind_color(k: ChannelKind) -> Color32 {
    match k {
        ChannelKind::Red => Color32::from_rgb(0xff, 0x45, 0x45),
        ChannelKind::Green => Color32::from_rgb(0x3d, 0xe0, 0x6a),
        ChannelKind::Blue => Color32::from_rgb(0x4a, 0x7d, 0xff),
        ChannelKind::White => Color32::from_rgb(0xf2, 0xf2, 0xf2),
        ChannelKind::Amber => Color32::from_rgb(0xff, 0xa8, 0x2e),
        ChannelKind::Uv => Color32::from_rgb(0xa2, 0x5c, 0xff),
        _ => theme::ACCENT,
    }
}

/// Ein Fader steuert gleichnamige Kanäle aller ausgewählten Geräte gemeinsam.
pub struct FaderGroup {
    pub name: String,
    pub kind: ChannelKind,
    pub channels: Vec<ChannelRef>,
}

pub fn fader_groups(e: &Engine, st: &AppState) -> Vec<FaderGroup> {
    let mut groups: Vec<FaderGroup> = Vec::new();
    for fid in &st.selected {
        let Some(f) = e.show().fixture(*fid) else {
            continue;
        };
        let Some(ty) = e.show().type_of(f) else {
            continue;
        };
        for (i, def) in ty.channels.iter().enumerate() {
            let ch = ChannelRef {
                fixture: f.id,
                channel: i as u16,
            };
            match groups
                .iter_mut()
                .find(|g| g.kind == def.kind && g.name == def.name)
            {
                Some(g) => g.channels.push(ch),
                None => groups.push(FaderGroup {
                    name: def.name.clone(),
                    kind: def.kind,
                    channels: vec![ch],
                }),
            }
        }
    }
    groups
}

#[derive(Default)]
pub struct ProgrammerView {
    category: Category,
}

impl View for ProgrammerView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let groups = fader_groups(cx.engine, cx.state);
        let set_count = cx.engine.programmer().len();
        let selected = cx.state.selected.len();
        let mut clear = false;
        section_header(ui, "Programmer", |ui| {
            if ui
                .add_enabled(set_count > 0, egui::Button::new("Leeren"))
                .on_hover_text("Alle Programmer-Werte freigeben")
                .clicked()
            {
                clear = true;
            }
            ui.label(
                egui::RichText::new(format!("{selected} Geräte · {set_count} Kanäle gesetzt"))
                    .color(theme::TEXT_DIM),
            );
        });
        if clear {
            cx.engine.clear_programmer();
        }

        // Speicherleiste unten, damit die Fader die restliche Höhe bekommen.
        egui::Panel::bottom("programmer_save")
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(0, 6)))
            .show(ui, |ui| save_bar(ui, cx.engine, cx.state));

        egui::Panel::left("programmer_categories")
            .resizable(false)
            .exact_size(124.0)
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                let dimmed =
                    |c: Category| c != Category::All && !groups.iter().any(|g| c.contains(g.kind));
                Segmented::new(&Category::ITEMS)
                    .vertical(116.0)
                    .dimmed(&dimmed)
                    .show(ui, &mut self.category);
            });

        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(8, 0)))
            .show(ui, |ui| {
                if cx.state.selected.is_empty() {
                    ui.add_space(theme::SP_L);
                    ui.label(
                        egui::RichText::new(
                            "Im Bühnenplan Geräte auswählen (Klick, Aufziehen, Strg/Shift+Klick) oder eine Gruppe wählen.",
                        )
                        .color(theme::TEXT_DIM),
                    );
                    return;
                }
                let visible: Vec<&FaderGroup> =
                    groups.iter().filter(|g| self.category.contains(g.kind)).collect();
                if self.category == Category::Color {
                    color_bar(ui, cx.engine, &groups, cx.frame);
                }
                if visible.is_empty() {
                    ui.label(
                        egui::RichText::new("Die ausgewählten Geräte haben keine Kanäle in dieser Kategorie.")
                            .color(theme::TEXT_DIM),
                    );
                    return;
                }
                let height = ui.available_height();
                egui::ScrollArea::horizontal()
                    .id_salt("faders")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            if self.category == Category::Position {
                                xy_pad(ui, cx.engine, &groups, cx.frame, height);
                                ui.add_space(theme::SP_S);
                            }
                            for grp in visible {
                                fader(ui, cx.engine, grp, cx.frame, height);
                            }
                        });
                    });
            });
    }
}

fn set_group(e: &mut Engine, grp: &FaderGroup, v: u8) {
    for ch in &grp.channels {
        e.set_programmer(*ch, v);
    }
}

fn current(e: &Engine, ch: ChannelRef, frame: &DmxFrame) -> u8 {
    e.programmer()
        .get(&ch)
        .copied()
        .or_else(|| e.output_value(frame, ch))
        .unwrap_or(0)
}

fn fader(ui: &mut egui::Ui, e: &mut Engine, grp: &FaderGroup, frame: &DmxFrame, height: f32) {
    let first = grp.channels[0];
    let active = e.programmer().contains_key(&first);
    let ev = Fader::new(current(e, first, frame), &grp.name)
        .active(active)
        .color(kind_color(grp.kind))
        .height(height)
        .hint(format!(
            "{} · {} – {} Kanal/Kanäle\nZiehen, Mausrad, Doppelklick = 0/voll, Rechtsklick = Menü",
            grp.name,
            grp.kind.label(),
            grp.channels.len()
        ))
        .show(ui);
    match ev {
        FaderEvent::Changed(v) => set_group(e, grp, v),
        FaderEvent::Release => {
            for ch in &grp.channels {
                e.clear_programmer_channel(*ch);
            }
        }
        FaderEvent::None => {}
    }
}

fn color_bar(ui: &mut egui::Ui, e: &mut Engine, groups: &[FaderGroup], frame: &DmxFrame) {
    let find = |k: ChannelKind| groups.iter().find(|g| g.kind == k);
    let (Some(r), Some(g), Some(b)) = (
        find(ChannelKind::Red),
        find(ChannelKind::Green),
        find(ChannelKind::Blue),
    ) else {
        return;
    };
    let mut rgb = [
        current(e, r.channels[0], frame),
        current(e, g.channels[0], frame),
        current(e, b.channels[0], frame),
    ];
    let mut changed = false;
    ui.horizontal(|ui| {
        changed |= ui.color_edit_button_srgb(&mut rgb).changed();
        ui.add_space(theme::SP_S);
        const PRESETS: [(&str, [u8; 3]); 10] = [
            ("Rot", [255, 0, 0]),
            ("Orange", [255, 90, 0]),
            ("Bernstein", [255, 150, 0]),
            ("Gelb", [255, 210, 0]),
            ("Grün", [0, 255, 0]),
            ("Türkis", [0, 255, 160]),
            ("Cyan", [0, 220, 255]),
            ("Blau", [0, 0, 255]),
            ("Magenta", [255, 0, 200]),
            ("Weiß", [255, 255, 255]),
        ];
        for (name, c) in PRESETS {
            if ui.add(Swatch(theme::rgb(c))).on_hover_text(name).clicked() {
                rgb = c;
                changed = true;
            }
        }
    });
    ui.add_space(theme::SP_XS);
    if changed {
        for (grp, v) in [(r, rgb[0]), (g, rgb[1]), (b, rgb[2])] {
            set_group(e, grp, v);
        }
    }
}

fn xy_pad(ui: &mut egui::Ui, e: &mut Engine, groups: &[FaderGroup], frame: &DmxFrame, height: f32) {
    let pan = groups.iter().find(|g| g.kind == ChannelKind::Pan);
    let tilt = groups.iter().find(|g| g.kind == ChannelKind::Tilt);
    if pan.is_none() && tilt.is_none() {
        return;
    }
    let pad = XyPad {
        x: pan.map_or(128, |g| current(e, g.channels[0], frame)),
        y: tilt.map_or(128, |g| current(e, g.channels[0], frame)),
        size: (height - 8.0).clamp(120.0, 260.0),
    };
    if let Some((x, y)) = pad.show(ui) {
        if let Some(g) = pan {
            set_group(e, g, x);
        }
        if let Some(g) = tilt {
            set_group(e, g, y);
        }
    }
}

/// Programmer-Inhalt als neue Szene oder als Schritt speichern.
fn save_bar(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
    if e.programmer().is_empty() {
        ui.label(
            egui::RichText::new("Fader bewegen, um Werte in den Programmer zu legen.")
                .color(theme::TEXT_FAINT),
        );
        return;
    }
    let values = programmer_values(e);
    ui.horizontal_wrapped(|ui| {
        ui.label(egui::RichText::new("Als Szene").color(theme::TEXT_DIM));
        ui.add(egui::TextEdit::singleline(&mut st.new_scene_name).desired_width(130.0));
        zone_combo(ui, e, &mut st.new_scene_zone, "new_scene_zone");
        if ui.button("Speichern").clicked() {
            let name = st.new_scene_name.trim().to_string();
            let id = e.show_mut().add_scene(
                name.clone(),
                st.new_scene_zone,
                vec![Step::new(values.clone())],
            );
            st.append_target = Some(id);
            st.info(format!("Szene „{name}“ angelegt"));
        }
        ui.separator();
        ui.label(egui::RichText::new("Schritt an").color(theme::TEXT_DIM));
        let name = st
            .append_target
            .and_then(|id| e.show().scene(id))
            .map_or_else(|| "Szene wählen".into(), |s| s.name.clone());
        egui::ComboBox::from_id_salt("append_target")
            .selected_text(name)
            .width(120.0)
            .show_ui(ui, |ui| {
                for s in &e.show().scenes {
                    ui.selectable_value(&mut st.append_target, Some(s.id), &s.name);
                }
            });
        if ui.button("Anhängen").clicked() {
            match st.append_target.and_then(|id| e.show_mut().scene_mut(id)) {
                Some(scene) => {
                    let mut step = Step::new(values.clone());
                    step.fade_ms = 1000;
                    scene.steps.push(step);
                    let msg = format!(
                        "Schritt {} an „{}“ angehängt",
                        scene.steps.len(),
                        scene.name
                    );
                    st.info(msg);
                }
                None => st.error("Bitte zuerst eine Szene wählen"),
            }
        }
    });
}

pub fn programmer_values(e: &Engine) -> Vec<ChannelValue> {
    let mut v: Vec<ChannelValue> = e
        .programmer()
        .iter()
        .map(|(ch, val)| ChannelValue {
            fixture: ch.fixture,
            channel: ch.channel,
            value: *val,
        })
        .collect();
    v.sort_by_key(|c| (c.fixture, c.channel));
    v
}

pub fn zone_combo(ui: &mut egui::Ui, e: &Engine, zone: &mut Option<ZoneId>, id: &str) -> bool {
    let name = zone
        .and_then(|z| e.show().zone(z))
        .map_or_else(|| "ohne Zone".into(), |z| z.name.clone());
    let before = *zone;
    egui::ComboBox::from_id_salt(id)
        .selected_text(name)
        .show_ui(ui, |ui| {
            ui.selectable_value(zone, None, "ohne Zone");
            for z in &e.show().zones {
                ui.selectable_value(zone, Some(z.id), &z.name);
            }
        });
    before != *zone
}
