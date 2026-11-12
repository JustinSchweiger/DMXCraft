# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 160 files · ~257,975 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4522 nodes · 11556 edges · 323 communities (175 shown, 148 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 600 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e3571385`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- show.rs
- programmer.rs
- SoundBoard
- steps
- fx.rs
- DmxOutput
- ViewCtx
- assist.rs
- server.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- fx/mapping.rs
- model.rs
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Control
- fx_ui.rs
- Host
- ofl/src/lib.rs
- fx_color.rs
- OrbitCamera
- dmxcraft
- app/src/audio.rs
- t
- core/src/lib.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_grid.rs
- missing_audio.rs
- AudioPlayer
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- Ui
- audio_trigger_ui.rs
- remote/mod.rs
- TouchLayout
- app/src/obs.rs
- steps
- window.rs
- stage.rs
- EffectKind
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- control/timeline.rs
- theme.rs
- OpenDmx
- time_field.rs
- titlebar.rs
- profile.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- Mapper
- MapPreset
- audio_triggers.rs
- autosave.rs
- system_fonts.rs
- Dialog<'a>
- target_shape
- range_slider.rs
- arc
- Snapshot
- mcp/src/profiles.rs
- Layer
- duration
- Shape
- Venue
- mapping/mod.rs
- .row
- Tab
- convert.rs
- src/timeline.rs
- widgets/mod.rs
- ProfileBuilder
- FixtureType
- app.rs
- egui
- CurveShape
- ColorPreset
- gltf_import.rs
- panels.rs
- i18n/mod.rs
- mcp/src/venue.rs
- IconButton
- stage_canvas.rs
- settings.rs
- T
- DeviceModel
- ShowError
- .push
- ObsSettings
- TouchView
- AudioGrid
- text_dim
- Theme
- TouchItem
- text_field
- Profile
- qlc-bundle.rs
- control/tracking.rs
- dip.rs
- Transform
- Problem
- views/touch.rs
- mcp.rs
- ShutterEffect
- menu.rs
- SoundBoard
- .add_scene
- midi.rs
- tools.rs
- .show
- ClipAction
- BankScope
- MoveShape
- fader.rs
- scene_props.rs
- drag_number
- Engine
- plan
- PlanShape
- scene_group.rs
- super
- RangeEffect
- fixture_settings.rs
- env
- FixtureLimits
- de.rs
- Knob<'a>
- Easing
- Purpose
- HeaderAction
- build-local-artifacts
- lock.rs
- TouchKind
- scene_card.rs
- AppState
- PaletteStore
- text_field
- FooterButton<'a>
- views/sound.rs
- announce
- .update
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- ProfileChannel
- with
- Scene
- .paint
- Obs
- FadeCurve
- DeckState
- WaveState
- ObsSettings
- SceneId
- Category
- ProgrammerMode
- src/tracking.rs
- UpdateBadge
- Kind
- Effect
- Arrange
- Layer
- ok
- Box<dyn serialport::SerialPort>
- PatchGrid
- venue_object_schema

## God Nodes (most connected - your core abstractions)
1. `Engine` - 249 edges
2. `t()` - 221 edges
3. `Show` - 172 edges
4. `Icon` - 119 edges
5. `AppState` - 112 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 63 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/tracking.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (323 total, 148 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (20): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+12 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "show.rs"
Cohesion: 0.23
Nodes (15): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), move_zone_reorders_banks(), next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names() (+7 more)

### Community 5 - "programmer.rs"
Cohesion: 0.18
Nodes (22): XyPad, color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup (+14 more)

### Community 6 - "SoundBoard"
Cohesion: 0.16
Nodes (9): Deck, DECKS, file_name(), MIN_WIDTH, Pad, PAD_GRID_MAX, PADS, Track (+1 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+15 more)

### Community 9 - "DmxOutput"
Cohesion: 0.09
Nodes (9): addresses_are_one_based(), DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+1 more)

### Community 10 - "ViewCtx"
Cohesion: 0.12
Nodes (15): fold_section(), ViewCtx, Item, apply_texture(), VizView, aim_fixtures(), button(), color_field() (+7 more)

### Community 11 - "assist.rs"
Cohesion: 0.14
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 12 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.18
Nodes (10): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+2 more)

### Community 21 - "model.rs"
Cohesion: 0.09
Nodes (9): Physics, strobe_ranges(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.11
Nodes (31): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve() (+23 more)

### Community 24 - "rig.rs"
Cohesion: 0.17
Nodes (23): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+15 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.20
Nodes (16): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor() (+8 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_color.rs"
Cohesion: 0.20
Nodes (13): rgb(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), ROW_H (+5 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.16
Nodes (14): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+6 more)

### Community 36 - "t"
Cohesion: 0.09
Nodes (34): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+26 more)

### Community 37 - "core/src/lib.rs"
Cohesion: 0.18
Nodes (11): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.08
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+42 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.17
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.16
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.14
Nodes (29): MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+21 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.23
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "app/src/obs.rs"
Cohesion: 0.17
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (23): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+15 more)

### Community 70 - "stage.rs"
Cohesion: 0.09
Nodes (30): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+22 more)

### Community 71 - "EffectKind"
Cohesion: 0.12
Nodes (12): effect_tiles(), EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+4 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (5): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (23): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+15 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.10
Nodes (21): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+13 more)

### Community 77 - "engine.rs"
Cohesion: 0.15
Nodes (46): blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below() (+38 more)

### Community 78 - "control/timeline.rs"
Cohesion: 0.06
Nodes (46): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+38 more)

### Community 79 - "theme.rs"
Cohesion: 0.06
Nodes (55): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+47 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "time_field.rs"
Cohesion: 0.24
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 87 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (22): camera_json(), beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys() (+14 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.07
Nodes (46): apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag (+38 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Mapper"
Cohesion: 0.22
Nodes (5): default_feedback(), is_value_control(), Mapper, Sent, WindowState

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 104 - "Dialog<'a>"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "target_shape"
Cohesion: 0.21
Nodes (15): alt_click(), button(), button_rect(), fader(), fader_channels(), in_rotated(), rotated_quad(), t_touch_hint() (+7 more)

### Community 106 - "range_slider.rs"
Cohesion: 0.17
Nodes (6): surface(), DialogResult, select_all(), from_pct(), pct(), value_field()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 110 - "Layer"
Cohesion: 0.21
Nodes (8): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.06
Nodes (27): renumber(), duplicate_object(), floor_height(), set_locked(), transfer(), load_file(), aim_at_points_beam_to_target(), apply() (+19 more)

### Community 118 - "mapping/mod.rs"
Cohesion: 0.11
Nodes (34): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fires_id(), fixed_channels_work_without_selection() (+26 more)

### Community 119 - ".row"
Cohesion: 0.10
Nodes (26): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+18 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "src/timeline.rs"
Cohesion: 0.08
Nodes (25): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Group, Obs, Scene, Video (+17 more)

### Community 127 - "widgets/mod.rs"
Cohesion: 0.21
Nodes (9): CanvasMode, Edit, Select, fit(), fixture_pos(), StageCanvas, Underlay, set_pos() (+1 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.08
Nodes (26): FileJob, Export, Import, list_header(), ListAction, Add, Duplicate, Remove (+18 more)

### Community 129 - "FixtureType"
Cohesion: 0.20
Nodes (18): ChannelDef, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer (+10 more)

### Community 130 - "app.rs"
Cohesion: 0.11
Nodes (13): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView (+5 more)

### Community 131 - "egui"
Cohesion: 0.07
Nodes (25): Knob, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+17 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "panels.rs"
Cohesion: 0.11
Nodes (14): Segmented, DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, OutlineTab (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (12): AMBER, BOX, MenuFn, paint_effects(), paint_fixture(), paint_live_corner(), paint_shape_icon(), UV (+4 more)

### Community 142 - "settings.rs"
Cohesion: 0.18
Nodes (5): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 148 - "ObsSettings"
Cohesion: 0.27
Nodes (8): fire(), bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length()

### Community 149 - "TouchView"
Cohesion: 0.16
Nodes (12): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Page, pick_image(), prepare_image() (+4 more)

### Community 150 - "AudioGrid"
Cohesion: 0.21
Nodes (4): AudioAnalysis, AudioGrid, AudioSegment, grid_helpers()

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (32): window(), accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border() (+24 more)

### Community 152 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 153 - "TouchItem"
Cohesion: 0.26
Nodes (11): state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), paint_cover() (+3 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.10
Nodes (13): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+5 more)

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "control/tracking.rs"
Cohesion: 0.19
Nodes (9): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView, Show (+1 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.14
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 167 - "mcp.rs"
Cohesion: 0.14
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 169 - "menu.rs"
Cohesion: 0.14
Nodes (14): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+6 more)

### Community 170 - "SoundBoard"
Cohesion: 0.22
Nodes (3): default_board_is_not_saved_and_old_shows_load(), pad_grid_keeps_used_pads(), SoundBoard

### Community 171 - ".add_scene"
Cohesion: 0.20
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), SceneSettings, steps_insert_remove_and_normalize()

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (56): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+48 more)

### Community 177 - ".show"
Cohesion: 0.30
Nodes (5): Camera, fixture_rect(), live_colors(), StageInteraction, strobe_hz()

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "BankScope"
Cohesion: 0.16
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 181 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (17): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+9 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (43): advanced(), apply_to_others(), bar_slider(), block_props(), chip(), effect_header(), fit_mapping(), FX_EXT (+35 more)

### Community 184 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (7): def_is_strobe(), Engine, lerp(), PlayingScene, Track, TrackLight, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 190 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "super"
Cohesion: 0.14
Nodes (13): glyph(), LIBRARY, names_are_unique(), paint(), clamp(), default_position(), grid(), grid_fits_inside() (+5 more)

### Community 193 - "RangeEffect"
Cohesion: 0.17
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 195 - "fixture_settings.rs"
Cohesion: 0.31
Nodes (5): RangeSlider, body(), caption(), hanging_of(), range_row()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 202 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 203 - "Purpose"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 208 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 211 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (18): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+10 more)

### Community 219 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 225 - "text_field"
Cohesion: 0.16
Nodes (3): text_field(), ZonesView, Windows

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - ".update"
Cohesion: 0.36
Nodes (7): media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 256 - "ProfileChannel"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "Scene"
Cohesion: 0.16
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Scene, Step

### Community 281 - "Obs"
Cohesion: 0.14
Nodes (10): Obs, run(), session(), Shared, Status, Connected, Connecting, Failed (+2 more)

### Community 285 - "FadeCurve"
Cohesion: 0.20
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 286 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 288 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 289 - "ObsSettings"
Cohesion: 0.25
Nodes (5): Link, ObsSettings, VideoOutput, Browser, Media

### Community 291 - "SceneId"
Cohesion: 0.10
Nodes (4): FadeState, In, Out, SceneProgress

### Community 292 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 303 - "ProgrammerMode"
Cohesion: 0.25
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 305 - "src/tracking.rs"
Cohesion: 0.29
Nodes (3): DEFAULT_HEIGHT, one(), white()

### Community 306 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 307 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 308 - "Effect"
Cohesion: 0.43
Nodes (7): apply_effect(), apply_effect_hold(), feature_levels(), luma(), to_rgb(), write_colors(), Effect

### Community 309 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 310 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

### Community 311 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 320 - "venue_object_schema"
Cohesion: 0.67
Nodes (3): props!, obj(), venue_object_schema()

## Knowledge Gaps
- **574 isolated node(s):** `Edit`, `New`, `Open`, `Template`, `ImportEv2` (+569 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1470 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **148 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app.rs`, `egui`, `runtime.rs`, `programmer.rs`, `ViewCtx`, `mcp/src/venue.rs`, `text_dim`, `rig.rs`, `TouchItem`, `Obs`, `Host`, `control/tracking.rs`, `app/src/audio.rs`, `SceneId`, `t`, `core/src/lib.rs`, `views/touch.rs`, `mcp.rs`, `scene_grid.rs`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `audio_trigger_ui.rs`, `remote/mod.rs`, `scene_props.rs`, `app/src/obs.rs`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `control/timeline.rs`, `ChannelKind`, `Mapper`, `audio_triggers.rs`, `.update`, `Snapshot`, `mcp/src/profiles.rs`, `Layer`, `mapping/mod.rs`, `src/timeline.rs`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._
- **Are the 190 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 190 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Edit`, `New`, `Open` to the rest of the system?**
  _574 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13675213675213677 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `library_panel.rs`, `app.rs`, `show.rs`, `fx.rs`, `Scene`, `stage_canvas.rs`, `ShowError`, `TouchView`, `rig.rs`, `TouchItem`, `Control`, `control/tracking.rs`, `ObsSettings`, `core/src/lib.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `SoundBoard`, `missing_audio.rs`, `.add_scene`, `DmxCraftApp`, `tools.rs`, `.show`, `src/tracking.rs`, `Effect`, `BankScope`, `TouchLayout`, `scene_props.rs`, `Engine`, `scene_group.rs`, `stage.rs`, `EffectKind`, `feature.rs`, `motion.rs`, `engine.rs`, `theme.rs`, `viz/mod.rs`, `autosave.rs`, `Snapshot`, `Venue`, `mapping/mod.rs`, `widgets/mod.rs`?**
  _High betweenness centrality (0.076) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.125 - nodes in this community are weakly interconnected._