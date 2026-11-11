# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 157 files · ~252,636 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 20 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4452 nodes · 11361 edges · 297 communities (158 shown, 139 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 590 edges (avg confidence: 0.79)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7e19ae60`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- ZoneId
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
- super
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- src/mapping.rs
- RangeEffect
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- OrbitCamera
- dmxcraft
- app/src/audio.rs
- profile_builder.rs
- Scene
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
- ClipContent
- remote/mod.rs
- TouchLayout
- MenuAction
- steps
- window.rs
- stage.rs
- EffectKind
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- .ui
- theme.rs
- OpenDmx
- time_field.rs
- DmxFrame
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
- Timeline
- Dialog<'a>
- show.rs
- Control
- arc
- Snapshot
- mcp/src/profiles.rs
- Theme
- audio_trigger_ui.rs
- Shape
- Venue
- mapping/mod.rs
- .row
- Tab
- convert.rs
- Scene
- Capability
- FileJob
- FixtureType
- app/src/obs.rs
- egui
- CurveShape
- ColorPreset
- gltf_import.rs
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- Mode
- stage_canvas.rs
- .objects_tab
- T
- DeviceModel
- ShowError
- ChannelRef
- TouchItem
- TouchView
- mapping/mod.rs
- text_dim
- BankScope
- icon_library.rs
- text_field
- Profile
- qlc-bundle.rs
- DeckState
- dip.rs
- scene_card.rs
- Problem
- views/touch.rs
- DialogKind
- ShutterEffect
- menu.rs
- FadeCurve
- Drag
- midi.rs
- tools.rs
- ChaserPreset
- ClipAction
- Transform
- UpdateBadge
- fader.rs
- scene_props.rs
- .fine_names
- Engine
- plan
- PlanShape
- scene_group.rs
- plan.rs
- ProfileChannel
- View
- env
- FixtureLimits
- de.rs
- Knob<'a>
- MenuItem<'a>
- Purpose
- HeaderAction
- build-local-artifacts
- IconButton
- TouchKind
- app.rs
- AppState
- Grip
- FooterButton<'a>
- views/sound.rs
- announce
- .check_placement
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- with
- Obs
- lock.rs
- super

## God Nodes (most connected - your core abstractions)
1. `Engine` - 237 edges
2. `t()` - 215 edges
3. `Show` - 165 edges
4. `Icon` - 118 edges
5. `AppState` - 108 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 60 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (297 total, 139 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.12
Nodes (3): DeckRt, SoundPlayer, SoundBoard

### Community 1 - "library_panel.rs"
Cohesion: 0.10
Nodes (22): LibraryDrag, FxDialog, BuilderRequest, Edit, New, Open, Template, pick_export() (+14 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (17): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+9 more)

### Community 4 - "ZoneId"
Cohesion: 0.12
Nodes (8): targets(), duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), SceneSettings, steps_insert_remove_and_normalize(), Zone

### Community 5 - "programmer.rs"
Cohesion: 0.06
Nodes (50): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+42 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+21 more)

### Community 9 - "DmxOutput"
Cohesion: 0.14
Nodes (6): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.09
Nodes (24): fold_section(), renumber(), apply_texture(), OutlineTab, Fixtures, Objects, TexAction, Builtin (+16 more)

### Community 11 - "assist.rs"
Cohesion: 0.12
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 12 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (17): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+9 more)

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
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 21 - "super"
Cohesion: 0.10
Nodes (8): strobe_ranges(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (30): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+22 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "src/mapping.rs"
Cohesion: 0.10
Nodes (24): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback (+16 more)

### Community 28 - "RangeEffect"
Cohesion: 0.19
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 29 - "profile_builder.rs"
Cohesion: 0.15
Nodes (6): FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, LABEL_W

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.14
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.10
Nodes (36): builtin(), load_user(), named(), NamedPalette, path(), save_user(), rgb(), color_fx_ui() (+28 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (13): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+5 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.06
Nodes (55): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+47 more)

### Community 37 - "Scene"
Cohesion: 0.12
Nodes (11): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Scene, SceneKind, Fx (+3 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+42 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.09
Nodes (20): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize (+12 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.13
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.11
Nodes (32): McpStatus, Error, Off, Running, hidden_tabs_from_old_setting(), Settings, ai(), audio() (+24 more)

### Community 50 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (33): ui_states(), addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value (+25 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (17): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+9 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 72 - "Show"
Cohesion: 0.21
Nodes (4): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (16): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+8 more)

### Community 74 - "OutputKind"
Cohesion: 0.19
Nodes (7): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation, list_ports(), UsbDmxPort

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.17
Nodes (42): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+34 more)

### Community 78 - ".ui"
Cohesion: 0.19
Nodes (8): fmt_ms(), grid_options(), mark_clipboard(), obs_label(), row_h(), TimelineView, track_h(), track_header()

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (51): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+43 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "time_field.rs"
Cohesion: 0.17
Nodes (14): drag_number(), endless_delta(), raw_available(), COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+6 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (21): camera_json(), beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+13 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.07
Nodes (41): apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles() (+33 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.12
Nodes (28): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+20 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Mapper"
Cohesion: 0.19
Nodes (8): fires_id(), is_value_control(), Mapper, overlay_id(), pick_id(), Sent, states_id(), UiFire

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "Timeline"
Cohesion: 0.13
Nodes (10): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), has_audio(), has_sound_cached(), media_track() (+2 more)

### Community 104 - "Dialog<'a>"
Cohesion: 0.11
Nodes (7): Button, Dialog, Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 105 - "show.rs"
Cohesion: 0.15
Nodes (20): UNIVERSE_SIZE, address_map_marks_ranges(), Group, groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once (+12 more)

### Community 106 - "Control"
Cohesion: 0.08
Nodes (24): Overlay, Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+16 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 110 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 112 - "audio_trigger_ui.rs"
Cohesion: 0.23
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.07
Nodes (20): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+12 more)

### Community 118 - "mapping/mod.rs"
Cohesion: 0.21
Nodes (18): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), index_targets() (+10 more)

### Community 119 - ".row"
Cohesion: 0.08
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (33): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+25 more)

### Community 123 - "Scene"
Cohesion: 0.10
Nodes (16): audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others(), Layer (+8 more)

### Community 127 - "Capability"
Cohesion: 0.15
Nodes (9): cap_for_kind(), Capability, SlotKind, .ALL, Color, Frost, Gobo, Open (+1 more)

### Community 128 - "FileJob"
Cohesion: 0.67
Nodes (3): FileJob, Export, Import

### Community 129 - "FixtureType"
Cohesion: 0.13
Nodes (20): ChannelDef, FixtureType, Physics, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget (+12 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.12
Nodes (19): auth_matches_obs_websocket_example(), auth_response(), b64(), DRIFT_MS, fire(), Link, media_action(), media_settings() (+11 more)

### Community 131 - "egui"
Cohesion: 0.05
Nodes (33): text_field(), DipSwitch, Knob, SIZE, SWEEP, LimitPad, from_pct(), HANDLE (+25 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.18
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (25): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, drop_hint(), EDGE, HEADER_W, is_video(), LANE_H (+17 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 141 - "stage_canvas.rs"
Cohesion: 0.14
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - ".objects_tab"
Cohesion: 0.32
Nodes (3): ViewCtx, VizView, button()

### Community 143 - "T"
Cohesion: 0.11
Nodes (5): cable_belongs_to_one_universe(), default_instance_name(), last_tab_stays(), RECENT_MAX, Segmented<'a, T>

### Community 144 - "DeviceModel"
Cohesion: 0.12
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 145 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - "ChannelRef"
Cohesion: 0.09
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 148 - "TouchItem"
Cohesion: 0.24
Nodes (12): find_item(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of() (+4 more)

### Community 149 - "TouchView"
Cohesion: 0.15
Nodes (13): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.13
Nodes (24): alt_click(), button_rect(), fader(), fader_channels(), group_set(), in_rotated(), Pick, pick() (+16 more)

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (29): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+21 more)

### Community 152 - "BankScope"
Cohesion: 0.29
Nodes (6): BankScope, All, Bank, Banks, Off, OutsideBank

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (16): CANVAS_COLORS, canvas_swatches(), color_picker(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS (+8 more)

### Community 158 - "Profile"
Cohesion: 0.12
Nodes (12): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+4 more)

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "DeckState"
Cohesion: 0.29
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.14
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 167 - "DialogKind"
Cohesion: 0.20
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 168 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 169 - "menu.rs"
Cohesion: 0.15
Nodes (14): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+6 more)

### Community 170 - "FadeCurve"
Cohesion: 0.20
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 171 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.05
Nodes (89): bridge_command(), installed_bridge(), McpLink, VizJob, AppEvent, ProfilesChanged, Saved, SelectScene (+81 more)

### Community 177 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 181 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (42): advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header(), effect_tiles() (+34 more)

### Community 185 - "Engine"
Cohesion: 0.10
Nodes (4): FadeState, In, Out, SceneProgress

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 190 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 193 - "ProfileChannel"
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 195 - "View"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 208 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 211 - "app.rs"
Cohesion: 0.20
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (14): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, Show (+6 more)

### Community 219 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (21): button(), color_swatches(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color() (+13 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

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

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 281 - "Obs"
Cohesion: 0.13
Nodes (13): browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), engine(), fake_obs(), Obs, Status, Connected, Connecting (+5 more)

### Community 286 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 311 - "super"
Cohesion: 0.20
Nodes (10): bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length(), VideoOutput (+2 more)

## Knowledge Gaps
- **574 isolated node(s):** `Setup`, `Control`, `Touch`, `Visualizer`, `Builder` (+569 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1440 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **139 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `SoundPlayer`, `FixtureType`, `app/src/obs.rs`, `ZoneId`, `fx.rs`, `stage_canvas.rs`, `ShowError`, `ChannelRef`, `TouchItem`, `TouchView`, `mapping/mod.rs`, `rig.rs`, `src/mapping.rs`, `Scene`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `missing_audio.rs`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `scene_props.rs`, `Engine`, `scene_group.rs`, `EffectKind`, `feature.rs`, `motion.rs`, `engine.rs`, `theme.rs`, `app.rs`, `viz/mod.rs`, `Mapper`, `autosave.rs`, `show.rs`, `.check_placement`, `Snapshot`, `Venue`, `mapping/mod.rs`?**
  _High betweenness centrality (0.129) - this node is a cross-community bridge._
- **Are the 185 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 185 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Touch` to the rest of the system?**
  _574 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.11522048364153627 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `ChannelRef` to `library_panel.rs`, `app/src/obs.rs`, `egui`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `mcp/src/venue.rs`, `.objects_tab`, `TouchItem`, `mapping/mod.rs`, `text_dim`, `rig.rs`, `Obs`, `app/src/audio.rs`, `profile_builder.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `AudioPlayer`, `DmxCraftApp`, `tools.rs`, `Ui`, `remote/mod.rs`, `scene_props.rs`, `Engine`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `.ui`, `app.rs`, `AppState`, `Mapper`, `audio_triggers.rs`, `show.rs`, `Snapshot`, `mcp/src/profiles.rs`, `audio_trigger_ui.rs`, `mapping/mod.rs`?**
  _High betweenness centrality (0.121) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0960960960960961 - nodes in this community are weakly interconnected._