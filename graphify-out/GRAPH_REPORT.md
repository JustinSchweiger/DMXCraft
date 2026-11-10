# Graph Report - dmxcraft-src  (2026-11-10)

## Corpus Check
- 148 files · ~224,602 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4104 nodes · 10946 edges · 266 communities (151 shown, 115 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 531 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `565d5911`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
- programmer.rs
- Clip
- steps
- fx.rs
- OutputError
- Shape
- DeviceModel
- .update
- viz/library.rs
- build-local-artifacts
- steps
- README.md
- titlebar.rs
- color_picker.rs
- textures.rs
- Icon
- core/src/audio.rs
- rig.rs
- steps
- host
- Profile
- VizView
- dialog.rs
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
- drag_number
- Deserialize
- render.rs
- DMXCraft – notes for AI assistants
- DmxCraftApp
- settings_window.rs
- ChannelRef
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
- fixture_settings.rs
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
- RangeEffect
- mcp/src/venue.rs
- core/src/venue.rs
- arc
- Control
- TimelineView
- app.rs
- server.rs
- Shape
- Venue
- gltf_import.rs
- outline.rs
- Box<dyn serialport::SerialPort>
- convert.rs
- Timeline
- Host
- widgets/mod.rs
- .viewport
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
- time_field.rs
- icon_library.rs
- ObsAction
- core/src/sound.rs
- OutputStatus
- .add_scene
- core/src/lib.rs
- block_props
- Transform
- Problem
- views/touch.rs
- app/src/profiles.rs
- Category
- Timeline
- Fullscreen
- midi.rs
- qlc-bundle.rs
- tools.rs
- PlanShape
- DmxFrame
- ClipAction
- scene_card.rs
- qxf.rs
- Fader
- scene_props.rs
- TouchKind
- Engine
- PaletteStore
- plan.rs
- Settings
- DeckState
- WaveState
- de.rs
- TouchItem
- ShowError
- ViewCtx
- Purpose
- MenuItem<'a>
- HeaderAction
- IconButton
- Drag
- Scene
- audio_trigger_ui.rs
- swatch.rs
- ok
- Quality
- Ui
- FooterButton<'a>
- Segmented<'a, T>
- Cmd
- Grip
- release.ps1
- .of
- desktop.rs
- release-install.md
- AudioPlayer
- FixtureLimits
- ViewCtx

## God Nodes (most connected - your core abstractions)
1. `Engine` - 228 edges
2. `t()` - 214 edges
3. `Show` - 147 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `text_dim()` - 95 edges
7. `ViewCtx` - 88 edges
8. `ChannelKind` - 75 edges
9. `border()` - 68 edges
10. `text()` - 67 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `placeholders_match()` --calls--> `names()`  [INFERRED]
  crates/app/src/i18n/mod.rs → crates/app/src/obs.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (266 total, 115 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.07
Nodes (30): UserProfile, AppState, Focus, Bank, Effect, Scenes, Timeline, LibraryDrag (+22 more)

### Community 2 - "runtime.rs"
Cohesion: 0.20
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.12
Nodes (16): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, MAB (+8 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 5 - "programmer.rs"
Cohesion: 0.16
Nodes (23): color_bar(), current(), elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block() (+15 more)

### Community 6 - "Clip"
Cohesion: 0.20
Nodes (7): audio_segment(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), Clip

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (29): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+21 more)

### Community 9 - "OutputError"
Cohesion: 0.15
Nodes (6): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer

### Community 10 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 11 - "DeviceModel"
Cohesion: 0.13
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 12 - ".update"
Cohesion: 0.19
Nodes (13): media_action(), media_settings(), names(), run(), seek(), send_json(), session(), Shared (+5 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "build-local-artifacts"
Cohesion: 0.07
Nodes (30): env, if, needs, runs-on, env, needs, runs-on, container (+22 more)

### Community 15 - "steps"
Cohesion: 0.22
Nodes (18): steps, steps, steps, steps, steps, id, if, name (+10 more)

### Community 17 - "README.md"
Cohesion: 0.04
Nodes (40): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+32 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.19
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+13 more)

### Community 20 - "color_picker.rs"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

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
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "steps"
Cohesion: 0.08
Nodes (24): env, runs-on, steps, timeout-minutes, GH_TOKEN, TAG, tag, jobs (+16 more)

### Community 26 - "host"
Cohesion: 0.15
Nodes (13): if, needs, outputs, runs-on, host, plan, publishing, tag (+5 more)

### Community 27 - "Profile"
Cohesion: 0.11
Nodes (15): import(), MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list() (+7 more)

### Community 28 - "VizView"
Cohesion: 0.11
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+6 more)

### Community 29 - "dialog.rs"
Cohesion: 0.08
Nodes (12): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind (+4 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.14
Nodes (24): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), ROW_H, slider_row() (+16 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.18
Nodes (10): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), start_player(), compute() (+2 more)

### Community 36 - "t"
Cohesion: 0.07
Nodes (55): t(), heading_text(), kind_color(), capability_fields(), category_label(), check(), color_label(), FileJob (+47 more)

### Community 37 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.09
Nodes (28): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+20 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "Deserialize"
Cohesion: 0.09
Nodes (12): Physics, bx(), find_box(), mp4_duration_ms(), reads_mp4_length(), B64, b64_decode(), b64_encode() (+4 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.18
Nodes (4): DmxCraftApp, fill_types(), moved_out(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.23
Nodes (21): ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint(), midi() (+13 more)

### Community 50 - "ChannelRef"
Cohesion: 0.18
Nodes (3): lerp(), PlayingScene, ChannelRef

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (27): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+19 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "assist.rs"
Cohesion: 0.24
Nodes (8): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (33): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+25 more)

### Community 69 - "window.rs"
Cohesion: 0.15
Nodes (17): channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button(), pick_export() (+9 more)

### Community 70 - "stage.rs"
Cohesion: 0.15
Nodes (15): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+7 more)

### Community 71 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (8): .VIRTUAL_DIMMER, Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (23): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+15 more)

### Community 74 - "OutputKind"
Cohesion: 0.19
Nodes (7): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation, list_ports(), UsbDmxPort

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.19
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.09
Nodes (17): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+9 more)

### Community 79 - "egui"
Cohesion: 0.08
Nodes (24): DipSwitch, Knob, SIZE, SWEEP, LimitPad, from_pct(), HANDLE, RangeSlider (+16 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.16
Nodes (23): button(), handle(), rgb(), surface(), AUDIO_EXT, audio_files(), BAR_H, clock() (+15 more)

### Community 87 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (24): add_range_until_every_value_is_used(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+16 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.15
Nodes (27): aim_selected(), apply_placement(), AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, handles(), hit_y() (+19 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): zone_color(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+42 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.04
Nodes (51): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+43 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (18): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+10 more)

### Community 102 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 104 - "RangeEffect"
Cohesion: 0.19
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 105 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 106 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, mul(), Placement, rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "Control"
Cohesion: 0.07
Nodes (38): Overlay, MapFile, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control (+30 more)

### Community 109 - "TimelineView"
Cohesion: 0.21
Nodes (4): fmt_ms(), mark_clipboard(), obs_label(), TimelineView

### Community 110 - "app.rs"
Cohesion: 0.11
Nodes (10): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView, View, SetupView (+2 more)

### Community 112 - "server.rs"
Cohesion: 0.12
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 113 - "Shape"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Venue"
Cohesion: 0.12
Nodes (9): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), Transform (+1 more)

### Community 118 - "gltf_import.rs"
Cohesion: 0.16
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (25): DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 123 - "Timeline"
Cohesion: 0.12
Nodes (13): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Obs, Scene, Video, fades(), group_moves_together_without_covering_others() (+5 more)

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 129 - ".viewport"
Cohesion: 0.16
Nodes (6): decode_texture(), Handles, model_tex_id(), QualityParams, seg_dist(), upload_assets()

### Community 130 - "app/src/obs.rs"
Cohesion: 0.15
Nodes (9): auth_matches_obs_websocket_example(), auth_response(), b64(), Link, open(), read_json(), VideoPlay, Ws (+1 more)

### Community 131 - "Obs"
Cohesion: 0.31
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

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
Cohesion: 0.10
Nodes (18): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, is_video(), LANE_H, Mark (+10 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (52): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+44 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (21): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+13 more)

### Community 142 - "Tab"
Cohesion: 0.15
Nodes (12): SettingsWindow, Tab, Ai, .ALL, Audio, Customize, General, Midi (+4 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.07
Nodes (26): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), AUDIO_EXT, CHECK_EVERY, file_name() (+18 more)

### Community 144 - "pointer"
Cohesion: 0.67
Nodes (3): hovering(), os_cursor(), pointer()

### Community 145 - "fader.rs"
Cohesion: 0.14
Nodes (12): DOT_H, FADER_W, FaderEvent, Changed, None, Release, ToggleFx, HANDLE (+4 more)

### Community 147 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 149 - "item"
Cohesion: 0.10
Nodes (25): select_all(), text_field(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+17 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (34): alt_click(), button_rect(), channel_name(), color(), describe(), describe_mapping(), fader(), fader_channels() (+26 more)

### Community 151 - "text_dim"
Cohesion: 0.12
Nodes (42): table(), sidebar(), theme_tiles(), accent(), accent_dim(), apply(), bg(), body() (+34 more)

### Community 152 - "time_field.rs"
Cohesion: 0.25
Nodes (12): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+4 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "ObsAction"
Cohesion: 0.50
Nodes (4): fire(), ObsAction, Scene, Source

### Community 157 - "core/src/sound.rs"
Cohesion: 0.13
Nodes (11): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+3 more)

### Community 158 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 159 - ".add_scene"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, steps_insert_remove_and_normalize(), Zone

### Community 160 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (16): UNIVERSE_SIZE, SceneProgress, BankScope, All, Bank, Banks, Off, OutsideBank (+8 more)

### Community 163 - "block_props"
Cohesion: 0.19
Nodes (9): block_props(), name_pick(), obs_props(), properties(), row(), scope_row(), drag_row(), Folder (+1 more)

### Community 164 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.11
Nodes (23): apply(), state_json(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag (+15 more)

### Community 167 - "app/src/profiles.rs"
Cohesion: 0.19
Nodes (9): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+1 more)

### Community 168 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 169 - "Timeline"
Cohesion: 0.16
Nodes (8): add_audio(), add_audio_dialog(), add_video(), has_audio(), GridMode, Beat, Time, Timeline

### Community 171 - "Fullscreen"
Cohesion: 0.17
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 173 - "midi.rs"
Cohesion: 0.13
Nodes (12): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+4 more)

### Community 174 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (55): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64(), boolean(), build_steps() (+47 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 181 - "qxf.rs"
Cohesion: 0.13
Nodes (15): ChannelDef, strobe_ranges(), ft(), channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group() (+7 more)

### Community 182 - "Fader"
Cohesion: 0.19
Nodes (3): Fader, FaderRange, fader_ranges()

### Community 183 - "scene_props.rs"
Cohesion: 0.09
Nodes (31): advanced(), apply_to_others(), banks_of(), fit_mapping(), FX_EXT, FxDialog, FxFile, header() (+23 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Engine"
Cohesion: 0.08
Nodes (4): Engine, FadeState, In, Out

### Community 187 - "PaletteStore"
Cohesion: 0.36
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 196 - "DeckState"
Cohesion: 0.15
Nodes (10): Player, DeckRt, DeckState, Empty, Ended, Fading, Paused, Playing (+2 more)

### Community 197 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 200 - "TouchItem"
Cohesion: 0.29
Nodes (8): content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of(), target_entry(), TouchItem

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.15
Nodes (10): section_header(), add_page(), ImageTarget, Page, item_title(), kind_name(), paint_cover(), pick_image() (+2 more)

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
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 217 - "audio_trigger_ui.rs"
Cohesion: 0.39
Nodes (3): apply_trigger_pick(), trigger_cx(), TriggerCx

### Community 223 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 226 - "Quality"
Cohesion: 0.29
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 232 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 236 - "release.ps1"
Cohesion: 0.60
Nodes (3): Fail(), Run(), Undo()

### Community 251 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 278 - "ViewCtx"
Cohesion: 0.13
Nodes (11): fold_section(), ViewCtx, Item, renumber(), vec3_row(), apply_texture(), TexAction, Builtin (+3 more)

## Knowledge Gaps
- **546 isolated node(s):** `Enter`, `Maximize`, `None`, `Available`, `Busy` (+541 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1315 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **115 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `app/src/obs.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `.update`, `missing_audio.rs`, `Mapper`, `item`, `mapping/mod.rs`, `text_dim`, `ViewCtx`, `rig.rs`, `Profile`, `core/src/lib.rs`, `app/src/audio.rs`, `block_props`, `t`, `views/touch.rs`, `app/src/profiles.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `ChannelRef`, `remote/mod.rs`, `DmxFrame`, `scene_props.rs`, `Settings`, `window.rs`, `stage.rs`, `mcp.rs`, `TouchItem`, `Show`, `engine.rs`, `fixture_settings.rs`, `fire`, `audio_trigger_ui.rs`, `scene_grid.rs`, `audio_triggers.rs`, `mcp/src/venue.rs`, `TimelineView`, `app.rs`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.130) - this node is a cross-community bridge._
- **Are the 177 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 177 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Enter`, `Maximize`, `None` to the rest of the system?**
  _546 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13368983957219252 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `widgets/mod.rs`, `library_panel.rs`, `app/src/obs.rs`, `.viewport`, `programmer.rs`, `control/timeline.rs`, `i18n/mod.rs`, `viz/library.rs`, `missing_audio.rs`, `fader.rs`, `titlebar.rs`, `color_picker.rs`, `item`, `mapping/mod.rs`, `text_dim`, `ViewCtx`, `textures.rs`, `dialog.rs`, `fx_ui.rs`, `core/src/lib.rs`, `block_props`, `views/touch.rs`, `app/src/profiles.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `Fader`, `scene_props.rs`, `PaletteStore`, `Settings`, `MenuAction`, `window.rs`, `stage.rs`, `TouchItem`, `ViewCtx`, `Purpose`, `fixture_settings.rs`, `egui`, `views/sound.rs`, `audio_trigger_ui.rs`, `scene_grid.rs`, `TimelineView`, `app.rs`, `outline.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06892230576441102 - nodes in this community are weakly interconnected._