# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 150 files · ~230,133 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4189 nodes · 10990 edges · 276 communities (161 shown, 115 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 549 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `640d6cac`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
- programmer.rs
- SoundBoard
- steps
- fx.rs
- OutputError
- ViewCtx
- ChannelRef
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
- mcp/src/profiles.rs
- bridge.rs
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- app/src/profiles.rs
- dmxcraft
- app/src/audio.rs
- t
- protocol.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- color_picker.rs
- Deserialize
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
- scene_grid.rs
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- autosave.rs
- show.rs
- FixtureType
- gltf_import.rs
- ShutterEffect
- arc
- Control
- TimelineView
- app.rs
- Control
- Shape
- Venue
- View
- .row
- DmxLine
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- assist.rs
- app/src/obs.rs
- scene_card.rs
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
- FaderEvent
- Mapper
- FixtureLimits
- TouchView
- mapping/mod.rs
- text_dim
- bank_menu
- icon_library.rs
- WaveState
- patch_grid.rs
- run
- .add_scene
- Scene
- dip.rs
- Transform
- Problem
- views/touch.rs
- AppState
- Category
- audio_trigger_ui.rs
- Obs
- Fullscreen
- Drag
- midi.rs
- qlc-bundle.rs
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- DeckState
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- .update
- knob.rs
- plan.rs
- Settings
- env
- Mode
- de.rs
- MoveShape
- ShowError
- MenuItem<'a>
- Purpose
- toolbar.rs
- HeaderAction
- build-local-artifacts
- IconButton
- ProgrammerMode
- Scene
- lock.rs
- TriggerBand
- FooterButton<'a>
- T
- views/sound.rs
- super
- GH_TOKEN
- Grip
- release.ps1
- swatch.rs
- desktop.rs
- pointer
- release-install.md
- Kind
- OutputStatus
- AudioPlayer
- with
- .check_placement
- toggle.rs
- .fade_state
- elide
- .fixture_list

## God Nodes (most connected - your core abstractions)
1. `Engine` - 230 edges
2. `t()` - 210 edges
3. `Show` - 151 edges
4. `Icon` - 115 edges
5. `AppState` - 103 edges
6. `ViewCtx` - 90 edges
7. `ChannelKind` - 76 edges
8. `text_dim()` - 74 edges
9. `ChannelRef` - 59 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (276 total, 115 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.27
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.14
Nodes (16): BREAK, DMX_BAUD, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB (+8 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 5 - "programmer.rs"
Cohesion: 0.22
Nodes (20): color_bar(), current(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks() (+12 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer, Failing

### Community 10 - "ViewCtx"
Cohesion: 0.09
Nodes (17): fold_section(), ViewCtx, DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue (+9 more)

### Community 11 - "ChannelRef"
Cohesion: 0.14
Nodes (3): lerp(), PlayingScene, ChannelRef

### Community 12 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

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
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 21 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.17
Nodes (17): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), kind() (+9 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 28 - "bridge.rs"
Cohesion: 0.33
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 29 - "profile_builder.rs"
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.09
Nodes (37): builtin(), load_user(), named(), NamedPalette, path(), save_user(), canvas_dim(), handle() (+29 more)

### Community 32 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (56): t(), heading_text(), text_field(), kind_color(), capability_fields(), category_label(), check(), color_label() (+48 more)

### Community 37 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

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
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 43 - "Deserialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (25): ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT, hint() (+17 more)

### Community 50 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (24): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, MapFile (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.22
Nodes (5): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.12
Nodes (16): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+8 more)

### Community 77 - "engine.rs"
Cohesion: 0.18
Nodes (36): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+28 more)

### Community 78 - "Layer"
Cohesion: 0.16
Nodes (12): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On (+4 more)

### Community 79 - "sense"
Cohesion: 0.12
Nodes (13): LimitPad, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over() (+5 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.13
Nodes (12): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+4 more)

### Community 87 - "fire"
Cohesion: 0.16
Nodes (19): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.06
Nodes (30): slug(), add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text() (+22 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (39): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+31 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.11
Nodes (34): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+26 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.15
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "show.rs"
Cohesion: 0.13
Nodes (22): SceneProgress, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once, id_type! (+14 more)

### Community 104 - "FixtureType"
Cohesion: 0.08
Nodes (31): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+23 more)

### Community 105 - "gltf_import.rs"
Cohesion: 0.16
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 106 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Control"
Cohesion: 0.09
Nodes (24): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback, FeedbackState, Input (+16 more)

### Community 109 - "TimelineView"
Cohesion: 0.17
Nodes (7): fmt_ms(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.12
Nodes (15): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), UpdateBadge (+7 more)

### Community 112 - "Control"
Cohesion: 0.11
Nodes (19): Overlay, Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+11 more)

### Community 113 - "Shape"
Cohesion: 0.11
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (19): renumber(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+11 more)

### Community 118 - "View"
Cohesion: 0.16
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 119 - ".row"
Cohesion: 0.11
Nodes (25): DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): placeholders_match(), names(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds() (+17 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (18): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video, fades() (+10 more)

### Community 127 - "Host"
Cohesion: 0.16
Nodes (10): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+2 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.12
Nodes (15): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+7 more)

### Community 129 - "assist.rs"
Cohesion: 0.13
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.24
Nodes (8): auth_matches_obs_websocket_example(), auth_response(), b64(), connects_and_plays_video_block(), engine(), fake_obs(), wait(), wrong_password_is_reported()

### Community 131 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "TouchItem"
Cohesion: 0.30
Nodes (10): find_item(), content_rect(), display_label(), fader_parts(), item_target(), mapping_of(), paint_cover(), paint_item() (+2 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (24): add_audio(), add_audio_dialog(), add_video(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_audio() (+16 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (50): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+42 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.16
Nodes (11): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+3 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output(), MidiDevice (+2 more)

### Community 145 - "FaderEvent"
Cohesion: 0.40
Nodes (5): FaderEvent, Changed, None, Release, ToggleFx

### Community 146 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 149 - "TouchView"
Cohesion: 0.14
Nodes (14): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+6 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (29): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+21 more)

### Community 151 - "text_dim"
Cohesion: 0.11
Nodes (42): window(), accent(), accent_dim(), apply(), bg(), body(), border(), caption() (+34 more)

### Community 152 - "bank_menu"
Cohesion: 0.24
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+5 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 157 - "patch_grid.rs"
Cohesion: 0.21
Nodes (11): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+3 more)

### Community 158 - "run"
Cohesion: 0.26
Nodes (4): Link, run(), session(), ObsSettings

### Community 159 - ".add_scene"
Cohesion: 0.11
Nodes (11): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank() (+3 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.15
Nodes (19): apply(), state_json(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), GAP (+11 more)

### Community 167 - "AppState"
Cohesion: 0.12
Nodes (8): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, audio_segment()

### Community 168 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 169 - "audio_trigger_ui.rs"
Cohesion: 0.22
Nodes (10): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+2 more)

### Community 170 - "Obs"
Cohesion: 0.23
Nodes (8): Obs, Shared, Status, Connected, Connecting, Failed, Off, VideoPlay

### Community 171 - "Fullscreen"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 172 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 174 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (71): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+63 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Fader"
Cohesion: 0.11
Nodes (10): DOT_H, Fader, FADER_W, FaderRange, HANDLE, NUM_H, SYMBOL_H, VALUE_H (+2 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (38): button(), zone_color(), advanced(), apply_to_others(), banks_of(), block_props(), fit_mapping(), FX_EXT (+30 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.09
Nodes (3): item_value(), Engine, preview_effects_only_run_when_active()

### Community 187 - ".update"
Cohesion: 0.33
Nodes (8): fire(), media_action(), media_settings(), open(), read_json(), seek(), send_json(), Ws

### Community 189 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 194 - "Settings"
Cohesion: 0.13
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 200 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "toolbar.rs"
Cohesion: 0.11
Nodes (19): select_all(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H (+11 more)

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 211 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 213 - "Scene"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 215 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 226 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 231 - "super"
Cohesion: 0.31
Nodes (7): bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length()

### Community 232 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 245 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 248 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 251 - "AudioPlayer"
Cohesion: 0.16
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

### Community 278 - ".fixture_list"
Cohesion: 0.31
Nodes (3): drag_row(), Folder, VizView

## Knowledge Gaps
- **552 isolated node(s):** `Loading`, `Failed`, `General`, `Customize`, `Startup` (+547 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1337 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **115 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `programmer.rs`, `TouchItem`, `.fade_state`, `control/timeline.rs`, `ViewCtx`, `ChannelRef`, `mcp/src/venue.rs`, `Mapper`, `text_dim`, `rig.rs`, `mcp/src/profiles.rs`, `app/src/profiles.rs`, `app/src/audio.rs`, `t`, `views/touch.rs`, `FixtureAction`, `audio_trigger_ui.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `scene_props.rs`, `.update`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `Layer`, `fire`, `ChannelKind`, `scene_grid.rs`, `audio_triggers.rs`, `show.rs`, `TimelineView`, `app.rs`, `View`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **Are the 181 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 181 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Failed`, `General` to the rest of the system?**
  _552 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `widgets/mod.rs`, `.check_placement`, `TouchItem`, `SoundBoard`, `fx.rs`, `ChannelRef`, `theme.rs`, `stage_canvas.rs`, `missing_audio.rs`, `TouchView`, `mapping/mod.rs`, `bank_menu`, `rig.rs`, `run`, `.add_scene`, `Scene`, `views/touch.rs`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `Snapshot`, `TouchLayout`, `scene_props.rs`, `Engine`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `Scene`, `fire`, `viz/mod.rs`, `scene_grid.rs`, `autosave.rs`, `show.rs`, `FixtureType`, `Control`, `app.rs`, `Venue`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11260504201680673 - nodes in this community are weakly interconnected._