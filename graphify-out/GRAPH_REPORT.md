# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 161 files · ~259,438 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4544 nodes · 11568 edges · 308 communities (159 shown, 149 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 606 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7c21b5b5`
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
- Timeline
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
- snap.rs
- steps
- window.rs
- stage.rs
- core/src/venue.rs
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- engine.rs
- control/timeline.rs
- theme.rs
- OpenDmx
- Fader
- SlotKind
- profile.rs
- Capability
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
- protocol.rs
- arc
- Snapshot
- mcp/src/profiles.rs
- bridge.rs
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
- toolbar.rs
- i18n/mod.rs
- mcp/src/venue.rs
- IconButton
- stage_canvas.rs
- DmxFrame
- icon_library.rs
- DeviceModel
- TimelineView
- Tab
- ChaserPreset
- TouchView
- Drag
- text_dim
- Theme
- .check_placement
- text_field
- Profile
- qlc-bundle.rs
- control/tracking.rs
- Grip
- Transform
- Problem
- views/touch.rs
- mcp.rs
- ShutterEffect
- menu.rs
- apply
- .add_scene
- midi.rs
- tools.rs
- Prepare
- ClipAction
- BankScope
- fader.rs
- scene_props.rs
- range_slider.rs
- Engine
- plan
- PlanShape
- scene_group.rs
- super
- FixtureType
- fixture_settings.rs
- env
- FixtureLimits
- de.rs
- Knob<'a>
- Purpose
- HeaderAction
- build-local-artifacts
- TouchKind
- AppState
- PaletteStore
- View
- FooterButton<'a>
- views/sound.rs
- announce
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
- app/src/obs.rs
- FadeCurve
- WaveState
- .fade_state
- Category
- Effect
- Layer
- DmxLine

## God Nodes (most connected - your core abstractions)
1. `Engine` - 249 edges
2. `t()` - 226 edges
3. `Show` - 174 edges
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
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/tracking.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (308 total, 149 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (20): UserProfile, LibraryDrag, BuilderRequest, Edit, New, Open, Template, template() (+12 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.14
Nodes (16): BREAK, DMX_BAUD, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB (+8 more)

### Community 4 - "show.rs"
Cohesion: 0.20
Nodes (15): addresses_are_one_based(), UNIVERSE_SIZE, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names() (+7 more)

### Community 5 - "programmer.rs"
Cohesion: 0.08
Nodes (37): ProgrammerMode, Blind, Edit, Live, kind_color(), color_bar(), current(), elide() (+29 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels() (+15 more)

### Community 9 - "DmxOutput"
Cohesion: 0.15
Nodes (6): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.08
Nodes (24): fold_section(), section_header(), ViewCtx, renumber(), apply_texture(), OutlineTab, Fixtures, Objects (+16 more)

### Community 11 - "assist.rs"
Cohesion: 0.12
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

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
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.05
Nodes (62): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+54 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

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
Cohesion: 0.12
Nodes (29): t(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), ROW_H, slider_row() (+21 more)

### Community 29 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "Timeline"
Cohesion: 0.15
Nodes (6): has_audio(), media_track(), Clip, linked_blocks_stay_together(), paste_finds_free_place_and_keeps_gaps(), Timeline

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.10
Nodes (21): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+13 more)

### Community 36 - "t"
Cohesion: 0.11
Nodes (28): capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H, full_combo(), general() (+20 more)

### Community 37 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (10): PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super, ShowError, Format, Io (+2 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (51): select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+43 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.06
Nodes (26): AppProfiles, AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id() (+18 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.13
Nodes (28): hidden_tabs_from_old_setting(), Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+20 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.17
Nodes (15): begin(), context(), dropdown(), item(), separator(), apply_trigger_pick(), trigger_cx(), trigger_ui() (+7 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "snap.rs"
Cohesion: 0.17
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (25): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.13
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.15
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), Placement, rotation() (+2 more)

### Community 72 - "Show"
Cohesion: 0.14
Nodes (6): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, old_show_files_without_new_fields_load(), Show

### Community 73 - "feature.rs"
Cohesion: 0.18
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), custom_value(), fill_ends_full() (+7 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.12
Nodes (10): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation, OutputStatus, Connecting, Error (+2 more)

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.15
Nodes (45): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+37 more)

### Community 78 - "control/timeline.rs"
Cohesion: 0.07
Nodes (38): Show, add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H (+30 more)

### Community 79 - "theme.rs"
Cohesion: 0.06
Nodes (55): main(), accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c() (+47 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 87 - "SlotKind"
Cohesion: 0.19
Nodes (8): slot_label(), SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 89 - "profile.rs"
Cohesion: 0.10
Nodes (19): camera_json(), beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+11 more)

### Community 90 - "Capability"
Cohesion: 0.23
Nodes (3): cap_for_kind(), Capability, last_matching()

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.09
Nodes (40): apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), floor_plan(), FloorShape, GizmoDrag (+32 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

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
Cohesion: 0.11
Nodes (29): alt_click(), button(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id() (+21 more)

### Community 106 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 110 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.18
Nodes (15): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+7 more)

### Community 117 - "Venue"
Cohesion: 0.09
Nodes (16): floor_height(), set_locked(), transfer(), load_file(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+8 more)

### Community 118 - "mapping/mod.rs"
Cohesion: 0.20
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 119 - ".row"
Cohesion: 0.08
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (12): sidebar(), Tab, Ai, .ALL, Audio, Customize, General, Midi (+4 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "src/timeline.rs"
Cohesion: 0.08
Nodes (22): Scene, audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video (+14 more)

### Community 127 - "widgets/mod.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+5 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.10
Nodes (20): FileJob, Export, Import, list_header(), ListAction, Add, Duplicate, Remove (+12 more)

### Community 129 - "FixtureType"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 130 - "app.rs"
Cohesion: 0.22
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 131 - "egui"
Cohesion: 0.05
Nodes (38): DipSwitch, Knob, SIZE, SWEEP, LimitPad, CARD_H, clip(), COMPACT_H (+30 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "toolbar.rs"
Cohesion: 0.13
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, chip(), divider() (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 143 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 147 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 148 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 149 - "TouchView"
Cohesion: 0.17
Nodes (10): add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image(), prepare_image() (+2 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (27): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_font() (+19 more)

### Community 152 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 153 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 155 - "text_field"
Cohesion: 0.11
Nodes (18): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+10 more)

### Community 158 - "Profile"
Cohesion: 0.14
Nodes (5): import(), MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "control/tracking.rs"
Cohesion: 0.15
Nodes (7): edit(), TrackingView, DEFAULT_HEIGHT, one(), Show, Tracker, white()

### Community 163 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.14
Nodes (20): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.11
Nodes (32): apply(), state_json(), apply_pick(), channels_of(), COLORS, content_rect(), current_color(), current_position() (+24 more)

### Community 167 - "mcp.rs"
Cohesion: 0.14
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 169 - "menu.rs"
Cohesion: 0.19
Nodes (7): ICON_SLOT, IconFn, ITEM_H, MenuItem, MIN_W, style(), sub()

### Community 171 - ".add_scene"
Cohesion: 0.15
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), Zone

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (65): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+57 more)

### Community 179 - "ClipAction"
Cohesion: 0.10
Nodes (18): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+10 more)

### Community 180 - "BankScope"
Cohesion: 0.20
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 182 - "fader.rs"
Cohesion: 0.10
Nodes (19): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, HANDLE (+11 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.05
Nodes (57): time_field(), time_field_compact(), advanced(), apply_to_others(), bar_slider(), block_props(), chip(), effect_header() (+49 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (9): blending_does_not_depend_on_the_order_of_values(), def_is_strobe(), Engine, lerp(), PlayingScene, SceneProgress, Track, TrackLight (+1 more)

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
Cohesion: 0.18
Nodes (15): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), clamp(), default_position() (+7 more)

### Community 193 - "FixtureType"
Cohesion: 0.09
Nodes (14): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+6 more)

### Community 195 - "fixture_settings.rs"
Cohesion: 0.38
Nodes (5): body(), caption(), FixtureSettings, hanging_of(), range_row()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

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

### Community 219 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 225 - "View"
Cohesion: 0.14
Nodes (10): View, SetupView, Drag, GridMode, Output, Patch, PatchGrid, StageEditorView (+2 more)

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 234 - "qxf.rs"
Cohesion: 0.22
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
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "Scene"
Cohesion: 0.23
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 281 - "app/src/obs.rs"
Cohesion: 0.05
Nodes (47): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+39 more)

### Community 285 - "FadeCurve"
Cohesion: 0.20
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 288 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 291 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 292 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 308 - "Effect"
Cohesion: 0.43
Nodes (7): apply_effect(), apply_effect_hold(), feature_levels(), luma(), to_rgb(), write_colors(), Effect

### Community 310 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

## Knowledge Gaps
- **574 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+569 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1472 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **149 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app.rs`, `egui`, `runtime.rs`, `programmer.rs`, `ViewCtx`, `mcp/src/venue.rs`, `DmxFrame`, `TimelineView`, `text_dim`, `rig.rs`, `app/src/obs.rs`, `Host`, `Timeline`, `control/tracking.rs`, `app/src/audio.rs`, `.fade_state`, `t`, `core/src/lib.rs`, `views/touch.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `apply`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `audio_trigger_ui.rs`, `remote/mod.rs`, `scene_props.rs`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `control/timeline.rs`, `Mapper`, `View`, `audio_triggers.rs`, `Snapshot`, `mcp/src/profiles.rs`, `mapping/mod.rs`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Are the 193 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 193 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _574 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12222222222222222 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `fx_ui.rs` to `ProfileBuilder`, `library_panel.rs`, `app.rs`, `egui`, `programmer.rs`, `toolbar.rs`, `i18n/mod.rs`, `ViewCtx`, `assist.rs`, `viz/library.rs`, `titlebar.rs`, `TouchView`, `text_dim`, `text_field`, `Timeline`, `control/tracking.rs`, `t`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `apply`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `audio_trigger_ui.rs`, `fader.rs`, `scene_props.rs`, `snap.rs`, `fixture_settings.rs`, `window.rs`, `stage.rs`, `Purpose`, `control/timeline.rs`, `Fader`, `SlotKind`, `PaletteStore`, `viz/mod.rs`, `textures.rs`, `View`, `views/sound.rs`, `Dialog<'a>`, `Venue`, `mapping/mod.rs`, `.row`, `widgets/mod.rs`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12473118279569892 - nodes in this community are weakly interconnected._