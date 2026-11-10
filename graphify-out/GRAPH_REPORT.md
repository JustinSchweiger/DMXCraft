# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 150 files · ~231,929 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4202 nodes · 11011 edges · 279 communities (157 shown, 122 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 550 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0c7d4625`
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
- outline.rs
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
- color_picker.rs
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- OrbitCamera
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
- Profile
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
- ChannelRef
- arc
- run
- .ui
- app.rs
- core/src/lib.rs
- Shape
- Venue
- View
- .row
- scene_card.rs
- convert.rs
- Timeline
- Host
- app/src/profiles.rs
- Obs
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
- Mode
- Mapper
- .update
- qlc-bundle.rs
- TouchView
- mapping/mod.rs
- text_dim
- Input
- icon_library.rs
- WaveState
- Fader
- lock.rs
- .add_scene
- EffectKind
- dip.rs
- mcp/src/profiles.rs
- Problem
- views/touch.rs
- AppState
- LookPart
- TriggerBand
- super
- ok
- palettes.rs
- midi.rs
- Category
- tools.rs
- FixtureLimits
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
- OutputStatus
- de.rs
- ObsSettings
- ShowError
- MenuItem<'a>
- Purpose
- toolbar.rs
- HeaderAction
- build-local-artifacts
- IconButton
- Drag
- Scene
- Knob<'a>
- ObsAction
- FooterButton<'a>
- T
- views/sound.rs
- announce
- Box<dyn serialport::SerialPort>
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
7. `ChannelKind` - 75 edges
8. `text_dim()` - 71 edges
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

## Communities (279 total, 122 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (20): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+12 more)

### Community 2 - "runtime.rs"
Cohesion: 0.18
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "call"
Cohesion: 0.21
Nodes (25): add_clips(), add_effect(), base64(), build_steps(), call(), changed_scene(), create_scene(), create_super_scene() (+17 more)

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
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.13
Nodes (9): fold_section(), ViewCtx, renumber(), vec3_row(), drag_row(), drop_marker(), Folder, VizView (+1 more)

### Community 11 - "outline.rs"
Cohesion: 0.07
Nodes (23): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, DragFixtures, DragObjects (+15 more)

### Community 12 - "server.rs"
Cohesion: 0.14
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

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
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.13
Nodes (30): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+22 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 21 - "Deserialize"
Cohesion: 0.11
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.11
Nodes (26): live_colors(), Beam, beam(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer(), gobo() (+18 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "color_picker.rs"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 29 - "profile_builder.rs"
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.08
Nodes (29): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+21 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.12
Nodes (26): canvas(), canvas_is_light(), handle(), rgb(), color_fx_ui(), edit_colors(), label_row(), LABEL_W (+18 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (60): t(), save(), heading_text(), surface(), text_field(), kind_color(), apply(), capability_fields() (+52 more)

### Community 37 - "audio_trigger_ui.rs"
Cohesion: 0.20
Nodes (10): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+2 more)

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
Nodes (51): zone_color(), select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete (+43 more)

### Community 43 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.17
Nodes (3): DmxCraftApp, fill_types(), resize_handles()

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
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.16
Nodes (17): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+9 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (8): .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (23): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+15 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (37): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+29 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.18
Nodes (10): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+2 more)

### Community 79 - "sense"
Cohesion: 0.05
Nodes (30): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+22 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.27
Nodes (6): drag_number(), endless_delta(), raw_available(), time_field(), time_field_compact(), time_field_ex()

### Community 87 - "fire"
Cohesion: 0.13
Nodes (26): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fires_id(), fixed_channels_work_without_selection() (+18 more)

### Community 89 - "Profile"
Cohesion: 0.07
Nodes (29): slug(), add_range_until_every_value_is_used(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), file_slug(), fine_names() (+21 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (41): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+33 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.04
Nodes (51): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+43 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.13
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

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
Cohesion: 0.26
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 104 - "look.rs"
Cohesion: 0.09
Nodes (29): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+21 more)

### Community 105 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 106 - "ChannelRef"
Cohesion: 0.15
Nodes (4): Layer, Live, Programmer, ChannelRef

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "run"
Cohesion: 0.24
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 109 - ".ui"
Cohesion: 0.18
Nodes (8): fmt_ms(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.14
Nodes (13): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), UpdateBadge, Available (+5 more)

### Community 112 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (11): UNIVERSE_SIZE, BankScope, All, Bank, Banks, Off, OutsideBank, Group (+3 more)

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (19): aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), IDENTITY (+11 more)

### Community 118 - "View"
Cohesion: 0.09
Nodes (21): View, SetupView, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+13 more)

### Community 119 - ".row"
Cohesion: 0.21
Nodes (14): Renaming, Object, Tree, TreeAction, Delete, Drop, Duplicate, GroupSelection (+6 more)

### Community 120 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (27): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+19 more)

### Community 123 - "Timeline"
Cohesion: 0.09
Nodes (20): add_video(), has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene (+12 more)

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (9): AppProfiles, delete(), file(), load_all(), load_user(), sort(), sort_user(), today() (+1 more)

### Community 129 - "Obs"
Cohesion: 0.33
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 130 - "app/src/obs.rs"
Cohesion: 0.22
Nodes (8): auth_matches_obs_websocket_example(), auth_response(), b64(), open(), read_json(), send_json(), VideoPlay, Ws

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
Cohesion: 0.24
Nodes (12): find_item(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of() (+4 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (21): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, is_video() (+13 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.12
Nodes (13): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (49): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas_dim() (+41 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.09
Nodes (31): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+23 more)

### Community 142 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 145 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 146 - "Mapper"
Cohesion: 0.22
Nodes (5): default_feedback(), is_value_control(), Mapper, Sent, UiFire

### Community 147 - ".update"
Cohesion: 0.33
Nodes (6): placeholders_match(), fire(), media_action(), media_settings(), names(), seek()

### Community 148 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 149 - "TouchView"
Cohesion: 0.15
Nodes (13): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.17
Nodes (21): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+13 more)

### Community 151 - "text_dim"
Cohesion: 0.13
Nodes (35): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_font() (+27 more)

### Community 152 - "Input"
Cohesion: 0.28
Nodes (6): Input, Key, Midi, MidiMessage, Cc, Note

### Community 153 - "icon_library.rs"
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 157 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 158 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 159 - ".add_scene"
Cohesion: 0.09
Nodes (13): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), LoopMode, Loop, Once, move_scenes_between_banks(), Scene, SceneKind (+5 more)

### Community 160 - "EffectKind"
Cohesion: 0.11
Nodes (13): FxPreview, Effect, EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx (+5 more)

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "mcp/src/profiles.rs"
Cohesion: 0.21
Nodes (11): MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R (+3 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.13
Nodes (22): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+14 more)

### Community 167 - "AppState"
Cohesion: 0.13
Nodes (8): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, audio_segment()

### Community 168 - "LookPart"
Cohesion: 0.25
Nodes (8): LookAction, Copy, Paste, LookPart, All, Color, Intensity, Position

### Community 169 - "TriggerBand"
Cohesion: 0.25
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 170 - "super"
Cohesion: 0.48
Nodes (4): bx(), find_box(), mp4_duration_ms(), reads_mp4_length()

### Community 171 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 172 - "palettes.rs"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 174 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.13
Nodes (30): add_fixtures(), any_obj(), apply_scene_fields(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values() (+22 more)

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

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
Nodes (39): ControlViews, button(), ProgrammerView, advanced(), apply_to_others(), banks_of(), bar_slider(), block_props() (+31 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, SceneProgress

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
Cohesion: 0.13
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "toolbar.rs"
Cohesion: 0.14
Nodes (18): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+10 more)

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 211 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 213 - "Scene"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 225 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

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
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1344 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **122 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `call`, `programmer.rs`, `TouchItem`, `.fade_state`, `control/timeline.rs`, `ViewCtx`, `mcp/src/venue.rs`, `Mapper`, `.update`, `text_dim`, `rig.rs`, `.add_scene`, `app/src/audio.rs`, `t`, `audio_trigger_ui.rs`, `views/touch.rs`, `mcp/src/profiles.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `DmxFrame`, `scene_props.rs`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `fixture_settings.rs`, `fire`, `audio_triggers.rs`, `ChannelRef`, `.ui`, `app.rs`, `core/src/lib.rs`, `View`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.128) - this node is a cross-community bridge._
- **Are the 181 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 181 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `Cube` to the rest of the system?**
  _552 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `app/src/obs.rs`, `programmer.rs`, `TouchItem`, `control/timeline.rs`, `i18n/mod.rs`, `ViewCtx`, `outline.rs`, `viz/library.rs`, `missing_audio.rs`, `titlebar.rs`, `TouchView`, `text_dim`, `color_picker.rs`, `profile_builder.rs`, `Fader`, `fx_ui.rs`, `audio_trigger_ui.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `palettes.rs`, `DmxCraftApp`, `settings_window.rs`, `fader.rs`, `scene_props.rs`, `Settings`, `window.rs`, `stage.rs`, `Purpose`, `toolbar.rs`, `fixture_settings.rs`, `sense`, `fire`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `.ui`, `app.rs`, `core/src/lib.rs`, `View`, `.row`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10661268556005399 - nodes in this community are weakly interconnected._