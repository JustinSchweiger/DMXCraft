//! Media on the timeline: video/audio tracks and blocks, lead-in marks, grouping.

#[allow(unused_imports)]
use super::*;

/// Media tracks get a name and color from what they hold (video, music,
/// OBS), as long as none was set by hand.
pub(crate) fn media_role(tr: &dmxcraft_core::timeline::Track) -> Option<(&'static str, Color32)> {
    if tr.kind != TrackKind::Media {
        return None;
    }
    let has = |f: fn(&ClipContent) -> bool| tr.clips.iter().any(|c| f(&c.content));
    if has(|c| matches!(c, ClipContent::Video { .. })) {
        Some((t("Video"), VIDEO_COLOR))
    } else if has(|c| matches!(c, ClipContent::Audio { .. })) {
        Some((t("Music"), MUSIC_COLOR))
    } else if has(|c| matches!(c, ClipContent::Obs(_))) {
        Some(("OBS", OBS_COLOR))
    } else {
        None
    }
}
pub(crate) const OBS_COLOR: Color32 = Color32::from_rgb(70, 130, 190);
pub(crate) const VIDEO_EXT: [&str; 6] = ["mp4", "mov", "m4v", "mkv", "webm", "avi"];

pub(crate) fn is_video(path: &std::path::Path) -> bool {
    path.extension()
        .and_then(|e| e.to_str())
        .is_some_and(|e| VIDEO_EXT.contains(&e.to_lowercase().as_str()))
}

/// Display name of an OBS block.
pub(crate) fn obs_label(a: &dmxcraft_core::obs::ObsAction) -> String {
    use dmxcraft_core::obs::ObsAction;
    match a {
        ObsAction::Scene(s) => tr!("OBS: {scene}", scene = s),
        ObsAction::Source {
            source,
            visible: true,
            ..
        } => tr!("OBS: {source} on", source = source),
        ObsAction::Source { source, .. } => tr!("OBS: {source} off", source = source),
    }
}

/// Create a block from a video file (OBS plays it); returns ID and length.
/// Video block plus – if the file has sound – a linked sound block on the
/// media track below (DMXCraft plays the sound, OBS only the picture).
pub(crate) fn add_video(tl: &mut Timeline, track: usize, at: u64, path: &str) -> (u32, u64) {
    let (id, len) = add_video_only(tl, track, at, path);
    let _ = add_video_sound(tl, id);
    (id, len)
}

/// Sound block for a video block (linked); nothing if the video has no
/// sound or already has one.
pub(crate) fn add_video_sound(tl: &mut Timeline, video: u32) -> bool {
    let Some((ti, c)) = tl.clip(video).map(|(t, c)| (t, c.clone())) else {
        return false;
    };
    let ClipContent::Video {
        path,
        offset_ms,
        length_ms,
        ..
    } = &c.content
    else {
        return false;
    };
    if c.link.is_some() || !has_sound_cached(path) {
        return false;
    }
    // media track right below, free at that time – otherwise a new one
    let free = |t: &dmxcraft_core::timeline::Track| {
        t.kind == TrackKind::Media
            && t.clips
                .iter()
                .all(|x| x.end_ms() <= c.start_ms || x.start_ms >= c.end_ms())
    };
    let track = if tl.tracks.get(ti + 1).is_some_and(free) {
        ti + 1
    } else {
        tl.add_track_of(Some(ti + 1), TrackKind::Media);
        ti + 1
    };
    let a = tl.add_clip(
        track,
        ClipContent::Audio {
            path: path.clone(),
            offset_ms: *offset_ms,
            length_ms: *length_ms,
            grid: None,
            segment_grids: Vec::new(),
            triggers: Vec::new(),
        },
        c.start_ms,
        c.duration_ms,
    );
    if let Some(n) = tl.clip_mut(a) {
        n.speed = c.speed;
        n.looped = c.looped;
    }
    tl.link_clips(&[video, a]);
    true
}

/// `audio::has_sound`, remembered per file (opening a video takes a moment).
pub(crate) fn has_sound_cached(path: &str) -> bool {
    use std::collections::HashMap;
    use std::sync::{Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<HashMap<String, bool>>> = OnceLock::new();
    let cache = CACHE.get_or_init(Default::default);
    if let Some(v) = cache.lock().ok().and_then(|c| c.get(path).copied()) {
        return v;
    }
    let v = crate::audio::has_sound(path);
    if let Ok(mut c) = cache.lock() {
        c.insert(path.to_string(), v);
    }
    v
}

pub(crate) fn add_video_only(tl: &mut Timeline, track: usize, at: u64, path: &str) -> (u32, u64) {
    let len = std::fs::File::open(path)
        .ok()
        .and_then(|mut f| dmxcraft_core::obs::mp4_duration_ms(&mut f))
        .filter(|l| *l > 0)
        .unwrap_or(DEFAULT_CLIP_MS);
    let id = tl.add_clip(
        track,
        ClipContent::Video {
            path: path.to_string(),
            offset_ms: 0,
            length_ms: len,
            obs_scene: String::new(),
            source: String::new(),
        },
        at,
        len,
    );
    (id, len)
}

/// Create a block from an audio file; returns ID and length.
pub(crate) fn add_audio(tl: &mut Timeline, track: usize, at: u64, path: &str) -> (u32, u64) {
    let len = crate::audio::duration_ms(path).unwrap_or(DEFAULT_CLIP_MS);
    let id = tl.add_clip(
        track,
        ClipContent::Audio {
            path: path.to_string(),
            offset_ms: 0,
            length_ms: len,
            grid: None,
            segment_grids: Vec::new(),
            triggers: Vec::new(),
        },
        at,
        len,
    );
    (id, len)
}

pub(crate) fn add_audio_dialog(tl: &mut Timeline, track: usize, at: u64) -> Option<u32> {
    let path = rfd::FileDialog::new()
        .add_filter("Audio", &["mp3", "wav", "flac", "ogg", "m4a", "aac"])
        .pick_file()?;
    Some(add_audio(tl, track, at, &path.to_string_lossy()).0)
}

/// Track for a media block near `ti`: the track itself if it is a media
/// track, otherwise the first media track, otherwise a new one at the end.
pub(crate) fn media_track(tl: &mut Timeline, ti: usize) -> usize {
    if let Some(t) = tl.tracks.get_mut(ti) {
        // an empty light track becomes a media track
        if t.clips.is_empty() {
            t.kind = TrackKind::Media;
        }
        if t.kind == TrackKind::Media {
            return ti;
        }
    }
    if let Some(i) = tl.tracks.iter().position(|t| t.kind == TrackKind::Media) {
        return i;
    }
    tl.add_track_of(None, TrackKind::Media);
    tl.tracks.len() - 1
}

/// Lead-in of the pre-position in front of a block: hatched in the scene
/// color, handle on the left, lead time as text.
pub(crate) fn paint_lead(p: &egui::Painter, r: Rect, color: Color32, prep: Prepare) {
    if r.width() < 2.0 {
        return;
    }
    p.rect_filled(r, 3.0, color.gamma_multiply(0.12));
    let clip = p.with_clip_rect(r.intersect(p.clip_rect()));
    let mut x = r.left() - r.height();
    while x < r.right() {
        clip.line_segment(
            [pos2(x, r.bottom()), pos2(x + r.height(), r.top())],
            Stroke::new(1.5, color.gamma_multiply(0.45)),
        );
        x += 7.0;
    }
    p.rect_stroke(r, 3.0, Stroke::new(1.0, color), StrokeKind::Inside);
    p.rect_filled(
        Rect::from_center_size(pos2(r.left(), r.center().y), Vec2::new(5.0, 16.0)),
        2.0,
        color,
    );
    let text = format!("{:.2}s", f64::from(prep.lead_ms) / 1000.0);
    let galley = p.layout_no_wrap(text, egui::FontId::proportional(theme::FS_CAPTION), color);
    if galley.size().x + 12.0 < r.width() {
        let at = r.center() - galley.size() / 2.0;
        p.rect_filled(
            Rect::from_min_size(
                at - Vec2::new(3.0, 1.0),
                galley.size() + Vec2::new(6.0, 2.0),
            ),
            3.0,
            theme::bg(),
        );
        p.galley(at, galley, color);
    }
}

/// Hint next to the pointer when a drop is not possible.
pub(crate) fn drop_hint(p: &egui::Painter, at: egui::Pos2, text: &str) {
    let galley = p.layout_no_wrap(
        text.to_string(),
        egui::FontId::proportional(theme::FS_CAPTION),
        theme::DANGER,
    );
    let r = Rect::from_min_size(
        at + Vec2::new(14.0, 10.0),
        galley.size() + Vec2::new(12.0, 6.0),
    );
    p.rect_filled(r, 3.0, theme::raised());
    p.rect_stroke(r, 3.0, Stroke::new(1.0, theme::DANGER), StrokeKind::Inside);
    p.galley(r.min + Vec2::new(6.0, 3.0), galley, theme::DANGER);
}

/// Replace a SCENE GROUP block by one block per scene: the first scene on the
/// block's track, the others on new light tracks below (same priority order).
pub(crate) fn ungroup_clip(tl: &mut Timeline, show: &dmxcraft_core::Show, cid: u32) {
    let Some((ti, c)) = tl.clip(cid).map(|(t, c)| (t, c.clone())) else {
        return;
    };
    let Some(members) = show.group_members(&c.content).map(<[_]>::to_vec) else {
        return;
    };
    tl.remove_clip(cid);
    let speed = f64::from(c.speed.max(0.01));
    for (k, m) in members.iter().enumerate() {
        let Some(sc) = show.scene(m.scene) else {
            continue;
        };
        let track = if k == 0 {
            ti
        } else {
            tl.add_track_of(Some(ti + k), TrackKind::Light);
            ti + k
        };
        let start = c.start_ms + (m.offset_ms as f64 / speed) as u64;
        if start >= c.end_ms() {
            continue;
        }
        let id = tl.add_scene_clip(track, sc, start);
        if let Some(n) = tl.clip_mut(id) {
            n.speed = c.speed;
            n.level = c.level;
            n.looped = false;
            n.duration_ms = n.duration_ms.min(c.end_ms() - start).max(40);
            if k == 0 {
                n.prepare = c.prepare;
            }
        }
    }
}
