# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 150 files · ~230,638 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4195 nodes · 11005 edges · 274 communities (160 shown, 114 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 549 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `179b5563`
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
- ViewCtx
- server.rs
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- fx/mapping.rs
- textures.rs
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
- app/src/profiles.rs
- dmxcraft
- app/src/audio.rs
- t
- scene_grid.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- color_picker.rs
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
- Layer
- sense
- OpenDmx
- range_slider.rs
- fire
- profile.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- Action
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
- src/mapping.rs
- TimelineView
- app.rs
- core/src/lib.rs
- Shape
- Venue
- egui
- .row
- scene_card.rs
- convert.rs
- Timeline
- Host
- Arrange
- bank_menu
- app/src/obs.rs
- .page_bar
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
- FixtureLimits
- TouchView
- mapping/mod.rs
- text_dim
- UpdateBadge
- icon_library.rs
- WaveState
- patch_grid.rs
- ClipContent
- .add_scene
- EffectKind
- dip.rs
- settings_window.rs
- Problem
- views/touch.rs
- AppState
- src/timeline.rs
- .update
- Obs
- ok
- Drag
- midi.rs
- Category
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- DmxLine
- ObsSettings
- fader.rs
- scene_props.rs
- TouchKind
- Engine
- .update
- ObsAction
- plan.rs
- Settings
- env
- de.rs
- ShowError
- MenuItem<'a>
- Purpose
- toolbar.rs
- HeaderAction
- build-local-artifacts
- IconButton
- Scene
- lock.rs
- Knob<'a>
- Swatch
- FooterButton<'a>
- T
- views/sound.rs
- Deserialize
- GH_TOKEN
- Grip
- release.ps1
- CanvasMode
- desktop.rs
- pointer
- release-install.md
- OutputStatus
- AudioPlayer
- with
- .check_placement
- .fade_state

## God Nodes (most connected - your core abstractions)
1. `Engine` - 230 edges
2. `t()` - 210 edges
3. `Show` - 151 edges
4. `Icon` - 115 edges
5. `AppState` - 103 edges
6. `ViewCtx` - 90 edges
7. `ChannelKind` - 75 edges
8. `text_dim()` - 74 edges
9. `ChannelRef` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (274 total, 114 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.10
Nodes (8): DeckState, Empty, Ended, Fading, Paused, Playing, Ready, SoundPlayer

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.14
Nodes (16): BREAK, DMX_BAUD, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB (+8 more)

### Community 4 - "call"
Cohesion: 0.24
Nodes (24): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), changed_scene(), create_scene(), create_super_scene() (+16 more)

### Community 5 - "programmer.rs"
Cohesion: 0.16
Nodes (23): color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+15 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+21 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.09
Nodes (15): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, vec3_row(), apply_texture() (+7 more)

### Community 11 - "ViewCtx"
Cohesion: 0.16
Nodes (8): ViewCtx, proposals_for_easy_view_names(), propose_type(), renumber(), drop_marker(), Folder, VizView, VizView

### Community 12 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.12
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "with"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.18
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 21 - "textures.rs"
Cohesion: 0.08
Nodes (25): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+17 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.07
Nodes (44): analyze(), Analyzer, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad, BPM_MAX (+36 more)

### Community 24 - "rig.rs"
Cohesion: 0.11
Nodes (26): Beam, beam(), build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of() (+18 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "Control"
Cohesion: 0.08
Nodes (24): Overlay, Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+16 more)

### Community 28 - "RangeEffect"
Cohesion: 0.17
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 29 - "profile_builder.rs"
Cohesion: 0.10
Nodes (10): Button, Dialog, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+2 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.08
Nodes (28): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+20 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.07
Nodes (43): builtin(), load_user(), named(), NamedPalette, path(), save_user(), dropdown(), item() (+35 more)

### Community 32 - "app/src/profiles.rs"
Cohesion: 0.08
Nodes (24): AppProfiles, delete(), file(), load_all(), load_user(), sort(), sort_user(), today() (+16 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (13): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+5 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (57): t(), save(), heading_text(), text_field(), kind_color(), capability_fields(), category_label(), check() (+49 more)

### Community 37 - "scene_grid.rs"
Cohesion: 0.19
Nodes (11): programmer_values(), MoveScenes, apply(), apply_bank(), chip(), COLUMN_W, drag_ghost(), drop_target() (+3 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "color_picker.rs"
Cohesion: 0.20
Nodes (13): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+5 more)

### Community 43 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.32
Nodes (17): ai(), audio(), customize(), full_combo(), general(), hint(), midi(), obs() (+9 more)

### Community 50 - "Snapshot"
Cohesion: 0.26
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Dialog<'a>"
Cohesion: 0.14
Nodes (5): Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 69 - "window.rs"
Cohesion: 0.13
Nodes (19): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.14
Nodes (17): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+9 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (37): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+29 more)

### Community 78 - "Layer"
Cohesion: 0.08
Nodes (14): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+6 more)

### Community 79 - "sense"
Cohesion: 0.05
Nodes (28): DipSwitch, Knob, SIZE, SWEEP, LimitPad, RangeSlider, fold_section(), section_header() (+20 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 87 - "fire"
Cohesion: 0.20
Nodes (15): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+7 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (26): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+18 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (37): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+29 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+20 more)

### Community 95 - "Action"
Cohesion: 0.19
Nodes (19): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+11 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.12
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, OrbitCamera (+2 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.11
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 104 - "look.rs"
Cohesion: 0.16
Nodes (20): ChannelDef, FixtureType, Physics, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget (+12 more)

### Community 105 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 106 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "src/mapping.rs"
Cohesion: 0.10
Nodes (24): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback (+16 more)

### Community 109 - "TimelineView"
Cohesion: 0.22
Nodes (7): fmt_ms(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.18
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 112 - "core/src/lib.rs"
Cohesion: 0.08
Nodes (19): addresses_are_one_based(), UNIVERSE_SIZE, SceneProgress, BankScope, All, Bank, Banks, Off (+11 more)

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (18): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+10 more)

### Community 118 - "egui"
Cohesion: 0.12
Nodes (11): View, body(), caption(), FixtureSettings, range_row(), SetupView, set_pos(), StageEditorView (+3 more)

### Community 119 - ".row"
Cohesion: 0.07
Nodes (31): Quality, .ALL, High, Low, Medium, Ultra, drag_row(), DragFixtures (+23 more)

### Community 120 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (27): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+19 more)

### Community 123 - "Timeline"
Cohesion: 0.18
Nodes (4): add_video(), has_audio(), Clip, Timeline

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 129 - "bank_menu"
Cohesion: 0.24
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+5 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.22
Nodes (8): auth_matches_obs_websocket_example(), auth_response(), b64(), open(), read_json(), send_json(), VideoPlay, Ws

### Community 131 - ".page_bar"
Cohesion: 0.31
Nodes (6): Bank, FocusBank, RenameBank, arrow(), hidden_banks_menu(), Zone

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 134 - "TouchItem"
Cohesion: 0.24
Nodes (12): find_item(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of() (+4 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (19): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, is_video() (+11 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.12
Nodes (13): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.05
Nodes (65): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+57 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.11
Nodes (20): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+12 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 145 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 146 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 149 - "TouchView"
Cohesion: 0.20
Nodes (9): add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image(), swatch_row() (+1 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (33): alt_click(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels(), fires_id() (+25 more)

### Community 151 - "text_dim"
Cohesion: 0.17
Nodes (34): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_font() (+26 more)

### Community 152 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 153 - "icon_library.rs"
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 157 - "patch_grid.rs"
Cohesion: 0.23
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 158 - "ClipContent"
Cohesion: 0.29
Nodes (5): ClipContent, Audio, Obs, Scene, Video

### Community 159 - ".add_scene"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, steps_insert_remove_and_normalize(), Zone

### Community 160 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "settings_window.rs"
Cohesion: 0.22
Nodes (8): AUTOSAVE_MINUTES, HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, UsbDmxPort

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.13
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), GAP, GRID_STEPS (+13 more)

### Community 167 - "AppState"
Cohesion: 0.09
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 168 - "src/timeline.rs"
Cohesion: 0.15
Nodes (11): audio_grid_sets_tempo_and_offset(), fades(), GridMode, Beat, Time, group_moves_together_without_covering_others(), one(), paste_finds_free_place_and_keeps_gaps() (+3 more)

### Community 169 - ".update"
Cohesion: 0.33
Nodes (6): placeholders_match(), fire(), media_action(), media_settings(), names(), seek()

### Community 170 - "Obs"
Cohesion: 0.33
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 171 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 172 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.23
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 174 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (31): any_obj(), apply_scene_fields(), base64(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+23 more)

### Community 177 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 178 - "DmxFrame"
Cohesion: 0.13
Nodes (3): DmxFrame, lerp(), PlayingScene

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (40): button(), zone_color(), advanced(), apply_to_others(), banks_of(), block_props(), fit_mapping(), FX_EXT (+32 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 187 - ".update"
Cohesion: 0.24
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 189 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 194 - "Settings"
Cohesion: 0.13
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Purpose"
Cohesion: 0.24
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "toolbar.rs"
Cohesion: 0.11
Nodes (18): begin(), context(), ICON_SLOT, IconFn, ITEM_H, MenuItem, MIN_W, separator() (+10 more)

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
Cohesion: 0.43
Nodes (5): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue

### Community 215 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 226 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 231 - "Deserialize"
Cohesion: 0.31
Nodes (4): bx(), find_box(), mp4_duration_ms(), reads_mp4_length()

### Community 232 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 237 - "CanvasMode"
Cohesion: 0.67
Nodes (3): CanvasMode, Edit, Select

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 248 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 251 - "AudioPlayer"
Cohesion: 0.15
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 259 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 261 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **552 isolated node(s):** `Json`, `NoFunction`, `Intensity`, `ColorIntensity`, `ColorPreset` (+547 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1340 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **114 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `.page_bar`, `call`, `programmer.rs`, `TouchItem`, `.fade_state`, `control/timeline.rs`, `runtime.rs`, `mcp/src/venue.rs`, `ViewCtx`, `Mapper`, `mapping/mod.rs`, `text_dim`, `rig.rs`, `fx_ui.rs`, `app/src/profiles.rs`, `app/src/audio.rs`, `settings_window.rs`, `scene_grid.rs`, `views/touch.rs`, `t`, `FixtureAction`, `.update`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `DmxFrame`, `scene_props.rs`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `toolbar.rs`, `engine.rs`, `Layer`, `fire`, `Action`, `audio_triggers.rs`, `Timeline`, `TimelineView`, `app.rs`, `core/src/lib.rs`, `egui`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Are the 181 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 181 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Json`, `NoFunction`, `Intensity` to the rest of the system?**
  _552 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.09716599190283401 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `bank_menu`, `.page_bar`, `.check_placement`, `call`, `TouchItem`, `SoundBoard`, `fx.rs`, `theme.rs`, `stage_canvas.rs`, `missing_audio.rs`, `TouchView`, `mapping/mod.rs`, `rig.rs`, `.add_scene`, `EffectKind`, `scene_grid.rs`, `views/touch.rs`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `Snapshot`, `DmxFrame`, `TouchLayout`, `ObsSettings`, `scene_props.rs`, `Engine`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `Scene`, `fire`, `viz/mod.rs`, `autosave.rs`, `show.rs`, `look.rs`, `src/mapping.rs`, `app.rs`, `Venue`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10793650793650794 - nodes in this community are weakly interconnected._