# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 148 files · ~224,136 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4095 nodes · 10977 edges · 260 communities (157 shown, 103 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 532 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `572f0def`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- panels.rs
- DeviceModel
- run
- viz/library.rs
- jobs
- steps
- README.md
- titlebar.rs
- GH_TOKEN
- build-local-artifacts
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- mcp/src/profiles.rs
- with
- Dialog<'a>
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- fx/mapping.rs
- DMXCraft – UI guidelines
- UpdateState
- FixtureAction
- range_slider.rs
- textures.rs
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- Mapping
- remote/mod.rs
- TouchLayout
- assist.rs
- MenuAction
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- View
- egui
- OpenDmx
- views/sound.rs
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
- Knob<'a>
- show.rs
- Deserialize
- mcp/src/venue.rs
- core/src/obs.rs
- arc
- Control
- TimelineView
- app.rs
- server.rs
- Shape
- Venue
- dialog.rs
- outline.rs
- send_frame
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- ShutterEffect
- app/src/obs.rs
- Obs
- CurveShape
- ColorPreset
- UpdateBadge
- control/timeline.rs
- i18n/mod.rs
- lock.rs
- theme.rs
- stage_canvas.rs
- Tab
- missing_audio.rs
- pointer
- fader.rs
- Mapper
- EffectKind
- item
- mapping/mod.rs
- text_dim
- Physics
- icon_library.rs
- .update
- core/src/sound.rs
- OutputStatus
- .add_scene
- core/src/lib.rs
- widgets/mod.rs
- Transform
- Problem
- views/touch.rs
- app/src/profiles.rs
- Category
- BankScope
- Fullscreen
- midi.rs
- ChaserPreset
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- TriggerBand
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- super
- plan.rs
- Settings
- DeckState
- WaveState
- de.rs
- dip.rs
- ShowError
- ViewCtx
- Purpose
- MenuItem<'a>
- HeaderAction
- IconButton
- Drag
- Scene
- Status
- ok
- Ui
- FooterButton<'a>
- Grip
- release.ps1
- Drag
- desktop.rs
- release-install.md
- AudioPlayer
- .fade_state
- FixtureLimits
- ViewCtx

## God Nodes (most connected - your core abstractions)
1. `Engine` - 228 edges
2. `t()` - 213 edges
3. `Show` - 147 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `text_dim()` - 101 edges
7. `ViewCtx` - 88 edges
8. `ChannelKind` - 75 edges
9. `text()` - 69 edges
10. `border()` - 68 edges

## Surprising Connections (you probably didn't know these)
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs

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

## Communities (260 total, 103 thin omitted)

### Community 0 - "SoundPlayer"
Cohesion: 0.13
Nodes (3): SoundPlayer, poll(), SoundBoard

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (16): UserProfile, LibraryDrag, BuilderRequest, Edit, New, Open, Template, template() (+8 more)

### Community 2 - "runtime.rs"
Cohesion: 0.24
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.18
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "Theme"
Cohesion: 0.09
Nodes (20): main(), Active, ACTIVE, Overrides, overrides_apply(), resolve(), set_overrides(), set_theme() (+12 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (37): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+29 more)

### Community 6 - "AppState"
Cohesion: 0.10
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.11
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer, Failing

### Community 10 - "panels.rs"
Cohesion: 0.07
Nodes (23): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, OutlineTab, Fixtures (+15 more)

### Community 11 - "DeviceModel"
Cohesion: 0.13
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 12 - "run"
Cohesion: 0.27
Nodes (5): Link, run(), session(), Shared, ObsSettings

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.16
Nodes (21): AudioInfo, clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu() (+13 more)

### Community 20 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 29 - "Dialog<'a>"
Cohesion: 0.14
Nodes (5): Dialog<'a>, Kind, Cancel, Primary, Secondary

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.12
Nodes (27): rgb(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore (+19 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (13): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+5 more)

### Community 36 - "t"
Cohesion: 0.06
Nodes (55): moved_out(), t(), heading_text(), text_field(), capability_fields(), category_label(), check(), color_label() (+47 more)

### Community 37 - "fx/mapping.rs"
Cohesion: 0.16
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.08
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 43 - "textures.rs"
Cohesion: 0.05
Nodes (35): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+27 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.16
Nodes (4): DmxCraftApp, fill_types(), LockDialog, viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.23
Nodes (21): ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint(), midi() (+13 more)

### Community 50 - "Mapping"
Cohesion: 0.27
Nodes (8): MapFile, WindowState, add_find_and_json(), MapKind, Keyboard, Midi, Mapping, Mappings

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "assist.rs"
Cohesion: 0.15
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (26): MenuAction, About, AudioFade, AudioMaster, AudioMute, CheckUpdates, DmxLevels, InstallUpdate (+18 more)

### Community 69 - "window.rs"
Cohesion: 0.14
Nodes (16): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, pick_export() (+8 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (8): .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.13
Nodes (20): FxPreview, apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx (+12 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "View"
Cohesion: 0.25
Nodes (6): View, body(), caption(), FixtureSettings, range_row(), SetupView

### Community 79 - "egui"
Cohesion: 0.04
Nodes (38): DipSwitch, Knob, SIZE, SWEEP, LimitPad, CARD_H, clip(), COMPACT_H (+30 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.19
Nodes (18): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+10 more)

### Community 87 - "fire"
Cohesion: 0.18
Nodes (19): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fixed_channels_work_without_selection(), group_set() (+11 more)

### Community 89 - "profile.rs"
Cohesion: 0.06
Nodes (30): import(), MemoryProfiles, add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity() (+22 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.09
Nodes (35): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), GizmoDrag, Handles, handles(), hit_y() (+27 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.05
Nodes (67): zone_color(), CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H (+59 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.19
Nodes (3): camera_json(), OrbitCamera, ray_through_center_hits_target()

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.13
Nodes (13): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+5 more)

### Community 102 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 104 - "Deserialize"
Cohesion: 0.10
Nodes (18): builtin(), load_user(), named(), NamedPalette, path(), save_user(), ChannelDef, ChannelRange (+10 more)

### Community 105 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 106 - "core/src/obs.rs"
Cohesion: 0.60
Nodes (4): bx(), find_box(), mp4_duration_ms(), reads_mp4_length()

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Control"
Cohesion: 0.07
Nodes (30): Overlay, channels_key(), Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout (+22 more)

### Community 109 - "TimelineView"
Cohesion: 0.17
Nodes (8): fmt_ms(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.39
Nodes (5): ControlViews, edit_preview(), load_step(), OUTPUT_HZ, ProgrammerView

### Community 112 - "server.rs"
Cohesion: 0.12
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 113 - "Shape"
Cohesion: 0.22
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (20): renumber(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+12 more)

### Community 118 - "dialog.rs"
Cohesion: 0.13
Nodes (8): Button, Dialog, DialogResult, FOOTER_H, FooterButton, HEADER_H, INPUT_H, LABEL_W

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (27): deg_quat(), euler_matches_core_rotation(), quat_deg(), DragFixtures, DragObjects, FixtureSort, Address, Name (+19 more)

### Community 120 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 121 - "convert.rs"
Cohesion: 0.10
Nodes (31): names(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+23 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (19): add_video(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video (+11 more)

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.16
Nodes (11): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), StageEditorView (+3 more)

### Community 129 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 130 - "app/src/obs.rs"
Cohesion: 0.24
Nodes (7): auth_matches_obs_websocket_example(), auth_response(), b64(), open(), read_json(), send_json(), Ws

### Community 131 - "Obs"
Cohesion: 0.28
Nodes (7): connects_and_plays_video_block(), engine(), fake_obs(), Obs, VideoPlay, wait(), wrong_password_is_reported()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (22): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, has_audio(), HEADER_W (+14 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 140 - "theme.rs"
Cohesion: 0.08
Nodes (44): BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas(), canvas_dim(), canvas_faint(), canvas_is_light() (+36 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "Tab"
Cohesion: 0.15
Nodes (12): SettingsWindow, Tab, Ai, .ALL, Audio, Customize, General, Midi (+4 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.07
Nodes (23): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+15 more)

### Community 144 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 145 - "fader.rs"
Cohesion: 0.18
Nodes (8): DOT_H, FADER_W, fine_mark(), HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP

### Community 146 - "Mapper"
Cohesion: 0.19
Nodes (7): fires_id(), Mapper, overlay_id(), pick_id(), Sent, states_id(), ui_states()

### Community 147 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 149 - "item"
Cohesion: 0.11
Nodes (25): select_all(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H (+17 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.15
Nodes (23): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), Pick, pick(), pick_full() (+15 more)

### Community 151 - "text_dim"
Cohesion: 0.14
Nodes (46): palette_button(), table(), sidebar(), theme_tiles(), audio_control(), load_meter(), missing_alert(), show() (+38 more)

### Community 152 - "Physics"
Cohesion: 0.40
Nodes (4): Physics, ButtonMode, Toggle, Value

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - ".update"
Cohesion: 0.27
Nodes (7): fire(), media_action(), media_settings(), seek(), ObsAction, Scene, Source

### Community 157 - "core/src/sound.rs"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 158 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 159 - ".add_scene"
Cohesion: 0.13
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (10): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+2 more)

### Community 163 - "widgets/mod.rs"
Cohesion: 0.17
Nodes (11): FaderEvent, Changed, None, Release, ToggleFx, CanvasMode, Edit, Select (+3 more)

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.13
Nodes (30): apply(), find_item(), state_json(), apply_pick(), channels_of(), COLORS, content_rect(), current_color() (+22 more)

### Community 167 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (9): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+1 more)

### Community 168 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 169 - "BankScope"
Cohesion: 0.19
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 171 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 174 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (55): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64(), boolean(), build_steps() (+47 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 178 - "DmxFrame"
Cohesion: 0.15
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "TriggerBand"
Cohesion: 0.29
Nodes (5): TriggerBand, .ALL, High, Low, Mid

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (39): button(), advanced(), apply_to_others(), banks_of(), bar_slider(), block_props(), effect_header(), fit_mapping() (+31 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.07
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 187 - "super"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 194 - "Settings"
Cohesion: 0.28
Nodes (3): Settings, SettingsCx, UsbDmxPort

### Community 196 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 197 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 200 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.15
Nodes (12): section_header(), add_page(), ImageTarget, Item, Page, pick_image(), prepare_image(), select_only() (+4 more)

### Community 203 - "Purpose"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 211 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 213 - "Scene"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 217 - "Status"
Cohesion: 0.33
Nodes (5): Status, Connected, Connecting, Failed, Off

### Community 223 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 237 - "Drag"
Cohesion: 0.50
Nodes (3): Drag, Move, Resize

### Community 251 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 273 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 278 - "ViewCtx"
Cohesion: 0.13
Nodes (8): fold_section(), ViewCtx, vec3_row(), drag_row(), drop_marker(), Folder, VizView, VizView

## Knowledge Gaps
- **546 isolated node(s):** `General`, `Customize`, `Startup`, `Outputs`, `Midi` (+541 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1309 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **103 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `.fade_state`, `Mapper`, `item`, `ViewCtx`, `text_dim`, `rig.rs`, `.update`, `mcp/src/profiles.rs`, `core/src/lib.rs`, `app/src/audio.rs`, `t`, `views/touch.rs`, `app/src/profiles.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `DmxFrame`, `remote/mod.rs`, `scene_props.rs`, `super`, `Settings`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `View`, `fire`, `scene_grid.rs`, `audio_triggers.rs`, `mcp/src/venue.rs`, `TimelineView`, `app.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.125) - this node is a cross-community bridge._
- **Are the 176 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 176 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Customize`, `Startup` to the rest of the system?**
  _546 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.12941176470588237 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `SoundPlayer`, `library_panel.rs`, `fx.rs`, `theme.rs`, `stage_canvas.rs`, `run`, `missing_audio.rs`, `Mapper`, `EffectKind`, `rig.rs`, `.add_scene`, `core/src/lib.rs`, `widgets/mod.rs`, `views/touch.rs`, `FixtureAction`, `BankScope`, `DmxCraftApp`, `tools.rs`, `Mapping`, `TouchLayout`, `scene_props.rs`, `Engine`, `super`, `feature.rs`, `ViewCtx`, `ShowError`, `motion.rs`, `engine.rs`, `Scene`, `fire`, `viz/mod.rs`, `scene_grid.rs`, `show.rs`, `app.rs`, `Venue`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13911290322580644 - nodes in this community are weakly interconnected._