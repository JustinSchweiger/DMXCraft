//! STEUERN → Programmer: ausgewählte Geräte live steuern und als Szene/Schritt
//! speichern.

use crate::i18n::t;
use dmxcraft_core::{ChannelKind, ChannelRef, ChannelValue, DmxFrame, Engine, FixtureId};
use eframe::egui::{self, Color32, Sense, Stroke, Vec2};

use crate::state::{AppState, ProgrammerMode};
use crate::ui::theme;
use crate::ui::widgets::fader::FADER_W;
use crate::ui::widgets::{section_header, Fader, FaderEvent, Icon, IconButton, Swatch, XyPad};
use crate::views::quick::{quick_action, Quick};
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
        _ => theme::accent(),
    }
}

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
}

impl View for ProgrammerView {
    fn ui(&mut self, ui: &mut egui::Ui, cx: &mut ViewCtx<'_>) {
        let groups = fader_groups(cx.engine, cx.state);
        let set_count = cx.engine.programmer().len();
        let selected = cx.state.selected.len();
        section_header(ui, t("Programmer"), |ui| {
            ui.label(
                egui::RichText::new(tr!(
                    "{selected} Geräte · {set_count} Kanäle gesetzt",
                    selected = selected,
                    set_count = set_count
                ))
                .color(theme::text_dim()),
            );
        });
        toolbar(ui, cx.engine, cx.state);
        let dot = mode_color(cx.state.programmer_mode);
        ui.add_space(theme::SP_S);

        egui::Panel::left("programmer_categories")
            .resizable(false)
            .exact_size(132.0)
            .frame(egui::Frame::new())
            .show(ui, |ui| {
                category_list(ui, cx.engine, &groups, &mut self.category, dot)
            });

        egui::CentralPanel::default_margins()
            .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(8, 0)))
            .show(ui, |ui| {
                if cx.state.selected.is_empty() {
                    return;
                }
                let visible: Vec<&FaderGroup> = groups
                    .iter()
                    .filter(|g| self.category.contains(g.kind))
                    .collect();
                let blocks = fixture_blocks(cx.engine, cx.state, &visible);
                if self.category == Category::Color {
                    color_bar(ui, cx.engine, &groups, cx.frame);
                }
                if visible.is_empty() {
                    ui.label(
                        egui::RichText::new(t(
                            "Die ausgewählten Geräte haben keine Kanäle in dieser Kategorie.",
                        ))
                        .color(theme::text_dim()),
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
                            ui.spacing_mut().item_spacing.x = 2.0;
                            for block in &blocks {
                                fixture_block(ui, cx.engine, block, cx.frame, height, dot);
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

/// Farbe des Statuspunkts je Betriebsmodus.
fn mode_color(m: ProgrammerMode) -> Color32 {
    match m {
        ProgrammerMode::Edit => theme::OK,
        ProgrammerMode::Blind => theme::DANGER,
        ProgrammerMode::Live => theme::WARN,
    }
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
    block: &FixtureBlock<'_>,
    frame: &DmxFrame,
    height: f32,
    dot: Color32,
) {
    const HEAD_H: f32 = 22.0;
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing = Vec2::ZERO;
        let w = block.faders.len() as f32 * FADER_W;
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
            theme::bg(),
        );
        let title = block.names.join(", ");
        let galley = p.layout_no_wrap(title.clone(), theme::heading_font(), theme::text_dim());
        let text = if galley.size().x > w - 12.0 && block.names.len() > 1 {
            tr!(
                "{name} + {n} weitere",
                name = block.names[0],
                n = block.names.len() - 1
            )
        } else {
            title.clone()
        };
        p.with_clip_rect(rect.shrink2(Vec2::new(4.0, 0.0))).text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            text.to_uppercase(),
            theme::heading_font(),
            theme::text_dim(),
        );
        resp.on_hover_text(title);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing = Vec2::ZERO;
            for grp in &block.faders {
                fader(ui, e, grp, frame, height - HEAD_H, dot);
            }
        });
    });
}

fn fader(
    ui: &mut egui::Ui,
    e: &mut Engine,
    grp: &FaderGroup,
    frame: &DmxFrame,
    height: f32,
    dot: Color32,
) {
    let first = grp.channels[0];
    let active = e.programmer().contains_key(&first);
    let ev = Fader::new(current(e, first, frame), grp.number, grp.kind)
        .active(active)
        .dot(dot)
        .color(kind_color(grp.kind))
        .height(height)
        .hint(tr!("{v} · {label} – {n} Kanal/Kanäle\nZiehen, Mausrad, Doppelklick = 0/voll, Rechtsklick = Menü", v = grp.name, label = t(grp.kind.label()), n = grp.channels.len()))
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
            quick_action(e, &ids, q);
        }
        tool_separator(ui);
        let any = !e.programmer().is_empty();
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
            e.clear_programmer();
        }
        let sel: Vec<ChannelRef> = e
            .programmer()
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
                e.clear_programmer_channel(c);
            }
        }
    });
}

fn tool_separator(ui: &mut egui::Ui) {
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
fn mode_switch(ui: &mut egui::Ui, mode: &mut ProgrammerMode, locked: bool) {
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
            theme::DANGER,
            t("Blind Edit: Werte bearbeiten, ohne sie auszugeben"),
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
        let active = *mode == m;
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
            *mode = m;
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
    groups: &[FaderGroup],
    current: &mut Category,
    dot: Color32,
) {
    let full = ui.available_rect_before_wrap();
    let p = ui.painter().clone();
    p.rect_filled(full, theme::R_M, theme::bg());
    p.rect_stroke(
        full,
        theme::R_M,
        Stroke::new(1.0, theme::border()),
        egui::StrokeKind::Inside,
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
        let n = groups.iter().filter(|g| c.contains(g.kind)).count();
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
            .any(|g| g.channels.iter().any(|ch| e.programmer().contains_key(ch)));
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
