//! Effect list of a scene: preview, import/export, tiles, steps table, headers.

#[allow(unused_imports)]
use super::*;

/// Preview of an effect for the sidebar.
pub(crate) struct FxPreview {
    /// Color or level (gray) per beam.
    pub(crate) colors: Vec<[u8; 3]>,
    /// MOVE FX: pan/tilt per beam.
    pub(crate) positions: Vec<[f32; 2]>,
    /// Channel types of the effect fixtures.
    pub(crate) kinds: Vec<dmxcraft_core::ChannelKind>,
}

impl FxPreview {
    pub(crate) fn of(show: &dmxcraft_core::Show, fx: &dmxcraft_core::Effect, cycles: f64) -> Self {
        use dmxcraft_core::fx;
        let mut kinds: Vec<dmxcraft_core::ChannelKind> = Vec::new();
        for f in fx.fixtures.iter().filter_map(|id| show.fixture(*id)) {
            for c in show
                .type_of(f)
                .map(|t| t.channels.as_slice())
                .unwrap_or(&[])
            {
                if !kinds.contains(&c.kind) {
                    kinds.push(c.kind);
                }
            }
        }
        let fbeams = fx::feature::fixture_beams(show, &fx.fixtures);
        let colors = match fx.kind {
            EffectKind::ColorFx => fx
                .color
                .as_ref()
                .map(|c| fx::render(c, &fx::beams(show, &fx.fixtures), cycles))
                .unwrap_or_default(),
            k if k.uses_features() => fx::feature_levels(fx, &fbeams, cycles)
                .into_iter()
                .map(|l| [(l.unwrap_or(0.0).clamp(0.0, 1.0) * 255.0).round() as u8; 3])
                .collect(),
            _ => Vec::new(),
        };
        let positions = fx
            .motion
            .as_ref()
            .map(|m| {
                let n = fbeams.len();
                (0..n).map(|i| m.position(i, n, cycles)).collect()
            })
            .unwrap_or_default();
        Self {
            colors,
            positions,
            kinds,
        }
    }
}

/// Content of a `.dmxfx` file. Older files are a plain list of effects.
#[derive(serde::Serialize, serde::Deserialize)]
pub(crate) struct FxFile {
    pub(crate) effects: Vec<Effect>,
    /// Steps of a STEPS effect.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) steps: Vec<dmxcraft_core::Step>,
}

impl FxFile {
    pub(crate) fn parse(json: &str) -> Result<Self, String> {
        serde_json::from_str::<FxFile>(json)
            .or_else(|_| {
                serde_json::from_str::<Vec<Effect>>(json).map(|effects| FxFile {
                    effects,
                    steps: Vec::new(),
                })
            })
            .map_err(|e| e.to_string())
    }
}

/// Take imported effects into the scene. STEPS only stands alone: in a step
/// scene the steps are appended, a scene without effects becomes a step
/// scene.
pub(crate) fn import_fx(s: &mut dmxcraft_core::Scene, file: FxFile) -> Result<String, String> {
    // exported STATIC scene: take over its values
    if file.effects.is_empty() {
        if !s.effects.is_empty() {
            return Err(t("A static scene can only be imported into a STATIC scene").into());
        }
        if let Some(st) = file.steps.into_iter().next() {
            s.steps = vec![st];
        }
        return Ok(t("Scene imported").into());
    }
    let with_steps = file.effects.iter().any(|e| e.kind == EffectKind::Steps);
    if with_steps {
        if s.has_steps() {
            let n = file.steps.len();
            s.steps.extend(file.steps);
            return Ok(tr!("{n} step(s) imported", n = n));
        }
        if !s.effects.is_empty() {
            return Err(t("STEPS must be the only effect of a scene").into());
        }
        s.add_effect(EffectKind::Steps);
        let n = file.steps.len();
        if !file.steps.is_empty() {
            s.steps = file.steps;
        } else if s.steps.is_empty() {
            s.steps.push(dmxcraft_core::Step::new(Vec::new()));
        }
        return Ok(tr!("{n} step(s) imported", n = n));
    }
    if s.has_steps() {
        return Err(t("A step scene cannot take further effects").into());
    }
    let n = file.effects.len();
    for fx in file.effects {
        // Take over settings and fixtures, only with a new ID
        let id = s.add_effect(fx.kind);
        if let Some(new) = s.effects.iter_mut().find(|e| e.id == id) {
            *new = dmxcraft_core::Effect { id, ..fx };
        }
    }
    Ok(tr!("{n} effect(s) imported", n = n))
}

/// Tiles of the effect kinds in two columns; returns the clicked one.
pub(crate) fn effect_tiles(
    ui: &mut egui::Ui,
    enabled: bool,
    allow_steps: bool,
) -> Option<EffectKind> {
    let gap = theme::SP_S;
    let w = ((ui.available_width() - gap) / 2.0).min(120.0);
    let h = w * 0.72;
    let mut pick = None;
    for row in EffectKind::ALL.chunks(2) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            let total = w * row.len() as f32 + gap * (row.len() as f32 - 1.0);
            ui.add_space(((ui.available_width() - total) / 2.0).max(0.0));
            for kind in row {
                // STEPS and SUPER SCENE only as the only effect of a scene
                let enabled = enabled && (allow_steps || !kind.exclusive());
                let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, h), Sense::click());
                let hot = enabled && resp.hovered();
                let p = ui.painter();
                p.rect_filled(
                    rect,
                    theme::R_S,
                    if hot {
                        theme::raised_hover()
                    } else {
                        theme::bg()
                    },
                );
                p.rect_stroke(
                    rect,
                    theme::R_S,
                    Stroke::new(
                        if hot { 2.0 } else { 1.5 },
                        if hot {
                            theme::accent()
                        } else {
                            theme::border()
                        },
                    ),
                    egui::StrokeKind::Inside,
                );
                let galley = p.layout(
                    kind.label().to_string(),
                    theme::heading_font(),
                    if enabled {
                        theme::text()
                    } else {
                        theme::text_faint()
                    },
                    w - 12.0,
                );
                let pos = rect.center() - galley.size() * 0.5;
                p.galley(pos, galley, theme::text());
                if enabled && resp.clicked() {
                    pick = Some(*kind);
                }
            }
        });
        ui.add_space(gap);
    }
    pick
}

/// Table of steps (compact, like a table): number, hold time, fade time. A
/// click on a row selects the step for editing (the faders then show its
/// values). `true` if a row was clicked.
pub(crate) fn steps_table(
    ui: &mut egui::Ui,
    steps: &mut [dmxcraft_core::Step],
    edit_step: &mut usize,
    playing: Option<usize>,
    color: Color32,
    enabled: bool,
) -> bool {
    use crate::ui::widgets::time_field::{time_field_compact, COMPACT_W};
    const ROW_H: f32 = 24.0;
    const GAP: f32 = 1.0;
    let mut clicked = false;
    let full_w = ui.available_width();
    // Columns: number narrow, the times share the rest
    let num_w = (full_w * 0.26).clamp(44.0, 80.0);
    let field_w = ((full_w - num_w - 2.0 * GAP) / 2.0).max(COMPACT_W + 4.0);
    let cols = [num_w, field_w, field_w];
    let left = ui.cursor().min.x;
    let cells = |top: f32| {
        let mut x = left;
        cols.map(|w| {
            let r = Rect::from_min_size(pos2(x, top), Vec2::new(w, ROW_H));
            x += w + GAP;
            r
        })
    };
    ui.spacing_mut().item_spacing.y = GAP;
    // Header row
    let (head, _) = ui.allocate_exact_size(Vec2::new(full_w, ROW_H), Sense::hover());
    for (r, text) in cells(head.top())
        .into_iter()
        .zip([t("Step"), t("Hold time"), t("Fade time")])
    {
        ui.painter().rect_filled(r, 0.0, theme::raised_hover());
        ui.painter().text(
            r.center(),
            egui::Align2::CENTER_CENTER,
            text,
            theme::caption(),
            theme::text(),
        );
    }
    for (i, step) in steps.iter_mut().enumerate() {
        let (row, resp) = ui.allocate_exact_size(Vec2::new(full_w, ROW_H), Sense::click());
        let sel = *edit_step == i;
        let fill = if sel {
            theme::accent_dim()
        } else if resp.hovered() {
            theme::raised_hover()
        } else {
            theme::bg()
        };
        let [num, hold, fade] = cells(row.top());
        let p = ui.painter().clone();
        for r in [num, hold, fade] {
            p.rect_filled(r, 0.0, fill);
        }
        if playing == Some(i) {
            // running step: bar in the scene color
            p.rect_filled(
                Rect::from_min_size(num.min, Vec2::new(3.0, num.height())),
                0.0,
                color,
            );
        }
        p.text(
            num.center(),
            egui::Align2::CENTER_CENTER,
            (i + 1).to_string(),
            theme::body(),
            if sel {
                theme::text()
            } else {
                theme::text_dim()
            },
        );
        ui.push_id(("step", i), |ui| {
            for (salt, cell, ms) in [
                ("hold", hold, &mut step.hold_ms),
                ("fade", fade, &mut step.fade_ms),
            ] {
                let r = Rect::from_center_size(cell.center(), Vec2::new(COMPACT_W, ROW_H - 2.0));
                let mut cui = ui.new_child(egui::UiBuilder::new().max_rect(r));
                if time_field_compact(&mut cui, salt, ms, enabled, color) {
                    *edit_step = i;
                    clicked = true;
                }
            }
        });
        if resp.clicked() {
            *edit_step = i;
            clicked = true;
        }
    }
    clicked
}

/// Header row of an effect: collapse arrow, name; response for selection
/// and arrow.
/// Remove an effect from the scene. Returns the effect selected afterwards
/// (the last remaining one).
pub(crate) fn remove_effect(
    scene: &mut dmxcraft_core::Scene,
    f: u32,
    state: &mut crate::state::AppState,
) -> Option<u32> {
    let was_steps = scene
        .effects
        .iter()
        .any(|e| e.id == f && e.kind == EffectKind::Steps);
    // no effects left -> STATIC again
    scene.effects.retain(|e| e.id != f);
    if was_steps {
        // with STEPS all steps and their values disappear too: what remains is an
        // empty STATIC scene
        scene.steps = vec![dmxcraft_core::Step::new(Vec::new())];
        state.edit_step = 0;
        state.edit_reload = true;
    }
    if state.fx_target.is_some_and(|(_, e)| e == f) {
        state.fx_target = None;
    }
    scene.effects.last().map(|e| e.id)
}

/// Header row of an effect: arrow to expand/collapse, name and an X on the
/// right to delete (only if `removable`). Returns (header, arrow, X).
pub(crate) fn effect_header(
    ui: &mut egui::Ui,
    fx: &Effect,
    selected: bool,
    removable: bool,
) -> (egui::Response, egui::Response, Option<egui::Response>) {
    let (rect, head) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), 30.0), Sense::click());
    let arrow_rect = Rect::from_min_size(rect.min, Vec2::new(28.0, rect.height()));
    let chevron = ui.interact(
        arrow_rect,
        ui.make_persistent_id(("fx_chevron", fx.id)),
        Sense::click(),
    );
    let close_rect = Rect::from_min_size(
        pos2(rect.right() - 30.0, rect.top()),
        Vec2::new(30.0, rect.height()),
    );
    let close = removable.then(|| {
        ui.interact(
            close_rect,
            ui.make_persistent_id(("fx_close", fx.id)),
            Sense::click(),
        )
        .on_hover_text(t("Delete effect"))
    });
    let p = ui.painter();
    let fill = if selected {
        theme::accent_dim()
    } else if head.hovered() {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S, fill);
    let icon = if fx.collapsed {
        Icon::ChevronRight
    } else {
        Icon::ChevronDown
    };
    icon.paint(
        p,
        Rect::from_center_size(arrow_rect.center(), Vec2::splat(12.0)),
        theme::text_dim(),
    );
    p.text(
        pos2(arrow_rect.right(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        fx.kind.label(),
        theme::heading_font(),
        if selected {
            theme::text()
        } else {
            theme::text_dim()
        },
    );
    if let Some(c) = &close {
        let hot = c.hovered();
        if hot {
            p.rect_filled(close_rect.shrink(3.0), theme::R_S, theme::raised_hover());
        }
        Icon::Close.paint(
            p,
            Rect::from_center_size(close_rect.center(), Vec2::splat(12.0)),
            if hot {
                theme::DANGER
            } else {
                theme::text_dim()
            },
        );
    }
    (head, chevron, close)
}
