//! Programmer: ausgewählte Geräte live per Fader steuern und das Ergebnis als
//! Szene oder Schritt speichern.

use dmxcraft_core::{ChannelKind, ChannelRef, ChannelValue, DmxFrame, Engine, Step};
use eframe::egui;

use crate::app::UiState;
use crate::theme;

/// Ein Fader in der Oberfläche; steuert gleichnamige Kanäle aller
/// ausgewählten Geräte gemeinsam.
struct FaderGroup {
    name: String,
    kind: ChannelKind,
    channels: Vec<ChannelRef>,
}

fn fader_groups(e: &Engine, st: &UiState) -> Vec<FaderGroup> {
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

/// Kanal-Kategorien wie auf einem Lichtpult.
#[derive(Clone, Copy, PartialEq, Eq, Default)]
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
    pub const ALL: [Category; 7] = [
        Category::Dimmer,
        Category::Color,
        Category::Position,
        Category::Gobo,
        Category::Beam,
        Category::Other,
        Category::All,
    ];

    fn label(self) -> &'static str {
        match self {
            Category::Dimmer => "DIMMER",
            Category::Color => "FARBE",
            Category::Position => "POSITION",
            Category::Gobo => "GOBO",
            Category::Beam => "STRAHL",
            Category::Other => "SONSTIGES",
            Category::All => "ALLE KANÄLE",
        }
    }

    fn contains(self, k: ChannelKind) -> bool {
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

pub fn ui(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState, frame: &DmxFrame) {
    let groups = fader_groups(e, st);
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new("PROGRAMMER")
                .small()
                .color(theme::TEXT_WEAK),
        );
        ui.label(format!("{} Gerät(e)", st.selected.len()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .small_button("Leeren")
                .on_hover_text("Alle Programmer-Werte freigeben")
                .clicked()
            {
                e.clear_programmer();
            }
            ui.weak(format!("{} Kanäle gesetzt", e.programmer().len()));
        });
    });
    ui.separator();

    ui.horizontal_top(|ui| {
        // Kategorien
        ui.vertical(|ui| {
            ui.set_width(104.0);
            for c in Category::ALL {
                let n = groups.iter().filter(|g| c.contains(g.kind)).count();
                let text = egui::RichText::new(c.label()).size(12.0);
                let text = if n == 0 && c != Category::All {
                    text.color(theme::TEXT_WEAK.gamma_multiply(0.6))
                } else {
                    text
                };
                if ui
                    .add_sized(
                        [100.0, 22.0],
                        egui::Button::selectable(st.prog_category == c, text),
                    )
                    .clicked()
                {
                    st.prog_category = c;
                }
            }
        });
        ui.separator();

        ui.vertical(|ui| {
            if st.selected.is_empty() {
                ui.add_space(12.0);
                ui.weak("Im Bühnenplan Geräte auswählen (Klick, Ziehen, Strg+Klick).");
                return;
            }
            let visible: Vec<&FaderGroup> = groups
                .iter()
                .filter(|g| st.prog_category.contains(g.kind))
                .collect();
            if st.prog_category == Category::Color {
                color_picker(ui, e, &groups, frame);
            }
            if visible.is_empty() {
                ui.weak("Die ausgewählten Geräte haben keine Kanäle in dieser Kategorie.");
                return;
            }
            ui.horizontal_top(|ui| {
                if st.prog_category == Category::Position {
                    xy_pad(ui, e, &groups, frame);
                    ui.add_space(8.0);
                }
                egui::ScrollArea::horizontal()
                    .id_salt("faders")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            for grp in visible {
                                fader(ui, e, grp, frame);
                            }
                        });
                    });
            });
        });
    });
    ui.separator();
    programmer_actions(ui, e, st);
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

fn fader(ui: &mut egui::Ui, e: &mut Engine, grp: &FaderGroup, frame: &DmxFrame) {
    let first = grp.channels[0];
    let in_programmer = e.programmer().contains_key(&first);
    let mut v = current(e, first, frame);
    ui.vertical(|ui| {
        ui.set_width(58.0);
        let title = egui::RichText::new(&grp.name).small();
        ui.label(if in_programmer {
            title.strong().color(theme::ACCENT)
        } else {
            title.color(theme::TEXT_WEAK)
        })
        .on_hover_text(format!(
            "{} – {} Kanal/Kanäle",
            grp.kind.label(),
            grp.channels.len()
        ));
        ui.spacing_mut().slider_width = 130.0;
        if ui
            .add(
                egui::Slider::new(&mut v, 0..=255)
                    .vertical()
                    .show_value(true),
            )
            .changed()
        {
            set_group(e, grp, v);
        }
        ui.horizontal(|ui| {
            if ui.small_button("0").clicked() {
                set_group(e, grp, 0);
            }
            if ui.small_button("FL").on_hover_text("Voll (255)").clicked() {
                set_group(e, grp, 255);
            }
        });
        if in_programmer
            && ui
                .small_button("frei")
                .on_hover_text("Kanal aus dem Programmer entfernen")
                .clicked()
        {
            for ch in &grp.channels {
                e.clear_programmer_channel(*ch);
            }
        }
    });
}

fn color_picker(ui: &mut egui::Ui, e: &mut Engine, groups: &[FaderGroup], frame: &DmxFrame) {
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
    ui.horizontal_wrapped(|ui| {
        ui.label("Farbe");
        changed |= ui.color_edit_button_srgb(&mut rgb).changed();
        const PRESETS: [(&str, [u8; 3]); 8] = [
            ("Rot", [255, 0, 0]),
            ("Orange", [255, 100, 0]),
            ("Gelb", [255, 200, 0]),
            ("Grün", [0, 255, 0]),
            ("Cyan", [0, 255, 255]),
            ("Blau", [0, 0, 255]),
            ("Magenta", [255, 0, 200]),
            ("Weiß", [255, 255, 255]),
        ];
        for (name, c) in PRESETS {
            let (rect, resp) = ui.allocate_exact_size(egui::vec2(20.0, 20.0), egui::Sense::click());
            ui.painter()
                .rect_filled(rect, 3.0, egui::Color32::from_rgb(c[0], c[1], c[2]));
            if resp.on_hover_text(name).clicked() {
                rgb = c;
                changed = true;
            }
        }
    });
    if changed {
        for (grp, v) in [(r, rgb[0]), (g, rgb[1]), (b, rgb[2])] {
            set_group(e, grp, v);
        }
    }
}

fn xy_pad(ui: &mut egui::Ui, e: &mut Engine, groups: &[FaderGroup], frame: &DmxFrame) {
    let pan = groups.iter().find(|g| g.kind == ChannelKind::Pan);
    let tilt = groups.iter().find(|g| g.kind == ChannelKind::Tilt);
    if pan.is_none() && tilt.is_none() {
        return;
    }
    let px = pan.map_or(128, |g| current(e, g.channels[0], frame));
    let ty = tilt.map_or(128, |g| current(e, g.channels[0], frame));
    let (rect, resp) =
        ui.allocate_exact_size(egui::vec2(150.0, 150.0), egui::Sense::click_and_drag());
    let p = ui.painter();
    p.rect_filled(rect, 4.0, theme::BG);
    p.line_segment(
        [rect.center_top(), rect.center_bottom()],
        egui::Stroke::new(1.0, theme::CARD),
    );
    p.line_segment(
        [rect.left_center(), rect.right_center()],
        egui::Stroke::new(1.0, theme::CARD),
    );
    let dot = egui::pos2(
        rect.left() + f32::from(px) / 255.0 * rect.width(),
        rect.top() + f32::from(ty) / 255.0 * rect.height(),
    );
    p.circle_filled(dot, 6.0, theme::ACCENT);
    p.text(
        rect.left_top() + egui::vec2(4.0, 2.0),
        egui::Align2::LEFT_TOP,
        format!("Pan {px} · Tilt {ty}"),
        egui::FontId::proportional(10.0),
        theme::TEXT_WEAK,
    );
    if let Some(pos) = resp.interact_pointer_pos() {
        if resp.dragged() || resp.clicked() {
            let nx = ((pos.x - rect.left()) / rect.width()).clamp(0.0, 1.0);
            let ny = ((pos.y - rect.top()) / rect.height()).clamp(0.0, 1.0);
            if let Some(g) = pan {
                set_group(e, g, (nx * 255.0).round() as u8);
            }
            if let Some(g) = tilt {
                set_group(e, g, (ny * 255.0).round() as u8);
            }
        }
    }
}

/// Speichern des Programmer-Inhalts als Szene / Schritt.
fn programmer_actions(ui: &mut egui::Ui, e: &mut Engine, st: &mut UiState) {
    if e.programmer().is_empty() {
        ui.weak("Fader bewegen, um Werte in den Programmer zu legen.");
        return;
    }
    let values = programmer_values(e);

    ui.horizontal(|ui| {
        ui.label("Als neue Szene:");
        ui.text_edit_singleline(&mut st.new_scene_name);
        zone_combo(ui, e, &mut st.new_scene_zone, "new_scene_zone");
        if ui.button("Speichern").clicked() {
            let id = e.show_mut().add_scene(
                st.new_scene_name.trim().to_string(),
                st.new_scene_zone,
                vec![Step::new(values.clone())],
            );
            st.edit_scene = Some(id);
            st.append_target = Some(id);
            st.info(format!("Szene „{}“ angelegt", st.new_scene_name.trim()));
        }
    });

    ui.horizontal(|ui| {
        ui.label("Als Schritt anhängen an:");
        let name = st
            .append_target
            .and_then(|id| e.show().scene(id))
            .map_or_else(|| "– Szene wählen –".into(), |s| s.name.clone());
        egui::ComboBox::from_id_salt("append_target")
            .selected_text(name)
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

pub fn zone_combo(
    ui: &mut egui::Ui,
    e: &Engine,
    zone: &mut Option<dmxcraft_core::ZoneId>,
    id: &str,
) -> bool {
    let name = zone
        .and_then(|z| e.show().zones.iter().find(|x| x.id == z))
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
