//! Reiter SOUND: zwei Player mit Wellenform und Playlist, Überblenden und
//! Jingle-Pads. Abgespielt wird in [`crate::sound::SoundPlayer`].

use dmxcraft_core::sound::{SoundBoard, Track};
use eframe::egui::{
    self, pos2, vec2, Align2, Color32, CornerRadius, FontId, Rect, RichText, Sense, Stroke,
    StrokeKind, Ui,
};

use super::{View, ViewCtx};
use crate::audio::WaveState;
use crate::i18n::t;
use crate::sound::{DeckState, SoundPlayer};
use crate::ui::theme;
use crate::ui::widgets::color_picker::color_swatches;
use crate::ui::widgets::menu::{self, MenuItem};
use crate::ui::widgets::{Icon, IconButton};

const AUDIO_EXT: [&str; 6] = ["mp3", "wav", "flac", "ogg", "m4a", "aac"];
const DECK_H: f32 = 236.0;
const BAR_H: f32 = 44.0;

#[derive(Default)]
pub struct SoundView {
    /// Markierter Titel je Player.
    sel: [Option<usize>; 2],
}

fn deck_color(d: usize) -> Color32 {
    theme::rgb(theme::ZONE_PALETTE[d % theme::ZONE_PALETTE.len()])
}

fn deck_name(d: usize) -> &'static str {
    ["A", "B", "C", "D"].get(d).copied().unwrap_or("?")
}

/// mm:ss.d
fn clock(ms: u64) -> String {
    format!("{:02}:{:02}.{}", ms / 60_000, ms / 1000 % 60, ms / 100 % 10)
}

/// m:ss
fn short(ms: u64) -> String {
    format!("{}:{:02}", ms / 60_000, ms / 1000 % 60)
}

fn label(text: &str, color: Color32) -> RichText {
    theme::heading_text(text).color(color)
}

impl View for SoundView {
    fn ui(&mut self, ui: &mut Ui, cx: &mut ViewCtx<'_>) {
        let mut board = cx.engine.show().sound.clone();
        board.normalize();
        let before = board.clone();
        let locked = cx.state.locked;
        let full = ui.available_rect_before_wrap();
        ui.allocate_rect(full, Sense::hover());
        let p = ui.painter().clone();
        p.rect_filled(full, 0.0, theme::surface());

        // Player oben
        let decks = board.decks.len().min(2);
        let top = Rect::from_min_size(full.min, vec2(full.width(), DECK_H));
        let w = top.width() / decks as f32;
        for d in 0..decks {
            let r =
                Rect::from_min_size(pos2(top.left() + w * d as f32, top.top()), vec2(w, DECK_H));
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
            deck_ui(&mut child, cx, &mut board, d, r);
            if d + 1 < decks {
                p.vline(r.right(), r.y_range(), Stroke::new(1.0, theme::border()));
            }
        }
        p.hline(
            full.x_range(),
            top.bottom(),
            Stroke::new(1.0, theme::border()),
        );

        // Überblenden + Master
        let bar = Rect::from_min_size(pos2(full.left(), top.bottom()), vec2(full.width(), BAR_H));
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(bar.shrink2(vec2(12.0, 0.0))));
        crossfade_bar(&mut child, &mut cx.state.sound, &mut board);
        p.hline(
            full.x_range(),
            bar.bottom(),
            Stroke::new(1.0, theme::border()),
        );

        // Playlisten + Pads
        let bottom = Rect::from_min_max(pos2(full.left(), bar.bottom() + 1.0), full.max);
        let cols = decks as f32 + 1.15;
        let cw = bottom.width() / cols;
        for d in 0..decks {
            let r = Rect::from_min_size(
                pos2(bottom.left() + cw * d as f32, bottom.top()),
                vec2(cw, bottom.height()),
            );
            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(r));
            self.playlist_ui(&mut child, cx, &mut board, d, r, locked);
            p.vline(r.right(), r.y_range(), Stroke::new(1.0, theme::border()));
        }
        let pads = Rect::from_min_max(
            pos2(bottom.left() + cw * decks as f32, bottom.top()),
            bottom.max,
        );
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(pads));
        pads_ui(&mut child, cx, &mut board, pads, locked);

        if board != before {
            cx.engine.show_mut().sound = board;
        }
        if cx.state.sound.any_running() {
            ui.ctx().request_repaint();
        }
    }
}

/// Ein Player: Kopf, Wellenform, Zeiten, Bedienung, Lautstärke.
fn deck_ui(ui: &mut Ui, cx: &mut ViewCtx<'_>, board: &mut SoundBoard, d: usize, rect: Rect) {
    let color = deck_color(d);
    let sp = &mut cx.state.sound;
    let state = sp.state(d);
    let track: Option<Track> = sp
        .current(d)
        .and_then(|i| board.decks[d].playlist.get(i).cloned());
    let wave = track
        .as_ref()
        .map(|t| cx.state.audio.wave(&t.path, ui.ctx()));
    let analysis = match &wave {
        Some(WaveState::Ready(a)) => Some(a.clone()),
        _ => None,
    };
    let len = track
        .as_ref()
        .map(|t| t.length_ms)
        .filter(|l| *l > 0)
        .or(analysis.as_ref().map(|a| a.duration_ms))
        .unwrap_or(0);
    let pos = sp.pos_ms(d);
    let p = ui.painter().clone();
    let vol_w = 52.0;
    // Lautstärke zur Mitte hin: bei A rechts, bei B links
    let vol_left = d % 2 == 1;
    let (main_l, main_r) = if vol_left {
        (rect.left() + vol_w, rect.right())
    } else {
        (rect.left(), rect.right() - vol_w)
    };
    let main = Rect::from_min_max(pos2(main_l, rect.top()), pos2(main_r, rect.bottom()))
        .shrink2(vec2(12.0, 8.0));
    let mut ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(main)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );
    let ui = &mut ui;
    ui.spacing_mut().item_spacing = vec2(theme::SP_XS, theme::SP_S);

    // Kopf
    ui.horizontal(|ui| {
        ui.set_height(24.0);
        ui.label(label(&tr!("PLAYER {d}", d = deck_name(d)), color));
        ui.label(RichText::new("|").color(theme::border()));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let deck = &mut board.decks[d];
            let r = ui
                .add(
                    IconButton::new(Icon::FollowRight)
                        .active(deck.auto_next)
                        .tint(color),
                )
                .on_hover_text(t("Automatisch weiter"));
            if r.clicked() {
                deck.auto_next = !deck.auto_next;
            }
            let r = ui
                .add(
                    IconButton::new(Icon::Repeat)
                        .active(deck.repeat)
                        .tint(color),
                )
                .on_hover_text(t("Titel wiederholen"));
            if r.clicked() {
                deck.repeat = !deck.repeat;
            }
            if let Some(g) = analysis.as_ref().and_then(|a| a.grid) {
                ui.label(
                    RichText::new(tr!("{bpm} BPM", bpm = format!("{:.1}", g.bpm)))
                        .font(theme::caption())
                        .color(theme::text_dim()),
                );
            }
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                let name = track.as_ref().map(|t| t.name()).unwrap_or_default();
                ui.add(egui::Label::new(RichText::new(name).size(14.0)).truncate());
            });
        });
    });

    // Wellenform
    let (wr, resp) =
        ui.allocate_exact_size(vec2(ui.available_width(), 96.0), Sense::click_and_drag());
    p.rect_filled(wr, CornerRadius::same(theme::R_S), theme::bg());
    if let Some(a) = &analysis {
        let bars = ((wr.width() - 4.0) / 3.0).max(1.0) as usize;
        let n = a.peaks.len().max(1);
        let played = if len > 0 {
            pos as f32 / len as f32
        } else {
            0.0
        };
        for b in 0..bars {
            let s = b * n / bars;
            let e = ((b + 1) * n / bars).max(s + 1).min(n);
            let peak = a.peaks[s.min(n - 1)..e].iter().copied().max().unwrap_or(0) as f32 / 255.0;
            let h = (peak * (wr.height() - 8.0)).max(2.0);
            let x = wr.left() + 2.0 + b as f32 * 3.0;
            let c = if (b as f32 / bars as f32) < played {
                color
            } else {
                theme::fixture_bar()
            };
            p.rect_filled(
                Rect::from_center_size(pos2(x + 1.0, wr.center().y), vec2(2.0, h)),
                1.0,
                c,
            );
        }
    } else if matches!(wave, Some(WaveState::Loading)) {
        ui.ctx()
            .request_repaint_after(std::time::Duration::from_millis(200));
    }
    if len > 0 {
        let x = wr.left() + wr.width() * (pos as f32 / len as f32).clamp(0.0, 1.0);
        p.vline(x, wr.y_range(), Stroke::new(2.0, theme::text()));
        let cue = sp.cue_ms(d);
        if cue > 0 {
            let cx_ = wr.left() + wr.width() * (cue as f32 / len as f32).clamp(0.0, 1.0);
            p.add(egui::Shape::convex_polygon(
                vec![
                    pos2(cx_ - 5.0, wr.top()),
                    pos2(cx_ + 5.0, wr.top()),
                    pos2(cx_, wr.top() + 7.0),
                ],
                theme::WARN,
                Stroke::NONE,
            ));
        }
        if let Some(at) = resp.interact_pointer_pos() {
            if resp.clicked() || resp.dragged() {
                let f = ((at.x - wr.left()) / wr.width()).clamp(0.0, 1.0);
                sp.seek(d, (f * len as f32) as u64);
            }
        }
    }

    // Zeiten
    ui.horizontal(|ui| {
        let elapsed_color = if state == DeckState::Ready || state == DeckState::Empty {
            theme::text_dim()
        } else {
            color
        };
        ui.label(RichText::new(clock(pos)).size(26.0).color(elapsed_color));
        let remain = len.saturating_sub(pos);
        let rc = if matches!(state, DeckState::Playing | DeckState::Fading)
            && len > 0
            && remain < 10_000
        {
            theme::WARN
        } else {
            theme::text_dim()
        };
        ui.label(
            RichText::new(format!("-{}", clock(remain)))
                .size(15.0)
                .color(rc),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let (txt, c) = match state {
                DeckState::Empty => (String::new(), theme::text_dim()),
                DeckState::Ready => (t("BEREIT").to_string(), theme::text_dim()),
                DeckState::Playing => (t("LÄUFT").to_string(), color),
                DeckState::Paused => (t("PAUSE").to_string(), theme::text_dim()),
                DeckState::Ended => (t("ENDE").to_string(), theme::text_dim()),
                DeckState::Fading => (
                    tr!(
                        "BLENDET AUS · {s} s",
                        s = format!("{:.1}", sp.fade_left(d).unwrap_or(0.0))
                    ),
                    theme::WARN,
                ),
            };
            ui.label(label(&txt, c));
        });
    });

    // Bedienung
    ui.horizontal(|ui| {
        let key = |k: &str| format!("sound/deck/{d}/{k}");
        let name = |what: &str| {
            let what = what.to_string();
            move || {
                (
                    t("Sound").to_string(),
                    tr!("Player {d}: {what}", d = deck_name(d), what = what),
                )
            }
        };
        let r = ui
            .add(IconButton::new(Icon::Prev).size(40.0))
            .on_hover_text(t("Zum Anfang"));
        if r.clicked() | crate::mapping::button(ui, &r, &key("start"), false, name(t("Zum Anfang")))
        {
            sp.rewind(d);
        }
        let playing = matches!(state, DeckState::Playing | DeckState::Fading);
        let r = ui
            .add(
                IconButton::new(if playing { Icon::Pause } else { Icon::Play })
                    .size(40.0)
                    .active(playing)
                    .tint(color),
            )
            .on_hover_text(t("Abspielen / Pause"));
        if r.clicked()
            | crate::mapping::button(ui, &r, &key("play"), playing, name(t("Abspielen / Pause")))
        {
            sp.play_pause(board, d);
        }
        let r = ui
            .add(IconButton::new(Icon::Stop).size(40.0))
            .on_hover_text(t("Stopp"));
        if r.clicked() | crate::mapping::button(ui, &r, &key("stop"), false, name(t("Stopp"))) {
            sp.stop(d);
        }
        let r = ui
            .add_sized(
                vec2(52.0, 40.0),
                egui::Button::new(theme::heading_text("CUE")),
            )
            .on_hover_text(t("Cue-Marke setzen bzw. zur Cue-Marke springen"));
        if r.clicked() | crate::mapping::button(ui, &r, &key("cue"), false, name("CUE")) {
            sp.cue(d);
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut secs = board.decks[d].fade_ms as f32 / 1000.0;
            if ui
                .add(
                    egui::DragValue::new(&mut secs)
                        .range(0.1..=30.0)
                        .speed(0.1)
                        .max_decimals(1)
                        .suffix(" s"),
                )
                .changed()
            {
                board.decks[d].fade_ms = (secs * 1000.0) as u32;
            }
            let r = ui.add_sized(vec2(96.0, 40.0), egui::Button::new(t("Ausfaden")));
            if r.clicked()
                | crate::mapping::button(
                    ui,
                    &r,
                    &key("fade"),
                    state == DeckState::Fading,
                    name(t("Ausfaden")),
                )
            {
                sp.fade_out(board, d);
            }
        });
    });

    // Lautstärke
    let vr = if vol_left {
        Rect::from_min_max(rect.min, pos2(rect.left() + vol_w, rect.bottom()))
    } else {
        Rect::from_min_max(pos2(rect.right() - vol_w, rect.top()), rect.max)
    };
    let edge = if vol_left { vr.right() } else { vr.left() };
    p.vline(edge, vr.y_range(), Stroke::new(1.0, theme::border()));
    let vol = &mut board.decks[d].volume;
    let db = if *vol <= 0.001 {
        t("aus").to_string()
    } else {
        format!("{:.0} dB", 20.0 * vol.log10())
    };
    p.text(
        pos2(vr.center().x, vr.top() + 12.0),
        Align2::CENTER_CENTER,
        db,
        theme::caption(),
        theme::text_dim(),
    );
    p.text(
        pos2(vr.center().x, vr.bottom() - 12.0),
        Align2::CENTER_CENTER,
        deck_name(d),
        theme::heading_font(),
        color,
    );
    let track_r = Rect::from_min_max(
        pos2(vr.center().x - 11.0, vr.top() + 26.0),
        pos2(vr.center().x + 11.0, vr.bottom() - 26.0),
    );
    vertical_slider(ui, track_r, vol, color, ("sound/deck/vol", d));
}

/// Senkrechter Lautstärkeregler (0–1).
fn vertical_slider(ui: &mut Ui, r: Rect, v: &mut f32, color: Color32, id: (&str, usize)) {
    let resp = ui.interact(
        r.expand(4.0),
        ui.scope_id().with(id),
        Sense::click_and_drag(),
    );
    if let Some(p) = resp.interact_pointer_pos() {
        if resp.dragged() || resp.clicked() {
            *v = ((r.bottom() - p.y) / r.height()).clamp(0.0, 1.0);
        }
    }
    if resp.double_clicked() {
        *v = 0.85;
    }
    let painter = ui.painter();
    painter.rect_filled(r, CornerRadius::same(theme::R_S), theme::bg());
    let y = r.bottom() - r.height() * *v;
    painter.rect_filled(
        Rect::from_min_max(pos2(r.left(), y), r.max),
        CornerRadius::same(theme::R_S),
        color.gamma_multiply(0.35),
    );
    painter.rect_filled(
        Rect::from_center_size(pos2(r.center().x, y), vec2(r.width() + 6.0, 6.0)),
        2.0,
        theme::handle(),
    );
}

/// Waagrechter Regler (0–1).
fn horizontal_slider(
    ui: &mut Ui,
    width: f32,
    v: &mut f32,
    fill: Color32,
    center: bool,
) -> egui::Response {
    let (r, resp) = ui.allocate_exact_size(vec2(width, 24.0), Sense::click_and_drag());
    let track = Rect::from_center_size(r.center(), vec2(r.width() - 14.0, 6.0));
    if let Some(p) = resp.interact_pointer_pos() {
        if resp.dragged() || resp.clicked() {
            *v = ((p.x - track.left()) / track.width()).clamp(0.0, 1.0);
        }
    }
    let painter = ui.painter();
    painter.rect_filled(track, 3.0, theme::bg());
    let x = track.left() + track.width() * *v;
    let from = if center {
        track.center().x
    } else {
        track.left()
    };
    painter.rect_filled(
        Rect::from_min_max(
            pos2(from.min(x), track.top()),
            pos2(from.max(x), track.bottom()),
        ),
        3.0,
        fill,
    );
    painter.rect_filled(
        Rect::from_center_size(pos2(x, r.center().y), vec2(12.0, 22.0)),
        3.0,
        theme::handle(),
    );
    resp
}

fn crossfade_bar(ui: &mut Ui, sp: &mut SoundPlayer, board: &mut SoundBoard) {
    ui.horizontal_centered(|ui| {
        ui.spacing_mut().item_spacing.x = theme::SP_M;
        ui.label(label("A", deck_color(0)));
        let mut x = sp.xfade;
        let r = horizontal_slider(ui, 360.0, &mut x, theme::accent_dim(), true);
        if r.double_clicked() {
            x = 0.5;
        }
        if (x - sp.xfade).abs() > f32::EPSILON && !sp.xfading() {
            sp.xfade = x;
        }
        ui.label(label("B", deck_color(1)));
        let to = deck_name(sp.xfade_target());
        let r = ui.add_sized(
            vec2(170.0, 28.0),
            egui::Button::new(tr!("Überblenden zu {dir}", dir = to)),
        );
        if r.clicked()
            | crate::mapping::button(ui, &r, "sound/xfade", sp.xfading(), || {
                (t("Sound").to_string(), t("Überblenden").to_string())
            })
        {
            sp.crossfade(board);
        }
        let mut secs = board.crossfade_ms as f32 / 1000.0;
        if ui
            .add(
                egui::DragValue::new(&mut secs)
                    .range(0.5..=30.0)
                    .speed(0.1)
                    .max_decimals(1)
                    .suffix(" s"),
            )
            .changed()
        {
            board.crossfade_ms = (secs * 1000.0) as u32;
        }
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                RichText::new(format!("{:.0} %", board.master * 100.0))
                    .font(theme::caption())
                    .color(theme::text_dim()),
            );
            horizontal_slider(ui, 160.0, &mut board.master, theme::accent(), false);
            ui.label(RichText::new(t("Master")).color(theme::text_dim()));
        });
    });
}

impl SoundView {
    fn playlist_ui(
        &mut self,
        ui: &mut Ui,
        cx: &mut ViewCtx<'_>,
        board: &mut SoundBoard,
        d: usize,
        rect: Rect,
        locked: bool,
    ) {
        let color = deck_color(d);
        let p = ui.painter().clone();
        let head = Rect::from_min_size(rect.min, vec2(rect.width(), 32.0));
        p.hline(
            rect.x_range(),
            head.bottom(),
            Stroke::new(1.0, theme::border()),
        );
        let mut hui = ui.new_child(egui::UiBuilder::new().max_rect(head.shrink2(vec2(8.0, 0.0))));
        hui.horizontal_centered(|ui| {
            ui.label(label(&tr!("PLAYLIST {d}", d = deck_name(d)), color));
            let list = &board.decks[d].playlist;
            let total: u64 = list.iter().map(|t| track_len(cx, t, ui.ctx())).sum();
            ui.label(
                RichText::new(tr!("{n} Titel · {len}", n = list.len(), len = short(total)))
                    .font(theme::caption())
                    .color(theme::text_dim()),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui
                    .add(IconButton::new(Icon::Plus).flat().enabled(!locked))
                    .on_hover_text(t("Dateien hinzufügen …"))
                    .clicked()
                {
                    if let Some(files) = rfd::FileDialog::new()
                        .add_filter(t("Audio"), &AUDIO_EXT)
                        .pick_files()
                    {
                        for f in files {
                            board.decks[d].playlist.push(Track {
                                path: f.to_string_lossy().into_owned(),
                                length_ms: 0,
                            });
                        }
                    }
                }
                if ui
                    .add(IconButton::new(Icon::FileOpen).flat().enabled(!locked))
                    .on_hover_text(t("Ordner hinzufügen …"))
                    .clicked()
                {
                    if let Some(dir) = rfd::FileDialog::new().pick_folder() {
                        board.decks[d].playlist.extend(audio_files(&dir));
                    }
                }
            });
        });

        let body = Rect::from_min_max(pos2(rect.left(), head.bottom() + 1.0), rect.max).shrink(4.0);
        let body_resp = ui.interact(body, ui.scope_id().with(("pl_drop", d)), Sense::hover());
        // Dateien auf die Liste ziehen
        if !locked && body_resp.contains_pointer() {
            let dropped = ui.input(|i| i.raw.dropped_files.clone());
            for f in dropped {
                let path = f.path().to_path_buf();
                if !path.as_os_str().is_empty() {
                    if path.is_dir() {
                        board.decks[d].playlist.extend(audio_files(&path));
                    } else {
                        board.decks[d].playlist.push(Track {
                            path: path.to_string_lossy().into_owned(),
                            length_ms: 0,
                        });
                    }
                }
            }
        }
        let mut bui = ui.new_child(egui::UiBuilder::new().max_rect(body));
        let sp = &mut cx.state.sound;
        let current = sp.current(d);
        let next = if matches!(
            sp.state(d),
            DeckState::Playing | DeckState::Fading | DeckState::Paused
        ) {
            current.map(|c| c + 1)
        } else {
            None
        };
        let mut action: Option<(usize, &str)> = None;
        egui::ScrollArea::vertical()
            .id_salt(("playlist", d))
            .auto_shrink([false, false])
            .show(&mut bui, |ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                let lens: Vec<u64> = board.decks[d]
                    .playlist
                    .iter()
                    .map(|t| track_len_audio(&cx.state.audio, t, ui.ctx()))
                    .collect();
                for (i, tr_) in board.decks[d].playlist.iter().enumerate() {
                    let (r, resp) =
                        ui.allocate_exact_size(vec2(ui.available_width(), 32.0), Sense::click());
                    let now = current == Some(i);
                    let bg = if now {
                        color.gamma_multiply(0.18)
                    } else if resp.hovered() {
                        theme::raised_hover()
                    } else {
                        theme::raised()
                    };
                    let painter = ui.painter();
                    painter.rect_filled(r, CornerRadius::same(theme::R_S), bg);
                    if now {
                        painter.rect_stroke(
                            r,
                            CornerRadius::same(theme::R_S),
                            Stroke::new(1.5, color),
                            StrokeKind::Inside,
                        );
                    } else if next == Some(i) {
                        painter.rect_stroke(
                            r,
                            CornerRadius::same(theme::R_S),
                            Stroke::new(1.0, color.gamma_multiply(0.5)),
                            StrokeKind::Inside,
                        );
                    } else if self.sel[d] == Some(i) {
                        painter.rect_stroke(
                            r,
                            CornerRadius::same(theme::R_S),
                            Stroke::new(1.0, theme::accent()),
                            StrokeKind::Inside,
                        );
                    }
                    let fg = if sp.played(d, i) && !now {
                        theme::text_faint()
                    } else {
                        theme::text()
                    };
                    painter.text(
                        pos2(r.left() + 26.0, r.center().y),
                        Align2::RIGHT_CENTER,
                        (i + 1).to_string(),
                        theme::caption(),
                        theme::text_dim(),
                    );
                    let len = if lens[i] > 0 {
                        short(lens[i])
                    } else {
                        String::new()
                    };
                    painter.text(
                        pos2(r.right() - 8.0, r.center().y),
                        Align2::RIGHT_CENTER,
                        len,
                        theme::body(),
                        fg,
                    );
                    let name_r = Rect::from_min_max(
                        pos2(r.left() + 34.0, r.top()),
                        pos2(r.right() - 56.0, r.bottom()),
                    );
                    let missing = !std::path::Path::new(&tr_.path).exists();
                    let name_c = if missing { theme::DANGER } else { fg };
                    let galley = painter.layout(
                        tr_.name(),
                        FontId::proportional(13.0),
                        name_c,
                        f32::INFINITY,
                    );
                    let clip = painter.with_clip_rect(name_r);
                    clip.galley(
                        pos2(name_r.left(), r.center().y - galley.size().y / 2.0),
                        galley,
                        name_c,
                    );
                    let resp = if missing {
                        resp.on_hover_text(t("Datei fehlt"))
                    } else {
                        resp
                    };
                    if resp.clicked() {
                        self.sel[d] = Some(i);
                    }
                    if resp.double_clicked() {
                        action = Some((i, "load"));
                    }
                    menu::context(&resp, |ui| {
                        if menu::item(ui, MenuItem::new(t("Laden")).icon(Icon::FileOpen)) {
                            action = Some((i, "load"));
                        }
                        if menu::item(ui, MenuItem::new(t("Abspielen")).icon(Icon::Play)) {
                            action = Some((i, "play"));
                        }
                        menu::separator(ui);
                        if menu::item(
                            ui,
                            MenuItem::new(t("Nach oben"))
                                .icon(Icon::ChevronUp)
                                .enabled(!locked && i > 0),
                        ) {
                            action = Some((i, "up"));
                        }
                        if menu::item(
                            ui,
                            MenuItem::new(t("Nach unten"))
                                .icon(Icon::ChevronDown)
                                .enabled(!locked),
                        ) {
                            action = Some((i, "down"));
                        }
                        menu::separator(ui);
                        if menu::item(
                            ui,
                            MenuItem::new(t("Entfernen"))
                                .icon(Icon::Trash)
                                .danger()
                                .enabled(!locked),
                        ) {
                            action = Some((i, "remove"));
                        }
                    });
                }
            });
        if let Some((i, what)) = action {
            let list = &mut board.decks[d].playlist;
            match what {
                "load" => sp.load(board, d, i),
                "play" => sp.play_track(board, d, i),
                "up" if i > 0 => list.swap(i, i - 1),
                "down" if i + 1 < list.len() => list.swap(i, i + 1),
                "remove" => {
                    list.remove(i);
                    if current == Some(i) {
                        sp.stop(d);
                    }
                    self.sel[d] = None;
                }
                _ => {}
            }
        }
    }
}

/// Länge eines Titels: gespeichert oder aus der (zwischengespeicherten)
/// Analyse, die dafür bei Bedarf im Hintergrund startet.
fn track_len_audio(audio: &crate::audio::AudioPlayer, t: &Track, ctx: &egui::Context) -> u64 {
    if t.length_ms > 0 {
        return t.length_ms;
    }
    match audio.wave(&t.path, ctx) {
        WaveState::Ready(a) => a.duration_ms,
        _ => 0,
    }
}

fn track_len(cx: &ViewCtx<'_>, t: &Track, ctx: &egui::Context) -> u64 {
    track_len_audio(&cx.state.audio, t, ctx)
}

/// Audiodateien eines Ordners, nach Namen sortiert.
fn audio_files(dir: &std::path::Path) -> Vec<Track> {
    let mut files: Vec<_> = std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| AUDIO_EXT.contains(&e.to_lowercase().as_str()))
        })
        .collect();
    files.sort();
    files
        .into_iter()
        .map(|p| Track {
            path: p.to_string_lossy().into_owned(),
            length_ms: 0,
        })
        .collect()
}

/// Jingle-Pads (3 × 4).
fn pads_ui(ui: &mut Ui, cx: &mut ViewCtx<'_>, board: &mut SoundBoard, rect: Rect, locked: bool) {
    let p = ui.painter().clone();
    let head = Rect::from_min_size(rect.min, vec2(rect.width(), 32.0));
    p.hline(
        rect.x_range(),
        head.bottom(),
        Stroke::new(1.0, theme::border()),
    );
    let mut hui = ui.new_child(egui::UiBuilder::new().max_rect(head.shrink2(vec2(8.0, 0.0))));
    hui.horizontal_centered(|ui| {
        ui.label(label(t("PADS"), theme::rgb(theme::ZONE_PALETTE[2])));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let r = ui
                .add(IconButton::new(Icon::Stop).flat())
                .on_hover_text(t("Alle Pads stoppen"));
            if r.clicked()
                | crate::mapping::button(ui, &r, "sound/pads/stop", false, || {
                    (t("Sound").to_string(), t("Alle Pads stoppen").to_string())
                })
            {
                cx.state.sound.stop_pads();
            }
        });
    });
    let body = Rect::from_min_max(pos2(rect.left(), head.bottom() + 1.0), rect.max).shrink(4.0);
    let (cols, rows) = (3usize, board.pads.len().div_ceil(3).max(1));
    let gap = 4.0;
    let cw = (body.width() - gap * (cols as f32 - 1.0)) / cols as f32;
    let ch = (body.height() - gap * (rows as f32 - 1.0)) / rows as f32;
    let dropped = ui.input(|i| i.raw.dropped_files.clone());
    let hover = ui.input(|i| i.pointer.hover_pos());
    for i in 0..board.pads.len() {
        let (c, r_) = (i % cols, i / cols);
        let r = Rect::from_min_size(
            pos2(
                body.left() + c as f32 * (cw + gap),
                body.top() + r_ as f32 * (ch + gap),
            ),
            vec2(cw, ch),
        );
        let resp = ui.interact(r, ui.scope_id().with(("pad", i)), Sense::click());
        let pad = board.pads[i].clone();
        let empty = pad.path.is_empty();
        let missing = !empty && !std::path::Path::new(&pad.path).exists();
        let pc = pad
            .color
            .map_or_else(|| theme::rgb(theme::ZONE_PALETTE[(i + 2) % 8]), theme::rgb);
        let pad_len = if empty {
            0
        } else {
            track_len_audio(
                &cx.state.audio,
                &Track {
                    path: pad.path.clone(),
                    length_ms: pad.length_ms,
                },
                ui.ctx(),
            )
        };
        let progress = cx.state.sound.pad_progress(i, pad_len);
        let painter = ui.painter();
        if empty {
            painter.rect_filled(r, CornerRadius::same(theme::R_S), theme::surface());
            painter.rect_stroke(
                r,
                CornerRadius::same(theme::R_S),
                Stroke::new(1.0, theme::border()),
                StrokeKind::Inside,
            );
            painter.text(
                r.left_top() + vec2(8.0, 8.0),
                Align2::LEFT_TOP,
                t("Leer"),
                theme::body(),
                theme::text_faint(),
            );
        } else {
            let bg = if resp.hovered() {
                theme::raised_hover()
            } else {
                theme::raised()
            };
            painter.rect_filled(r, CornerRadius::same(theme::R_S), bg);
            if let Some(f) = progress {
                painter.rect_filled(
                    Rect::from_min_size(r.min, vec2(r.width() * f, r.height())),
                    CornerRadius::same(theme::R_S),
                    pc.gamma_multiply(0.25),
                );
                painter.rect_stroke(
                    r,
                    CornerRadius::same(theme::R_S),
                    Stroke::new(1.5, pc),
                    StrokeKind::Inside,
                );
            } else if missing {
                painter.rect_stroke(
                    r,
                    CornerRadius::same(theme::R_S),
                    Stroke::new(1.0, theme::DANGER),
                    StrokeKind::Inside,
                );
            }
            painter.rect_filled(
                Rect::from_min_size(r.left_top() + vec2(8.0, 12.0), vec2(8.0, 8.0)),
                2.0,
                pc,
            );
            let name_r = Rect::from_min_max(
                r.left_top() + vec2(22.0, 4.0),
                pos2(r.right() - 6.0, r.top() + 28.0),
            );
            let galley =
                painter.layout_no_wrap(pad.label(), FontId::proportional(13.0), theme::text());
            painter.with_clip_rect(name_r).galley(
                pos2(name_r.left(), r.top() + 16.0 - galley.size().y / 2.0),
                galley,
                theme::text(),
            );
            let (status, sc) = if missing {
                (t("Datei fehlt").to_string(), theme::DANGER)
            } else if progress.is_some() {
                (t("LÄUFT").to_string(), pc)
            } else if pad.repeat {
                (t("Schleife").to_string(), theme::text_dim())
            } else {
                (String::new(), theme::text_dim())
            };
            painter.text(
                r.left_bottom() + vec2(8.0, -8.0),
                Align2::LEFT_BOTTOM,
                status,
                theme::caption(),
                sc,
            );
            if pad.length_ms > 0 {
                painter.text(
                    r.right_bottom() + vec2(-8.0, -8.0),
                    Align2::RIGHT_BOTTOM,
                    short(pad.length_ms),
                    theme::caption(),
                    theme::text_dim(),
                );
            }
        }
        let key = format!("sound/pad/{i}");
        let label_ = pad.label();
        let mapped = crate::mapping::button(ui, &resp, &key, progress.is_some(), || {
            (
                t("Sound").to_string(),
                tr!("Pad {n}: {name}", n = i + 1, name = label_),
            )
        });
        if resp.clicked() || mapped {
            if empty && !locked && resp.clicked() {
                if let Some(f) = rfd::FileDialog::new()
                    .add_filter(t("Audio"), &AUDIO_EXT)
                    .pick_file()
                {
                    board.pads[i].path = f.to_string_lossy().into_owned();
                    board.pads[i].length_ms = 0;
                }
            } else {
                cx.state.sound.pad(board, i);
            }
        }
        // Datei auf ein Pad ziehen
        if !locked && hover.is_some_and(|h| r.contains(h)) {
            if let Some(path) = dropped
                .iter()
                .map(|f| f.path().to_path_buf())
                .find(|p| !p.as_os_str().is_empty())
            {
                board.pads[i].path = path.to_string_lossy().into_owned();
                board.pads[i].length_ms = 0;
            }
        }
        menu::context(&resp, |ui| {
            let pad = &mut board.pads[i];
            if menu::item(
                ui,
                MenuItem::new(t("Datei wählen …"))
                    .icon(Icon::FileOpen)
                    .enabled(!locked),
            ) {
                if let Some(f) = rfd::FileDialog::new()
                    .add_filter(t("Audio"), &AUDIO_EXT)
                    .pick_file()
                {
                    pad.path = f.to_string_lossy().into_owned();
                    pad.length_ms = 0;
                }
            }
            ui.add_enabled_ui(!locked, |ui| {
                ui.horizontal(|ui| {
                    ui.add_space(theme::SP_S);
                    ui.label(RichText::new(t("Name")).color(theme::text_dim()));
                    ui.add(
                        egui::TextEdit::singleline(&mut pad.name)
                            .hint_text(dmxcraft_core::sound::file_name(&pad.path))
                            .desired_width(150.0),
                    );
                });
                ui.horizontal(|ui| {
                    ui.add_space(theme::SP_S);
                    ui.label(RichText::new(t("Pegel")).color(theme::text_dim()));
                    let mut v = pad.volume;
                    horizontal_slider(ui, 150.0, &mut v, theme::accent(), false);
                    pad.volume = v;
                    ui.label(
                        RichText::new(format!("{:.0} %", pad.volume * 100.0))
                            .font(theme::caption())
                            .color(theme::text_dim()),
                    );
                });
                if let Some(c) = color_swatches(
                    ui,
                    ui.make_persistent_id(("pad_color", i)),
                    pad.color,
                    t("Automatisch"),
                ) {
                    pad.color = c;
                }
            });
            menu::separator(ui);
            if menu::item(
                ui,
                MenuItem::new(t("Wiederholen"))
                    .checked(pad.repeat)
                    .enabled(!locked),
            ) {
                pad.repeat = !pad.repeat;
            }
            if menu::item(
                ui,
                MenuItem::new(t("Andere Pads stoppen"))
                    .checked(pad.exclusive)
                    .enabled(!locked),
            ) {
                pad.exclusive = !pad.exclusive;
            }
            menu::separator(ui);
            if menu::item(
                ui,
                MenuItem::new(t("Leeren"))
                    .icon(Icon::Trash)
                    .danger()
                    .enabled(!locked && !pad.path.is_empty()),
            ) {
                *pad = dmxcraft_core::sound::Pad::default();
            }
        });
    }
}

/// Zugeordnete Tasten/MIDI auch auswerten, wenn der Reiter nicht offen ist.
pub fn poll(ctx: &egui::Context, board: &SoundBoard, sp: &mut SoundPlayer) {
    use crate::mapping::poll as fired;
    for d in 0..board.decks.len().min(2) {
        let state = sp.state(d);
        let playing = matches!(state, DeckState::Playing | DeckState::Fading);
        if fired(ctx, &format!("sound/deck/{d}/play"), playing) {
            sp.play_pause(board, d);
        }
        if fired(ctx, &format!("sound/deck/{d}/stop"), false) {
            sp.stop(d);
        }
        if fired(ctx, &format!("sound/deck/{d}/start"), false) {
            sp.rewind(d);
        }
        if fired(ctx, &format!("sound/deck/{d}/cue"), false) {
            sp.cue(d);
        }
        if fired(
            ctx,
            &format!("sound/deck/{d}/fade"),
            state == DeckState::Fading,
        ) {
            sp.fade_out(board, d);
        }
    }
    if fired(ctx, "sound/xfade", sp.xfading()) {
        sp.crossfade(board);
    }
    if fired(ctx, "sound/pads/stop", false) {
        sp.stop_pads();
    }
    for i in 0..board.pads.len() {
        let running = sp.pad_progress(i, 1).is_some();
        if fired(ctx, &format!("sound/pad/{i}"), running) {
            sp.pad(board, i);
        }
    }
}
