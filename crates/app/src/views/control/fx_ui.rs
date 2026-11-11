//! Settings of the effects other than STEPS in the “Effects” sidebar:
//! COLOR/VALUE FX (see `fx_color`), CHASER FX, CURVE FX, MOVE FX, MAPPINGS
//! and COLOR MAPPINGS, plus the list of assigned channels (“features”).

use crate::i18n::t;
use dmxcraft_core::fx::feature::{ChaserPreset, CurveFx, CurveShape, Feature};
use dmxcraft_core::fx::mapping::{MapPreset, MappingFx};
use dmxcraft_core::fx::motion::{EaseScope, Easing, MoveFx, MoveShape};
use dmxcraft_core::fx::Transform;
use dmxcraft_core::{ChannelKind, Effect, EffectKind};
use eframe::egui::{self, pos2, Color32, Pos2, Rect, Sense, Shape, Stroke, Vec2};

use super::fx_color::{
    color_fx_ui, edit_colors, label_row, paint_strip, palette_editor, slider_row, toggle_row,
    PaletteStore,
};
use crate::ui::theme;

/// What the sidebar needs to draw an effect.
pub struct BodyCtx<'a> {
    /// Current color per beam (color effects) or level as a gray value.
    pub preview: &'a [[u8; 3]],
    /// Position 0..1 (pan/tilt) per beam (MOVE FX).
    pub positions: &'a [[f32; 2]],
    /// Effect time in cycles.
    pub cycles: f64,
    /// Channel types of the effect fixtures (for “Add feature”).
    pub kinds: &'a [ChannelKind],
    pub accent: Color32,
    pub enabled: bool,
    pub palettes: &'a mut PaletteStore,
    /// MOVE FX: the selected control point (pan/tilt) – the beams stay there.
    pub hold: &'a mut Option<[f32; 2]>,
}

/// Content of an effect; `true` if something changed.
pub fn effect_body(ui: &mut egui::Ui, fx: &mut Effect, cx: BodyCtx<'_>) -> bool {
    let before = fx.clone();
    let salt = fx.id;
    let BodyCtx {
        preview,
        positions,
        cycles,
        kinds,
        accent,
        enabled,
        palettes,
        hold,
    } = cx;
    match fx.kind {
        EffectKind::ColorFx | EffectKind::ValueFx => {
            if let Some(c) = fx.color.as_mut() {
                let values = fx.kind == EffectKind::ValueFx;
                color_fx_ui(ui, salt, c, preview, accent, enabled, palettes, values);
            }
        }
        EffectKind::ChaserFx => {
            if let Some(c) = fx.chaser.as_mut() {
                let id = |s: &str| egui::Id::unique(("chaser", salt, s.to_owned()));
                combo(ui, id("preset"), c.preset.label(), enabled, |ui| {
                    for p in ChaserPreset::ALL {
                        ui.selectable_value(&mut c.preset, p, p.label());
                    }
                });
                strip(ui, preview);
                if c.preset != ChaserPreset::Random && c.preset != ChaserPreset::Fill {
                    toggle_row(ui, t("One direction only"), &mut c.one_direction, enabled);
                }
                toggle_row(ui, t("Crossfade"), &mut c.crossfade, enabled);
                if c.preset != ChaserPreset::Fill {
                    slider_row(
                        ui,
                        id("px"),
                        t("Pixel count"),
                        &mut c.pixels,
                        (1.0, 32.0),
                        0,
                        enabled,
                        accent,
                    );
                }
                if c.preset == ChaserPreset::Random {
                    slider_row(
                        ui,
                        id("flash"),
                        t("Flash length"),
                        &mut c.flash,
                        (1.0, 100.0),
                        0,
                        enabled,
                        accent,
                    );
                    slider_row(
                        ui,
                        id("seed"),
                        t("Random seed"),
                        &mut c.seed,
                        (0.0, 100.0),
                        0,
                        enabled,
                        accent,
                    );
                    slider_row(
                        ui,
                        id("rep"),
                        t("Repeats"),
                        &mut c.repeats,
                        (1.0, 32.0),
                        0,
                        enabled,
                        accent,
                    );
                }
            }
        }
        EffectKind::CurveFx => {
            if let Some(c) = fx.curve.as_mut() {
                let id = |s: &str| egui::Id::unique(("curve", salt, s.to_owned()));
                combo(ui, id("shape"), t(c.shape.label()), enabled, |ui| {
                    for s in CurveShape::ALL {
                        ui.selectable_value(&mut c.shape, s, t(s.label()));
                    }
                });
                curve_graph(ui, id("graph"), c, cycles, accent, enabled);
                slider_row(
                    ui,
                    id("rep"),
                    t("Repeat rate"),
                    &mut c.repeat,
                    (0.0, 16.0),
                    0,
                    enabled,
                    accent,
                );
                slider_row(
                    ui,
                    id("size"),
                    t("Size"),
                    &mut c.size,
                    (0.0, 2.0),
                    1,
                    enabled,
                    accent,
                );
                slider_row(
                    ui,
                    id("phase"),
                    t("Phase"),
                    &mut c.phase,
                    (0.0, 1.0),
                    2,
                    enabled,
                    accent,
                );
                slider_row(
                    ui,
                    id("off"),
                    t("Offset"),
                    &mut c.offset,
                    (-1.0, 1.0),
                    2,
                    enabled,
                    accent,
                );
                slider_row(
                    ui,
                    id("phasing"),
                    t("Phasing"),
                    &mut c.phasing,
                    (0.0, 4.0),
                    2,
                    enabled,
                    accent,
                );
                mode_row(ui, id("mode"), &mut c.relative, enabled);
            }
        }
        EffectKind::MoveFx => {
            if let Some(m) = fx.motion.as_mut() {
                let id = |s: &str| egui::Id::unique(("move", salt, s.to_owned()));
                let mut shape = m.shape;
                combo(ui, id("shape"), t(m.shape.label()), enabled, |ui| {
                    for s in MoveShape::ALL {
                        ui.selectable_value(&mut shape, s, t(s.label()));
                    }
                });
                if shape != m.shape {
                    m.set_shape(shape);
                }
                move_editor(ui, id("editor"), m, positions, accent, enabled);
                point_row(ui, id("editor"), m, accent, enabled);
                if enabled {
                    *hold = selected_point(ui, id("editor"))
                        .and_then(|i| m.points.get(i).copied())
                        .map(|q| m.to_field(q));
                }
                if m.shape == MoveShape::Points {
                    toggle_row(ui, t("Jump"), &mut m.jump, enabled);
                }
                if m.shape == MoveShape::Line {
                    toggle_row(ui, t("Back and forth"), &mut m.line_return, enabled);
                }
                speed_ui(ui, id("speed"), m, cycles, accent, enabled);
                slider_row(
                    ui,
                    id("phasing"),
                    t("Phasing"),
                    &mut m.phasing,
                    (0.0, 4.0),
                    2,
                    enabled,
                    accent,
                );
                toggle_row(ui, t("Symmetry"), &mut m.symmetry, enabled);
                mode_row(ui, id("mode"), &mut m.relative, enabled);
            }
        }
        EffectKind::Mappings | EffectKind::ColorMappings => {
            let color = fx.kind == EffectKind::ColorMappings;
            if let Some(m) = fx.mapping.as_mut() {
                mapping_ui(ui, salt, m, color, accent, enabled, palettes);
            }
        }
        EffectKind::Steps | EffectKind::SuperScene => {}
    }
    // Beam count bottom right
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 20.0), Sense::hover());
    ui.painter().text(
        r.right_center(),
        egui::Align2::RIGHT_CENTER,
        tr!("{n} beam(s)", n = fx.fixtures.len().max(preview.len())),
        egui::FontId::proportional(theme::FS_CAPTION),
        theme::text_dim(),
    );
    if fx.kind.uses_features() {
        features_ui(ui, salt, &mut fx.features, kinds, enabled, accent);
    }
    *fx != before
}

fn combo(
    ui: &mut egui::Ui,
    id: egui::Id,
    text: &str,
    enabled: bool,
    add: impl FnOnce(&mut egui::Ui),
) {
    ui.add_enabled_ui(enabled, |ui| {
        egui::ComboBox::from_id_salt(id)
            .selected_text(text)
            .width(ui.available_width() - 4.0)
            .height(400.0)
            .show_ui(ui, add);
    });
    ui.add_space(theme::SP_XS);
}

/// Preview strip as for COLOR FX.
fn strip(ui: &mut egui::Ui, colors: &[[u8; 3]]) {
    let (r, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 22.0), Sense::hover());
    paint_strip(ui.painter(), r, colors);
    ui.add_space(theme::SP_XS);
}

/// “Mode of operation”: absolute or relative to the value from the steps.
fn mode_row(ui: &mut egui::Ui, id: egui::Id, relative: &mut bool, enabled: bool) {
    label_row(ui, t("Mode"), |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            egui::ComboBox::from_id_salt(id)
                .selected_text(if *relative {
                    t("Relative")
                } else {
                    t("Absolute")
                })
                .width(ui.available_width() - 4.0)
                .show_ui(ui, |ui| {
                    ui.selectable_value(relative, false, t("Absolute"));
                    ui.selectable_value(relative, true, t("Relative"));
                });
        });
    });
}

fn transform_row(ui: &mut egui::Ui, id: egui::Id, tr: &mut Transform, enabled: bool) {
    label_row(ui, t("Transform"), |ui| {
        ui.add_enabled_ui(enabled, |ui| {
            egui::ComboBox::from_id_salt(id)
                .selected_text(t(tr.label()))
                .width(ui.available_width() - 4.0)
                .show_ui(ui, |ui| {
                    for x in Transform::ALL {
                        ui.selectable_value(tr, x, t(x.label()));
                    }
                });
        });
    });
}

fn mapping_ui(
    ui: &mut egui::Ui,
    salt: u32,
    m: &mut MappingFx,
    color: bool,
    accent: Color32,
    enabled: bool,
    store: &mut PaletteStore,
) {
    let id = |s: &str| egui::Id::unique(("mapping", salt, s.to_owned()));
    let mut preset = m.preset;
    combo(ui, id("preset"), m.preset.label(), enabled, |ui| {
        for p in MapPreset::ALL {
            ui.selectable_value(&mut preset, p, p.label());
        }
    });
    if preset != m.preset {
        m.set_preset(preset, color);
    }
    label_row(ui, if color { t("Palette") } else { t("Values") }, |ui| {
        palette_editor(ui, &mut m.palette, enabled, store);
    });
    ui.horizontal_wrapped(|ui| {
        edit_colors(ui, ("map", salt), &mut m.palette, enabled, 18.0);
    });
    ui.add_space(theme::SP_XS);
    if color {
        toggle_row(ui, t("Grayscale"), &mut m.grayscale, enabled);
    }
    transform_row(ui, id("tr"), &mut m.transform, enabled);
    let s = |ui: &mut egui::Ui, k: &str, l: &str, v: &mut f32, r: (f32, f32), d: usize| {
        slider_row(ui, id(k), l, v, r, d, enabled, accent);
    };
    s(ui, "spin", "Rotation", &mut m.spin, (-4.0, 4.0), 1);
    match m.preset {
        MapPreset::Rainbow => {
            s(ui, "w", t("Color width"), &mut m.width, (0.0, 100.0), 1);
            s(ui, "a", t("Angle"), &mut m.angle, (0.0, 360.0), 0);
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::Spiral => {
            s(ui, "r", t("Radius"), &mut m.radius, (0.0, 100.0), 0);
            s(
                ui,
                "sw",
                t("Number of swirls"),
                &mut m.swirls,
                (1.0, 12.0),
                0,
            );
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::Burst => {
            s(ui, "w", t("Color width"), &mut m.width, (0.0, 100.0), 0);
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::Butterfly => {
            s(ui, "sw", t("Wings"), &mut m.swirls, (1.0, 8.0), 0);
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::Plasma | MapPreset::Tube => {
            if m.preset == MapPreset::Tube {
                s(ui, "c", t("Count"), &mut m.count, (1.0, 20.0), 0);
            } else {
                s(ui, "z", t("Zoom"), &mut m.zoom, (0.0, 100.0), 0);
            }
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::Bounce => {
            s(ui, "o", t("Size"), &mut m.object, (2.0, 100.0), 0);
            s(ui, "c", t("Count"), &mut m.count, (1.0, 10.0), 0);
        }
        MapPreset::Fire => {
            s(ui, "z", t("Zoom"), &mut m.zoom, (0.0, 100.0), 0);
            s(ui, "amp", t("Amplitude"), &mut m.amplitude, (0.0, 100.0), 0);
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
        MapPreset::KnightRider => {
            s(ui, "o", t("Size"), &mut m.object, (2.0, 100.0), 0);
            s(ui, "a", t("Angle"), &mut m.angle, (0.0, 360.0), 0);
        }
        MapPreset::Text => {
            label_row(ui, t("Text"), |ui| {
                ui.add_enabled(
                    enabled,
                    egui::TextEdit::singleline(&mut m.text)
                        .desired_width(ui.available_width() - 6.0),
                );
            });
            s(ui, "o", t("Size"), &mut m.object, (30.0, 100.0), 0);
        }
        MapPreset::Sweep => {
            s(ui, "a", t("Angle"), &mut m.angle, (0.0, 360.0), 0);
        }
        MapPreset::Sparkle => {
            s(ui, "c", t("Count"), &mut m.count, (1.0, 20.0), 0);
            s(ui, "o", t("Lifetime"), &mut m.object, (2.0, 100.0), 0);
        }
        MapPreset::RandomFill => {
            s(ui, "c", t("Cells"), &mut m.count, (1.0, 20.0), 0);
        }
        MapPreset::Rain => {
            s(ui, "c", t("Count"), &mut m.count, (1.0, 20.0), 0);
            s(ui, "o", t("Length"), &mut m.object, (2.0, 100.0), 0);
        }
        MapPreset::Perlin => {
            s(ui, "oct", t("Octaves"), &mut m.octaves, (1.0, 8.0), 0);
            s(ui, "z", t("Zoom"), &mut m.zoom, (0.0, 100.0), 0);
            s(ui, "amp", t("Amplitude"), &mut m.amplitude, (0.0, 100.0), 0);
        }
        MapPreset::Explosion => {
            s(ui, "c", t("Rings"), &mut m.count, (1.0, 8.0), 0);
            s(ui, "o", t("Size"), &mut m.object, (2.0, 100.0), 0);
        }
        MapPreset::Starfield => {
            s(ui, "c", t("Count"), &mut m.count, (1.0, 20.0), 0);
            s(ui, "o", t("Size"), &mut m.object, (2.0, 100.0), 0);
        }
        MapPreset::Graph => {
            s(ui, "c", t("Bars"), &mut m.count, (1.0, 32.0), 0);
        }
        MapPreset::Lines => {
            s(ui, "c", t("Count"), &mut m.count, (1.0, 20.0), 0);
            s(ui, "a", t("Angle"), &mut m.angle, (0.0, 360.0), 0);
            s(ui, "b", t("Blend"), &mut m.blend, (0.0, 100.0), 1);
        }
    }
    ui.label(
        egui::RichText::new(t("Move, scale and rotate the rectangle on the stage."))
            .small()
            .color(theme::text_faint()),
    );
}

/// Curve image: one period, the dot shows the first beam. With “Custom”
/// control points can be dragged, added by double-click and removed by
/// right-click.
fn curve_graph(
    ui: &mut egui::Ui,
    id: egui::Id,
    c: &mut CurveFx,
    cycles: f64,
    accent: Color32,
    enabled: bool,
) {
    let w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 110.0), Sense::click());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, theme::R_S, theme::canvas());
    grid(&p, rect, 4, 2);
    let inner = rect.shrink(6.0);
    let to = |x: f32, y: f32| {
        pos2(
            inner.left() + x * inner.width(),
            inner.bottom() - y * inner.height(),
        )
    };
    let pts: Vec<Pos2> = (0..=120)
        .map(|i| {
            let x = i as f32 / 120.0;
            to(x, c.level(x.min(0.9999), 0))
        })
        .collect();
    p.add(Shape::line(pts, Stroke::new(1.5, theme::canvas_dim())));
    // current value of the first beam
    let x = (cycles * f64::from(c.repeat) + f64::from(c.phase)).rem_euclid(1.0) as f32;
    p.circle_filled(to(x, c.level(x, 0)), 4.0, theme::handle());
    if c.shape != CurveShape::Custom || !enabled {
        return;
    }
    let from = |q: Pos2| {
        [
            ((q.x - inner.left()) / inner.width()).clamp(0.0, 1.0),
            ((inner.bottom() - q.y) / inner.height()).clamp(0.0, 1.0),
        ]
    };
    let mut remove = None;
    for (i, pt) in c.custom.iter_mut().enumerate() {
        let at = to(pt[0], pt[1]);
        let r = ui.interact(
            Rect::from_center_size(at, Vec2::splat(12.0)),
            id.with(i),
            Sense::click_and_drag(),
        );
        if r.dragged() {
            if let Some(q) = r.interact_pointer_pos() {
                *pt = from(q);
            }
        }
        if r.secondary_clicked() {
            remove = Some(i);
        }
        let hot = r.hovered() || r.dragged();
        p.circle_stroke(
            at,
            if hot { 6.0 } else { 4.5 },
            Stroke::new(1.5, if hot { accent } else { theme::handle() }),
        );
    }
    if let Some(i) = remove.filter(|_| c.custom.len() > 2) {
        c.custom.remove(i);
    }
    if resp.double_clicked() {
        if let Some(q) = resp.interact_pointer_pos() {
            c.custom.push(from(q));
        }
    }
}

fn grid(p: &egui::Painter, r: Rect, cols: usize, rows: usize) {
    let s = Stroke::new(1.0, theme::canvas_line().gamma_multiply(0.6));
    for i in 1..cols {
        let x = r.left() + r.width() * i as f32 / cols as f32;
        p.line_segment([pos2(x, r.top()), pos2(x, r.bottom())], s);
    }
    for i in 1..rows {
        let y = r.top() + r.height() * i as f32 / rows as f32;
        p.line_segment([pos2(r.left(), y), pos2(r.right(), y)], s);
    }
}

/// Selected control point of the shape editor (remembered in egui).
fn selected_point(ui: &egui::Ui, id: egui::Id) -> Option<usize> {
    ui.data(|d| d.get_temp::<Option<usize>>(id.with("sel")))
        .flatten()
}

fn select_point(ui: &egui::Ui, id: egui::Id, sel: Option<usize>) {
    ui.data_mut(|d| d.insert_temp(id.with("sel"), sel));
}

/// Below the shape editor: selected point with rounding and delete.
fn point_row(ui: &mut egui::Ui, id: egui::Id, m: &mut MoveFx, accent: Color32, enabled: bool) {
    if !m.editable() {
        return;
    }
    let Some(i) = selected_point(ui, id).filter(|i| *i < m.points.len()) else {
        return;
    };
    let mut remove = false;
    ui.horizontal(|ui| {
        ui.set_min_height(28.0);
        ui.label(egui::RichText::new(tr!("Point {n}", n = i + 1)).color(theme::text()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            remove = ui
                .add(
                    crate::ui::widgets::IconButton::new(crate::ui::widgets::Icon::Trash)
                        .flat()
                        .size(24.0)
                        .enabled(enabled && m.points.len() > m.min_points()),
                )
                .on_hover_text(t("Delete point (Del)"))
                .clicked();
        });
    });
    // The curve is round by itself
    if m.shape != MoveShape::Curve {
        let mut pct = m.smooth_at(i) * 100.0;
        slider_row(
            ui,
            id.with(("smooth", i)),
            t("Rounding"),
            &mut pct,
            (0.0, 100.0),
            0,
            enabled,
            accent,
        );
        if (pct / 100.0 - m.smooth_at(i)).abs() > 1e-4 {
            m.set_smooth(i, pct / 100.0);
        }
    }
    if remove && m.remove_point(i) {
        select_point(ui, id, None);
    }
}

/// Shape editor of the MOVE FX: field = pan (x) × tilt (y). Dragging the
/// frame moves it, corners scale, the handle at the top rotates, control
/// points can be dragged individually. Double-click adds a control point,
/// clicking a point selects it (rounding and delete below, Delete removes).
fn move_editor(
    ui: &mut egui::Ui,
    id: egui::Id,
    m: &mut MoveFx,
    positions: &[[f32; 2]],
    accent: Color32,
    enabled: bool,
) {
    let w = ui.available_width();
    let (rect, bg) = ui.allocate_exact_size(
        Vec2::new(w, w.min(200.0)),
        if enabled {
            Sense::click()
        } else {
            Sense::hover()
        },
    );
    let mut sel = selected_point(ui, id).filter(|i| *i < m.points.len());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, theme::R_S, theme::canvas());
    grid(&p, rect, 4, 4);
    let to = |q: [f32; 2]| {
        pos2(
            rect.left() + q[0] * rect.width(),
            rect.top() + q[1] * rect.height(),
        )
    };
    let from = |s: Pos2| {
        [
            (s.x - rect.left()) / rect.width(),
            (s.y - rect.top()) / rect.height(),
        ]
    };
    // Path
    let path: Vec<Pos2> = (0..=160)
        .map(|i| to(m.to_field(m.local_at(i as f32 / 160.0))))
        .collect();
    p.add(Shape::line(path, Stroke::new(1.5, theme::canvas_dim())));
    // Equal time steps along the way: close together = slow
    if m.speed.easing != Easing::Linear {
        for k in 0..32 {
            let q = to(m.to_field(m.local_at(k as f32 / 32.0)));
            p.circle_filled(q, 1.6, accent.gamma_multiply(0.8));
        }
    }
    // Frame
    let corners = [[-1.0, -1.0], [1.0, -1.0], [1.0, 1.0], [-1.0, 1.0]].map(|c| to(m.to_field(c)));
    for i in 0..4 {
        p.add(Shape::dashed_line(
            &[corners[i], corners[(i + 1) % 4]],
            Stroke::new(1.0, theme::canvas_faint()),
            4.0,
            3.0,
        ));
    }
    let rot_handle = to(m.to_field([0.0, -1.0 - 0.25 / m.size[1].max(0.05) * 0.1]));
    let top_mid = to(m.to_field([0.0, -1.0]));
    p.line_segment(
        [top_mid, rot_handle],
        Stroke::new(1.0, theme::canvas_faint()),
    );

    let mut ids = 0usize;
    let mut handle = |ui: &mut egui::Ui, at: Pos2, size: f32| {
        ids += 1;
        ui.interact(
            Rect::from_center_size(at, Vec2::splat(size)),
            id.with(ids),
            if enabled {
                Sense::drag()
            } else {
                Sense::hover()
            },
        )
    };
    // Move center/frame: area of the frame
    let body = Rect::from_points(&corners);
    let r = ui.interact(
        body,
        id.with("body"),
        if enabled {
            Sense::click_and_drag()
        } else {
            Sense::hover()
        },
    );
    // Double-click (inside the frame or next to it): add a control point,
    // a single click into empty space clears the selection
    for resp in [&bg, &r] {
        if m.editable() && resp.double_clicked() {
            if let Some(q) = resp.interact_pointer_pos() {
                sel = Some(m.insert_point(m.to_local(from(q))));
            }
        } else if resp.clicked() {
            sel = None;
        }
    }
    if m.editable() {
        bg.clone()
            .union(r.clone())
            .on_hover_text(t(
                "Double-click: add point\nPoints snap into line with each other and to the center – hold Alt to move freely",
            ));
    }
    // Snapping (Alt held: free). Guide lines are drawn at the end.
    let snapping = !ui.input(|i| i.modifiers.alt);
    let mut guides: Vec<(bool, f32)> = Vec::new();
    if r.drag_started() {
        ui.data_mut(|d| d.insert_temp(id.with("raw_center"), m.center));
    }
    if r.dragged() {
        // unsnapped position follows the mouse, the frame center snaps to
        // the middle of the field
        let d = r.drag_delta();
        let mut raw: [f32; 2] = ui
            .data(|dd| dd.get_temp(id.with("raw_center")))
            .unwrap_or(m.center);
        raw[0] = (raw[0] + d.x / rect.width()).clamp(0.0, 1.0);
        raw[1] = (raw[1] + d.y / rect.height()).clamp(0.0, 1.0);
        ui.data_mut(|dd| dd.insert_temp(id.with("raw_center"), raw));
        let at = to(raw);
        let (sx, sy) = if snapping {
            (
                snap_to(at.x, &[rect.center().x]),
                snap_to(at.y, &[rect.center().y]),
            )
        } else {
            (None, None)
        };
        if let Some(x) = sx {
            guides.push((true, x));
        }
        if let Some(y) = sy {
            guides.push((false, y));
        }
        m.center = from(pos2(sx.unwrap_or(at.x), sy.unwrap_or(at.y)));
    }
    // Corners scale
    for c in corners {
        let r = handle(ui, c, 12.0);
        if r.dragged() {
            if let Some(q) = r.interact_pointer_pos() {
                let f = from(q);
                let (s, co) = m.rotation.to_radians().sin_cos();
                let (dx, dy) = (f[0] - m.center[0], f[1] - m.center[1]);
                m.size = [
                    (dx * co + dy * s).abs().max(0.01),
                    (-dx * s + dy * co).abs().max(0.01),
                ];
            }
        }
        let hot = r.hovered() || r.dragged();
        p.rect_filled(
            Rect::from_center_size(c, Vec2::splat(7.0)),
            1.0,
            if hot { accent } else { theme::canvas_dim() },
        );
    }
    // Rotate
    let r = handle(ui, rot_handle, 14.0);
    if r.dragged() {
        if let Some(q) = r.interact_pointer_pos() {
            let f = from(q);
            let a = (f[1] - m.center[1]).atan2(f[0] - m.center[0]).to_degrees() + 90.0;
            m.rotation = (a / 5.0).round() * 5.0;
        }
    }
    let hot = r.hovered() || r.dragged();
    p.circle_filled(
        rot_handle,
        4.0,
        if hot { accent } else { theme::canvas_dim() },
    );
    // Control points
    if m.shape != MoveShape::Circle {
        let pts = m.points.clone();
        // rounded corners: show the original corners as a fine guide line
        if m.shape != MoveShape::Curve && (0..pts.len()).any(|i| m.smooth_at(i) > 0.0) {
            let mut guide: Vec<Pos2> = pts.iter().map(|q| to(m.to_field(*q))).collect();
            if m.closed() && guide.len() > 2 {
                guide.push(guide[0]);
            }
            p.add(Shape::dashed_line(
                &guide,
                Stroke::new(1.0, theme::canvas_faint()),
                3.0,
                3.0,
            ));
        }
        for (i, pt) in pts.iter().enumerate() {
            let at = to(m.to_field(*pt));
            ids += 1;
            let r = ui.interact(
                Rect::from_center_size(at, Vec2::splat(14.0)),
                id.with(("pt", ids)),
                if enabled {
                    Sense::click_and_drag()
                } else {
                    Sense::hover()
                },
            );
            if r.clicked() || r.drag_started() {
                sel = Some(i);
            }
            if r.dragged() {
                if let Some(q) = r.interact_pointer_pos() {
                    let mut q = q;
                    if snapping {
                        // align with the other points (same line horizontally
                        // or vertically), the middle of the frame and of the field
                        let others: Vec<Pos2> = pts
                            .iter()
                            .enumerate()
                            .filter(|(j, _)| *j != i)
                            .map(|(_, o)| to(m.to_field(*o)))
                            .collect();
                        let mid = to(m.center);
                        let xs: Vec<f32> = others
                            .iter()
                            .map(|o| o.x)
                            .chain([mid.x, rect.center().x])
                            .collect();
                        let ys: Vec<f32> = others
                            .iter()
                            .map(|o| o.y)
                            .chain([mid.y, rect.center().y])
                            .collect();
                        if let Some(x) = snap_to(q.x, &xs) {
                            q.x = x;
                            guides.push((true, x));
                        }
                        if let Some(y) = snap_to(q.y, &ys) {
                            q.y = y;
                            guides.push((false, y));
                        }
                    }
                    let l = m.to_local(from(q));
                    m.points[i] = [l[0].clamp(-1.0, 1.0), l[1].clamp(-1.0, 1.0)];
                }
            }
            let hot = r.hovered() || r.dragged();
            if sel == Some(i) {
                p.circle_filled(at, 5.5, accent);
                p.circle_stroke(at, 5.5, Stroke::new(1.5, theme::handle()));
            } else {
                p.circle_stroke(
                    at,
                    4.5,
                    Stroke::new(1.5, if hot { accent } else { theme::handle() }),
                );
            }
            // Number next to the point
            p.text(
                at + Vec2::new(7.0, -7.0),
                egui::Align2::LEFT_BOTTOM,
                (i + 1).to_string(),
                egui::FontId::proportional(10.0),
                theme::canvas_faint(),
            );
        }
        // Delete removes the selected point while the mouse is over the field
        if let Some(i) = sel {
            let hovered = ui.rect_contains_pointer(rect);
            if enabled
                && hovered
                && ui.input_mut(|inp| inp.consume_key(egui::Modifiers::NONE, egui::Key::Delete))
                && m.remove_point(i)
            {
                sel = None;
            }
        }
    }
    select_point(ui, id, sel);
    // Guide lines of the active snap
    for (vertical, v) in guides {
        let (a, b) = if vertical {
            (pos2(v, rect.top()), pos2(v, rect.bottom()))
        } else {
            (pos2(rect.left(), v), pos2(rect.right(), v))
        };
        p.add(Shape::dashed_line(
            &[a, b],
            Stroke::new(1.0, accent.gamma_multiply(0.8)),
            4.0,
            3.0,
        ));
    }
    // current position of the beams
    for (i, q) in positions.iter().enumerate() {
        p.circle_filled(to(*q), if i == 0 { 4.5 } else { 3.0 }, theme::handle());
    }
}

/// Nearest candidate within the snap distance (screen pixels).
fn snap_to(v: f32, candidates: &[f32]) -> Option<f32> {
    const SNAP_PX: f32 = 6.0;
    candidates
        .iter()
        .copied()
        .filter(|c| (c - v).abs() <= SNAP_PX)
        .min_by(|a, b| (a - v).abs().total_cmp(&(b - v).abs()))
}

/// List of the assigned channels with value range; “+” adds one.
fn features_ui(
    ui: &mut egui::Ui,
    salt: u32,
    features: &mut Vec<Feature>,
    kinds: &[ChannelKind],
    enabled: bool,
    accent: Color32,
) {
    ui.add_space(theme::SP_S);
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new(t("Assigned features")).color(theme::text()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let add = ui
                .add_enabled(enabled, egui::Button::new("+").frame(false))
                .on_hover_text(t("Add channel (or FX icon on the fader)"));
            crate::ui::widgets::menu::dropdown(&add, |ui| {
                use crate::ui::widgets::menu::{self, MenuItem};
                menu::begin(ui);
                let free: Vec<ChannelKind> = kinds
                    .iter()
                    .copied()
                    .filter(|k| !features.iter().any(|f| f.kind == *k))
                    .collect();
                if free.is_empty() {
                    ui.label(
                        egui::RichText::new(t("No further channels")).color(theme::text_faint()),
                    );
                }
                for k in free {
                    if menu::item(ui, MenuItem::new(t(k.label()))) {
                        features.push(Feature::new(k));
                    }
                }
            });
        });
    });
    let frame = egui::Frame::new()
        .fill(theme::bg())
        .corner_radius(theme::R_S)
        .inner_margin(egui::Margin::same(4));
    frame.show(ui, |ui| {
        ui.set_min_width(ui.available_width());
        ui.set_min_height(60.0);
        if features.is_empty() {
            ui.label(
                egui::RichText::new(t("No channels assigned yet."))
                    .small()
                    .color(theme::text_faint()),
            );
        }
        let mut remove = None;
        for (i, f) in features.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.set_min_height(24.0);
                let (lr, _) = ui.allocate_exact_size(Vec2::new(90.0, 22.0), Sense::hover());
                ui.painter().text(
                    pos2(lr.left() + 4.0, lr.center().y),
                    egui::Align2::LEFT_CENTER,
                    t(f.kind.label()),
                    theme::caption(),
                    theme::text_dim(),
                );
                let w = (ui.available_width() - 28.0).max(60.0);
                range_bar(
                    ui,
                    egui::Id::unique(("feat", salt, i)),
                    f,
                    w,
                    accent,
                    enabled,
                );
                if ui
                    .add_enabled(enabled, egui::Button::new("×").frame(false))
                    .on_hover_text(t("Remove channel"))
                    .clicked()
                {
                    remove = Some(i);
                }
            });
        }
        if let Some(i) = remove {
            features.remove(i);
        }
    });
}

/// Horizontal range with two handles (lower/upper bound, 0..255).
fn range_bar(
    ui: &mut egui::Ui,
    id: egui::Id,
    f: &mut Feature,
    w: f32,
    accent: Color32,
    enabled: bool,
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 22.0), Sense::hover());
    let track = Rect::from_center_size(rect.center(), Vec2::new(rect.width() - 12.0, 3.0));
    let x_of = |v: u8| track.left() + f32::from(v) / 255.0 * track.width();
    let v_of =
        |x: f32| (((x - track.left()) / track.width()).clamp(0.0, 1.0) * 255.0).round() as u8;
    let sense = if enabled {
        Sense::drag()
    } else {
        Sense::hover()
    };
    let hmin = Rect::from_center_size(pos2(x_of(f.min), rect.center().y), Vec2::new(10.0, 16.0));
    let hmax = Rect::from_center_size(pos2(x_of(f.max), rect.center().y), Vec2::new(10.0, 16.0));
    let rmin = ui.interact(hmin, id.with("min"), sense);
    let rmax = ui.interact(hmax, id.with("max"), sense);
    if let Some(q) = rmin.interact_pointer_pos().filter(|_| rmin.dragged()) {
        f.min = v_of(q.x).min(f.max);
    }
    if let Some(q) = rmax.interact_pointer_pos().filter(|_| rmax.dragged()) {
        f.max = v_of(q.x).max(f.min);
    }
    let p = ui.painter();
    p.rect_filled(track, 1.5, theme::raised());
    p.rect_filled(
        Rect::from_min_max(
            pos2(x_of(f.min), track.top()),
            pos2(x_of(f.max), track.bottom()),
        ),
        1.5,
        accent.gamma_multiply(0.7),
    );
    for (h, r) in [(hmin, &rmin), (hmax, &rmax)] {
        let hot = r.hovered() || r.dragged();
        p.circle_filled(
            h.center(),
            5.0,
            if hot {
                theme::text()
            } else {
                theme::text_dim()
            },
        );
    }
    let label = |v: u8| {
        if v == 0 {
            t("OFF").to_string()
        } else {
            v.to_string()
        }
    };
    let r = ui.interact(rect, id.with("hover"), Sense::hover());
    r.on_hover_text(format!("{} – {}", label(f.min), label(f.max)));
}

/// Speed curve of the MOVE FX: type, strength, restart per leg and the curve
/// (time → way; with “Custom” the points can be dragged, added by
/// double-click and removed by right-click).
fn speed_ui(
    ui: &mut egui::Ui,
    id: egui::Id,
    m: &mut MoveFx,
    cycles: f64,
    accent: Color32,
    enabled: bool,
) {
    let sp = &mut m.speed;
    label_row(ui, t("Speed"), |ui| {
        combo(ui, id.with("easing"), t(sp.easing.label()), enabled, |ui| {
            for e in Easing::ALL {
                ui.selectable_value(&mut sp.easing, e, t(e.label()));
            }
        });
    });
    if sp.easing == Easing::Linear {
        return;
    }
    if sp.easing != Easing::Custom {
        let mut pct = sp.strength * 100.0;
        slider_row(
            ui,
            id.with("strength"),
            t("Strength"),
            &mut pct,
            (0.0, 100.0),
            0,
            enabled,
            accent,
        );
        sp.strength = (pct / 100.0).clamp(0.0, 1.0);
    }
    let mut per_leg = sp.scope == EaseScope::Leg;
    toggle_row(ui, t("Restart at every point"), &mut per_leg, enabled);
    sp.scope = if per_leg {
        EaseScope::Leg
    } else {
        EaseScope::Cycle
    };
    // Graph: x = time, y = way
    let w = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, 110.0), Sense::click());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, theme::R_S, theme::canvas());
    grid(&p, rect, 4, 4);
    let inner = rect.shrink(6.0);
    let to = |x: f32, y: f32| {
        pos2(
            inner.left() + x * inner.width(),
            inner.bottom() - y * inner.height(),
        )
    };
    let pts: Vec<Pos2> = (0..=100)
        .map(|i| {
            let x = i as f32 / 100.0;
            to(x, sp.ease(x))
        })
        .collect();
    p.add(Shape::line(pts, Stroke::new(1.5, theme::canvas_dim())));
    let x = cycles.rem_euclid(1.0) as f32;
    p.circle_filled(to(x, sp.ease(x)), 4.0, theme::handle());
    if sp.easing != Easing::Custom || !enabled {
        return;
    }
    let from = |q: Pos2| {
        [
            ((q.x - inner.left()) / inner.width()).clamp(0.0, 1.0),
            ((inner.bottom() - q.y) / inner.height()).clamp(0.0, 1.0),
        ]
    };
    let mut remove = None;
    for (i, pt) in sp.custom.iter_mut().enumerate() {
        let at = to(pt[0], pt[1]);
        let r = ui.interact(
            Rect::from_center_size(at, Vec2::splat(12.0)),
            id.with(("pt", i)),
            Sense::click_and_drag(),
        );
        if r.dragged() {
            if let Some(q) = r.interact_pointer_pos() {
                *pt = from(q);
            }
        }
        if r.secondary_clicked() {
            remove = Some(i);
        }
        let hot = r.hovered() || r.dragged();
        p.circle_stroke(
            at,
            if hot { 6.0 } else { 4.5 },
            Stroke::new(1.5, if hot { accent } else { theme::handle() }),
        );
    }
    if let Some(i) = remove.filter(|_| sp.custom.len() > 1) {
        sp.custom.remove(i);
    }
    if resp.double_clicked() {
        if let Some(q) = resp.interact_pointer_pos() {
            sp.custom.push(from(q));
        }
    }
    resp.on_hover_text(t("Double-click: add point · right-click a point: remove"));
}
