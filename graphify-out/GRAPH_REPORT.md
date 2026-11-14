# Graph Report - dmxcraft-src  (2026-11-14)

## Corpus Check
- 168 files · ~280,024 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4789 nodes · 12245 edges · 330 communities (167 shown, 163 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 665 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b59a2f0c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- DmxOutput
- open_dmx.rs
- Venue
- programmer.rs
- SoundBoard
- steps
- fx.rs
- autosave.rs
- props.rs
- ChannelRef
- mapping/mod.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- Control
- Serialize
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Mapping
- fx_ui.rs
- Host
- ofl/src/lib.rs
- missing_audio.rs
- TouchView
- dmxcraft
- ChannelDef
- t
- menu.rs
- DMXCraft – UI guidelines
- system_fonts.rs
- FixtureAction
- scene_grid.rs
- file_dialog.rs
- app/src/audio.rs
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- Ui
- crash.rs
- remote/mod.rs
- TouchLayout
- Timeline
- control/timeline.rs
- steps
- window.rs
- stage.rs
- runtime.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
- theme.rs
- PaletteStore
- drag_number
- ShutterEffect
- profile.rs
- VizView
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- .end_frame
- MapPreset
- audio_triggers.rs
- qxf.rs
- show.rs
- dialog.rs
- mcp/src/profiles.rs
- core/src/venue.rs
- arc
- Placement
- Dialog<'a>
- fx/mapping.rs
- duration
- snap.rs
- Object
- TouchKind
- outline.rs
- .update
- convert.rs
- app/src/obs.rs
- stage_editor.rs
- Layer
- Obs
- global.rs
- IconButton
- CurveShape
- ColorPreset
- qlc-bundle.rs
- measure.rs
- i18n/mod.rs
- mcp/src/venue.rs
- egui
- stage_canvas.rs
- Deserialize
- Clip
- DeviceModel
- .ui
- Theme
- assist.rs
- FixtureType
- pathbuf
- OrbitCamera
- text_dim
- ProfileBuilder
- Category
- UpdateState
- text_field
- gltf_import.rs
- Profile
- FxHasher
- View
- ViewCtx
- ev2.rs
- Problem
- server.rs
- mcp.rs
- ShowError
- scene_card.rs
- Snapshot
- protocol.rs
- midi.rs
- tools.rs
- OpenDmx
- ClipAction
- control/tracking.rs
- bridge.rs
- fader.rs
- scene_props.rs
- Mode
- .push
- MenuItem<'a>
- Transform
- ButtonStyle
- scene_group.rs
- DmxFrame
- ProgrammerMode
- core/src/lib.rs
- env
- de.rs
- menu.rs
- Knob<'a>
- viz/calibrate.rs
- Step
- build-local-artifacts
- ClipContent
- lock.rs
- AppState
- DeckState
- plan.rs
- autosave.rs
- OutputStatus
- FooterButton<'a>
- views/sound.rs
- GH_TOKEN
- HeaderAction
- time_field.rs
- release.ps1
- desktop.rs
- Layer
- pointer
- video_out.rs
- release-install.md
- dip.rs
- Tab
- Ui
- with
- views/touch.rs
- views/touch.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 267 edges
2. `t()` - 244 edges
3. `Show` - 183 edges
4. `AppState` - 124 edges
5. `Icon` - 119 edges
6. `ViewCtx` - 100 edges
7. `ChannelKind` - 79 edges
8. `ChannelRef` - 65 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `angle_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `grid_label()` --calls--> `t()`  [INFERRED]
  crates/app/src/viz/snap.rs → crates/app/src/i18n/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (330 total, 163 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.09
Nodes (21): LibraryDrag, BuilderRequest, Edit, New, Open, Template, pick_export(), pick_import() (+13 more)

### Community 2 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Venue"
Cohesion: 0.11
Nodes (12): load_file(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Texture, mul() (+4 more)

### Community 5 - "programmer.rs"
Cohesion: 0.17
Nodes (25): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+17 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (24): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+16 more)

### Community 9 - "autosave.rs"
Cohesion: 0.30
Nodes (3): AutoSave, now_secs(), RecoveryInfo

### Community 10 - "props.rs"
Cohesion: 0.11
Nodes (21): fold_section(), apply_placement(), renumber(), apply_texture(), TexAction, Builtin, Load, Set (+13 more)

### Community 11 - "ChannelRef"
Cohesion: 0.06
Nodes (10): Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, SceneProgress, Track, TrackLight, FastSet (+2 more)

### Community 12 - "mapping/mod.rs"
Cohesion: 0.13
Nodes (33): alt_click(), bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fader(), fader_channels() (+25 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.05
Nodes (62): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+54 more)

### Community 20 - "Control"
Cohesion: 0.08
Nodes (24): Overlay, Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+16 more)

### Community 21 - "Serialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.08
Nodes (34): analyze(), AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid() (+26 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (30): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+22 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "Mapping"
Cohesion: 0.10
Nodes (24): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback (+16 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.10
Nodes (34): rgb(), zone_color(), canvas_swatches(), color_picker(), color_swatches(), swatches(), swatches_row(), color_fx_ui() (+26 more)

### Community 29 - "Host"
Cohesion: 0.16
Nodes (10): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+2 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 32 - "TouchView"
Cohesion: 0.16
Nodes (11): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+3 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "ChannelDef"
Cohesion: 0.12
Nodes (13): ChannelDef, ChannelRange, open_value_takes_the_highest_open_range(), RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (36): t(), INPUT_H, text_field(), kind_color(), capability_fields(), card_row(), category_label(), check() (+28 more)

### Community 37 - "menu.rs"
Cohesion: 0.15
Nodes (15): begin(), context(), dropdown(), item(), separator(), pct(), apply_trigger_pick(), trigger_cx() (+7 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.06
Nodes (54): CARD_H, COMPACT_H, Segmented<'a, T>, programmer_values(), Action, Bank, Compact, CreateSuper (+46 more)

### Community 43 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 44 - "app/src/audio.rs"
Cohesion: 0.08
Nodes (23): analysis(), analyze_file(), AudioPlayer, cache_file(), DECLICK, duration_ms(), fader_pickup_after_reset(), has_audio() (+15 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.11
Nodes (12): block_input(), DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out(), OUTPUT_HZ (+4 more)

### Community 48 - "Ui"
Cohesion: 0.12
Nodes (29): MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+21 more)

### Community 50 - "crash.rs"
Cohesion: 0.14
Nodes (13): arm(), Armed, ARMED, civil(), emergency_save(), install(), log_file(), marker_file() (+5 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (36): addresses(), APP_JS, apply(), CLIENT_TIMEOUT, Cmd, Color, Press, TrackToggle (+28 more)

### Community 52 - "TouchLayout"
Cohesion: 0.16
Nodes (8): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), LONG_PRESS_MS, pages_can_be_reordered(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Timeline"
Cohesion: 0.10
Nodes (19): audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others(), is_light() (+11 more)

### Community 66 - "control/timeline.rs"
Cohesion: 0.08
Nodes (38): Track, add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H (+30 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.13
Nodes (18): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+10 more)

### Community 70 - "stage.rs"
Cohesion: 0.13
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "runtime.rs"
Cohesion: 0.20
Nodes (7): engine_keeps_running_without_device(), Failing, output_keeps_running_after_a_panic_elsewhere(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 72 - "Show"
Cohesion: 0.16
Nodes (10): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (27): apply_effect(), apply_effect_hold(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+19 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.13
Nodes (53): a_pulsing_dimmer_effect_does_not_pull_the_position_down(), blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), crossfade_is_a_morph(), cv(), defaults_are_rendered(), end_scene_follows_jump_to() (+45 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.32
Nodes (6): chip(), divider(), level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (50): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+42 more)

### Community 80 - "PaletteStore"
Cohesion: 0.33
Nodes (8): builtin(), load_user(), named(), NamedPalette, path(), save_user(), palette_editor(), PaletteStore

### Community 86 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (27): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+19 more)

### Community 90 - "VizView"
Cohesion: 0.09
Nodes (17): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+9 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (68): alpha(), api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect() (+60 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.10
Nodes (36): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles (+28 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - ".end_frame"
Cohesion: 0.12
Nodes (16): button_rect(), default_feedback(), fires_id(), is_value_control(), Mapper, overlay_id(), Pick, pick() (+8 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (18): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+10 more)

### Community 101 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 102 - "show.rs"
Cohesion: 0.07
Nodes (28): address_map_marks_ranges(), duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once (+20 more)

### Community 104 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, FOOTER_H, FooterButton, HEADER_H, Kind, Cancel, Primary, Secondary (+2 more)

### Community 105 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 106 - "core/src/venue.rs"
Cohesion: 0.15
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 110 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "snap.rs"
Cohesion: 0.16
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 117 - "Object"
Cohesion: 0.24
Nodes (8): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE

### Community 118 - "TouchKind"
Cohesion: 0.18
Nodes (8): TouchKind, Button, Color, Fader, Knob, Label, Tracking, Xy

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (24): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+16 more)

### Community 120 - ".update"
Cohesion: 0.18
Nodes (15): media_action(), media_settings(), open(), read_json(), run(), seek(), send_json(), session() (+7 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (7): auth_matches_obs_websocket_example(), auth_response(), b64(), DRIFT_MS, Link, PRELOAD_MS, SETTLE

### Community 127 - "stage_editor.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+5 more)

### Community 128 - "Layer"
Cohesion: 0.16
Nodes (8): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On

### Community 129 - "Obs"
Cohesion: 0.18
Nodes (8): browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), engine(), fake_obs(), Obs, VideoPlay, wait(), wrong_password_is_reported()

### Community 130 - "global.rs"
Cohesion: 0.25
Nodes (15): apply(), button(), cells_toggle_but_never_all_off(), fader(), hidden_tracking_follow_works(), Input, Press, Value (+7 more)

### Community 131 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "measure.rs"
Cohesion: 0.23
Nodes (4): End, Measure, snap_points(), SNAP_PX

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "egui"
Cohesion: 0.06
Nodes (24): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+16 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.09
Nodes (31): in_rotated(), rotated_quad(), AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect() (+23 more)

### Community 142 - "Deserialize"
Cohesion: 0.17
Nodes (12): fire(), bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, ObsSettings (+4 more)

### Community 143 - "Clip"
Cohesion: 0.14
Nodes (7): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 144 - "DeviceModel"
Cohesion: 0.13
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - ".ui"
Cohesion: 0.10
Nodes (19): Drag, FadeIn, FadeOut, Lead, Left, Move, Right, fmt_ms() (+11 more)

### Community 146 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 147 - "assist.rs"
Cohesion: 0.14
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 148 - "FixtureType"
Cohesion: 0.16
Nodes (19): FixtureType, Physics, color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer (+11 more)

### Community 149 - "pathbuf"
Cohesion: 0.18
Nodes (5): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX

### Community 151 - "text_dim"
Cohesion: 0.12
Nodes (28): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+20 more)

### Community 152 - "ProfileBuilder"
Cohesion: 0.08
Nodes (26): FileJob, Export, Import, list_header(), ListAction, Add, Duplicate, Remove (+18 more)

### Community 153 - "Category"
Cohesion: 0.17
Nodes (11): ControlViews, Category, All, Beam, Color, Dimmer, Gobo, .ITEMS (+3 more)

### Community 154 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 155 - "text_field"
Cohesion: 0.14
Nodes (12): CANVAS_COLORS, hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch (+4 more)

### Community 156 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 158 - "Profile"
Cohesion: 0.19
Nodes (11): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+3 more)

### Community 159 - "FxHasher"
Cohesion: 0.18
Nodes (4): FastBuild, FastMap, FxHasher, SEED

### Community 160 - "View"
Cohesion: 0.12
Nodes (18): View, body(), caption(), FixtureSettings, hanging_of(), range_row(), SetupView, CELL_H_MAX (+10 more)

### Community 163 - "ViewCtx"
Cohesion: 0.25
Nodes (5): ViewCtx, Folder, VizView, VizView, button()

### Community 164 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 165 - "Problem"
Cohesion: 0.12
Nodes (20): fine_names(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "server.rs"
Cohesion: 0.21
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 167 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 169 - "scene_card.rs"
Cohesion: 0.23
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 170 - "Snapshot"
Cohesion: 0.29
Nodes (6): following_is_no_undo_step(), History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 173 - "midi.rs"
Cohesion: 0.23
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.07
Nodes (75): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+67 more)

### Community 177 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "control/tracking.rs"
Cohesion: 0.09
Nodes (13): room_json(), trackers_json(), Show, bar_row(), color(), edit(), RoomMap, TrackingView (+5 more)

### Community 181 - "bridge.rs"
Cohesion: 0.39
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (17): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+9 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.07
Nodes (52): button(), Timeline, time_field(), advanced(), apply_to_others(), banks_of(), BAR_H, bar_slider() (+44 more)

### Community 184 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 185 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 189 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 190 - "ButtonStyle"
Cohesion: 0.14
Nodes (14): icons_only(), label_with_glyphs(), style_name(), ButtonStyle, .ALL, Led, Normal, Outline (+6 more)

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 193 - "ProgrammerMode"
Cohesion: 0.29
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 195 - "core/src/lib.rs"
Cohesion: 0.09
Nodes (14): UNIVERSE_SIZE, FadeState, In, Out, BankScope, All, Bank, Banks (+6 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 200 - "menu.rs"
Cohesion: 0.11
Nodes (12): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY, ICON_SLOT, IconFn, ITEM_H (+4 more)

### Community 203 - "Step"
Cohesion: 0.21
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "ClipContent"
Cohesion: 0.25
Nodes (6): ClipContent, Audio, Group, Obs, Scene, Video

### Community 212 - "lock.rs"
Cohesion: 0.33
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), MASTER, unlocks(), verify()

### Community 218 - "AppState"
Cohesion: 0.12
Nodes (8): AppState, Focus, Bank, Effect, Scenes, PatchForm, select_only(), toggle_selected()

### Community 219 - "DeckState"
Cohesion: 0.16
Nodes (10): Player, DeckRt, DeckState, Empty, Ended, Fading, Paused, Playing (+2 more)

### Community 221 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 225 - "autosave.rs"
Cohesion: 0.38
Nodes (4): autosave_only_when_due_dirty_and_saved_before(), off(), RECOVERY_EVERY, without_folder_nothing_pending()

### Community 226 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 233 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 234 - "time_field.rs"
Cohesion: 0.31
Nodes (8): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 239 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 243 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 244 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 291 - "views/touch.rs"
Cohesion: 0.12
Nodes (24): state_json(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+16 more)

### Community 317 - "views/touch.rs"
Cohesion: 0.24
Nodes (16): content_rect(), display_label(), fader_parts(), item_target(), label_galley(), led_color(), mapping_of(), paint_cover() (+8 more)

## Knowledge Gaps
- **585 isolated node(s):** `Edit`, `New`, `Open`, `Template`, `Builtin` (+580 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1540 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **163 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `ChannelRef` to `Layer`, `Obs`, `global.rs`, `library_panel.rs`, `programmer.rs`, `mcp/src/venue.rs`, `mapping/mod.rs`, `egui`, `.ui`, `text_dim`, `ProfileBuilder`, `rig.rs`, `Host`, `views/touch.rs`, `ViewCtx`, `menu.rs`, `t`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `app/src/audio.rs`, `DmxCraftApp`, `Ui`, `tools.rs`, `crash.rs`, `remote/mod.rs`, `control/tracking.rs`, `scene_props.rs`, `views/touch.rs`, `DmxFrame`, `control/timeline.rs`, `core/src/lib.rs`, `window.rs`, `stage.rs`, `runtime.rs`, `Show`, `viz/calibrate.rs`, `engine.rs`, `.end_frame`, `audio_triggers.rs`, `mcp/src/profiles.rs`, `.update`, `app/src/obs.rs`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Are the 206 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 206 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Edit`, `New`, `Open` to the rest of the system?**
  _585 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `Venue`, `programmer.rs`, `measure.rs`, `i18n/mod.rs`, `props.rs`, `mapping/mod.rs`, `egui`, `viz/library.rs`, `.ui`, `titlebar.rs`, `assist.rs`, `pathbuf`, `text_dim`, `ProfileBuilder`, `text_field`, `fx_ui.rs`, `Profile`, `missing_audio.rs`, `View`, `TouchView`, `views/touch.rs`, `ViewCtx`, `menu.rs`, `FixtureAction`, `scene_grid.rs`, `file_dialog.rs`, `DmxCraftApp`, `Ui`, `control/tracking.rs`, `fader.rs`, `scene_props.rs`, `views/touch.rs`, `ButtonStyle`, `ProgrammerMode`, `control/timeline.rs`, `window.rs`, `stage.rs`, `viz/calibrate.rs`, `toolbar.rs`, `PaletteStore`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `dialog.rs`, `snap.rs`, `outline.rs`, `stage_editor.rs`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09246088193456614 - nodes in this community are weakly interconnected._