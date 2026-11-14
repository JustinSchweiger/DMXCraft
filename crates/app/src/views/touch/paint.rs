//! Drawing of the touch elements (shared by play and edit mode).

#[allow(unused_imports)]
use super::*;

/// Label with the icon tokens replaced by their glyphs (web remote: the
/// icon font is a fallback of the text font there).
pub(crate) fn label_with_glyphs(label: &str) -> String {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    label_tokens(label)
        .into_iter()
        .map(|part| match part {
            LabelPart::Text(t) => t.to_string(),
            LabelPart::Icon(name) => {
                icon_library::glyph(name).map_or_else(|| format!(":{name}:"), |c| c.to_string())
            }
        })
        .collect()
}

/// Does the label consist of icons only (then they are drawn large)?
pub(crate) fn icons_only(label: &str) -> bool {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    let parts = label_tokens(label);
    parts.iter().any(|p| matches!(p, LabelPart::Icon(_)))
        && parts.iter().all(|p| {
            matches!(p, LabelPart::Icon(_))
                || matches!(p, LabelPart::Text(t) if t.trim().is_empty())
        })
}

/// Text with icons (`:name:`), centered and wrapped.
pub(crate) fn label_galley(
    p: &egui::Painter,
    label: &str,
    size: f32,
    color: Color32,
    width: f32,
) -> std::sync::Arc<egui::Galley> {
    use dmxcraft_core::touch::{label_tokens, LabelPart};
    use egui::text::{LayoutJob, TextFormat};
    let text_fmt = TextFormat::simple(egui::FontId::proportional(size), color);
    let icon_fmt = TextFormat::simple(
        egui::FontId::new(
            size * 1.15,
            egui::FontFamily::Name(crate::ui::widgets::icons::ICON_FAMILY.into()),
        ),
        color,
    );
    let mut job = LayoutJob::default();
    for part in label_tokens(label) {
        match part {
            LabelPart::Text(t) => job.append(t, 0.0, text_fmt.clone()),
            LabelPart::Icon(name) => match icon_library::glyph(name) {
                Some(c) => job.append(&c.to_string(), 0.0, icon_fmt.clone()),
                None => job.append(&format!(":{name}:"), 0.0, text_fmt.clone()),
            },
        }
    }
    job.wrap.max_width = width.max(10.0);
    job.halign = Align::Center;
    p.layout_job(job)
}

/// Area below the label (color wheel, XY pad).
pub(crate) fn content_rect(rect: Rect, item: &TouchItem) -> Rect {
    let top = if item.label.is_empty() { 8.0 } else { 28.0 };
    Rect::from_min_max(rect.min + vec2(8.0, top), rect.max - vec2(8.0, 8.0))
}

/// Fader: (track with handle, travel of the handle center).
pub(crate) fn fader_parts(rect: Rect, horizontal: bool) -> (Rect, Rect) {
    const HANDLE_H: f32 = 34.0;
    if horizontal {
        // value on the left, track to the right of it
        let track = Rect::from_min_max(
            pos2(rect.left() + H_VALUE_W, rect.top()),
            pos2(rect.right() - 8.0, rect.bottom()),
        );
        let travel = Rect::from_min_max(
            pos2(track.left() + HANDLE_H * 0.5, track.top()),
            pos2(track.right() - HANDLE_H * 0.5, track.bottom()),
        );
        return (track, travel);
    }
    let track = Rect::from_min_max(
        pos2(rect.left(), rect.top() + 30.0),
        pos2(rect.right(), rect.bottom() - 26.0),
    );
    let travel = Rect::from_min_max(
        pos2(track.left(), track.top() + HANDLE_H * 0.5),
        pos2(track.right(), track.bottom() - HANDLE_H * 0.5),
    );
    (track, travel)
}

/// Width of the value at the left of a horizontal fader.
pub(crate) const H_VALUE_W: f32 = 52.0;

/// Horizontal fader: value on the left, label above the track.
#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_h_fader(
    p: &egui::Painter,
    item: &TouchItem,
    rect: Rect,
    value: f32,
    hot: bool,
    accent: Color32,
    label: &str,
    size: f32,
) {
    p.rect_filled(rect, theme::R_M, theme::raised());
    let (track, travel) = fader_parts(rect, true);
    let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
    p.text(
        pos2(rect.left() + H_VALUE_W * 0.5, rect.center().y),
        Align2::CENTER_CENTER,
        format!("{:.0}", value * 100.0),
        theme::heading_font(),
        text_color,
    );
    // label above the track (if there is room for it)
    let mut cy = track.center().y;
    if !label.is_empty() && rect.height() >= 56.0 {
        let g = p.layout(
            label.to_string(),
            egui::FontId::proportional(size.min(14.0)),
            text_color,
            track.width(),
        );
        p.galley(pos2(track.left(), rect.top() + 5.0), g.clone(), text_color);
        cy = (rect.top() + 5.0 + g.size().y + rect.bottom()) * 0.5;
    }
    // Slot with level in the element color
    let slot = Rect::from_center_size(pos2(track.center().x, cy), vec2(track.width() - 6.0, 6.0));
    p.rect_filled(slot, 3.0, theme::bg());
    let hx = travel.left() + travel.width() * value;
    let level = Rect::from_min_max(slot.min, pos2(hx, slot.bottom()));
    p.rect_filled(level, 3.0, accent);
    let handle = Rect::from_center_size(
        pos2(hx, cy),
        vec2(34.0, (rect.height() - 16.0).clamp(16.0, 40.0)),
    );
    p.rect_filled(handle, theme::R_S, theme::raised_hover());
    p.rect_stroke(
        handle,
        theme::R_S,
        Stroke::new(
            1.5,
            if hot {
                theme::text()
            } else {
                theme::text_dim()
            },
        ),
        StrokeKind::Inside,
    );
    p.line_segment(
        [
            pos2(handle.center().x, handle.top() + 7.0),
            pos2(handle.center().x, handle.bottom() - 7.0),
        ],
        Stroke::new(2.5, theme::text()),
    );
}

/// Readable text color on a background.
pub(crate) fn text_on(bg: Color32) -> Color32 {
    let l = 0.299 * f32::from(bg.r()) + 0.587 * f32::from(bg.g()) + 0.114 * f32::from(bg.b());
    if l > 150.0 {
        Color32::from_gray(20)
    } else {
        Color32::WHITE
    }
}

/// Draw an element. `value`: fader/knob 0–1; `hot`: pressed or under the
/// pointer.
#[allow(clippy::too_many_arguments)]
pub(crate) fn paint_item(
    p: &egui::Painter,
    e: &Engine,
    st: &AppState,
    states: &HashMap<u64, bool>,
    item: &TouchItem,
    rect: Rect,
    value: f32,
    hot: bool,
    tex: Option<&egui::TextureHandle>,
) {
    let show = e.show();
    let base = item.color.map(theme::rgb);
    let label = display_label(show, item);
    let auto = if item.kind == TouchKind::Label {
        (rect.height() * 0.42).clamp(13.0, 48.0)
    } else {
        (rect.height().min(rect.width()) * 0.16).clamp(11.0, 22.0)
    };
    // a fixed text size grows and shrinks with the view zoom
    let zoom = rect.width() / (item.w * UNIT).max(1.0);
    let size = if item.text_size > 0.0 {
        item.text_size * zoom
    } else if icons_only(&label) {
        (rect.height().min(rect.width()) * 0.42).max(14.0)
    } else {
        auto
    };
    let accent = base.unwrap_or(theme::accent());
    match item.kind {
        TouchKind::Label | TouchKind::Button => {
            let on = item.kind == TouchKind::Button
                && mapping_of(item).is_some_and(|m| crate::mapping::is_on(e, st, states, &m));
            let style = if item.kind == TouchKind::Button {
                item.style
            } else {
                ButtonStyle::Normal
            };
            let fill = match (item.kind, base) {
                (TouchKind::Button, _) if style == ButtonStyle::Outline => {
                    if on {
                        accent
                    } else {
                        Color32::TRANSPARENT
                    }
                }
                (_, Some(b)) => b,
                (TouchKind::Button, None) => theme::raised(),
                _ => Color32::TRANSPARENT,
            };
            let fill = if hot && fill != Color32::TRANSPARENT {
                fill.gamma_multiply(0.7)
            } else {
                fill
            };
            // round: a circle in the middle of the element
            let round = style == ButtonStyle::Round;
            let shape = if round {
                Rect::from_center_size(
                    rect.center(),
                    egui::Vec2::splat(rect.width().min(rect.height())),
                )
            } else {
                rect
            };
            let radius = if round {
                shape.width() / 2.0
            } else {
                f32::from(theme::R_M)
            };
            if fill != Color32::TRANSPARENT {
                p.rect_filled(shape, radius, fill);
            } else if hot {
                p.rect_filled(shape, radius, theme::raised());
            }
            if let Some(tex) = tex {
                paint_cover(p, rect.shrink(1.0), tex, if hot { 0.7 } else { 1.0 });
            }
            if item.kind == TouchKind::Button && style != ButtonStyle::Normal {
                paint_style(p, item, shape, radius, style, on, accent);
            } else if item.kind == TouchKind::Button {
                p.rect_stroke(
                    rect,
                    theme::R_M,
                    if on {
                        Stroke::new(3.0, Color32::WHITE)
                    } else {
                        Stroke::new(1.0, theme::border())
                    },
                    StrokeKind::Inside,
                );
                if on {
                    // glowing strip at the top
                    let bar = Rect::from_min_size(
                        rect.min + vec2(8.0, 6.0),
                        vec2(rect.width() - 16.0, 3.0),
                    );
                    p.rect_filled(bar, 1.5, Color32::WHITE);
                }
            }
            let text_color = item.text_color.map(theme::rgb).unwrap_or_else(|| {
                if tex.is_some() {
                    Color32::WHITE
                } else if fill == Color32::TRANSPARENT {
                    theme::text()
                } else {
                    text_on(fill)
                }
            });
            // Switch: the text sits next to / above the switch
            let rect = if item.kind == TouchKind::Button && style == ButtonStyle::Switch {
                switch_parts(rect).0
            } else {
                rect
            };
            if !label.is_empty() {
                let p = &p.with_clip_rect(p.clip_rect().intersect(rect));
                let galley = label_galley(p, &label, size, text_color, rect.width() - 12.0);
                // halign center: the galley is laid out around x = 0
                let pos = pos2(rect.center().x, rect.center().y - galley.size().y * 0.5);
                if tex.is_some() && item.text_color.is_none() {
                    // Shadow for readability on images
                    p.galley(
                        pos + vec2(1.0, 1.0),
                        galley.clone(),
                        Color32::from_black_alpha(200),
                    );
                }
                p.galley(pos, galley, text_color);
            }
            if item.kind == TouchKind::Button && item.control.is_none() {
                p.circle_filled(rect.right_top() + vec2(-9.0, 9.0), 3.5, theme::DANGER);
            }
        }
        TouchKind::Fader if item.horizontal => {
            paint_h_fader(p, item, rect, value, hot, accent, &label, size);
        }
        TouchKind::Fader => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let (track, travel) = fader_parts(rect, false);
            let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
            p.text(
                pos2(rect.center().x, rect.top() + 15.0),
                Align2::CENTER_CENTER,
                format!("{:.0}", value * 100.0),
                theme::heading_font(),
                text_color,
            );
            // Slot with level in the element color
            let slot = Rect::from_center_size(
                pos2(track.center().x, track.center().y),
                vec2(6.0, track.height() - 6.0),
            );
            p.rect_filled(slot, 3.0, theme::bg());
            let hy = travel.bottom() - travel.height() * value;
            let level = Rect::from_min_max(pos2(slot.left(), hy), slot.max);
            p.rect_filled(level, 3.0, accent);
            // Handle like in the programmer
            let handle = Rect::from_center_size(
                pos2(track.center().x, hy),
                vec2((rect.width() - 16.0).min(40.0), 34.0),
            );
            p.rect_filled(handle, theme::R_S, theme::raised_hover());
            p.rect_stroke(
                handle,
                theme::R_S,
                Stroke::new(
                    1.5,
                    if hot {
                        theme::text()
                    } else {
                        theme::text_dim()
                    },
                ),
                StrokeKind::Inside,
            );
            p.line_segment(
                [
                    pos2(handle.left() + 7.0, handle.center().y),
                    pos2(handle.right() - 7.0, handle.center().y),
                ],
                Stroke::new(2.5, theme::text()),
            );
            if !label.is_empty() {
                let g = p.layout(
                    label,
                    egui::FontId::proportional(size.min(14.0)),
                    text_color,
                    rect.width() - 6.0,
                );
                p.galley(
                    pos2(
                        rect.center().x - g.size().x * 0.5,
                        rect.bottom() - 6.0 - g.size().y,
                    ),
                    g,
                    text_color,
                );
            }
        }
        TouchKind::Knob => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            let text_color = item.text_color.map(theme::rgb).unwrap_or(theme::text());
            let has_label = !label.is_empty();
            let avail_h = rect.height() - if has_label { 22.0 } else { 8.0 };
            let r = (avail_h.min(rect.width() - 12.0) * 0.5).max(12.0);
            let c = pos2(rect.center().x, rect.top() + 4.0 + avail_h * 0.5);
            let start = std::f32::consts::PI * 0.75;
            let sweep = std::f32::consts::PI * 1.5;
            let arc = |from: f32, to: f32, radius: f32| -> Vec<Pos2> {
                (0..=40)
                    .map(|i| c + egui::Vec2::angled(from + (to - from) * i as f32 / 40.0) * radius)
                    .collect()
            };
            let ring = (r * 0.14).clamp(3.0, 7.0);
            p.add(Shape::line(
                arc(start, start + sweep, r - ring),
                Stroke::new(ring, theme::bg()),
            ));
            if value > 0.001 {
                p.add(Shape::line(
                    arc(start, start + sweep * value, r - ring),
                    Stroke::new(ring, accent),
                ));
            }
            let body = r - ring * 2.2;
            p.circle_filled(
                c,
                body,
                if hot {
                    theme::raised_hover()
                } else {
                    theme::raised()
                },
            );
            p.circle_stroke(c, body, Stroke::new(1.0, theme::border()));
            let a = start + sweep * value;
            p.line_segment(
                [
                    c + egui::Vec2::angled(a) * (body * 0.35),
                    c + egui::Vec2::angled(a) * (body * 0.85),
                ],
                Stroke::new(2.5, theme::text()),
            );
            p.text(
                c + vec2(0.0, body * 0.35 + 4.0),
                Align2::CENTER_TOP,
                format!("{:.0}", value * 100.0),
                theme::caption(),
                theme::text_dim(),
            );
            if has_label {
                p.text(
                    pos2(rect.center().x, rect.bottom() - 11.0),
                    Align2::CENTER_CENTER,
                    label,
                    egui::FontId::proportional(size.min(14.0)),
                    text_color,
                );
            }
        }
        TouchKind::Color => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(10.0, 15.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    item.text_color.map(theme::rgb).unwrap_or(theme::text()),
                );
            }
            let field = content_rect(rect, item);
            let mut mesh = Mesh::default();
            const N: u32 = 24;
            for yi in 0..=1u32 {
                for xi in 0..=N {
                    let h = xi as f32 / N as f32;
                    let s = 1.0 - yi as f32;
                    mesh.colored_vertex(
                        pos2(
                            field.left() + field.width() * h,
                            field.top() + field.height() * yi as f32,
                        ),
                        egui::ecolor::Hsva::new(h, s, 1.0, 1.0).into(),
                    );
                }
            }
            for xi in 0..N {
                mesh.add_triangle(xi, xi + 1, xi + N + 1);
                mesh.add_triangle(xi + 1, xi + N + 2, xi + N + 1);
            }
            p.add(Shape::mesh(mesh));
            if let Some(rgb) = item.fixtures.iter().find_map(|f| current_color(e, *f)) {
                let hsv = egui::ecolor::Hsva::from_srgb(rgb);
                let m = pos2(
                    field.left() + hsv.h * field.width(),
                    field.top() + (1.0 - hsv.s) * field.height(),
                );
                p.circle_stroke(m, 8.0, Stroke::new(2.0, Color32::BLACK));
                p.circle_stroke(m, 6.0, Stroke::new(2.0, Color32::WHITE));
            }
        }
        TouchKind::Xy => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(10.0, 15.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    item.text_color.map(theme::rgb).unwrap_or(theme::text()),
                );
            }
            let field = content_rect(rect, item);
            p.rect_filled(field, theme::R_S, theme::bg());
            for i in 1..4 {
                let f = i as f32 / 4.0;
                let s = Stroke::new(1.0, theme::raised());
                p.line_segment(
                    [
                        pos2(field.left() + field.width() * f, field.top()),
                        pos2(field.left() + field.width() * f, field.bottom()),
                    ],
                    s,
                );
                p.line_segment(
                    [
                        pos2(field.left(), field.top() + field.height() * f),
                        pos2(field.right(), field.top() + field.height() * f),
                    ],
                    s,
                );
            }
            if let Some((x, y)) = item.fixtures.iter().find_map(|f| current_position(e, *f)) {
                let m = pos2(
                    field.left() + x * field.width(),
                    field.top() + y * field.height(),
                );
                p.line_segment(
                    [pos2(m.x, field.top()), pos2(m.x, field.bottom())],
                    Stroke::new(1.0, accent.gamma_multiply(0.5)),
                );
                p.line_segment(
                    [pos2(field.left(), m.y), pos2(field.right(), m.y)],
                    Stroke::new(1.0, accent.gamma_multiply(0.5)),
                );
                p.circle_filled(m, 8.0, accent);
                p.circle_stroke(m, 8.0, Stroke::new(2.0, Color32::WHITE));
            }
        }
        TouchKind::Tracking => {
            p.rect_filled(rect, theme::R_M, theme::raised());
            if !item.label.is_empty() {
                p.text(
                    rect.left_top() + vec2(10.0, 15.0),
                    Align2::LEFT_CENTER,
                    &item.label,
                    theme::body(),
                    item.text_color.map(theme::rgb).unwrap_or(theme::text()),
                );
            }
            let field = content_rect(rect, item);
            RoomMap::new(show, field).paint(
                &p.with_clip_rect(field.expand(2.0)),
                show,
                None,
                false,
            );
        }
    }
}

/// Switch style: (area for icon and text, switch track).
pub(crate) fn switch_parts(rect: Rect) -> (Rect, Rect) {
    let size = vec2(
        (rect.height() * 0.5).clamp(26.0, 46.0),
        (rect.height() * 0.28).clamp(14.0, 24.0),
    );
    if rect.width() >= rect.height() * 1.4 {
        // wide: switch on the right, text to its left
        let track = Rect::from_center_size(
            pos2(rect.right() - 10.0 - size.x / 2.0, rect.center().y),
            size,
        );
        (rect.with_max_x(track.left() - 4.0), track)
    } else {
        // narrow: switch at the bottom, text above
        let track = Rect::from_center_size(
            pos2(rect.center().x, rect.bottom() - 8.0 - size.y / 2.0),
            size,
        );
        (rect.with_max_y(track.top() - 2.0), track)
    }
}

/// Frame and state display of the button styles other than “Normal”.
pub(crate) fn paint_style(
    p: &egui::Painter,
    item: &TouchItem,
    shape: Rect,
    radius: f32,
    style: ButtonStyle,
    on: bool,
    accent: Color32,
) {
    match style {
        ButtonStyle::Normal => {}
        ButtonStyle::Outline => {
            p.rect_stroke(shape, radius, Stroke::new(2.0, accent), StrokeKind::Inside);
        }
        ButtonStyle::Round => {
            p.rect_stroke(
                shape,
                radius,
                if on {
                    Stroke::new(3.0, Color32::WHITE)
                } else {
                    Stroke::new(1.0, theme::border())
                },
                StrokeKind::Inside,
            );
        }
        ButtonStyle::Led => {
            p.rect_stroke(
                shape,
                radius,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let c = shape.left_top() + vec2(12.0, 12.0);
            if on {
                p.circle_filled(c, 8.0, accent.gamma_multiply(0.35));
                p.circle_filled(c, 5.0, led_color(item, accent));
            } else {
                p.circle_filled(c, 5.0, theme::bg());
            }
            p.circle_stroke(c, 5.0, Stroke::new(1.0, theme::border()));
        }
        ButtonStyle::Switch => {
            p.rect_stroke(
                shape,
                radius,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let track = switch_parts(shape).1;
            let r = track.height() / 2.0;
            p.rect_filled(
                track,
                r,
                if on {
                    led_color(item, accent)
                } else {
                    theme::bg()
                },
            );
            p.rect_stroke(
                track,
                r,
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            let knob = if on {
                track.right() - r
            } else {
                track.left() + r
            };
            p.circle_filled(pos2(knob, track.center().y), r - 3.0, Color32::WHITE);
        }
    }
}

/// Color of the on state: the element color, on a colored surface the accent.
pub(crate) fn led_color(item: &TouchItem, accent: Color32) -> Color32 {
    if item.color.is_some() {
        theme::OK
    } else {
        accent
    }
}

/// Draw an image filling the area (cropped).
pub(crate) fn paint_cover(p: &egui::Painter, rect: Rect, tex: &egui::TextureHandle, alpha: f32) {
    let [w, h] = tex.size();
    let (iw, ih) = (w as f32, h as f32);
    if iw <= 0.0 || ih <= 0.0 || rect.width() <= 0.0 || rect.height() <= 0.0 {
        return;
    }
    let ra = rect.width() / rect.height();
    let ia = iw / ih;
    let uv = if ia > ra {
        let f = ra / ia;
        Rect::from_min_max(pos2((1.0 - f) * 0.5, 0.0), pos2((1.0 + f) * 0.5, 1.0))
    } else {
        let f = ia / ra;
        Rect::from_min_max(pos2(0.0, (1.0 - f) * 0.5), pos2(1.0, (1.0 + f) * 0.5))
    };
    p.image(tex.id(), rect, uv, Color32::WHITE.gamma_multiply(alpha));
}
