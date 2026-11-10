//! Endless dragging for number fields and sliders: the mouse pointer is
//! hidden and held in place while dragging (locked or reset to the start
//! point), the movement comes from the raw mouse data. That way a value can
//! be changed as far as you like, even beyond the window edge.

use eframe::egui::{self, CursorIcon, Pos2, Response, Ui, Vec2, ViewportCommand};

/// Have raw mouse movements been received already? (otherwise normal dragging)
fn raw_available(ctx: &egui::Context) -> bool {
    let id = egui::Id::unique("raw_mouse_motion_seen");
    if ctx.input(|i| i.pointer.motion().is_some()) {
        ctx.data_mut(|d| d.insert_temp(id, true));
        return true;
    }
    ctx.data(|d| d.get_temp(id)).unwrap_or(false)
}

/// Mouse travel of this frame for a dragged element (in points).
pub fn endless_delta(ui: &Ui, resp: &Response) -> Vec2 {
    let ctx = ui.ctx();
    let origin_id = resp.id.with("endless_origin");
    if resp.drag_stopped() {
        ctx.send_viewport_cmd(ViewportCommand::CursorGrab(egui::CursorGrab::None));
        ctx.data_mut(|d| d.remove::<Pos2>(origin_id));
        return Vec2::ZERO;
    }
    if !resp.dragged() {
        return Vec2::ZERO;
    }
    if !raw_available(ctx) {
        return resp.drag_delta();
    }
    ctx.set_cursor_icon(CursorIcon::None);
    let origin: Option<Pos2> = ctx.data(|d| d.get_temp(origin_id));
    let origin = match origin {
        Some(o) => o,
        None => {
            let o = ctx
                .input(|i| i.pointer.press_origin())
                .or(resp.interact_pointer_pos())
                .unwrap_or_default();
            ctx.data_mut(|d| d.insert_temp(origin_id, o));
            // Wayland/macOS: lock the pointer; otherwise the reset below helps
            ctx.send_viewport_cmd(ViewportCommand::CursorGrab(egui::CursorGrab::Locked));
            o
        }
    };
    ctx.send_viewport_cmd(ViewportCommand::CursorPosition(origin));
    let ppp = ctx.pixels_per_point();
    ctx.input(|i| i.pointer.motion())
        .map_or(Vec2::ZERO, |m| m / ppp)
}

/// Small number field in the style of the icon buttons: dragging up/down
/// (endless) or the mouse wheel changes the value. `true` if it changed.
pub fn drag_number(
    ui: &mut Ui,
    id_salt: &str,
    value: &mut u32,
    range: std::ops::RangeInclusive<u32>,
    enabled: bool,
    size: Vec2,
) -> Response {
    use crate::ui::theme;
    let sense = if enabled {
        egui::Sense::click_and_drag()
    } else {
        egui::Sense::hover()
    };
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
    let mut resp = ui.interact(rect, ui.make_persistent_id(("drag_number", id_salt)), sense);
    if enabled && resp.hovered() && !resp.dragged() {
        ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
    }
    let delta = endless_delta(ui, &resp);
    let acc_id = resp.id.with("acc");
    let mut steps = 0i64;
    if resp.dragged() {
        let mut acc: f32 = ui.data(|d| d.get_temp(acc_id)).unwrap_or(0.0);
        acc -= delta.y / 4.0;
        steps = acc.trunc() as i64;
        acc -= steps as f32;
        ui.data_mut(|d| d.insert_temp(acc_id, acc));
    }
    if enabled && resp.hovered() {
        let wheel: f32 = ui.input(|i| {
            i.events
                .iter()
                .filter_map(|e| match e {
                    egui::Event::MouseWheel { delta, .. } => Some(delta.y.signum()),
                    _ => None,
                })
                .sum()
        });
        steps += wheel as i64;
    }
    if steps != 0 {
        let v =
            (i64::from(*value) + steps).clamp(i64::from(*range.start()), i64::from(*range.end()));
        if v as u32 != *value {
            *value = v as u32;
            resp.mark_changed();
        }
    }
    let p = ui.painter();
    let hot = enabled && (resp.hovered() || resp.dragged());
    p.rect_filled(
        rect,
        theme::R_S,
        if hot {
            theme::raised_hover()
        } else {
            theme::raised()
        },
    );
    p.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        value.to_string(),
        theme::caption(),
        if enabled {
            theme::text()
        } else {
            theme::text_faint()
        },
    );
    resp
}
