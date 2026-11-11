//! SCENE GROUPS in CONTROL: the strip above the banks (chips, drag them into
//! a SUPER SCENE) and the group editor in the sidebar (scenes with offsets,
//! order = priority).

use dmxcraft_core::scene_group::GroupMember;
use dmxcraft_core::{Engine, SceneId};
use eframe::egui::{self, pos2, Color32, Rect, Sense, Stroke, StrokeKind, Vec2};

use crate::i18n::t;
use crate::state::AppState;
use crate::ui::theme;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::time_field::time_field_compact;
use crate::ui::widgets::{text_field, Icon, IconButton};

/// Drag payload: a SCENE GROUP (into a SUPER SCENE).
#[derive(Clone, Copy, Debug)]
pub struct GroupDrag(pub u32);

/// Color of SCENE GROUPS (blocks, chips).
pub const GROUP_COLOR: Color32 = Color32::from_rgb(160, 120, 225);

const CHIP_H: f32 = 26.0;

/// Strip “SCENE GROUPS” with one chip per group and “+”.
pub fn strip(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState) {
    let locked = st.locked;
    let mut delete = None;
    ui.horizontal(|ui| {
        ui.set_min_height(CHIP_H);
        ui.spacing_mut().item_spacing.x = theme::SP_XS;
        ui.label(theme::heading_text(t("Scene groups")));
        ui.add_space(theme::SP_XS);
        let groups: Vec<(u32, String, usize)> = e
            .show()
            .scene_groups
            .iter()
            .map(|g| (g.id, g.name.clone(), g.members.len()))
            .collect();
        for (id, name, n) in groups {
            let active = st.selected_group == Some(id);
            let r = chip(ui, &name, n, active);
            r.dnd_set_drag_payload(GroupDrag(id));
            if r.clicked() {
                st.selected_group = if active { None } else { Some(id) };
            }
            if !locked {
                menu::context(&r, |ui| {
                    menu::begin(ui);
                    if menu::item(ui, MenuItem::new(t("Edit")).icon(Icon::Edit)) {
                        st.selected_group = Some(id);
                    }
                    if menu::item(ui, MenuItem::new(t("Delete")).icon(Icon::Trash).danger()) {
                        delete = Some(id);
                    }
                });
            }
            r.on_hover_text(t("Drag into a SUPER SCENE · click: edit"));
        }
        if !locked
            && ui
                .add(IconButton::new(Icon::Plus).size(CHIP_H).flat())
                .on_hover_text(t("New scene group"))
                .clicked()
        {
            let n = e.show().scene_groups.len() + 1;
            let id = e.show_mut().add_scene_group(tr!("Group {n}", n = n));
            st.selected_group = Some(id);
        }
    });
    if let Some(id) = delete {
        e.show_mut().remove_scene_group(id);
        if st.selected_group == Some(id) {
            st.selected_group = None;
        }
    }
}

fn chip(ui: &mut egui::Ui, name: &str, n: usize, active: bool) -> egui::Response {
    let font = theme::body();
    let galley = ui
        .painter()
        .layout_no_wrap(name.to_string(), font.clone(), theme::text());
    let count = n.to_string();
    let w = 10.0 + galley.size().x + 8.0 + 8.0 * count.len() as f32 + 10.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(w, CHIP_H), Sense::click_and_drag());
    let p = ui.painter();
    let fill = if active || resp.hovered() {
        theme::raised_hover()
    } else {
        theme::raised()
    };
    p.rect_filled(rect, theme::R_S as f32, fill);
    p.rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(3.0, rect.height())),
        1.0,
        GROUP_COLOR,
    );
    if active {
        p.rect_stroke(
            rect,
            theme::R_S as f32,
            Stroke::new(1.5, theme::accent()),
            StrokeKind::Inside,
        );
    }
    p.galley(
        pos2(rect.left() + 10.0, rect.center().y - galley.size().y / 2.0),
        galley,
        theme::text(),
    );
    p.text(
        pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        count,
        egui::FontId::proportional(theme::FS_CAPTION),
        theme::text_faint(),
    );
    resp
}

/// Editor of the selected group in the sidebar.
pub fn editor(ui: &mut egui::Ui, e: &mut Engine, st: &mut AppState, id: u32) {
    let locked = st.locked;
    let show = e.show();
    let Some(g) = show.scene_group(id).cloned() else {
        st.selected_group = None;
        return;
    };
    let show_len_before = show.scene_group_length(id);
    let len = show_len_before.max(1);
    // scene name, color, length
    let info = |s: SceneId| {
        show.scene(s).map_or(
            (t("Deleted").to_string(), theme::text_faint(), 1000),
            |sc| {
                (
                    sc.name.clone(),
                    sc.color
                        .map_or_else(|| theme::zone_color(show, sc.zone), theme::rgb),
                    sc.duration_ms().max(1),
                )
            },
        )
    };
    let rows: Vec<(String, Color32, u64)> = g.members.iter().map(|m| info(m.scene)).collect();
    let mut name = g.name.clone();
    let mut members = g.members.clone();
    let mut close = false;
    let mut remove_group = false;

    // Header: name, length, close
    ui.horizontal(|ui| {
        ui.set_min_height(28.0);
        ui.spacing_mut().item_spacing.x = theme::SP_XS;
        ui.label(
            egui::RichText::new(t("SCENE GROUP"))
                .font(theme::caption())
                .color(GROUP_COLOR),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            close = ui
                .add(IconButton::new(Icon::Close).size(24.0).flat())
                .on_hover_text(t("Close"))
                .clicked();
            remove_group = ui
                .add(
                    IconButton::new(Icon::Trash)
                        .size(24.0)
                        .flat()
                        .enabled(!locked),
                )
                .on_hover_text(t("Delete group"))
                .clicked();
            ui.label(
                egui::RichText::new(crate::views::control::timeline::fmt_ms(len as f64))
                    .monospace()
                    .color(theme::text_dim()),
            );
            ui.add_enabled(
                !locked,
                text_field(&mut name).desired_width(ui.available_width()),
            );
        });
    });
    ui.add_space(theme::SP_S);

    // Scenes: name, bar (offset and length within the group), offset, order
    let mut remove = None;
    let mut swap = None;
    let n = members.len();
    for (i, m) in members.iter_mut().enumerate() {
        let (sname, color, dur) = &rows[i];
        ui.horizontal(|ui| {
            ui.set_min_height(28.0);
            ui.spacing_mut().item_spacing.x = theme::SP_XS;
            let (nr, _) = ui.allocate_exact_size(Vec2::new(120.0, 24.0), Sense::hover());
            let p = ui.painter_at(nr);
            p.text(
                pos2(nr.left(), nr.center().y),
                egui::Align2::LEFT_CENTER,
                sname,
                theme::body(),
                theme::text(),
            );
            let right = 3.0 * 26.0 + crate::ui::widgets::time_field::COMPACT_W + 4.0 * theme::SP_XS;
            let (bar, br) = ui.allocate_exact_size(
                Vec2::new((ui.available_width() - right).max(40.0), 18.0),
                if locked {
                    Sense::hover()
                } else {
                    Sense::drag()
                },
            );
            let p = ui.painter();
            p.rect_filled(bar, 3.0, theme::bg());
            let x = |ms: u64| bar.left() + bar.width() * (ms as f32 / len as f32).min(1.0);
            let seg = Rect::from_min_max(
                pos2(x(m.offset_ms), bar.top() + 2.0),
                pos2(
                    x(m.offset_ms + dur).max(x(m.offset_ms) + 3.0),
                    bar.bottom() - 2.0,
                ),
            );
            p.rect_filled(seg, 2.0, *color);
            // drag the bar: move the scene in time (10 ms steps)
            if br.dragged() {
                let d = br.drag_delta().x / bar.width() * len as f32;
                let v = (m.offset_ms as f32 + d).max(0.0);
                m.offset_ms = (v / 10.0).round() as u64 * 10;
            }
            if br.hovered() || br.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }
            let mut off = m.offset_ms.min(u64::from(u32::MAX)) as u32;
            if time_field_compact(
                ui,
                &format!("grp_off_{id}_{i}"),
                &mut off,
                !locked,
                GROUP_COLOR,
            ) {
                m.offset_ms = u64::from(off);
            }
            if ui
                .add(
                    IconButton::new(Icon::ChevronUp)
                        .size(24.0)
                        .flat()
                        .enabled(!locked && i > 0),
                )
                .on_hover_text(t("Higher priority"))
                .clicked()
            {
                swap = Some((i - 1, i));
            }
            if ui
                .add(
                    IconButton::new(Icon::ChevronDown)
                        .size(24.0)
                        .flat()
                        .enabled(!locked && i + 1 < n),
                )
                .on_hover_text(t("Lower priority"))
                .clicked()
            {
                swap = Some((i, i + 1));
            }
            if ui
                .add(
                    IconButton::new(Icon::Trash)
                        .size(24.0)
                        .flat()
                        .enabled(!locked),
                )
                .on_hover_text(t("Remove from the group"))
                .clicked()
            {
                remove = Some(i);
            }
        });
    }
    if let Some((a, b)) = swap {
        members.swap(a, b);
    }
    if let Some(i) = remove {
        members.remove(i);
    }

    // Drop zone: scenes from the banks
    let (zone, zr) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 34.0), Sense::hover());
    let hover = zr.dnd_hover_payload::<SceneId>();
    let p = ui.painter();
    p.rect_stroke(
        zone,
        theme::R_S as f32,
        Stroke::new(
            1.0,
            if hover.is_some() {
                theme::accent()
            } else {
                theme::border()
            },
        ),
        StrokeKind::Inside,
    );
    p.text(
        zone.center(),
        egui::Align2::CENTER_CENTER,
        t("Drag scenes here"),
        theme::body(),
        if hover.is_some() {
            theme::accent()
        } else {
            theme::text_faint()
        },
    );
    if let Some(s) = zr.dnd_release_payload::<SceneId>() {
        let ok = e.show().scene(*s).is_some_and(|sc| sc.timeline().is_none());
        if !locked && ok {
            // all selected scenes if the dragged one belongs to them
            let list: Vec<SceneId> = if st.selected_scenes.contains(&s) {
                st.selected_scenes.iter().copied().collect()
            } else {
                vec![*s]
            };
            for sc in list {
                if e.show().scene(sc).is_some_and(|x| x.timeline().is_none()) {
                    members.push(GroupMember {
                        scene: sc,
                        offset_ms: 0,
                    });
                }
            }
        } else if !ok {
            st.error(t("SUPER SCENES cannot be in a group"));
        }
    }

    if locked {
        if close {
            st.selected_group = None;
        }
        return;
    }
    if remove_group {
        e.show_mut().remove_scene_group(id);
        st.selected_group = None;
        return;
    }
    if let Some(gm) = e.show_mut().scene_group_mut(id) {
        if gm.name != name && !name.trim().is_empty() {
            gm.name = name;
        }
        if gm.members != members {
            gm.members = members;
        }
    }
    // linked blocks with the group's length follow a new length
    let old = show_len_before;
    let len = e.show().scene_group_length(id);
    if len > 0 && len != old {
        for sc in &mut e.show_mut().scenes {
            if let Some(tl) = sc.timeline_mut() {
                let mut changed = false;
                for c in tl.tracks.iter_mut().flat_map(|t| t.clips.iter_mut()) {
                    if matches!(c.content, dmxcraft_core::timeline::ClipContent::Group { group, members: None } if group == id)
                        && c.duration_ms == old
                    {
                        c.duration_ms = len;
                        changed = true;
                    }
                }
                if changed {
                    sc.sync_timeline_length();
                }
            }
        }
    }
    if close {
        st.selected_group = None;
    }
}
