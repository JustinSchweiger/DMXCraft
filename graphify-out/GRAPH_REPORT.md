# Graph Report - dmxcraft-src  (2026-11-12)

## Corpus Check
- 157 files · ~252,742 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 20 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4453 nodes · 11364 edges · 301 communities (158 shown, 143 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 590 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `700c05b3`
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
- Control
- Deserialize
- Host
- ofl/src/lib.rs
- fx_ui.rs
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
- ClipContent
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
- time_field.rs
- DmxFrame
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
- hashmap
- Dialog<'a>
- show.rs
- range_slider.rs
- arc
- Snapshot
- mcp/src/profiles.rs
- Theme
- duration
- Shape
- Venue
- mapping/mod.rs
- .row
- Tab
- convert.rs
- Scene
- open_dmx.rs
- ProfileBuilder
- FixtureType
- app/src/obs.rs
- egui
- CurveShape
- ColorPreset
- gltf_import.rs
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- fixture_settings.rs
- stage_canvas.rs
- Model
- T
- DeviceModel
- ShowError
- ChannelRef
- Tab
- TouchView
- mapping/mod.rs
- text_dim
- BankScope
- icon_library.rs
- text_field
- Profile
- qlc-bundle.rs
- Quality
- dip.rs
- LookPart
- Problem
- views/touch.rs
- mcp.rs
- ShutterEffect
- menu.rs
- Clip
- Drag
- midi.rs
- tools.rs
- ChaserPreset
- ClipAction
- Transform
- UpdateBadge
- fader.rs
- scene_props.rs
- OutputStatus
- Engine
- plan
- PlanShape
- scene_group.rs
- plan.rs
- make
- View
- env
- FixtureLimits
- de.rs
- Knob<'a>
- MenuItem<'a>
- Purpose
- HeaderAction
- build-local-artifacts
- IconButton
- TouchKind
- app.rs
- AppState
- FooterButton<'a>
- views/sound.rs
- announce
- .check_placement
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- video_out.rs
- release-install.md
- GridMode
- with
- Obs
- lock.rs
- super

## God Nodes (most connected - your core abstractions)
1. `Engine` - 237 edges
2. `t()` - 215 edges
3. `Show` - 165 edges
4. `Icon` - 118 edges
5. `AppState` - 108 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 60 edges
9. `FixtureType` - 57 edges
10. `Timeline` - 56 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/app/src/viz/panels.rs -> crates/app/src/viz/props.rs -> crates/app/src/viz/panels.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (301 total, 143 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (19): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), builtin_types() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 4 - "ZoneId"
Cohesion: 0.15
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), move_zone_reorders_banks(), steps_insert_remove_and_normalize(), Zone

### Community 5 - "programmer.rs"
Cohesion: 0.11
Nodes (32): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+24 more)

### Community 6 - "SoundBoard"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): all_presets_render_every_beam(), apply(), apply_effect(), apply_effect_hold(), Beam, beams(), burst_width_widens_rings(), color_span() (+21 more)

### Community 9 - "DmxOutput"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "ViewCtx"
Cohesion: 0.12
Nodes (15): fold_section(), VizView, aim_fixtures(), button(), color_field(), floor_height(), number(), percent() (+7 more)

### Community 11 - "assist.rs"
Cohesion: 0.14
Nodes (11): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset (+3 more)

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
Nodes (41): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+33 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 21 - "super"
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
Cohesion: 0.07
Nodes (36): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+28 more)

### Community 28 - "Deserialize"
Cohesion: 0.11
Nodes (14): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+6 more)

### Community 29 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.09
Nodes (38): builtin(), load_user(), named(), NamedPalette, path(), save_user(), rgb(), color_fx_ui() (+30 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.09
Nodes (21): analysis(), analyze_file(), cache_file(), duration_ms(), has_sound(), mp3_decoder_can_seek(), open_decoder(), Player (+13 more)

### Community 36 - "t"
Cohesion: 0.10
Nodes (33): t(), text_field(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label() (+25 more)

### Community 37 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (10): SceneProgress, ChannelValue, Group, PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super (+2 more)

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
Nodes (50): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+42 more)

### Community 43 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 44 - "AudioPlayer"
Cohesion: 0.14
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 48 - "Ui"
Cohesion: 0.14
Nodes (28): Settings, ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT (+20 more)

### Community 50 - "ClipContent"
Cohesion: 0.14
Nodes (11): ClipContent, Audio, Group, Obs, Scene, Video, is_light(), Track (+3 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (33): ui_states(), addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value (+25 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (34): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 68 - "steps"
Cohesion: 0.10
Nodes (19): jobs, msi, runs-on, steps, name, on, push, workflow_dispatch (+11 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.16
Nodes (17): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+9 more)

### Community 71 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (8): cv(), demo_show(), empty_show(), ensure_builtin_types(), aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (16): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+8 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.06
Nodes (35): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+27 more)

### Community 77 - "engine.rs"
Cohesion: 0.19
Nodes (41): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), fading_flash_returns_to_the_color_below(), finished_scene_holds_effects_at_their_end() (+33 more)

### Community 78 - ".ui"
Cohesion: 0.13
Nodes (16): add_audio(), add_audio_dialog(), add_video(), add_video_only(), add_video_sound(), audio_marks(), fmt_ms(), grid_options() (+8 more)

### Community 79 - "theme.rs"
Cohesion: 0.07
Nodes (51): Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim() (+43 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "time_field.rs"
Cohesion: 0.27
Nodes (10): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+2 more)

### Community 87 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (24): add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+16 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.08
Nodes (41): apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles() (+33 more)

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
Cohesion: 0.17
Nodes (8): default_feedback(), fires_id(), is_value_control(), Mapper, overlay_id(), pick_id(), Sent, states_id()

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.09
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "hashmap"
Cohesion: 0.11
Nodes (13): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+5 more)

### Community 104 - "Dialog<'a>"
Cohesion: 0.07
Nodes (13): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+5 more)

### Community 105 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 106 - "range_slider.rs"
Cohesion: 0.17
Nodes (8): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), value_field(), VALUE_H

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 109 - "mcp/src/profiles.rs"
Cohesion: 0.24
Nodes (12): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+4 more)

### Community 110 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 112 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 113 - "Shape"
Cohesion: 0.11
Nodes (23): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+15 more)

### Community 117 - "Venue"
Cohesion: 0.07
Nodes (22): renumber(), duplicate_object(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder (+14 more)

### Community 118 - "mapping/mod.rs"
Cohesion: 0.18
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+12 more)

### Community 119 - ".row"
Cohesion: 0.05
Nodes (42): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, drag_row(), DragFixtures (+34 more)

### Community 120 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (29): names(), angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases() (+21 more)

### Community 123 - "Scene"
Cohesion: 0.12
Nodes (11): Scene, audio_grid_sets_tempo_and_offset(), fade_curve_fast_then_slow(), fades(), group_moves_together_without_covering_others(), linked_blocks_stay_together(), media_and_light_tracks_stay_apart(), one() (+3 more)

### Community 127 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.10
Nodes (20): FileJob, Export, Import, import(), list_header(), ListAction, Add, Duplicate (+12 more)

### Community 129 - "FixtureType"
Cohesion: 0.18
Nodes (16): FixtureType, Physics, color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer (+8 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (13): auth_matches_obs_websocket_example(), auth_response(), b64(), browser_mode_creates_the_browser_source_and_serves_the_page(), connects_and_plays_video_block(), DRIFT_MS, engine(), fake_obs() (+5 more)

### Community 131 - "egui"
Cohesion: 0.05
Nodes (31): DipSwitch, Knob, SIZE, SWEEP, Grip, Body, Corner, EdgeX (+23 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (26): audio_segment(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, drop_hint(), EDGE, HEADER_W, LANE_H, OBS_COLOR (+18 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.12
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "fixture_settings.rs"
Cohesion: 0.31
Nodes (5): RangeSlider, body(), caption(), hanging_of(), range_row()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.12
Nodes (25): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+17 more)

### Community 142 - "Model"
Cohesion: 0.18
Nodes (3): Model, ModelPart, part_roundtrip_through_json()

### Community 144 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 145 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - "ChannelRef"
Cohesion: 0.13
Nodes (5): blending_does_not_depend_on_the_order_of_values(), lerp(), move_hold_does_not_stop_a_playing_scene(), PlayingScene, ChannelRef

### Community 148 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 149 - "TouchView"
Cohesion: 0.14
Nodes (15): section_header(), ViewCtx, add_page(), add_tile(), Drag, Move, Resize, icon_picker() (+7 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.15
Nodes (23): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+15 more)

### Community 151 - "text_dim"
Cohesion: 0.13
Nodes (33): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), caption() (+25 more)

### Community 152 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 153 - "icon_library.rs"
Cohesion: 0.33
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "Profile"
Cohesion: 0.10
Nodes (14): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+6 more)

### Community 159 - "qlc-bundle.rs"
Cohesion: 0.11
Nodes (13): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, git(), main(), run() (+5 more)

### Community 160 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 163 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 164 - "LookPart"
Cohesion: 0.25
Nodes (8): LookAction, Copy, Paste, LookPart, All, Color, Intensity, Position

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.12
Nodes (32): apply(), find_item(), state_json(), apply_pick(), channels_of(), COLORS, content_rect(), current_color() (+24 more)

### Community 167 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 168 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 169 - "menu.rs"
Cohesion: 0.11
Nodes (20): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+12 more)

### Community 170 - "Clip"
Cohesion: 0.10
Nodes (11): Clip, FadeCurve, .ALL, FastSlow, Linear, SlowFast, Smooth, Layer (+3 more)

### Community 171 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (65): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+57 more)

### Community 177 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 181 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.08
Nodes (46): time_field(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), chip(), effect_header() (+38 more)

### Community 184 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 185 - "Engine"
Cohesion: 0.09
Nodes (4): Engine, FadeState, In, Out

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

### Community 193 - "make"
Cohesion: 0.40
Nodes (5): builtin(), builtins_are_seamless_and_decode(), make(), Pattern, render()

### Community 195 - "View"
Cohesion: 0.09
Nodes (15): apply(), View, FixtureSettings, SetupView, Arrange, AlignH, AlignV, Grid (+7 more)

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 206 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 208 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 211 - "app.rs"
Cohesion: 0.20
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 218 - "AppState"
Cohesion: 0.06
Nodes (21): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+13 more)

### Community 230 - "views/sound.rs"
Cohesion: 0.17
Nodes (20): button(), AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H (+12 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 234 - "qxf.rs"
Cohesion: 0.20
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

### Community 251 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 257 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 281 - "Obs"
Cohesion: 0.12
Nodes (17): media_action(), media_settings(), Obs, open(), read_json(), run(), seek(), send_json() (+9 more)

### Community 286 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 311 - "super"
Cohesion: 0.16
Nodes (12): fire(), bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, ObsSettings (+4 more)

## Knowledge Gaps
- **574 isolated node(s):** `Edit`, `New`, `Open`, `Template`, `Fixtures` (+569 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1442 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **143 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app/src/obs.rs`, `egui`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `mcp/src/venue.rs`, `ChannelRef`, `TouchView`, `text_dim`, `rig.rs`, `Obs`, `Host`, `app/src/audio.rs`, `t`, `core/src/lib.rs`, `views/touch.rs`, `mcp.rs`, `FixtureAction`, `menu.rs`, `scene_grid.rs`, `AudioPlayer`, `DmxCraftApp`, `Ui`, `tools.rs`, `remote/mod.rs`, `scene_props.rs`, `View`, `window.rs`, `stage.rs`, `Show`, `engine.rs`, `.ui`, `app.rs`, `AppState`, `Mapper`, `audio_triggers.rs`, `hashmap`, `Snapshot`, `mcp/src/profiles.rs`, `mapping/mod.rs`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 185 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 185 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Edit`, `New`, `Open` to the rest of the system?**
  _574 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12222222222222222 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `ProfileBuilder`, `library_panel.rs`, `egui`, `programmer.rs`, `control/timeline.rs`, `i18n/mod.rs`, `ViewCtx`, `fixture_settings.rs`, `viz/library.rs`, `titlebar.rs`, `TouchView`, `super`, `text_dim`, `text_field`, `Profile`, `fx_ui.rs`, `Quality`, `views/touch.rs`, `FixtureAction`, `menu.rs`, `scene_grid.rs`, `missing_audio.rs`, `DmxCraftApp`, `Ui`, `fader.rs`, `scene_props.rs`, `make`, `MenuAction`, `View`, `window.rs`, `stage.rs`, `Purpose`, `.ui`, `app.rs`, `AppState`, `viz/mod.rs`, `textures.rs`, `views/sound.rs`, `Dialog<'a>`, `mapping/mod.rs`, `.row`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12043010752688173 - nodes in this community are weakly interconnected._