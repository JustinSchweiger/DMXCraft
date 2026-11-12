# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 165 files · ~267,350 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4624 nodes · 11841 edges · 328 communities (175 shown, 153 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 633 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e9742230`
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
- viz/library.rs
- jobs
- with
- README.md
- titlebar.rs
- Step
- Deserialize
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Control
- fx_ui.rs
- Host
- ofl/src/lib.rs
- src/timeline.rs
- Action
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
- select_all
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
- show.rs
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
- audio_trigger_ui.rs
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
- Theme
- outline.rs
- Tab
- convert.rs
- scene_card.rs
- egui
- ProfileBuilder
- FixtureType
- PaletteStore
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
- deg_quat
- DeviceModel
- Timeline
- panels.rs
- Scene
- time_field.rs
- views/touch.rs
- Drag
- text_dim
- bank_menu
- ObsSettings
- text_field
- mcp/src/profiles.rs
- settings.rs
- run
- Capability
- Transform
- Problem
- state_json
- mcp.rs
- fixture_settings.rs
- icon_library.rs
- Snapshot
- Effect
- midi.rs
- tools.rs
- super
- ClipAction
- Deserialize
- paint_symbol
- scene_props.rs
- drag_number
- Engine
- plan
- DialogKind
- T
- scene_group.rs
- Dialog<'a>
- FixtureType
- apply_effect_hold
- env
- lock.rs
- de.rs
- Knob<'a>
- Calibrate
- bridge.rs
- HeaderAction
- build-local-artifacts
- MoveShape
- AppState
- DeckState
- Mode
- plan.rs
- Mapper
- FooterButton<'a>
- Quality
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
- WaveState
- Easing
- PlanShape
- with
- core/src/lib.rs
- .paint
- Obs
- core/src/obs.rs
- ModelPart
- apply_pick
- .check_placement
- Profile
- Camera
- FixtureLimits
- ObsAction
- StageCanvas
- .fade_state
- Box<dyn serialport::SerialPort>
- Arrange
- send_json
- Mode
- OutlineTab

## God Nodes (most connected - your core abstractions)
1. `Engine` - 255 edges
2. `t()` - 230 edges
3. `Show` - 179 edges
4. `Icon` - 119 edges
5. `AppState` - 113 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 78 edges
8. `ChannelRef` - 64 edges
9. `Timeline` - 58 edges
10. `FixtureType` - 57 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (328 total, 153 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (21): UserProfile, LibraryDrag, BuilderRequest, Edit, New, Open, Template, template() (+13 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (18): UNIVERSE_SIZE, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Venue"
Cohesion: 0.12
Nodes (10): renumber(), duplicate_object(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object (+2 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (36): ProgrammerMode, Blind, Edit, Live, color_bar(), current(), elide(), fader() (+28 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.25
Nodes (18): all_presets_render_every_beam(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels(), hard_edges_use_palette_colors_only(), knight_rider(), knight_rider_lights_one_bar() (+10 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.08
Nodes (26): text_field(), fold_section(), section_header(), Toggle, ViewCtx, Item, VizView, apply_texture() (+18 more)

### Community 11 - "assist.rs"
Cohesion: 0.21
Nodes (10): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, decode_texture() (+2 more)

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
Cohesion: 0.05
Nodes (62): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+54 more)

### Community 20 - "Step"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 21 - "Deserialize"
Cohesion: 0.17
Nodes (7): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Texture

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
Cohesion: 0.06
Nodes (42): Overlay, MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key() (+34 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.16
Nodes (22): color_fx_ui(), edit_colors(), label_row(), slider_row(), toggle_row(), combo(), curve_graph(), effect_body() (+14 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "src/timeline.rs"
Cohesion: 0.05
Nodes (37): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Group, Obs, Scene, Video (+29 more)

### Community 32 - "Action"
Cohesion: 0.18
Nodes (20): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+12 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (14): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+6 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (52): kind_color(), paint_bands(), paint_beats(), paint_clip(), paint_lead(), paint_segments(), paint_triggers(), track_header() (+44 more)

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
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.17
Nodes (17): programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow(), BankDrag (+9 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.07
Nodes (19): DmxCraftApp, fill_types(), LockDialog, moved_out(), viewport_input(), AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs() (+11 more)

### Community 48 - "Ui"
Cohesion: 0.11
Nodes (35): t(), MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo() (+27 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.13
Nodes (19): heading_font(), FaderEvent, Changed, None, Release, ToggleFx, begin(), context() (+11 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.19
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "select_all"
Cohesion: 0.13
Nodes (7): surface(), DialogResult, Form, select_all(), from_pct(), pct(), value_field()

### Community 66 - "snap.rs"
Cohesion: 0.18
Nodes (10): Aabb, angle(), angle_label(), ANGLES, edges_meet_and_grid_otherwise(), FIXTURE_HALF, grid_label(), GRIDS (+2 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (18): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+10 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.12
Nodes (12): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), mul(), rotation() (+4 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (5): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Group, Show

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
Cohesion: 0.16
Nodes (47): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+39 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.23
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 79 - "theme.rs"
Cohesion: 0.06
Nodes (58): main(), accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c() (+50 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.26
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (21): beam_angle_order_gives_the_zoom_direction(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+13 more)

### Community 90 - "app/src/obs.rs"
Cohesion: 0.17
Nodes (12): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+4 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.16
Nodes (20): floor_plan(), FloorShape, Handles, handles(), hit_y(), model_tex_id(), paint_box(), project() (+12 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.09
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "audio_trigger_ui.rs"
Cohesion: 0.23
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 102 - "system_fonts.rs"
Cohesion: 0.13
Nodes (3): Emoji, install(), SystemFonts

### Community 104 - "Dialog<'a>"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 105 - "mapping/mod.rs"
Cohesion: 0.10
Nodes (44): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), fader(), fader_channels(), fire(), fires_id() (+36 more)

### Community 106 - "server.rs"
Cohesion: 0.19
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "app/src/profiles.rs"
Cohesion: 0.21
Nodes (9): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+1 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Object"
Cohesion: 0.23
Nodes (9): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+1 more)

### Community 118 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (29): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+21 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (28): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+20 more)

### Community 123 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 127 - "egui"
Cohesion: 0.12
Nodes (13): ControlViews, edit_preview(), load_step(), OUTPUT_HZ, ProgrammerView, View, FixtureSettings, SetupView (+5 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 129 - "FixtureType"
Cohesion: 0.18
Nodes (18): ChannelDef, ft(), color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer (+10 more)

### Community 130 - "PaletteStore"
Cohesion: 0.32
Nodes (8): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore, BodyCtx

### Community 131 - "egui"
Cohesion: 0.06
Nodes (35): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+27 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 136 - "VizView"
Cohesion: 0.17
Nodes (12): apply_placement(), GizmoDrag, Missing, move_all(), Pick, Fixture, Object, rotate_all() (+4 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (15): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.16
Nodes (18): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+10 more)

### Community 143 - "deg_quat"
Cohesion: 0.67
Nodes (3): deg_quat(), euler_matches_core_rotation(), quat_deg()

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - "Timeline"
Cohesion: 0.07
Nodes (39): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS (+31 more)

### Community 146 - "panels.rs"
Cohesion: 0.23
Nodes (4): End, Measure, snap_points(), SNAP_PX

### Community 147 - "Scene"
Cohesion: 0.12
Nodes (7): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), Scene, steps_insert_remove_and_normalize(), Zone

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 149 - "views/touch.rs"
Cohesion: 0.08
Nodes (37): add_page(), add_tile(), COLORS, content_rect(), display_label(), fader_parts(), GAP, GRID_STEPS (+29 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.15
Nodes (22): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_text() (+14 more)

### Community 152 - "bank_menu"
Cohesion: 0.28
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, Move, New (+5 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.37
Nodes (10): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), R, save() (+2 more)

### Community 159 - "settings.rs"
Cohesion: 0.11
Nodes (13): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, git(), main(), run() (+5 more)

### Community 160 - "run"
Cohesion: 0.26
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 163 - "Capability"
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 164 - "Transform"
Cohesion: 0.22
Nodes (8): normalized(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "state_json"
Cohesion: 0.21
Nodes (12): apply(), Cmd, Color, Press, Value, Xy, state_json(), channels_of() (+4 more)

### Community 167 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "fixture_settings.rs"
Cohesion: 0.36
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 169 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 170 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 171 - "Effect"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (65): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+57 more)

### Community 177 - "super"
Cohesion: 0.29
Nodes (7): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), addresses_are_one_based()

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "Deserialize"
Cohesion: 0.12
Nodes (11): VIDEO_COLOR, bar_row(), color(), edit(), TrackingView, AXIS_COLORS, all_cells(), DEFAULT_HEIGHT (+3 more)

### Community 182 - "paint_symbol"
Cohesion: 0.11
Nodes (12): DOT_H, Fader, FADER_W, FaderRange, fine_mark(), HANDLE, NUM_H, paint_symbol() (+4 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (43): time_field(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+35 more)

### Community 184 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (9): blending_does_not_depend_on_the_order_of_values(), Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, SceneProgress, Track, TrackLight (+1 more)

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 189 - "DialogKind"
Cohesion: 0.20
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 193 - "FixtureType"
Cohesion: 0.12
Nodes (12): ChannelRange, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color, Gobo, Open (+4 more)

### Community 195 - "apply_effect_hold"
Cohesion: 0.31
Nodes (9): apply(), apply_effect(), apply_effect_hold(), Beam, beams(), feature_levels(), luma(), to_rgb() (+1 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 203 - "bridge.rs"
Cohesion: 0.33
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 218 - "AppState"
Cohesion: 0.11
Nodes (9): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, Show (+1 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 225 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 229 - "Quality"
Cohesion: 0.20
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "SlotKind"
Cohesion: 0.19
Nodes (8): slot_label(), SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 234 - "qxf.rs"
Cohesion: 0.22
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 239 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 252 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 256 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (16): BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode, Loop (+8 more)

### Community 281 - "Obs"
Cohesion: 0.18
Nodes (6): fire(), media_action(), media_settings(), Obs, seek(), VideoPlay

### Community 285 - "core/src/obs.rs"
Cohesion: 0.33
Nodes (7): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media

### Community 289 - "apply_pick"
Cohesion: 0.38
Nodes (7): channel_name(), describe(), describe_mapping(), t_deleted(), apply_pick(), pick_adds_scene_button_and_assigns_to_selection(), programmer_fader_becomes_fixed_channels()

### Community 291 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 292 - "Profile"
Cohesion: 0.13
Nodes (5): import(), MemoryProfiles, fine_names(), Profile, rename_and_resolution_update_modes()

### Community 305 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 308 - "StageCanvas"
Cohesion: 0.33
Nodes (5): CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 309 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 312 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 316 - "send_json"
Cohesion: 0.83
Nodes (4): open(), read_json(), send_json(), Ws

### Community 323 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

### Community 324 - "OutlineTab"
Cohesion: 0.67
Nodes (3): OutlineTab, Fixtures, Objects

## Knowledge Gaps
- **576 isolated node(s):** `Loading`, `Off`, `Connecting`, `Connected`, `Level` (+571 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1484 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **153 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `runtime.rs`, `egui`, `programmer.rs`, `props.rs`, `core/src/lib.rs`, `mcp/src/venue.rs`, `DmxFrame`, `Timeline`, `views/touch.rs`, `text_dim`, `rig.rs`, `Obs`, `Host`, `mcp/src/profiles.rs`, `Action`, `apply_pick`, `app/src/audio.rs`, `t`, `state_json`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `Deserialize`, `.fade_state`, `scene_props.rs`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `app/src/obs.rs`, `Mapper`, `audio_triggers.rs`, `audio_trigger_ui.rs`, `mapping/mod.rs`, `egui`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Are the 197 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 197 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Off`, `Connecting` to the rest of the system?**
  _576 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12222222222222222 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `SoundPlayer`, `library_panel.rs`, `Venue`, `VizView`, `fx.rs`, `core/src/lib.rs`, `assist.rs`, `stage_canvas.rs`, `panels.rs`, `Scene`, `Step`, `views/touch.rs`, `bank_menu`, `rig.rs`, `ObsSettings`, `Control`, `apply_pick`, `.check_placement`, `ShowError`, `state_json`, `FixtureAction`, `scene_grid.rs`, `Snapshot`, `missing_audio.rs`, `Effect`, `DmxCraftApp`, `tools.rs`, `StageCanvas`, `TouchLayout`, `Deserialize`, `scene_props.rs`, `Engine`, `scene_group.rs`, `snap.rs`, `apply_effect_hold`, `feature.rs`, `motion.rs`, `engine.rs`, `theme.rs`, `show.rs`, `viz/mod.rs`, `mapping/mod.rs`, `Placement`, `Object`, `egui`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12477718360071301 - nodes in this community are weakly interconnected._