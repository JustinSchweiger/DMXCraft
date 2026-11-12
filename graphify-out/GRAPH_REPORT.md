# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 162 files · ~260,400 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4560 nodes · 11620 edges · 312 communities (168 shown, 144 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 609 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `071ed25f`
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
- props.rs
- assist.rs
- server.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- ChannelRef
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
- Timeline
- call
- dmxcraft
- app/src/audio.rs
- t
- ShowError
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
- OrbitCamera
- steps
- window.rs
- stage.rs
- core/src/venue.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- control/timeline.rs
- theme.rs
- OpenDmx
- titlebar.rs
- ShutterEffect
- profile.rs
- app/src/obs.rs
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
- EffectKind
- arc
- Snapshot
- app/src/profiles.rs
- bridge.rs
- duration
- Shape
- Object
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
- VizView
- i18n/mod.rs
- mcp/src/venue.rs
- IconButton
- stage_canvas.rs
- DmxFrame
- icon_library.rs
- DeviceModel
- .ui
- ClipContent
- Tab
- time_field.rs
- TouchView
- Drag
- text_dim
- Theme
- .update
- text_field
- Profile
- Model
- control/tracking.rs
- Grip
- Transform
- Problem
- views/touch.rs
- mcp.rs
- MoveShape
- core/src/obs.rs
- ObsSettings
- .add_scene
- midi.rs
- tools.rs
- Prepare
- ClipAction
- LookPart
- fader.rs
- scene_props.rs
- range_slider.rs
- Engine
- plan
- PlanShape
- scene_group.rs
- super
- FixtureType
- Easing
- env
- lock.rs
- de.rs
- Knob<'a>
- Purpose
- HeaderAction
- build-local-artifacts
- TouchKind
- AppState
- DeckState
- ViewCtx
- UpdateBadge
- FooterButton<'a>
- Kind
- views/sound.rs
- announce
- dip.rs
- qxf.rs
- release.ps1
- desktop.rs
- apply_pick
- pointer
- video_out.rs
- release-install.md
- make
- ok
- PickTarget
- with
- core/src/lib.rs
- .paint
- Obs
- FadeCurve
- .fade_state
- Layer
- Box<dyn serialport::SerialPort>
- Settings

## God Nodes (most connected - your core abstractions)
1. `Engine` - 249 edges
2. `t()` - 226 edges
3. `Show` - 176 edges
4. `Icon` - 119 edges
5. `AppState` - 112 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 63 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/tracking.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (312 total, 144 thin omitted)

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
Cohesion: 0.13
Nodes (17): address_map_marks_ranges(), aim_moving_head_respects_limits(), FixtureLimits, groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), move_zone_reorders_banks() (+9 more)

### Community 5 - "programmer.rs"
Cohesion: 0.07
Nodes (46): ProgrammerMode, Blind, Edit, Live, kind_color(), Category, All, Beam (+38 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.15
Nodes (33): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+25 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.07
Nodes (25): apply_placement(), renumber(), targets(), apply_texture(), TexAction, Builtin, Load, Set (+17 more)

### Community 11 - "assist.rs"
Cohesion: 0.12
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 12 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 13 - "viz/library.rs"
Cohesion: 0.12
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

### Community 20 - "ChannelRef"
Cohesion: 0.11
Nodes (6): blending_does_not_depend_on_the_order_of_values(), def_is_strobe(), lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, ChannelRef

### Community 21 - "model.rs"
Cohesion: 0.17
Nodes (7): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Texture

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.16
Nodes (23): Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), cube() (+15 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.06
Nodes (42): Overlay, MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key() (+34 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.11
Nodes (36): t(), builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui() (+28 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (10): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), fill_details(), fill_physical(), Library, LibraryFixture, Source (+2 more)

### Community 31 - "Timeline"
Cohesion: 0.15
Nodes (6): has_audio(), media_track(), ungroup_clip(), Clip, linked_blocks_stay_together(), Timeline

### Community 32 - "call"
Cohesion: 0.19
Nodes (24): add_clips(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), base64(), build_steps(), call(), changed_scene() (+16 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.16
Nodes (14): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+6 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (42): capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H, full_combo(), general() (+34 more)

### Community 37 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (33): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+25 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (51): select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+43 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.06
Nodes (25): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+17 more)

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
Cohesion: 0.17
Nodes (26): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+18 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.11
Nodes (20): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+12 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "OrbitCamera"
Cohesion: 0.07
Nodes (18): OrbitCamera, ray_through_center_hits_target(), Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF (+10 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (19): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.19
Nodes (16): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+8 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.09
Nodes (16): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, moving_head_aim() (+8 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (9): .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+1 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.10
Nodes (21): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+13 more)

### Community 77 - "engine.rs"
Cohesion: 0.17
Nodes (43): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+35 more)

### Community 78 - "control/timeline.rs"
Cohesion: 0.09
Nodes (23): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+15 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (54): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+46 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.09
Nodes (20): slot_label(), ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse (+12 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (26): camera_json(), add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity() (+18 more)

### Community 90 - "app/src/obs.rs"
Cohesion: 0.17
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.12
Nodes (32): decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles, handles() (+24 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.16
Nodes (22): asphalt(), brick(), carpet(), checker(), cobblestone(), concrete(), fbm(), light_wood() (+14 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 99 - "MapPreset"
Cohesion: 0.07
Nodes (31): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+23 more)

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
Nodes (12): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, Kind (+4 more)

### Community 105 - "target_shape"
Cohesion: 0.20
Nodes (15): alt_click(), button(), button_rect(), fader(), fader_channels(), in_rotated(), report(), rotated_quad() (+7 more)

### Community 106 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.26
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "app/src/profiles.rs"
Cohesion: 0.21
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 110 - "bridge.rs"
Cohesion: 0.26
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.18
Nodes (15): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+7 more)

### Community 117 - "Object"
Cohesion: 0.14
Nodes (16): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+8 more)

### Community 118 - "mapping/mod.rs"
Cohesion: 0.13
Nodes (28): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fires_id(), fixed_channels_work_without_selection(), group_set(), index_targets(), mapping() (+20 more)

### Community 119 - ".row"
Cohesion: 0.09
Nodes (26): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+18 more)

### Community 120 - "Tab"
Cohesion: 0.15
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (33): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+25 more)

### Community 123 - "src/timeline.rs"
Cohesion: 0.15
Nodes (12): Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others() (+4 more)

### Community 127 - "widgets/mod.rs"
Cohesion: 0.18
Nodes (10): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+2 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 129 - "FixtureType"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 130 - "app.rs"
Cohesion: 0.20
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 131 - "egui"
Cohesion: 0.05
Nodes (36): text_field(), DipSwitch, Knob, SIZE, SWEEP, LimitPad, RangeSlider, CARD_H (+28 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 136 - "VizView"
Cohesion: 0.07
Nodes (19): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+11 more)

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
Cohesion: 0.14
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "DmxFrame"
Cohesion: 0.20
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 143 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 145 - ".ui"
Cohesion: 0.14
Nodes (15): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+7 more)

### Community 146 - "ClipContent"
Cohesion: 0.13
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 147 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 149 - "TouchView"
Cohesion: 0.19
Nodes (10): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, prepare_image() (+2 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (27): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+19 more)

### Community 152 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 153 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.10
Nodes (18): MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R (+10 more)

### Community 159 - "Model"
Cohesion: 0.18
Nodes (3): Model, ModelPart, part_roundtrip_through_json()

### Community 160 - "control/tracking.rs"
Cohesion: 0.12
Nodes (11): Show, bar_row(), color(), edit(), TrackingView, AXIS_COLORS, DEFAULT_HEIGHT, one() (+3 more)

### Community 163 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.13
Nodes (21): problem_text(), problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode (+13 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.10
Nodes (34): is_on(), sound_volume(), apply(), find_item(), state_json(), channels_of(), COLORS, content_rect() (+26 more)

### Community 167 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 169 - "core/src/obs.rs"
Cohesion: 0.31
Nodes (7): bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length()

### Community 170 - "ObsSettings"
Cohesion: 0.25
Nodes (5): Link, ObsSettings, VideoOutput, Browser, Media

### Community 171 - ".add_scene"
Cohesion: 0.10
Nodes (11): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank() (+3 more)

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (34): add_effect(), any_obj(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values(), existing_fixtures() (+26 more)

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "LookPart"
Cohesion: 0.22
Nodes (9): LookAction, Copy, Paste, part_label(), LookPart, All, Color, Intensity (+1 more)

### Community 182 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (38): advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header(), effect_tiles() (+30 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (4): Engine, preview_effects_only_run_when_active(), Track, TrackLight

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
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 193 - "FixtureType"
Cohesion: 0.10
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 195 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 203 - "Purpose"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+9 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 225 - "ViewCtx"
Cohesion: 0.10
Nodes (14): fold_section(), View, ViewCtx, body(), caption(), FixtureSettings, hanging_of(), range_row() (+6 more)

### Community 226 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 229 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 230 - "views/sound.rs"
Cohesion: 0.19
Nodes (18): AUDIO_EXT, BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name(), deck_ui() (+10 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 239 - "apply_pick"
Cohesion: 0.47
Nodes (6): channel_name(), describe(), describe_mapping(), t_deleted(), apply_pick(), pick_adds_scene_button_and_assigns_to_selection()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "make"
Cohesion: 0.40
Nodes (5): builtin(), builtins_are_seamless_and_decode(), make(), Pattern, render()

### Community 252 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 256 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (16): cv(), demo_show(), empty_show(), ensure_builtin_types(), SceneProgress, ChannelValue, LoopMode, Loop (+8 more)

### Community 281 - "Obs"
Cohesion: 0.14
Nodes (10): Obs, run(), session(), Shared, Status, Connected, Connecting, Failed (+2 more)

### Community 285 - "FadeCurve"
Cohesion: 0.22
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 291 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 310 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

## Knowledge Gaps
- **574 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+569 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1475 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **144 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app.rs`, `egui`, `runtime.rs`, `programmer.rs`, `core/src/lib.rs`, `mcp/src/venue.rs`, `.ui`, `ChannelRef`, `text_dim`, `rig.rs`, `.update`, `Obs`, `Host`, `Profile`, `control/tracking.rs`, `call`, `app/src/audio.rs`, `.fade_state`, `t`, `views/touch.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `audio_trigger_ui.rs`, `remote/mod.rs`, `scene_props.rs`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `control/timeline.rs`, `app/src/obs.rs`, `Mapper`, `ViewCtx`, `audio_triggers.rs`, `Snapshot`, `apply_pick`, `mapping/mod.rs`?**
  _High betweenness centrality (0.090) - this node is a cross-community bridge._
- **Are the 193 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 193 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _574 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `library_panel.rs`, `app.rs`, `show.rs`, `SoundBoard`, `fx.rs`, `core/src/lib.rs`, `props.rs`, `stage_canvas.rs`, `ChannelRef`, `TouchView`, `rig.rs`, `Control`, `control/tracking.rs`, `call`, `ShowError`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `ObsSettings`, `missing_audio.rs`, `.add_scene`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `scene_props.rs`, `Engine`, `scene_group.rs`, `OrbitCamera`, `feature.rs`, `motion.rs`, `engine.rs`, `theme.rs`, `viz/mod.rs`, `autosave.rs`, `EffectKind`, `Snapshot`, `apply_pick`, `Object`, `mapping/mod.rs`, `src/timeline.rs`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11942959001782531 - nodes in this community are weakly interconnected._