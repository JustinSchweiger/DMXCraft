//! STEUERN → Programmer: ausgewählte Geräte live steuern und als Szene/Schritt
//! speichern.

use crate::i18n::t;
use dmxcraft_core::{ChannelKind, ChannelRef, ChannelValue, DmxFrame, Engine, FixtureId};
use eframe::egui::{self, Color32, Sense, Stroke, Vec2};

use crate::state::{AppState, ProgrammerMode};
use crate::ui::theme;
use crate::ui::widgets::fader::FADER_W;
use crate::ui::widgets::{Fader, FaderEvent, Icon, IconButton, Swatch, XyPad};
use crate::views::quick::{quick_action, Layer, Quick};
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
            Category::Beam => matches!(k, K::Strobe | K::Speed | K::Zoom | K::Focus),
            Category::Other => matches!(
                k,
                K::Generic | K::Laser | K::Fog | K::Fan | K::Program | K::Control | K::NoFunction
            ),
            Category::All => true,
        }
    }
}

use crate::ui::widgets::fader::kind_color;

/// Ein Fader steuert denselben Kanal aller ausgewählten Geräte eines Typs.
pub struct FaderGroup {
    pub name: String,
    pub kind: ChannelKind,
    /// Kanalnummer im Gerät (1-basiert).
    pub number: u16,
    /// Gerätetyp ([`dmxcraft_core::FixtureType::key`]).
    pub type_key: String,
    pub channels: Vec<ChannelRef>,
}

/// Fader eines Gerätetyps samt Namen der ausgewählten Geräte dieses Typs.
struct FixtureBlock<'a> {
    /// Typbezeichnung (Hersteller Modell)
    type_name: String,
    names: Vec<String>,
    faders: Vec<&'a FaderGroup>,
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
                .find(|g| g.type_key == f.type_key && g.number == i as u16 + 1)
            {
                Some(g) => g.channels.push(ch),
                None => groups.push(FaderGroup {
                    name: def.name.clone(),
                    kind: def.kind,
                    number: i as u16 + 1,
                    type_key: f.type_key.clone(),
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
    timeline: super::timeline::TimelineView,
}

impl View for ProgrammerView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        // SUPER SCENE markiert: Zeitleiste statt Fadern
        if let Some(id) = super::timeline::super_scene(cx.engine, cx.state) {
            self.timeline.ui(ui, cx.engine, cx.state, id);
            return;
        }
        let groups = fader_groups(cx.engine, cx.state);
        let layer = Layer::of(cx.state.programmer_mode);
        // angeklickter Feature-Effekt: Fader zeigen ihr FX-Symbol
        let target = cx.state.fx_target.and_then(|(scene, fx)| {
            let f = cx
                .engine
                .show()
                .scene(scene)?
                .effects
                .iter()
                .find(|f| f.id == fx)?;
            f.kind.uses_features().then(|| FxTarget {
                scene,
                fx,
                kinds: f.features.iter().map(|x| x.kind).collect(),
            })
        });
        ui.data_mut(|d| match target {
            Some(t) => {
                d.insert_temp(fx_target_id(), t);
            }
            None => d.remove::<FxTarget>(fx_target_id()),
        });
        toolbar(ui, cx.engine, cx.state);
        let dot = mode_color(cx.state.programmer_mode);
        ui.add_space(theme::SP_S);

        egui::Panel::left("programmer_categories")
            .resizable(false)
            .exact_size(132.0)
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                category_list(ui, cx.engine, layer, &groups, &mut self.category, dot)
            });

        // Fader-Bereich als eigene, abgesetzte Fläche (wie der Bühnenplan)
        egui::CentralPanel::default_margins()
            .frame(
                egui::Frame::new()
                    .fill(theme::bg())
                    .outer_margin(egui::Margin {
                        left: 8,
                        ..Default::default()
                    })
                    .inner_margin(egui::Margin::same(6)),
            )
            .show(ui, |ui| {
                if cx.state.selected.is_empty() {
                    return;
                }
                // „Alle Kanäle“ einzeln je Gerätetyp, alle anderen Kategorien
                // gemeinsam: ein Fader steuert die Kanäle aller ausgewählten Geräte
                let merged;
                let (visible, blocks): (Vec<&FaderGroup>, Vec<FixtureBlock<'_>>) =
                    if self.category == Category::All {
                        let v: Vec<&FaderGroup> = groups.iter().collect();
                        let b = fixture_blocks(cx.engine, cx.state, &v);
                        (v, b)
                    } else {
                        merged = merge_groups(&groups, self.category);
                        let v: Vec<&FaderGroup> = merged.iter().collect();
                        let b = merged_block(cx.engine, cx.state, &v);
                        (v, b)
                    };
                if self.category == Category::Color {
                    color_bar(ui, cx.engine, layer, &merged_or(&visible), cx.frame);
                }
                if visible.is_empty() {
                    return;
                }
                // eigener Platz für die Scrollleiste unter den Fadern – nur wenn
                // sie gebraucht wird (Stand aus dem letzten Frame)
                ui.spacing_mut().scroll = egui::style::ScrollStyle::solid();
                let overflow_id = ui.make_persistent_id("faders_overflow");
                let overflow: bool = ui.data(|d| d.get_temp(overflow_id)).unwrap_or(false);
                let height = if overflow {
                    ui.available_height() - ui.spacing().scroll.allocated_width() - 4.0
                } else {
                    ui.available_height()
                };
                let out = egui::ScrollArea::horizontal()
                    .id_salt("faders")
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            if self.category == Category::Position {
                                xy_pad(
                                    ui,
                                    cx.engine,
                                    layer,
                                    &merged_or(&visible),
                                    cx.frame,
                                    height,
                                );
                                ui.add_space(theme::SP_S);
                            }
                            ui.spacing_mut().item_spacing.x = 2.0;
                            // außer in „Alle Kanäle“ deutlich breitere Fader
                            let fader_w = if self.category == Category::All {
                                FADER_W
                            } else {
                                76.0
                            };
                            for block in &blocks {
                                fixture_block(ui, cx.engine, layer, block, height, dot, fader_w);
                            }
                        });
                    });
                let needs = out.content_size.x > out.inner_rect.width() + 0.5;
                if needs != overflow {
                    ui.data_mut(|d| d.insert_temp(overflow_id, needs));
                    ui.ctx().request_repaint();
                }
            });
    }
}

fn set_group(e: &mut Engine, layer: Layer, grp: &FaderGroup, v: u8) {
    for ch in &grp.channels {
        layer.set(e, *ch, v);
    }
}

fn current(e: &Engine, layer: Layer, ch: ChannelRef, frame: &DmxFrame) -> u8 {
    layer
        .get(e, ch)
        .or_else(|| e.output_value(frame, ch))
        .unwrap_or(0)
}

/// Farbe des Statuspunkts je Betriebsmodus.
fn mode_color(m: ProgrammerMode) -> Color32 {
    match m {
        ProgrammerMode::Edit => theme::OK,
        ProgrammerMode::Blind => theme::OK,
        ProgrammerMode::Live => theme::WARN,
    }
}

/// Fader einer Kategorie über alle Gerätetypen zusammenfassen: gleiche
/// Kanalart (bei „Sonstiges“ gleicher Name) wird zu einem Fader.
fn merge_groups(groups: &[FaderGroup], cat: Category) -> Vec<FaderGroup> {
    let mut out: Vec<FaderGroup> = Vec::new();
    for g in groups.iter().filter(|g| cat.contains(g.kind)) {
        let by_name = g.kind == ChannelKind::Generic;
        match out
            .iter_mut()
            .find(|o| o.kind == g.kind && (!by_name || o.name == g.name))
        {
            Some(o) => {
                o.channels.extend(g.channels.iter().copied());
                if o.type_key != g.type_key {
                    o.type_key.clear();
                }
            }
            None => out.push(FaderGroup {
                name: g.name.clone(),
                kind: g.kind,
                number: out.len() as u16 + 1,
                type_key: g.type_key.clone(),
                channels: g.channels.clone(),
            }),
        }
    }
    out
}

/// Gemeinsamer Block für zusammengefasste Fader.
fn merged_block<'a>(e: &Engine, st: &AppState, faders: &[&'a FaderGroup]) -> Vec<FixtureBlock<'a>> {
    if faders.is_empty() {
        return Vec::new();
    }
    let show = e.show();
    let fixtures: Vec<_> = st
        .selected
        .iter()
        .filter_map(|id| show.fixture(*id))
        .collect();
    let one_type = fixtures.windows(2).all(|w| w[0].type_key == w[1].type_key);
    let type_name = match (one_type, fixtures.first()) {
        (true, Some(f)) => show.type_of(f).map_or_else(
            || f.type_key.clone(),
            |t| format!("{} {}", t.manufacturer, t.model).trim().to_string(),
        ),
        _ => tr!("Alle ausgewählten Geräte ({n})", n = fixtures.len()),
    };
    vec![FixtureBlock {
        type_name,
        names: fixtures.iter().map(|f| f.name.clone()).collect(),
        faders: faders.to_vec(),
    }]
}

/// Besitzende Kopie der sichtbaren Fader (für Farb- und Positionsfeld).
fn merged_or(v: &[&FaderGroup]) -> Vec<FaderGroup> {
    v.iter()
        .map(|g| FaderGroup {
            name: g.name.clone(),
            kind: g.kind,
            number: g.number,
            type_key: g.type_key.clone(),
            channels: g.channels.clone(),
        })
        .collect()
}

/// Fader nach Gerätetyp bündeln (Reihenfolge wie in der Auswahl).
fn fixture_blocks<'a>(
    e: &Engine,
    st: &AppState,
    faders: &[&'a FaderGroup],
) -> Vec<FixtureBlock<'a>> {
    let mut blocks: Vec<(String, FixtureBlock<'a>)> = Vec::new();
    for f in st.selected.iter().filter_map(|id| e.show().fixture(*id)) {
        match blocks.iter_mut().find(|(k, _)| *k == f.type_key) {
            Some((_, b)) => b.names.push(f.name.clone()),
            None => blocks.push((
                f.type_key.clone(),
                FixtureBlock {
                    type_name: e.show().type_of(f).map_or_else(
                        || f.type_key.clone(),
                        |t| format!("{} {}", t.manufacturer, t.model).trim().to_string(),
                    ),
                    names: vec![f.name.clone()],
                    faders: faders
                        .iter()
                        .copied()
                        .filter(|g| g.type_key == f.type_key)
                        .collect(),
                },
            )),
        }
    }
    blocks
        .into_iter()
        .map(|(_, b)| b)
        .filter(|b| !b.faders.is_empty())
        .collect()
}

/// Ein Block: Kopfzeile mit den Gerätenamen, darunter die Fader.
fn fixture_block(
    ui: &mut egui::Ui,
    e: &mut Engine,
    layer: Layer,
    block: &FixtureBlock<'_>,
    height: f32,
    dot: Color32,
    fader_w: f32,
) {
    const HEAD_H: f32 = 22.0;
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let w = block.faders.len() as f32 * fader_w;
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, HEAD_H), Sense::hover());
        let p = ui.painter();
        p.rect_filled(
            rect,
            egui::CornerRadius {
                nw: theme::R_S,
                ne: theme::R_S,
                sw: 0,
                se: 0,
            },
            theme::raised_hover(),
        );
        // ein Gerät: sein Name; mehrere desselben Typs: die Typbezeichnung
        let title = if block.names.len() == 1 {
            block.names[0].clone()
        } else {
            block.type_name.clone()
        };
        let text = elide(p, &title.to_uppercase(), theme::heading_font(), w - 10.0);
        p.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text,
            theme::heading_font(),
            theme::text_dim(),
        );
        let title = format!("{title}\n{}", block.names.join(", "));
        resp.on_hover_text(title);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for grp in &block.faders {
                fader(ui, e, layer, grp, height - HEAD_H, dot, fader_w);
            }
        });
    });
}

fn fader(
    ui: &mut egui::Ui,
    e: &mut Engine,
    layer: Layer,
    grp: &FaderGroup,
    height: f32,
    dot: Color32,
    width: f32,
) {
    let first = grp.channels[0];
    let active = layer.map(e).contains_key(&first);
    // nicht gespeicherte Kanäle stehen auf 0
    let value = layer.get(e, first).unwrap_or(0);
    // FX-Symbol, solange ein Feature-Effekt angeklickt ist
    let target: Option<FxTarget> = ui.data(|d| d.get_temp(fx_target_id()));
    let assigned = target.as_ref().map(|t| t.kinds.contains(&grp.kind));
    let ev = Fader::new(value, grp.number, grp.kind)
        .active(active)
        .dot(dot)
        .color(kind_color(grp.kind))
        .height(height)
        .width(width)
        .hint(grp.name.clone())
        .fx(assigned)
        .show(ui);
    if ev == FaderEvent::ToggleFx {
        if let Some(t) = target {
            toggle_feature(e, t.scene, t.fx, grp.kind);
        }
        return;
    }
    match ev {
        FaderEvent::Changed(v) => set_group(e, layer, grp, v),
        FaderEvent::Release => {
            for ch in &grp.channels {
                layer.clear(e, *ch);
            }
        }
        FaderEvent::None | FaderEvent::ToggleFx => {}
    }
}

/// Feature-Effekt, dem Fader zugeordnet werden können (für diesen Frame).
#[derive(Clone)]
struct FxTarget {
    scene: dmxcraft_core::SceneId,
    fx: u32,
    kinds: Vec<dmxcraft_core::ChannelKind>,
}

fn fx_target_id() -> egui::Id {
    egui::Id::unique("programmer_fx_target")
}

/// Kanaltyp dem Effekt zuordnen bzw. wieder entfernen.
fn toggle_feature(
    e: &mut Engine,
    scene: dmxcraft_core::SceneId,
    fx: u32,
    kind: dmxcraft_core::ChannelKind,
) {
    let Some(f) = e
        .show_mut()
        .scene_mut(scene)
        .and_then(|s| s.effects.iter_mut().find(|f| f.id == fx))
    else {
        return;
    };
    if let Some(i) = f.features.iter().position(|x| x.kind == kind) {
        f.features.remove(i);
    } else {
        f.features
            .push(dmxcraft_core::fx::feature::Feature::new(kind));
    }
}

fn color_bar(
    ui: &mut egui::Ui,
    e: &mut Engine,
    layer: Layer,
    groups: &[FaderGroup],
    frame: &DmxFrame,
) {
    let find = |k: ChannelKind| groups.iter().find(|g| g.kind == k);
    let (Some(r), Some(g), Some(b)) = (
        find(ChannelKind::Red),
        find(ChannelKind::Green),
        find(ChannelKind::Blue),
    ) else {
        return;
    };
    let mut rgb = [
        current(e, layer, r.channels[0], frame),
        current(e, layer, g.channels[0], frame),
        current(e, layer, b.channels[0], frame),
    ];
    let mut changed = false;
    ui.horizontal(|ui| {
        // aktuelle Farbe; Klick öffnet den Farbwähler (wie bei Szenen/Bänken)
        let current = ui
            .add(Swatch(theme::rgb(rgb)))
            .on_hover_text(t("Eigene Farbe wählen"));
        crate::ui::widgets::menu::dropdown(&current, |ui| {
            ui.set_min_width(212.0);
            changed |= crate::ui::widgets::color_picker::color_picker(
                ui,
                ui.make_persistent_id("programmer_color"),
                &mut rgb,
            );
        });
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
            if ui
                .add(Swatch(theme::rgb(c)))
                .on_hover_text(t(name))
                .clicked()
            {
                rgb = c;
                changed = true;
            }
        }
    });
    ui.add_space(theme::SP_XS);
    if changed {
        for (grp, v) in [(r, rgb[0]), (g, rgb[1]), (b, rgb[2])] {
            set_group(e, layer, grp, v);
        }
    }
}

fn xy_pad(
    ui: &mut egui::Ui,
    e: &mut Engine,
    layer: Layer,
    groups: &[FaderGroup],
    frame: &DmxFrame,
    height: f32,
) {
    let pan = groups.iter().find(|g| g.kind == ChannelKind::Pan);
    let tilt = groups.iter().find(|g| g.kind == ChannelKind::Tilt);
    if pan.is_none() && tilt.is_none() {
        return;
    }
    let pad = XyPad {
        x: pan.map_or(128, |g| current(e, layer, g.channels[0], frame)),
        y: tilt.map_or(128, |g| current(e, layer, g.channels[0], frame)),
        size: (height - 8.0).clamp(120.0, 260.0),
    };
    if let Some((x, y)) = pad.show(ui) {
        if let Some(g) = pan {
            set_group(e, layer, g, x);
        }
        if let Some(g) = tilt {
            set_group(e, layer, g, y);
        }
    }
}

/// Aktuelle Programmer-Werte (z. B. für eine neue Szene).
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

/// Werkzeugleiste: Betriebsmodus (Edit/Blind/Live) und Schnellaktionen.
fn toolbar(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
    let ids: Vec<FixtureId> = st.selected.iter().copied().collect();
    let some = !ids.is_empty();
    let locked = st.locked;
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = theme::SP_XS;
        mode_switch(ui, &mut st.programmer_mode, locked);
        let layer = Layer::of(st.programmer_mode);
        tool_separator(ui);
        let mut quick = None;
        for (q, icon, tip) in [
            (Quick::Off, Icon::LightOff, t("Beams ausschalten")),
            (Quick::On, Icon::LightOn, t("Beams anschalten")),
            (
                Quick::Center,
                Icon::Center,
                t("Beams zentrieren (Pan/Tilt 50 %)"),
            ),
        ] {
            if ui
                .add(
                    IconButton::new(icon)
                        .size(34.0)
                        .flat()
                        .enabled(some && !locked),
                )
                .on_hover_text(tip)
                .clicked()
            {
                quick = Some(q);
            }
        }
        if let Some(q) = quick {
            quick_action(e, layer, &ids, q);
        }
        tool_separator(ui);
        let any = !layer.map(e).is_empty();
        if ui
            .add(
                IconButton::new(Icon::Undo)
                    .caption(t("ALLE"))
                    .size(34.0)
                    .flat()
                    .enabled(any && !locked),
            )
            .on_hover_text(t("Alle Beams zurücksetzen"))
            .clicked()
        {
            layer.clear_all(e);
        }
        // nur die Ebene des aktuellen Modus (Live bzw. Edit)
        let sel: Vec<ChannelRef> = layer
            .map(e)
            .keys()
            .filter(|c| st.selected.contains(&c.fixture))
            .copied()
            .collect();
        if ui
            .add(
                IconButton::new(Icon::Undo)
                    .caption(t("AUSW."))
                    .size(34.0)
                    .flat()
                    .enabled(!sel.is_empty() && !locked),
            )
            .on_hover_text(t("Ausgewählte Beams zurücksetzen"))
            .clicked()
        {
            for c in sel {
                layer.clear(e, c);
            }
        }
    });
}

pub(super) fn tool_separator(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(theme::SP_M, 24.0), Sense::hover());
    ui.painter().line_segment(
        [
            egui::pos2(rect.center().x, rect.top()),
            egui::pos2(rect.center().x, rect.bottom()),
        ],
        Stroke::new(1.0, theme::border()),
    );
}

/// Drei zusammenhängende Knöpfe EDIT · BLIND · LIVE; der aktive Modus ist in
/// seiner Farbe gefüllt.
pub(super) fn mode_switch(ui: &mut egui::Ui, mode: &mut ProgrammerMode, locked: bool) {
    let items = [
        (
            ProgrammerMode::Edit,
            Some(t("EDIT")),
            None,
            theme::OK,
            t("Edit: Werte bearbeiten und sofort ausgeben"),
        ),
        (
            ProgrammerMode::Blind,
            None,
            Some(Icon::EyeOff),
            theme::OK,
            t("Blind Edit: wie Edit, aber der DMX-Ausgang bleibt unverändert"),
        ),
        (
            ProgrammerMode::Live,
            Some(t("LIVE")),
            None,
            theme::WARN,
            t("Live: Show-Betrieb"),
        ),
    ];
    let h = 34.0;
    let font = theme::heading_font();
    let widths: Vec<f32> = items
        .iter()
        .map(|(_, label, _, _, _)| match label {
            Some(l) => {
                ui.painter()
                    .layout_no_wrap(l.to_string(), font.clone(), Color32::WHITE)
                    .size()
                    .x
                    + 2.0 * theme::SP_M
            }
            None => h + 6.0,
        })
        .collect();
    let total: f32 = widths.iter().sum();
    let (rect, _) = ui.allocate_exact_size(Vec2::new(total, h), Sense::hover());
    let r = theme::R_S;
    ui.painter().rect_filled(rect, r, theme::raised());
    let mut x = rect.left();
    for (i, ((m, label, icon, color, tip), w)) in items.into_iter().zip(widths).enumerate() {
        let seg = egui::Rect::from_min_size(egui::pos2(x, rect.top()), Vec2::new(w, h));
        x += w;
        let resp = ui
            .interact(seg, ui.make_persistent_id(("prog_mode", i)), Sense::click())
            .on_hover_text(tip);
        // Blind ist ein Zusatz zu Edit: beide Knöpfe leuchten
        let active = *mode == m || (m == ProgrammerMode::Edit && *mode == ProgrammerMode::Blind);
        let corner = egui::CornerRadius {
            nw: if i == 0 { r } else { 0 },
            sw: if i == 0 { r } else { 0 },
            ne: if i == 2 { r } else { 0 },
            se: if i == 2 { r } else { 0 },
        };
        let p = ui.painter();
        if active {
            p.rect_filled(seg, corner, color);
        } else if resp.hovered() && !locked {
            p.rect_filled(seg, corner, theme::raised_hover());
        }
        if i == 1 && *mode == ProgrammerMode::Blind {
            // Edit + Blind: feine Fuge zwischen den beiden leuchtenden Knöpfen
            p.line_segment(
                [seg.left_top(), seg.left_bottom()],
                Stroke::new(1.0, theme::bg()),
            );
        }
        if i > 0 && !active && *mode != items_mode(i - 1) {
            p.line_segment(
                [
                    seg.left_top() + Vec2::new(0.0, 6.0),
                    seg.left_bottom() - Vec2::new(0.0, 6.0),
                ],
                Stroke::new(1.0, theme::border()),
            );
        }
        let fg = if active {
            theme::bg()
        } else {
            theme::text_dim()
        };
        match (label, icon) {
            (Some(l), _) => {
                p.text(
                    seg.center(),
                    egui::Align2::CENTER_CENTER,
                    l,
                    font.clone(),
                    fg,
                );
            }
            (None, Some(ic)) => {
                ic.paint(
                    p,
                    egui::Rect::from_center_size(seg.center(), Vec2::splat(16.0)),
                    fg,
                );
            }
            _ => {}
        }
        if resp.clicked() && !locked {
            // Auge schaltet Blind ein/aus (immer zusammen mit Edit)
            *mode = match (m, *mode) {
                (ProgrammerMode::Blind, ProgrammerMode::Blind) => ProgrammerMode::Edit,
                (m, _) => m,
            };
        }
    }
    ui.painter().rect_stroke(
        rect,
        r,
        Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
    );
}

fn items_mode(i: usize) -> ProgrammerMode {
    [
        ProgrammerMode::Edit,
        ProgrammerMode::Blind,
        ProgrammerMode::Live,
    ][i]
}

/// Seitenleiste der Kategorien: Liste mit Markierungsbalken links; die Zahl
/// rechts nennt die Fader der Auswahl in dieser Kategorie.
fn category_list(
    ui: &mut egui::Ui,
    e: &Engine,
    layer: Layer,
    groups: &[FaderGroup],
    current: &mut Category,
    dot: Color32,
) {
    let full = ui.available_rect_before_wrap();
    let p = ui.painter().clone();
    // schlichte Liste auf der Fläche, rechts eine Trennlinie wie zwischen
    // den übrigen Bereichen
    p.line_segment(
        [full.right_top(), full.right_bottom()],
        Stroke::new(1.0, theme::border()),
    );
    let row_h = 32.0;
    let mut y = full.top() + theme::SP_XS;
    for (i, (c, label)) in Category::ITEMS.iter().enumerate() {
        if *c == Category::All {
            // Trennlinie vor „Alle Kanäle“
            y += theme::SP_XS;
            p.line_segment(
                [
                    egui::pos2(full.left() + theme::SP_S, y),
                    egui::pos2(full.right() - theme::SP_S, y),
                ],
                Stroke::new(1.0, theme::border()),
            );
            y += theme::SP_XS;
        }
        let row = egui::Rect::from_min_size(
            egui::pos2(full.left() + 3.0, y),
            Vec2::new(full.width() - 6.0, row_h),
        );
        y += row_h + 1.0;
        let n = if *c == Category::All {
            groups.len()
        } else {
            merge_groups(groups, *c).len()
        };
        let resp = ui.interact(row, ui.make_persistent_id(("prog_cat", i)), Sense::click());
        let active = *current == *c;
        let empty = n == 0;
        if active {
            p.rect_filled(row, theme::R_S, theme::raised());
            let bar = egui::Rect::from_min_size(
                row.left_top() + Vec2::new(0.0, 6.0),
                Vec2::new(3.0, row_h - 12.0),
            );
            p.rect_filled(bar, 1.5, theme::accent());
        } else if resp.hovered() {
            p.rect_filled(row, theme::R_S, theme::raised_hover());
        }
        let fg = if active {
            theme::text()
        } else if empty {
            theme::text_faint()
        } else {
            theme::text_dim()
        };
        p.text(
            egui::pos2(row.left() + theme::SP_M, row.center().y),
            egui::Align2::LEFT_CENTER,
            t(label),
            theme::body(),
            fg,
        );
        // Punkt: in dieser Kategorie gibt der Programmer Werte aus
        let set = groups
            .iter()
            .filter(|g| c.contains(g.kind))
            .any(|g| g.channels.iter().any(|ch| layer.map(e).contains_key(ch)));
        if set {
            p.circle_filled(egui::pos2(row.right() - 26.0, row.center().y), 3.5, dot);
        }
        if n > 0 {
            p.text(
                egui::pos2(row.right() - theme::SP_S, row.center().y),
                egui::Align2::RIGHT_CENTER,
                n.to_string(),
                theme::caption(),
                if active {
                    theme::accent()
                } else {
                    theme::text_faint()
                },
            );
        }
        if resp.clicked() {
            *current = *c;
        }
    }
}

/// Text auf `max` Pixel kürzen und mit „...“ abschließen.
fn elide(p: &egui::Painter, text: &str, font: egui::FontId, max: f32) -> String {
    let width = |s: &str| {
        p.layout_no_wrap(s.to_string(), font.clone(), Color32::WHITE)
            .size()
            .x
    };
    if width(text) <= max {
        return text.to_string();
    }
    let chars: Vec<char> = text.chars().collect();
    let mut n = chars.len();
    while n > 0 {
        n -= 1;
        let s: String = chars[..n].iter().collect::<String>().trim_end().to_string() + "...";
        if width(&s) <= max {
            return s;
        }
    }
    "...".into()
}
