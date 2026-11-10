# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 150 files · ~231,564 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4201 nodes · 11005 edges · 277 communities (157 shown, 120 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 551 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9d92f9ff`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- call
- programmer.rs
- SoundBoard
- steps
- fx.rs
- OutputError
- ViewCtx
- heading_text
- server.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- fx/mapping.rs
- Deserialize
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Control
- RangeEffect
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- Profile
- dmxcraft
- app/src/audio.rs
- t
- audio_trigger_ui.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- scene_grid.rs
- duration
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- Snapshot
- remote/mod.rs
- TouchLayout
- Dialog<'a>
- MenuAction
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- fixture_settings.rs
- sense
- OpenDmx
- range_slider.rs
- fire
- profile.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- textures.rs
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- autosave.rs
- show.rs
- look.rs
- gltf_import.rs
- ShutterEffect
- arc
- updater.rs
- TimelineView
- app.rs
- core/src/lib.rs
- Shape
- Venue
- stage_editor.rs
- .row
- scene_card.rs
- convert.rs
- Timeline
- Host
- app/src/profiles.rs
- qxf.rs
- app/src/obs.rs
- Theme
- CurveShape
- ColorPreset
- TouchItem
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- theme.rs
- stage_canvas.rs
- Tab
- missing_audio.rs
- DeviceModel
- ChaserPreset
- Mapper
- Capability
- qlc-bundle.rs
- TouchView
- mapping/mod.rs
- text_dim
- UpdateBadge
- icon_library.rs
- WaveState
- Fader
- ProfileChannel
- .add_scene
- EffectKind
- dip.rs
- mcp/src/profiles.rs
- Problem
- views/touch.rs
- AppState
- GridMode
- paint_clip
- SlotKind
- ok
- palettes.rs
- midi.rs
- Category
- tools.rs
- time_field.rs
- DmxFrame
- ClipAction
- DeckState
- Transform
- fader.rs
- scene_props.rs
- TouchKind
- Engine
- ProgrammerMode
- plan
- plan.rs
- Settings
- env
- elide
- de.rs
- resize_handles
- ShowError
- MenuItem<'a>
- Purpose
- toolbar.rs
- HeaderAction
- build-local-artifacts
- IconButton
- Scene
- Knob<'a>
- FooterButton<'a>
- T
- views/sound.rs
- announce
- Grip
- release.ps1
- desktop.rs
- pointer
- release-install.md
- AudioPlayer
- with
- .fade_state

## God Nodes (most connected - your core abstractions)
1. `Engine` - 230 edges
2. `t()` - 210 edges
3. `Show` - 152 edges
4. `Icon` - 115 edges
5. `AppState` - 103 edges
6. `ViewCtx` - 90 edges
7. `ChannelKind` - 73 edges
8. `text_dim()` - 71 edges
9. `ChannelRef` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (277 total, 120 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.10
Nodes (17): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+9 more)

### Community 4 - "call"
Cohesion: 0.21
Nodes (25): add_clips(), add_effect(), base64(), build_steps(), call(), changed_scene(), create_scene(), create_super_scene() (+17 more)

### Community 5 - "programmer.rs"
Cohesion: 0.19
Nodes (23): color_bar(), current(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks() (+15 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer, Failing

### Community 10 - "ViewCtx"
Cohesion: 0.09
Nodes (18): fold_section(), ViewCtx, DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue (+10 more)

### Community 11 - "heading_text"
Cohesion: 0.25
Nodes (4): heading_text(), drag_row(), Folder, VizView

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
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.13
Nodes (30): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+22 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 21 - "Deserialize"
Cohesion: 0.09
Nodes (9): Physics, strobe_ranges(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.10
Nodes (34): handles(), project(), ray_aabb_exit(), ray_box(), ray_sphere(), ray_tests(), slab(), Beam (+26 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "RangeEffect"
Cohesion: 0.17
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 29 - "profile_builder.rs"
Cohesion: 0.11
Nodes (10): Button, Dialog, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+2 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.14
Nodes (24): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+16 more)

### Community 32 - "Profile"
Cohesion: 0.15
Nodes (4): MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (55): t(), text_field(), kind_color(), capability_fields(), category_label(), check(), color_label(), FileJob (+47 more)

### Community 37 - "audio_trigger_ui.rs"
Cohesion: 0.13
Nodes (16): dropdown(), apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick (+8 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.15
Nodes (11): run_updater(), Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (39): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+31 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.06
Nodes (62): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+54 more)

### Community 43 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.23
Nodes (23): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+15 more)

### Community 50 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Dialog<'a>"
Cohesion: 0.14
Nodes (5): Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (24): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+16 more)

### Community 71 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (8): .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.14
Nodes (17): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+9 more)

### Community 74 - "OutputKind"
Cohesion: 0.19
Nodes (7): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation, list_ports(), UsbDmxPort

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.18
Nodes (37): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+29 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.16
Nodes (10): Show, Quick, quick_action(), Center, Off, On, body(), caption() (+2 more)

### Community 79 - "sense"
Cohesion: 0.06
Nodes (28): DipSwitch, SIZE, SWEEP, LimitPad, ICON_SLOT, ITEM_H, MIN_W, style() (+20 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.16
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), value_field(), time_field(), time_field_compact(), time_field_ex()

### Community 87 - "fire"
Cohesion: 0.20
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 89 - "profile.rs"
Cohesion: 0.11
Nodes (19): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), file_slug(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+11 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (32): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+24 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.18
Nodes (17): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+9 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.09
Nodes (16): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+8 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.16
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "show.rs"
Cohesion: 0.16
Nodes (14): address_map_marks_ranges(), FixtureLimits, groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range() (+6 more)

### Community 104 - "look.rs"
Cohesion: 0.18
Nodes (19): ChannelDef, FixtureType, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel (+11 more)

### Community 105 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 106 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "updater.rs"
Cohesion: 0.16
Nodes (17): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+9 more)

### Community 109 - "TimelineView"
Cohesion: 0.16
Nodes (7): Drag, FadeIn, FadeOut, Left, Move, Right, TimelineView

### Community 110 - "app.rs"
Cohesion: 0.12
Nodes (15): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView (+7 more)

### Community 112 - "core/src/lib.rs"
Cohesion: 0.09
Nodes (18): UNIVERSE_SIZE, BankScope, All, Bank, Banks, Off, OutsideBank, Group (+10 more)

### Community 113 - "Shape"
Cohesion: 0.11
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (20): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+12 more)

### Community 118 - "stage_editor.rs"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 119 - ".row"
Cohesion: 0.10
Nodes (25): DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+17 more)

### Community 120 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (27): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+19 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (16): has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video (+8 more)

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 129 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.07
Nodes (40): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+32 more)

### Community 131 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "TouchItem"
Cohesion: 0.26
Nodes (11): state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), paint_cover() (+3 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (24): add_audio(), add_audio_dialog(), add_video(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms() (+16 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.12
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (53): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+45 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.16
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.12
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 145 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 146 - "Mapper"
Cohesion: 0.18
Nodes (8): fires_id(), is_value_control(), Mapper, overlay_id(), pick_id(), Sent, states_id(), ui_states()

### Community 147 - "Capability"
Cohesion: 0.17
Nodes (3): cap_for_kind(), Capability, last_matching()

### Community 148 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 149 - "TouchView"
Cohesion: 0.14
Nodes (14): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+6 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.15
Nodes (23): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+15 more)

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (37): window(), accent(), accent_dim(), apply(), bg(), body(), border(), caption() (+29 more)

### Community 152 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 153 - "icon_library.rs"
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 157 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 158 - "ProfileChannel"
Cohesion: 0.29
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 159 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.14
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 167 - "AppState"
Cohesion: 0.08
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 168 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 169 - "paint_clip"
Cohesion: 0.32
Nodes (5): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 170 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 171 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 172 - "palettes.rs"
Cohesion: 0.36
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 174 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.13
Nodes (30): add_fixtures(), any_obj(), apply_scene_fields(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+22 more)

### Community 177 - "time_field.rs"
Cohesion: 0.31
Nodes (8): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 181 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 182 - "fader.rs"
Cohesion: 0.14
Nodes (12): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, HANDLE (+4 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (40): button(), zone_color(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), fit_mapping() (+32 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 187 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 189 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 194 - "Settings"
Cohesion: 0.11
Nodes (7): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings, SettingsCx

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "toolbar.rs"
Cohesion: 0.14
Nodes (14): surface(), begin(), context(), IconFn, item(), MenuItem, separator(), sub() (+6 more)

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 213 - "Scene"
Cohesion: 0.16
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Scene, Step

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 251 - "AudioPlayer"
Cohesion: 0.16
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 261 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **552 isolated node(s):** `Select`, `Edit`, `Cube`, `Cylinder`, `Par` (+547 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1342 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **120 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `app/src/profiles.rs`, `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `call`, `programmer.rs`, `TouchItem`, `.fade_state`, `control/timeline.rs`, `ViewCtx`, `mcp/src/venue.rs`, `Mapper`, `text_dim`, `rig.rs`, `app/src/audio.rs`, `t`, `audio_trigger_ui.rs`, `views/touch.rs`, `mcp/src/profiles.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `scene_props.rs`, `Settings`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `toolbar.rs`, `engine.rs`, `fixture_settings.rs`, `fire`, `ChannelKind`, `audio_triggers.rs`, `TimelineView`, `app.rs`, `core/src/lib.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **Are the 181 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 181 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `Cube` to the rest of the system?**
  _552 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `app/src/profiles.rs`, `library_panel.rs`, `app/src/obs.rs`, `programmer.rs`, `TouchItem`, `control/timeline.rs`, `i18n/mod.rs`, `ViewCtx`, `heading_text`, `viz/library.rs`, `missing_audio.rs`, `titlebar.rs`, `TouchView`, `text_dim`, `profile_builder.rs`, `Fader`, `fx_ui.rs`, `audio_trigger_ui.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `palettes.rs`, `DmxCraftApp`, `settings_window.rs`, `fader.rs`, `scene_props.rs`, `Settings`, `window.rs`, `stage.rs`, `Purpose`, `toolbar.rs`, `fixture_settings.rs`, `sense`, `fire`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `app.rs`, `core/src/lib.rs`, `stage_editor.rs`, `.row`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10793650793650794 - nodes in this community are weakly interconnected._