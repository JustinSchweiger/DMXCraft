//! Drawing of the timeline: track headers, blocks, bands, triggers, beats.

#[allow(unused_imports)]
use super::*;

/// Header of a track: arrow (sub-tracks), number, eye (mute), lock.
/// Header of a track: color strip, expand, name (double-click: rename),
/// show/hide; right-click: menu with color.
pub(crate) fn track_header(
    ui: &mut egui::Ui,
    p: &egui::Painter,
    row: Rect,
    i: usize,
    tr: &mut dmxcraft_core::timeline::Track,
    locked: bool,
    rename: &mut Option<(u32, String)>,
) -> Option<HeaderAction> {
    let head = Rect::from_min_max(row.min, pos2(row.left() + HEADER_W, row.bottom()));
    let th = track_h(tr);
    let top = Rect::from_min_size(head.min, Vec2::new(HEADER_W, th));
    let role = media_role(tr);
    let color = tr.color.map(theme::rgb).or(role.map(|r| r.1));
    p.rect_filled(head, 0.0, theme::surface());
    if let Some(c) = color {
        p.rect_filled(top, 0.0, c.gamma_multiply(0.14));
    }
    // Color strip on the left
    p.rect_filled(
        Rect::from_min_size(top.min + Vec2::new(0.0, 3.0), Vec2::new(4.0, th - 6.0)),
        2.0,
        color.unwrap_or_else(theme::border),
    );
    p.line_segment(
        [head.right_top(), head.right_bottom()],
        Stroke::new(1.0, theme::border()),
    );
    let mut action = None;
    let resp = ui.interact(
        top,
        ui.scope_id().with(("track_head", tr.id)),
        Sense::click(),
    );
    let arrow = Rect::from_min_size(
        pos2(top.left() + 6.0, top.center().y - 11.0),
        Vec2::splat(22.0),
    );
    if ui
        .put(
            arrow,
            IconButton::new(if tr.expanded {
                Icon::ChevronDown
            } else {
                Icon::ChevronRight
            })
            .size(22.0)
            .flat(),
        )
        .clicked()
    {
        action = Some(HeaderAction::Toggle);
    }
    let eye = Rect::from_min_size(
        pos2(top.right() - 26.0, top.center().y - 11.0),
        Vec2::splat(22.0),
    );
    if ui
        .put(
            eye,
            IconButton::new(if tr.muted { Icon::EyeOff } else { Icon::Eye })
                .size(22.0)
                .flat(),
        )
        .on_hover_text(t("Show/hide track"))
        .clicked()
        && !locked
    {
        tr.muted = !tr.muted;
    }
    // Kind: light (scenes) or media (audio, video, OBS)
    let kind_rect = Rect::from_min_size(
        pos2(arrow.right() + 1.0, top.center().y - 8.0),
        Vec2::splat(16.0),
    );
    let media = tr.kind == dmxcraft_core::timeline::TrackKind::Media;
    if media { Icon::Music } else { Icon::LightOn }.paint(p, kind_rect, theme::text_dim());
    ui.interact(
        kind_rect,
        ui.scope_id().with(("track_kind", tr.id)),
        Sense::hover(),
    )
    .on_hover_text(if media {
        t("Media track: audio, video and OBS")
    } else {
        t("Light track: scenes")
    });
    // Name
    let name_rect = Rect::from_min_max(
        pos2(kind_rect.right() + 4.0, top.center().y - 10.0),
        pos2(eye.left() - 2.0, top.center().y + 10.0),
    );
    let editing = matches!(rename, Some((id, _)) if *id == tr.id);
    if editing {
        if let Some((_, text)) = rename.as_mut() {
            let te = ui.put(
                name_rect,
                egui::TextEdit::singleline(text)
                    .font(theme::body())
                    .margin(Vec2::new(4.0, 1.0)),
            );
            if !te.has_focus() && !te.lost_focus() {
                te.request_focus();
            }
            let enter = ui.input(|i| i.key_pressed(egui::Key::Enter));
            let esc = ui.input(|i| i.key_pressed(egui::Key::Escape));
            if te.lost_focus() || enter || esc {
                if !esc {
                    tr.name = text.trim().to_string();
                }
                *rename = None;
            }
        }
    } else {
        let label = if let Some((name, _)) = role.filter(|_| tr.name.is_empty()) {
            name.to_string()
        } else if tr.name.is_empty() {
            tr!("Track {n}", n = i + 1)
        } else {
            tr.name.clone()
        };
        let clip = p.with_clip_rect(name_rect.intersect(p.clip_rect()));
        clip.text(
            pos2(name_rect.left(), name_rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::body(),
            if tr.muted {
                theme::text_faint()
            } else {
                theme::text()
            },
        );
    }
    if tr.expanded {
        p.text(
            pos2(head.left() + 12.0, head.top() + th + LANE_H / 2.0),
            egui::Align2::LEFT_CENTER,
            if tr.kind == TrackKind::Media {
                t("Volume")
            } else {
                t("Dimmer")
            },
            egui::FontId::proportional(theme::FS_CAPTION - 1.0),
            theme::text_faint(),
        );
    }
    if locked {
        return action;
    }
    if resp.double_clicked() {
        *rename = Some((tr.id, tr.name.clone()));
    }
    menu::context(&resp, |ui| {
        menu::begin(ui);
        if menu::item(ui, MenuItem::new(t("Rename")).icon(Icon::Edit)) {
            *rename = Some((tr.id, tr.name.clone()));
        }
        let label = if tr.expanded {
            t("Collapse")
        } else {
            t("Expand")
        };
        if menu::item(ui, MenuItem::new(label)) {
            action = Some(HeaderAction::Toggle);
        }
        if menu::item(ui, MenuItem::new(t("Expand all"))) {
            action = Some(HeaderAction::ExpandAll);
        }
        menu::separator(ui);
        ui.horizontal(|ui| {
            ui.add_space(theme::SP_S);
            ui.label(
                egui::RichText::new(t("Track colour"))
                    .small()
                    .color(theme::text_dim()),
            );
        });
        if let Some(c) = color_swatches(
            ui,
            ui.make_persistent_id(("track_color", tr.id)),
            tr.color,
            t("No colour"),
        ) {
            tr.color = c;
        }
        menu::separator(ui);
        if menu::item(ui, MenuItem::new(t("Insert new track")).icon(Icon::Plus)) {
            action = Some(HeaderAction::Insert);
        }
        if menu::item(ui, MenuItem::new(t("Duplicate")).icon(Icon::Copy)) {
            action = Some(HeaderAction::Duplicate);
        }
        if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash)) {
            action = Some(HeaderAction::Delete);
        }
    });
    action
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_clip(
    ui: &egui::Ui,
    p: &egui::Painter,
    r: Rect,
    c: &Clip,
    name: &str,
    color: Color32,
    sel: bool,
    scene_ms: u64,
    st: &AppState,
) {
    let body = color.gamma_multiply(0.55);
    p.rect_filled(r, 3.0, theme::raised());
    p.rect_filled(r, 3.0, body);
    let head = Rect::from_min_max(r.min, pos2(r.right(), r.top() + 15.0));
    p.rect_filled(head, 3.0, color);
    // Audio: waveform
    if let ClipContent::Audio {
        path, offset_ms, ..
    } = &c.content
    {
        let state = st.audio.wave(path, ui.ctx());
        if let crate::audio::WaveState::Failed(e) = &state {
            p.text(
                pos2(r.left() + 4.0, r.center().y + 4.0),
                egui::Align2::LEFT_CENTER,
                tr!("File not readable: {e}", e = e),
                egui::FontId::proportional(theme::FS_CAPTION - 1.0),
                theme::DANGER,
            );
        }
        if let crate::audio::WaveState::Ready(a) = state {
            let wave = Rect::from_min_max(pos2(r.left(), head.bottom()), r.max);
            paint_bands(p, wave, c, *offset_ms, &a);
            let segs = match &c.content {
                ClipContent::Audio {
                    grid,
                    segment_grids,
                    ..
                } => a.effective_segments(*grid, segment_grids),
                _ => Vec::new(),
            };
            if let ClipContent::Audio { triggers, .. } = &c.content {
                paint_triggers(ui, p, wave, c, *offset_ms, path, triggers, st);
            }
            paint_segments(p, r, head, wave, c, *offset_ms, &segs);
        }
    }
    // Loop markers of the scene
    if c.looped && scene_ms > 0 && c.scene().is_some() {
        let period = scene_ms as f64 / f64::from(c.speed.max(0.01));
        let per_px = c.duration_ms as f64 / f64::from(r.width().max(1.0));
        if period / per_px > 8.0 {
            let mut k = 1.0;
            while k * period < c.duration_ms as f64 {
                let x = r.left() + (k * period / per_px) as f32;
                let tri = vec![
                    pos2(x - 4.0, head.bottom()),
                    pos2(x + 4.0, head.bottom()),
                    pos2(x, head.bottom() + 5.0),
                ];
                p.add(Shape::convex_polygon(tri, color, Stroke::NONE));
                k += 1.0;
            }
        }
    }
    // Fade in/out as a slope
    let per_ms = r.width() / c.duration_ms.max(1) as f32;
    let shade = Color32::from_black_alpha(110);
    // only the slope as a line (a polygon outline forms spikes beyond the block
    // at acute angles) – and everything clipped to the block
    let clip = p.with_clip_rect(r.intersect(p.clip_rect()));
    let line = Stroke::new(1.0, Color32::WHITE);
    if c.fade_in_ms > 0 {
        let w = (c.fade_in_ms as f32 * per_ms).min(r.width());
        let top = pos2(r.left() + w, r.top());
        clip.add(Shape::convex_polygon(
            vec![r.left_top(), top, r.left_bottom()],
            shade,
            Stroke::NONE,
        ));
        clip.line_segment([r.left_bottom(), top], line);
    }
    if c.fade_out_ms > 0 {
        let w = (c.fade_out_ms as f32 * per_ms).min(r.width());
        let top = pos2(r.right() - w, r.top());
        clip.add(Shape::convex_polygon(
            vec![top, r.right_top(), r.right_bottom()],
            shade,
            Stroke::NONE,
        ));
        clip.line_segment([top, r.right_bottom()], line);
    }
    clip.text(
        pos2(r.left() + 4.0, head.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        egui::FontId::proportional(theme::FS_CAPTION),
        Color32::BLACK,
    );
    clip.text(
        pos2(r.left() + 4.0, r.bottom() - 3.0),
        egui::Align2::LEFT_BOTTOM,
        fmt_ms(c.duration_ms as f64),
        egui::FontId::proportional(theme::FS_CAPTION - 1.0),
        Color32::WHITE,
    );
    if (c.speed - 1.0).abs() > 0.01 {
        clip.text(
            pos2(r.right() - 4.0, r.bottom() - 3.0),
            egui::Align2::RIGHT_BOTTOM,
            format!("×{:.2}", c.speed),
            egui::FontId::proportional(theme::FS_CAPTION - 1.0),
            Color32::WHITE,
        );
    }
    p.rect_stroke(
        r,
        3.0,
        Stroke::new(
            if sel { 2.0 } else { 1.0 },
            if sel {
                Color32::WHITE
            } else {
                color.gamma_multiply(0.8)
            },
        ),
        StrokeKind::Inside,
    );
}

/// Waveform by bands: low (outside), mid, high (inside).
pub(crate) fn paint_bands(
    p: &egui::Painter,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    a: &dmxcraft_core::audio::AudioAnalysis,
) {
    use dmxcraft_core::audio::AudioAnalysis as A;
    p.rect_filled(wave, 0.0, Color32::from_black_alpha(150));
    let visible = wave.intersect(p.clip_rect());
    if visible.width() <= 0.0 {
        return;
    }
    let mid = wave.center().y;
    let half = wave.height() * 0.48;
    let per_px = c.duration_ms as f64 / f64::from(wave.width().max(1.0));
    let x0 = (visible.left() - wave.left()).floor().max(0.0) as usize;
    let x1 = (visible.right() - wave.left()).ceil() as usize;
    for x in x0..x1 {
        let from = offset_ms as f64 + x as f64 * per_px;
        let to = from + per_px.max(crate::audio::FRAME_MS);
        let px = wave.left() + x as f32 + 0.5;
        for (band, scale, color) in [
            (&a.low, 1.0, theme::BAND_LOW),
            (&a.mid, 0.7, theme::BAND_MID),
            (&a.high, 0.45, theme::BAND_HIGH),
        ] {
            let h = A::band_max(band, from, to) * half * scale;
            if h >= 0.5 {
                p.line_segment(
                    [pos2(px, mid - h), pos2(px, mid + h)],
                    Stroke::new(1.0, color),
                );
            }
        }
    }
}

/// Events of the triggers as bars along the bottom edge (one row per
/// trigger).
#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_triggers(
    ui: &egui::Ui,
    p: &egui::Painter,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    path: &str,
    triggers: &[dmxcraft_core::timeline::AudioTrigger],
    st: &AppState,
) {
    const ROW: f32 = 5.0;
    let px_per_ms = wave.width() / c.duration_ms.max(1) as f32;
    let from = offset_ms as f64;
    let to = from + c.duration_ms as f64;
    let clip = p.with_clip_rect(wave.intersect(p.clip_rect()));
    let vis = clip.clip_rect();
    for (row, tr) in triggers.iter().filter(|t| !t.muted).enumerate() {
        let Some(events) = st.triggers.events(path, tr, ui.ctx()) else {
            continue;
        };
        let [r, g, b] = theme::ZONE_PALETTE[row % theme::ZONE_PALETTE.len()];
        let color = Color32::from_rgb(r, g, b);
        let y1 = wave.bottom() - 1.0 - row as f32 * (ROW + 1.0);
        let y0 = y1 - ROW;
        let lead = f64::from(tr.lead_ms);
        let start = events.partition_point(|e| e.1 - lead < from);
        for &(a, b) in &events[start..] {
            let (a, b) = (a - lead, b - lead);
            if a >= to {
                break;
            }
            let x0 = wave.left() + ((a - from) as f32) * px_per_ms;
            let x1 = (wave.left() + ((b - from) as f32) * px_per_ms).max(x0 + 1.5);
            if x1 < vis.left() || x0 > vis.right() {
                continue;
            }
            clip.rect_filled(Rect::from_min_max(pos2(x0, y0), pos2(x1, y1)), 0.0, color);
        }
    }
}

/// Pieces in the block: alternately lightly tinted, boundary as a light line
/// with number and tempo; inside it the beat grid of the piece.
#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_segments(
    p: &egui::Painter,
    r: Rect,
    head: Rect,
    wave: Rect,
    c: &Clip,
    offset_ms: u64,
    segs: &[dmxcraft_core::audio::AudioSegment],
) {
    let px_per_ms = wave.width() / c.duration_ms.max(1) as f32;
    let from = offset_ms as f64;
    let to = from + c.duration_ms as f64;
    let x_of = |ms: f64| wave.left() + ((ms - from) as f32) * px_per_ms;
    let clip = p.with_clip_rect(r.intersect(p.clip_rect()));
    let many = segs.len() > 1;
    for (i, sg) in segs.iter().enumerate() {
        if sg.end_ms <= from || sg.start_ms >= to {
            continue;
        }
        let (a, b) = (sg.start_ms.max(from), sg.end_ms.min(to));
        let (x0, x1) = (x_of(a), x_of(b));
        if many && i % 2 == 1 {
            clip.rect_filled(
                Rect::from_min_max(pos2(x0, wave.top()), pos2(x1, wave.bottom())),
                0.0,
                Color32::from_white_alpha(10),
            );
        }
        if let Some(g) = sg.grid {
            paint_beats(&clip, wave, x_of, a, b, &g);
        }
        // Marker in the header: number and tempo at the start of the piece
        let text = match (many, sg.grid) {
            (true, Some(g)) => format!("{} · {:.1} BPM", i + 1, g.bpm),
            (true, None) => format!("{}", i + 1),
            (false, Some(g)) => format!("{:.1} BPM", g.bpm),
            (false, None) => String::new(),
        };
        if many && sg.start_ms > from {
            clip.line_segment(
                [pos2(x0, r.top()), pos2(x0, r.bottom())],
                Stroke::new(2.0, Color32::WHITE),
            );
        }
        if !text.is_empty() {
            let font = egui::FontId::proportional(theme::FS_CAPTION - 1.0);
            if !many {
                clip.text(
                    pos2(r.right() - 4.0, head.center().y),
                    egui::Align2::RIGHT_CENTER,
                    text,
                    font,
                    Color32::BLACK,
                );
            } else if x1 - x0 > 90.0 {
                // at the start of the piece; if the block name is there, at the end
                let (pos, align) = if x0 < r.left() + 90.0 {
                    (
                        pos2(x1.min(r.right()) - 4.0, head.center().y),
                        egui::Align2::RIGHT_CENTER,
                    )
                } else {
                    (pos2(x0 + 4.0, head.center().y), egui::Align2::LEFT_CENTER)
                };
                clip.text(pos, align, text, font, Color32::BLACK);
            }
        }
    }
}

/// Beat grid in the block: downbeats as lines, beats as short ticks.
pub(crate) fn paint_beats(
    clip: &egui::Painter,
    wave: Rect,
    x_of: impl Fn(f64) -> f32,
    from: f64,
    to: f64,
    g: &dmxcraft_core::audio::AudioGrid,
) {
    let beat_px = x_of(from + g.beat_ms()) - x_of(from);
    if beat_px * 4.0 < 6.0 {
        return;
    }
    let vis = clip.clip_rect();
    for (ms, down) in g.beats(from, to) {
        let x = x_of(ms);
        if x < vis.left() - 1.0 || x > vis.right() + 1.0 {
            continue;
        }
        if down {
            clip.line_segment(
                [pos2(x, wave.top()), pos2(x, wave.bottom())],
                Stroke::new(1.0, Color32::from_white_alpha(130)),
            );
        } else if beat_px >= 6.0 {
            let s = Stroke::new(1.0, Color32::from_white_alpha(90));
            clip.line_segment([pos2(x, wave.top()), pos2(x, wave.top() + 5.0)], s);
            clip.line_segment([pos2(x, wave.bottom() - 5.0), pos2(x, wave.bottom())], s);
        }
    }
}
