# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 165 files · ~264,799 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4607 nodes · 11784 edges · 319 communities (171 shown, 148 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 627 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7a40c112`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Venue
- programmer.rs
- SoundBoard
- steps
- fx.rs
- DmxOutput
- props.rs
- assist.rs
- control/timeline.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- Step
- model.rs
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Control
- fx_ui.rs
- protocol.rs
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
- snap.rs
- steps
- window.rs
- stage.rs
- core/src/venue.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- toolbar.rs
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
- fx/mapping.rs
- MapPreset
- audio_triggers.rs
- autosave.rs
- system_fonts.rs
- Dialog<'a>
- mapping/mod.rs
- server.rs
- arc
- Placement
- app/src/profiles.rs
- OrbitCamera
- duration
- Shape
- Object
- fire
- .row
- Tab
- convert.rs
- TouchItem
- widgets/mod.rs
- ProfileBuilder
- FixtureType
- UpdateBadge
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
- panels.rs
- Tab
- time_field.rs
- TouchView
- Drag
- text_dim
- Timeline
- .update
- text_field
- mcp/src/profiles.rs
- qlc-bundle.rs
- control/tracking.rs
- Capability
- Transform
- Problem
- views/touch.rs
- mcp.rs
- fixture_settings.rs
- core/src/obs.rs
- Snapshot
- Scene
- midi.rs
- tools.rs
- scene_card.rs
- ClipAction
- Deserialize
- paint_symbol
- scene_props.rs
- range_slider.rs
- Engine
- plan
- T
- scene_group.rs
- super
- FixtureType
- show
- env
- lock.rs
- de.rs
- Knob<'a>
- Calibrate
- Purpose
- HeaderAction
- build-local-artifacts
- TouchKind
- AppState
- DeckState
- View
- Mapper
- FooterButton<'a>
- palettes.rs
- views/sound.rs
- announce
- SlotKind
- qxf.rs
- release.ps1
- desktop.rs
- Category
- pointer
- video_out.rs
- release-install.md
- Layer
- ok
- PlanShape
- with
- core/src/lib.rs
- .paint
- Obs
- Clip
- Shape
- ProgrammerMode
- dip.rs
- .fine_names
- Cmd
- widgets/mod.rs
- elide
- resize_handles
- settings.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 254 edges
2. `t()` - 232 edges
3. `Show` - 180 edges
4. `Icon` - 119 edges
5. `AppState` - 112 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 78 edges
8. `ChannelRef` - 66 edges
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

## Communities (319 total, 148 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (17): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+9 more)

### Community 4 - "Venue"
Cohesion: 0.12
Nodes (8): Environment, fixture_folders_are_exclusive(), FixtureFolder, Texture, mul(), room_and_tree(), Transform, Venue

### Community 5 - "programmer.rs"
Cohesion: 0.27
Nodes (16): category_list(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+8 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+20 more)

### Community 9 - "DmxOutput"
Cohesion: 0.14
Nodes (6): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.13
Nodes (19): fold_section(), ViewCtx, apply_placement(), renumber(), VizView, aim_fixtures(), button(), color_field() (+11 more)

### Community 11 - "assist.rs"
Cohesion: 0.13
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 12 - "control/timeline.rs"
Cohesion: 0.09
Nodes (23): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+15 more)

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
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 20 - "Step"
Cohesion: 0.27
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 21 - "model.rs"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

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

### Community 27 - "Control"
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.11
Nodes (30): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), ROW_H, slider_row(), toggle_row() (+22 more)

### Community 29 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "Timeline"
Cohesion: 0.11
Nodes (19): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+11 more)

### Community 32 - "call"
Cohesion: 0.21
Nodes (27): add_clips(), add_effect(), add_fixtures(), apply_clip_fields(), apply_scene_fields(), call(), changed_scene(), create_scene() (+19 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.16
Nodes (14): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+6 more)

### Community 36 - "t"
Cohesion: 0.08
Nodes (44): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+36 more)

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
Cohesion: 0.07
Nodes (28): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+20 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (52): select_all(), text_field(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete (+44 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.15
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.12
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.12
Nodes (9): ControlViews, DmxCraftApp, edit_preview(), fill_types(), load_step(), LockDialog, moved_out(), OUTPUT_HZ (+1 more)

### Community 48 - "Ui"
Cohesion: 0.14
Nodes (28): MidiDevice, hidden_tabs_from_old_setting(), Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo() (+20 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.10
Nodes (22): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+14 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (24): Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.13
Nodes (11): aim_at_points_beam_to_target(), apply(), close(), hang_below_nearest_truss(), IDENTITY, moving_head_aim(), moving_head_direction(), rotation() (+3 more)

### Community 72 - "Show"
Cohesion: 0.08
Nodes (27): builtin_fixture_types(), address_map_marks_ranges(), aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, duplicate_zone_copies_scenes_behind_original(), Group, groups_follow_fixture_removal(), json_roundtrip() (+19 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.19
Nodes (7): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation, list_ports(), UsbDmxPort

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (35): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+27 more)

### Community 77 - "engine.rs"
Cohesion: 0.15
Nodes (46): blending_does_not_depend_on_the_order_of_values(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below() (+38 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.23
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.04
Nodes (53): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, CONTRAST, DANGER (+45 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "titlebar.rs"
Cohesion: 0.21
Nodes (20): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+12 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (21): camera_json(), beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+13 more)

### Community 90 - "app/src/obs.rs"
Cohesion: 0.17
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.10
Nodes (36): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag, Handles (+28 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.12
Nodes (28): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+20 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.09
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.19
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 104 - "Dialog<'a>"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "mapping/mod.rs"
Cohesion: 0.11
Nodes (37): alt_click(), button(), button_rect(), channel_name(), color(), describe(), describe_mapping(), fader() (+29 more)

### Community 106 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "app/src/profiles.rs"
Cohesion: 0.11
Nodes (13): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+5 more)

### Community 112 - "duration"
Cohesion: 0.18
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Object"
Cohesion: 0.24
Nodes (9): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+1 more)

### Community 118 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), mapping(), resolve() (+5 more)

### Community 119 - ".row"
Cohesion: 0.09
Nodes (26): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+18 more)

### Community 120 - "Tab"
Cohesion: 0.15
Nodes (12): SettingsWindow, Tab, Ai, .ALL, Audio, Customize, General, Midi (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "TouchItem"
Cohesion: 0.23
Nodes (13): is_on(), sound_volume(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value() (+5 more)

### Community 127 - "widgets/mod.rs"
Cohesion: 0.15
Nodes (12): MenuPlace, Patch, Stage, StageEditor, Arrange, AlignH, AlignV, Grid (+4 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.17
Nodes (8): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, drag_ghost()

### Community 129 - "FixtureType"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 130 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 131 - "egui"
Cohesion: 0.07
Nodes (24): DipSwitch, SIZE, SWEEP, LimitPad, Swatch, Toggle, CHIP_H, GROUP_COLOR (+16 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "VizView"
Cohesion: 0.09
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+6 more)

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

### Community 143 - "icon_library.rs"
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 144 - "DeviceModel"
Cohesion: 0.15
Nodes (9): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), nearest() (+1 more)

### Community 145 - ".ui"
Cohesion: 0.15
Nodes (14): drop_hint(), fmt_ms(), grid_options(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_lead() (+6 more)

### Community 146 - "panels.rs"
Cohesion: 0.12
Nodes (12): Segmented, End, Measure, snap_points(), SNAP_PX, OutlineTab, Fixtures, Objects (+4 more)

### Community 147 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 149 - "TouchView"
Cohesion: 0.15
Nodes (13): section_header(), add_page(), icon_picker(), ImageTarget, Item, Page, pick_image(), prepare_image() (+5 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (38): window(), accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border() (+30 more)

### Community 152 - "Timeline"
Cohesion: 0.17
Nodes (7): has_audio(), media_track(), GridMode, Beat, Time, linked_blocks_stay_together(), Timeline

### Community 153 - ".update"
Cohesion: 0.22
Nodes (11): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws (+3 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "control/tracking.rs"
Cohesion: 0.23
Nodes (6): bar_row(), color(), edit(), TrackingView, Show, Tracker

### Community 163 - "Capability"
Cohesion: 0.17
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.14
Nodes (20): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.15
Nodes (19): apply(), channels_of(), COLORS, current_color(), current_position(), Drag, Move, Resize (+11 more)

### Community 167 - "mcp.rs"
Cohesion: 0.11
Nodes (18): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+10 more)

### Community 168 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), hanging_of() (+1 more)

### Community 169 - "core/src/obs.rs"
Cohesion: 0.17
Nodes (9): Link, bx(), find_box(), mp4_duration_ms(), ObsSettings, reads_mp4_length(), VideoOutput, Browser (+1 more)

### Community 170 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "Scene"
Cohesion: 0.08
Nodes (15): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+7 more)

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (33): Prepare, any_obj(), base64(), boolean(), build_steps(), camera_schema(), channel_kind(), clip_schema() (+25 more)

### Community 177 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "Deserialize"
Cohesion: 0.29
Nodes (3): DEFAULT_HEIGHT, one(), white()

### Community 182 - "paint_symbol"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (46): time_field(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+38 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (7): def_is_strobe(), Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, SceneProgress, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 191 - "scene_group.rs"
Cohesion: 0.20
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 192 - "super"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 193 - "FixtureType"
Cohesion: 0.10
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 195 - "show"
Cohesion: 0.20
Nodes (10): audio_control(), AudioInfo, load_meter(), AudioMaster, Mapping, ShowTab, missing_alert(), show() (+2 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "TouchKind"
Cohesion: 0.18
Nodes (8): add_tile(), TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 218 - "AppState"
Cohesion: 0.07
Nodes (18): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+10 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 225 - "View"
Cohesion: 0.15
Nodes (6): View, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 229 - "palettes.rs"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 234 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 239 - "Category"
Cohesion: 0.20
Nodes (10): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+2 more)

### Community 240 - "pointer"
Cohesion: 0.25
Nodes (8): hovering(), os_cursor(), pointer(), Grip, Body, Corner, EdgeX, EdgeY

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "Layer"
Cohesion: 0.31
Nodes (8): color_bar(), current(), fixtures_of(), set_group(), xy_pad(), Layer, Live, Programmer

### Community 252 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 256 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "core/src/lib.rs"
Cohesion: 0.09
Nodes (17): FadeState, In, Out, BankScope, All, Bank, Banks, Off (+9 more)

### Community 281 - "Obs"
Cohesion: 0.14
Nodes (10): Obs, run(), session(), Shared, Status, Connected, Connecting, Failed (+2 more)

### Community 285 - "Clip"
Cohesion: 0.11
Nodes (11): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+3 more)

### Community 288 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 289 - "ProgrammerMode"
Cohesion: 0.25
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 291 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 303 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 305 - "widgets/mod.rs"
Cohesion: 0.40
Nodes (4): CanvasMode, Edit, Select, XyPad

### Community 316 - "settings.rs"
Cohesion: 0.18
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), last_tab_stays(), RECENT_MAX

## Knowledge Gaps
- **575 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+570 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1482 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **148 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `SoundPlayer`, `library_panel.rs`, `Venue`, `fx.rs`, `core/src/lib.rs`, `stage_canvas.rs`, `panels.rs`, `Step`, `TouchView`, `rig.rs`, `Control`, `control/tracking.rs`, `call`, `ShowError`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `missing_audio.rs`, `core/src/obs.rs`, `Scene`, `DmxCraftApp`, `tools.rs`, `TouchLayout`, `Deserialize`, `scene_props.rs`, `Engine`, `scene_group.rs`, `snap.rs`, `feature.rs`, `motion.rs`, `engine.rs`, `theme.rs`, `viz/mod.rs`, `autosave.rs`, `mapping/mod.rs`, `Placement`, `Object`, `fire`, `TouchItem`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **Are the 197 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 197 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _575 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12222222222222222 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `ProfileBuilder`, `library_panel.rs`, `egui`, `programmer.rs`, `VizView`, `i18n/mod.rs`, `props.rs`, `assist.rs`, `control/timeline.rs`, `viz/library.rs`, `.ui`, `panels.rs`, `TouchView`, `text_dim`, `text_field`, `fx_ui.rs`, `control/tracking.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `fixture_settings.rs`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `audio_trigger_ui.rs`, `paint_symbol`, `scene_props.rs`, `snap.rs`, `show`, `window.rs`, `stage.rs`, `Calibrate`, `Purpose`, `toolbar.rs`, `titlebar.rs`, `viz/mod.rs`, `textures.rs`, `View`, `palettes.rs`, `views/sound.rs`, `Dialog<'a>`, `mapping/mod.rs`, `app/src/profiles.rs`, `Layer`, `.row`, `TouchItem`, `widgets/mod.rs`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10793650793650794 - nodes in this community are weakly interconnected._