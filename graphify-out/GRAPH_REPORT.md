# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 156 files · ~248,469 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 20 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4400 nodes · 11235 edges · 314 communities (163 shown, 151 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 586 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2022c00a`
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
- Settings
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- OrbitCamera
- dmxcraft
- app/src/audio.rs
- profile_builder.rs
- hashmap
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_grid.rs
- duration
- Control
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- fire
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
- range_slider.rs
- AudioGrid
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
- updater.rs
- show.rs
- gltf_import.rs
- arc
- Snapshot
- time_field.rs
- Theme
- audio_trigger_ui.rs
- Shape
- Venue
- View
- .row
- Tab
- convert.rs
- Scene
- Host
- ProfileBuilder
- FixtureType
- app/src/obs.rs
- egui
- CurveShape
- ColorPreset
- qlc-bundle.rs
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- Model
- stage_canvas.rs
- settings_window.rs
- missing_audio.rs
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
- Category
- Knob<'a>
- super
- Lang
- Problem
- views/touch.rs
- scene_card.rs
- ShutterEffect
- menu.rs
- FadeCurve
- Drag
- midi.rs
- tools.rs
- DmxLine
- ClipAction
- send_frame
- UpdateBadge
- fader.rs
- scene_props.rs
- InstallKind
- Engine
- plan
- PlanShape
- scene_group.rs
- plan.rs
- VizView
- T
- View
- env
- FixtureLimits
- de.rs
- bridge.rs
- MenuItem<'a>
- Purpose
- Effect
- HeaderAction
- build-local-artifacts
- IconButton
- TouchKind
- app.rs
- Scene
- AppState
- Grip
- PaletteStore
- FooterButton<'a>
- make
- views/sound.rs
- announce
- Status
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- release-install.md
- elide
- with
- ObsSettings
- lock.rs
- View
- Swatch
- ObsAction

## God Nodes (most connected - your core abstractions)
1. `Engine` - 235 edges
2. `t()` - 215 edges
3. `Show` - 164 edges
4. `Icon` - 118 edges
5. `AppState` - 107 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 59 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (314 total, 151 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.10
Nodes (8): DeckState, Empty, Ended, Fading, Paused, Playing, Ready, SoundPlayer

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "ZoneId"
Cohesion: 0.14
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), Zone

### Community 5 - "programmer.rs"
Cohesion: 0.21
Nodes (21): kind_color(), color_bar(), current(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+13 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.21
Nodes (22): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+14 more)

### Community 9 - "DmxOutput"
Cohesion: 0.08
Nodes (9): addresses_are_one_based(), DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+1 more)

### Community 10 - "ViewCtx"
Cohesion: 0.10
Nodes (19): fold_section(), ViewCtx, apply_texture(), TexAction, Builtin, Load, Set, VizView (+11 more)

### Community 11 - "assist.rs"
Cohesion: 0.12
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 12 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

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
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.12
Nodes (31): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+23 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 21 - "super"
Cohesion: 0.17
Nodes (7): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Texture

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.08
Nodes (36): analyze(), Analyzer, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX, BPM_MIN, energy_curve() (+28 more)

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
Cohesion: 0.11
Nodes (19): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback, FeedbackState, Input (+11 more)

### Community 28 - "Settings"
Cohesion: 0.12
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 29 - "profile_builder.rs"
Cohesion: 0.06
Nodes (15): surface(), Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form (+7 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.11
Nodes (29): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+21 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.07
Nodes (23): analysis(), analyze_file(), AudioPlayer, cache_file(), duration_ms(), fader_pickup_after_reset(), has_audio(), has_sound() (+15 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.09
Nodes (32): t(), chip(), GroupDrag, strip(), capability_fields(), card_row(), category_label(), check() (+24 more)

### Community 37 - "hashmap"
Cohesion: 0.10
Nodes (13): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+5 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.16
Nodes (11): run_updater(), Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.07
Nodes (52): ControlViews, programmer_values(), ProgrammerView, Action, Bank, Compact, CreateSuper, Delete (+44 more)

### Community 43 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 44 - "Control"
Cohesion: 0.09
Nodes (23): Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel, Channels (+15 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.32
Nodes (17): ai(), audio(), customize(), full_combo(), general(), hint(), midi(), obs() (+9 more)

### Community 50 - "fire"
Cohesion: 0.18
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (33): ui_states(), addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value (+25 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (25): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (9): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+1 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.17
Nodes (41): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_scene_holds_effects_at_their_end(), finished_super_scene_restarts_on_play() (+33 more)

### Community 78 - ".ui"
Cohesion: 0.15
Nodes (14): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+6 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (52): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+44 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 87 - "AudioGrid"
Cohesion: 0.23
Nodes (8): AudioGrid, AudioSegment, detect_grid(), detect_segments(), local_tempo(), onsets(), tempo_coef(), tempo_score()

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (25): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+17 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.11
Nodes (34): apply_placement(), decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles(), hit_y() (+26 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.16
Nodes (22): asphalt(), brick(), carpet(), checker(), cobblestone(), concrete(), fbm(), light_wood() (+14 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (31): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+23 more)

### Community 97 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.16
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "Timeline"
Cohesion: 0.14
Nodes (5): has_audio(), media_track(), Clip, linked_blocks_stay_together(), Timeline

### Community 104 - "updater.rs"
Cohesion: 0.22
Nodes (10): APP_NAME, client(), github_latest(), is_newer(), latest_release(), MSI_ASSET, ReleaseInfo, REPO (+2 more)

### Community 105 - "show.rs"
Cohesion: 0.15
Nodes (20): UNIVERSE_SIZE, address_map_marks_ranges(), Group, groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once (+12 more)

### Community 106 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 110 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 112 - "audio_trigger_ui.rs"
Cohesion: 0.13
Nodes (15): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+7 more)

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.07
Nodes (25): renumber(), duplicate_object(), set_locked(), transfer(), aim_at_points_beam_to_target(), apply(), close(), Environment (+17 more)

### Community 118 - "View"
Cohesion: 0.36
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 119 - ".row"
Cohesion: 0.10
Nodes (26): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+18 more)

### Community 120 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Scene"
Cohesion: 0.10
Nodes (23): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+15 more)

### Community 127 - "Host"
Cohesion: 0.09
Nodes (28): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+20 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.07
Nodes (29): FileJob, Export, Import, list_header(), ListAction, Add, Duplicate, Remove (+21 more)

### Community 129 - "FixtureType"
Cohesion: 0.08
Nodes (30): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+22 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.11
Nodes (24): auth_matches_obs_websocket_example(), auth_response(), b64(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs(), Link (+16 more)

### Community 131 - "egui"
Cohesion: 0.07
Nodes (22): text_field(), DipSwitch, SIZE, SWEEP, LimitPad, RangeSlider, Segmented, Swatch (+14 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "control/timeline.rs"
Cohesion: 0.08
Nodes (24): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+16 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.19
Nodes (8): all_texts_translated(), DE, fill(), fill_replaces_names(), LANG, tr!, placeholders_match(), texts_in_source()

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "Model"
Cohesion: 0.18
Nodes (3): Model, ModelPart, part_roundtrip_through_json()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.14
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "settings_window.rs"
Cohesion: 0.22
Nodes (8): AUTOSAVE_MINUTES, HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, UsbDmxPort

### Community 143 - "missing_audio.rs"
Cohesion: 0.15
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 145 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - "ChannelRef"
Cohesion: 0.19
Nodes (3): lerp(), PlayingScene, ChannelRef

### Community 148 - "TouchItem"
Cohesion: 0.28
Nodes (10): content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), paint_cover(), paint_item() (+2 more)

### Community 149 - "TouchView"
Cohesion: 0.17
Nodes (12): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+4 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.13
Nodes (27): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+19 more)

### Community 151 - "text_dim"
Cohesion: 0.18
Nodes (29): window(), accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border() (+21 more)

### Community 152 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 153 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 155 - "text_field"
Cohesion: 0.20
Nodes (13): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+5 more)

### Community 158 - "Profile"
Cohesion: 0.08
Nodes (25): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+17 more)

### Community 159 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 163 - "super"
Cohesion: 0.25
Nodes (10): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), bx(), find_box() (+2 more)

### Community 164 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.12
Nodes (23): apply(), state_json(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag (+15 more)

### Community 167 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 168 - "ShutterEffect"
Cohesion: 0.09
Nodes (20): slot_label(), ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse (+12 more)

### Community 169 - "menu.rs"
Cohesion: 0.14
Nodes (14): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+6 more)

### Community 170 - "FadeCurve"
Cohesion: 0.22
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 171 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (65): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+57 more)

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "send_frame"
Cohesion: 0.39
Nodes (5): frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 181 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (44): GROUP_COLOR, advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+36 more)

### Community 184 - "InstallKind"
Cohesion: 0.33
Nodes (7): install_kind(), installed_by_msi(), InstallKind, Browser, Msi, Updater, Release

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (5): Engine, FadeState, In, Out, SceneProgress

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

### Community 193 - "VizView"
Cohesion: 0.08
Nodes (17): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+9 more)

### Community 195 - "View"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, apply(), Arrange, AlignH, AlignV, Grid (+4 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 201 - "bridge.rs"
Cohesion: 0.26
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "Effect"
Cohesion: 0.43
Nodes (7): apply_effect(), apply_effect_hold(), feature_levels(), luma(), to_rgb(), write_colors(), Effect

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
Cohesion: 0.23
Nodes (6): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 213 - "Scene"
Cohesion: 0.12
Nodes (11): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Scene, SceneKind, Fx (+3 more)

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (18): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+10 more)

### Community 219 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 227 - "PaletteStore"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 229 - "make"
Cohesion: 0.40
Nodes (5): builtin(), builtins_are_seamless_and_decode(), make(), Pattern, render()

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "Status"
Cohesion: 0.40
Nodes (5): Status, Connected, Connecting, Failed, Off

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 286 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 287 - "View"
Cohesion: 0.16
Nodes (6): View, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 309 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 311 - "ObsAction"
Cohesion: 0.50
Nodes (4): fire(), ObsAction, Scene, Source

## Knowledge Gaps
- **570 isolated node(s):** `Off`, `Connecting`, `Connected`, `Edit`, `New` (+565 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1428 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **151 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app/src/obs.rs`, `egui`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `DmxOutput`, `ViewCtx`, `mcp/src/venue.rs`, `settings_window.rs`, `ChannelRef`, `TouchItem`, `text_dim`, `rig.rs`, `Profile`, `View`, `app/src/audio.rs`, `profile_builder.rs`, `hashmap`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `fire`, `remote/mod.rs`, `scene_props.rs`, `View`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `.ui`, `app.rs`, `ChannelKind`, `Mapper`, `audio_triggers.rs`, `show.rs`, `Snapshot`, `audio_trigger_ui.rs`, `Host`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._
- **Are the 185 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 185 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Connecting`, `Connected` to the rest of the system?**
  _570 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.09716599190283401 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `FixtureType`, `ObsSettings`, `ZoneId`, `SoundBoard`, `fx.rs`, `stage_canvas.rs`, `missing_audio.rs`, `ShowError`, `ChannelRef`, `TouchItem`, `TouchView`, `mapping/mod.rs`, `rig.rs`, `src/mapping.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `tools.rs`, `fire`, `TouchLayout`, `scene_props.rs`, `Engine`, `scene_group.rs`, `EffectKind`, `feature.rs`, `Effect`, `engine.rs`, `motion.rs`, `theme.rs`, `app.rs`, `Scene`, `viz/mod.rs`, `autosave.rs`, `show.rs`, `Snapshot`, `Venue`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11827956989247312 - nodes in this community are weakly interconnected._