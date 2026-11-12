# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 165 files · ~265,982 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 21 file(s) not represented in the graph (top: (none) 5, .ttf 4, .ico 2)

## Summary
- 4613 nodes · 11798 edges · 323 communities (169 shown, 154 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 629 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4bcb4162`
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
- Host
- ofl/src/lib.rs
- Timeline
- R
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
- View
- ProfileBuilder
- FixtureType
- SceneId
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
- outline.rs
- DeviceModel
- Timeline
- panels.rs
- Scene
- time_field.rs
- TouchView
- Drag
- text_dim
- Clip
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
- .render_frame
- super
- Effect
- midi.rs
- tools.rs
- BankScope
- ClipAction
- Deserialize
- paint_symbol
- scene_props.rs
- range_slider.rs
- Engine
- plan
- T
- scene_group.rs
- Dialog<'a>
- FixtureType
- Step
- env
- lock.rs
- de.rs
- Knob<'a>
- Calibrate
- bridge.rs
- HeaderAction
- build-local-artifacts
- TouchKind
- AppState
- DeckState
- Mode
- text_field
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
- call
- PlanShape
- with
- core/src/lib.rs
- .paint
- Obs
- FadeCurve
- FixtureId
- Layer
- .check_placement
- Profile
- Layer
- ObsAction
- elide
- .fade_state
- Box<dyn serialport::SerialPort>
- fader_ranges
- settings.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 255 edges
2. `t()` - 231 edges
3. `Show` - 179 edges
4. `Icon` - 119 edges
5. `AppState` - 112 edges
6. `ViewCtx` - 97 edges
7. `ChannelKind` - 78 edges
8. `ChannelRef` - 65 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/tracking.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (323 total, 154 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Venue"
Cohesion: 0.14
Nodes (6): fixture_folders_are_exclusive(), FixtureFolder, mul(), room_and_tree(), TrussHang, Venue

### Community 5 - "programmer.rs"
Cohesion: 0.23
Nodes (19): color_bar(), current(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks() (+11 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+18 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "props.rs"
Cohesion: 0.08
Nodes (28): fold_section(), Segmented, ViewCtx, VizView, apply_placement(), renumber(), Folder, VizView (+20 more)

### Community 11 - "assist.rs"
Cohesion: 0.22
Nodes (8): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset

### Community 12 - "control/timeline.rs"
Cohesion: 0.08
Nodes (24): audio_segment(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, drop_hint(), EDGE, HEADER_W, is_video(), LANE_H (+16 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.10
Nodes (19): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+11 more)

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
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 21 - "model.rs"
Cohesion: 0.09
Nodes (14): bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), VideoOutput, Browser, Media, B64 (+6 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (31): live_colors(), Beam, beam(), bracket(), build(), cell_light(), color_wheel(), core_direction_matches_visualizer() (+23 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "Control"
Cohesion: 0.06
Nodes (42): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+34 more)

### Community 28 - "fx_ui.rs"
Cohesion: 0.10
Nodes (35): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+27 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "Timeline"
Cohesion: 0.09
Nodes (22): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Group, Obs, Scene, Video, fade_curve_fast_then_slow() (+14 more)

### Community 32 - "R"
Cohesion: 0.31
Nodes (15): add_effect(), apply_scene_fields(), changed_scene(), create_scene(), create_super_scene(), dmx_output(), existing_fixtures(), get_scene() (+7 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (14): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+6 more)

### Community 36 - "t"
Cohesion: 0.07
Nodes (52): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+44 more)

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
Nodes (32): paint_shape_icon(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (52): ControlViews, programmer_values(), ProgrammerView, Action, Bank, Compact, CreateSuper, Delete (+44 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.18
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
Cohesion: 0.14
Nodes (6): DmxCraftApp, fill_types(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 48 - "Ui"
Cohesion: 0.14
Nodes (29): MidiDevice, Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general() (+21 more)

### Community 50 - "audio_trigger_ui.rs"
Cohesion: 0.11
Nodes (22): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+14 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

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
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.13
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "core/src/venue.rs"
Cohesion: 0.11
Nodes (13): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, moving_head_aim() (+5 more)

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
Cohesion: 0.06
Nodes (36): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+28 more)

### Community 77 - "engine.rs"
Cohesion: 0.19
Nodes (38): color_fx_drives_rgb_and_moves(), cv(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+30 more)

### Community 78 - "toolbar.rs"
Cohesion: 0.16
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, chip(), divider() (+6 more)

### Community 79 - "theme.rs"
Cohesion: 0.04
Nodes (53): main(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, CONTRAST, DANGER (+45 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.16
Nodes (17): address_map_marks_ranges(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), move_zone_reorders_banks(), next_free_address_skips_used() (+9 more)

### Community 87 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.12
Nodes (13): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX, last_matching() (+5 more)

### Community 90 - "app/src/obs.rs"
Cohesion: 0.13
Nodes (15): edit_preview(), load_step(), auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS (+7 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.19
Nodes (17): floor_plan(), FloorShape, Handles, handles(), hit_y(), object_box(), paint_box(), project() (+9 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (28): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Control (+20 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "fx/mapping.rs"
Cohesion: 0.14
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

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
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 105 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (34): alt_click(), button(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels() (+26 more)

### Community 106 - "server.rs"
Cohesion: 0.23
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Placement"
Cohesion: 0.29
Nodes (12): angles(), Calibration, CalPoint, finds_position_and_rotation_from_four_marks(), fit(), Head, norm(), placement_of() (+4 more)

### Community 109 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (10): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+2 more)

### Community 112 - "duration"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.18
Nodes (15): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+7 more)

### Community 117 - "Object"
Cohesion: 0.15
Nodes (16): axes(), Face, FaceDrag, faces(), hit(), MIN_LEN, paint(), SIZE (+8 more)

### Community 118 - "fire"
Cohesion: 0.20
Nodes (15): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+7 more)

### Community 119 - ".row"
Cohesion: 0.16
Nodes (15): drag_row(), drop_marker(), Tree, TreeAction, Delete, Drop, Duplicate, GroupSelection (+7 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "TouchItem"
Cohesion: 0.27
Nodes (10): content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), paint_cover(), paint_item() (+2 more)

### Community 127 - "View"
Cohesion: 0.10
Nodes (16): CanvasMode, Edit, Select, apply(), View, FixtureSettings, SetupView, Arrange (+8 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.16
Nodes (8): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 129 - "FixtureType"
Cohesion: 0.25
Nodes (16): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, nearest_slot(), position_in_degrees() (+8 more)

### Community 131 - "egui"
Cohesion: 0.05
Nodes (31): SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+23 more)

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
Cohesion: 0.09
Nodes (21): decode_texture(), GizmoDrag, Missing, Mode, Build, Live, model_tex_id(), Pick (+13 more)

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
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), MenuFn, paint_effects() (+11 more)

### Community 143 - "outline.rs"
Cohesion: 0.12
Nodes (16): deg_quat(), euler_matches_core_rotation(), quat_deg(), rotate_all(), DragFixtures, DragObjects, FixtureSort, Address (+8 more)

### Community 144 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 145 - "Timeline"
Cohesion: 0.11
Nodes (17): Show, add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), fmt_ms() (+9 more)

### Community 146 - "panels.rs"
Cohesion: 0.23
Nodes (5): End, Measure, snap_points(), SNAP_PX, move_all()

### Community 148 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 149 - "TouchView"
Cohesion: 0.16
Nodes (13): add_page(), add_tile(), Drag, Move, Resize, icon_picker(), ImageTarget, Item (+5 more)

### Community 150 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 151 - "text_dim"
Cohesion: 0.12
Nodes (39): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+31 more)

### Community 153 - ".update"
Cohesion: 0.19
Nodes (11): Link, media_action(), media_settings(), open(), read_json(), run(), seek(), send_json() (+3 more)

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "mcp/src/profiles.rs"
Cohesion: 0.18
Nodes (12): MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text() (+4 more)

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.24
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 160 - "control/tracking.rs"
Cohesion: 0.27
Nodes (9): bar_row(), color(), edit(), LABEL_W, LIST_W, MAX_HEIGHT, TrackingView, AXIS_COLORS (+1 more)

### Community 163 - "Capability"
Cohesion: 0.19
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.14
Nodes (19): apply(), state_json(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), GAP (+11 more)

### Community 167 - "mcp.rs"
Cohesion: 0.17
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "fixture_settings.rs"
Cohesion: 0.36
Nodes (4): body(), caption(), hanging_of(), range_row()

### Community 169 - ".render_frame"
Cohesion: 0.17
Nodes (9): blind_keeps_output_but_monitor_shows_edits(), defaults_are_rendered(), limits_cap_dimmer_and_position(), limits_leave_rest_position_alone(), programmer_overrides_and_master_scales(), sync_drops_removed_scenes_and_fixtures(), tracking_aims_heads_at_the_point_and_releases_softly(), virtual_dimmer_scales_colors_and_keeps_ratio() (+1 more)

### Community 170 - "super"
Cohesion: 0.08
Nodes (24): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), glyph(), LIBRARY, names_are_unique() (+16 more)

### Community 171 - "Effect"
Cohesion: 0.12
Nodes (12): Effect, EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+4 more)

### Community 173 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 176 - "tools.rs"
Cohesion: 0.11
Nodes (30): add_clips(), add_fixtures(), any_obj(), apply_clip_fields(), boolean(), camera_schema(), clip_schema(), find_type() (+22 more)

### Community 177 - "BankScope"
Cohesion: 0.16
Nodes (8): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), Zone

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "Deserialize"
Cohesion: 0.16
Nodes (5): all_cells(), DEFAULT_HEIGHT, one(), Show, white()

### Community 182 - "paint_symbol"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (42): advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header(), effect_tiles() (+34 more)

### Community 184 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 185 - "Engine"
Cohesion: 0.10
Nodes (6): blending_does_not_depend_on_the_order_of_values(), Engine, lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, ChannelRef

### Community 188 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 191 - "scene_group.rs"
Cohesion: 0.22
Nodes (4): GroupMember, members_length(), SceneGroup, Show

### Community 193 - "FixtureType"
Cohesion: 0.09
Nodes (15): ChannelDef, ChannelRange, FixtureType, open_value_takes_the_highest_open_range(), Physics, RangeEffect, Closed, Color (+7 more)

### Community 195 - "Step"
Cohesion: 0.23
Nodes (8): ChannelValue, Step, build_steps(), channel_kind(), collect_values(), fixture_json(), resolve_channel(), update_fixture()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 203 - "bridge.rs"
Cohesion: 0.26
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 205 - "HeaderAction"
Cohesion: 0.22
Nodes (10): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, row_h(), track_h() (+2 more)

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 208 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 218 - "AppState"
Cohesion: 0.10
Nodes (9): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, select_only() (+1 more)

### Community 219 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 220 - "Mode"
Cohesion: 0.18
Nodes (8): Mode, Builder, Control, Setup, Sound, .TABS, Touch, Visualizer

### Community 225 - "text_field"
Cohesion: 0.14
Nodes (5): text_field(), section_header(), drag_ghost(), ZonesView, Windows

### Community 226 - "Mapper"
Cohesion: 0.24
Nodes (4): default_feedback(), is_value_control(), Mapper, Sent

### Community 229 - "Quality"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 230 - "views/sound.rs"
Cohesion: 0.19
Nodes (18): AUDIO_EXT, BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name(), deck_ui() (+10 more)

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
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 240 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 241 - "video_out.rs"
Cohesion: 0.12
Nodes (18): CHUNK, DEFAULT_PORT, get(), header(), host_ok(), lock(), media_id(), mime() (+10 more)

### Community 244 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 252 - "call"
Cohesion: 0.20
Nodes (14): base64(), call(), failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), kind_name(), obs_action_clips() (+6 more)

### Community 256 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 266 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): UNIVERSE_SIZE, FixtureLimits, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 281 - "Obs"
Cohesion: 0.16
Nodes (7): Obs, Shared, Status, Connected, Connecting, Failed, Off

### Community 285 - "FadeCurve"
Cohesion: 0.22
Nodes (6): FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth

### Community 289 - "Layer"
Cohesion: 0.15
Nodes (13): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+5 more)

### Community 291 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 292 - "Profile"
Cohesion: 0.12
Nodes (9): beam_angle_order_gives_the_zoom_direction(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), own_format_round_trip(), Profile, rename_and_resolution_update_modes(), spot(), template_from_types() (+1 more)

### Community 303 - "Layer"
Cohesion: 0.33
Nodes (4): Layer, .ALL, Blend, Cover

### Community 305 - "ObsAction"
Cohesion: 0.50
Nodes (4): fire(), ObsAction, Scene, Source

### Community 309 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 316 - "settings.rs"
Cohesion: 0.09
Nodes (12): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), cable_belongs_to_one_universe() (+4 more)

## Knowledge Gaps
- **575 isolated node(s):** `Copy`, `New`, `Dimmer`, `Red`, `Green` (+570 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1485 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **154 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `SceneId`, `egui`, `runtime.rs`, `programmer.rs`, `props.rs`, `core/src/lib.rs`, `control/timeline.rs`, `mcp/src/venue.rs`, `Timeline`, `text_dim`, `rig.rs`, `.update`, `Obs`, `Host`, `mcp/src/profiles.rs`, `control/tracking.rs`, `Layer`, `app/src/audio.rs`, `FixtureId`, `t`, `R`, `views/touch.rs`, `mcp.rs`, `FixtureAction`, `scene_grid.rs`, `super`, `.render_frame`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `audio_trigger_ui.rs`, `remote/mod.rs`, `.fade_state`, `scene_props.rs`, `Step`, `window.rs`, `stage.rs`, `Show`, `Calibrate`, `engine.rs`, `app/src/obs.rs`, `ChannelKind`, `text_field`, `Mapper`, `audio_triggers.rs`, `mapping/mod.rs`, `Category`, `fire`, `TouchItem`, `call`, `View`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Are the 197 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 197 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Copy`, `New`, `Dimmer` to the rest of the system?**
  _575 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.14022988505747128 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `ProfileBuilder`, `library_panel.rs`, `egui`, `programmer.rs`, `VizView`, `i18n/mod.rs`, `props.rs`, `control/timeline.rs`, `viz/library.rs`, `outline.rs`, `Timeline`, `panels.rs`, `titlebar.rs`, `TouchView`, `text_dim`, `text_field`, `fx_ui.rs`, `control/tracking.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `fixture_settings.rs`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `audio_trigger_ui.rs`, `paint_symbol`, `scene_props.rs`, `fader_ranges`, `settings.rs`, `snap.rs`, `window.rs`, `stage.rs`, `Calibrate`, `HeaderAction`, `toolbar.rs`, `viz/mod.rs`, `textures.rs`, `text_field`, `Quality`, `views/sound.rs`, `Dialog<'a>`, `mapping/mod.rs`, `app/src/profiles.rs`, `Category`, `.row`, `TouchItem`, `View`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Are the 7 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 7 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11822660098522167 - nodes in this community are weakly interconnected._