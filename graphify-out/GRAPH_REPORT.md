# Graph Report - dmxcraft-src  (2026-11-11)

## Corpus Check
- 154 files · ~244,916 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 4340 nodes · 11124 edges · 304 communities (165 shown, 139 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 580 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9a992ded`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SoundPlayer
- library_panel.rs
- VizView
- open_dmx.rs
- fx_color.rs
- programmer.rs
- SoundBoard
- steps
- fx.rs
- DmxFrame
- ViewCtx
- DialogKind
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
- src/mapping.rs
- Settings
- profile_builder.rs
- ofl/src/lib.rs
- fx_ui.rs
- OrbitCamera
- dmxcraft
- app/src/audio.rs
- profile_builder.rs
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
- MenuAction
- core/src/venue.rs
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- .ui
- egui
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
- Obs
- look.rs
- updater.rs
- gltf_import.rs
- arc
- outline.rs
- Rect
- Theme
- audio_trigger_ui.rs
- Shape
- Venue
- View
- .row
- Tab
- convert.rs
- Timeline
- Host
- ProfileBuilder
- run
- app/src/obs.rs
- scene_card.rs
- CurveShape
- ColorPreset
- qlc-bundle.rs
- control/timeline.rs
- i18n/mod.rs
- mcp/src/venue.rs
- theme.rs
- stage_canvas.rs
- core/src/lib.rs
- missing_audio.rs
- DeviceModel
- ShowError
- RangeEffect
- Scene
- TouchView
- mapping/mod.rs
- text_dim
- send_frame
- icon_library.rs
- text_field
- app/src/profiles.rs
- BankScope
- EffectKind
- super
- mcp/src/profiles.rs
- Problem
- views/touch.rs
- lock.rs
- ShutterEffect
- menu.rs
- show
- Drag
- midi.rs
- tools.rs
- show.rs
- Easing
- ClipAction
- DeckState
- Transform
- fader.rs
- scene_props.rs
- .objects_tab
- Engine
- plan
- PlanShape
- scene_group.rs
- plan.rs
- TouchItem
- T
- stage_editor.rs
- env
- apply_effect_hold
- de.rs
- MoveShape
- MenuItem<'a>
- Purpose
- Tab
- HeaderAction
- build-local-artifacts
- IconButton
- TouchKind
- app.rs
- Scene
- AppState
- Knob<'a>
- ChannelDef
- FooterButton<'a>
- Shape
- views/sound.rs
- announce
- UpdateBadge
- qxf.rs
- release.ps1
- desktop.rs
- pointer
- release-install.md
- AudioPlayer
- with
- Physics
- FixtureLimits
- ObsSettings
- windows.rs
- .check_placement
- WaveState
- ok
- RemoteStatus
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Engine` - 235 edges
2. `t()` - 214 edges
3. `Show` - 163 edges
4. `Icon` - 118 edges
5. `AppState` - 106 edges
6. `ViewCtx` - 93 edges
7. `ChannelKind` - 76 edges
8. `ChannelRef` - 60 edges
9. `FixtureType` - 57 edges
10. `Profile` - 53 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `strip()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_groups.rs → crates/app/src/i18n/mod.rs

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

## Communities (304 total, 139 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (22): UserProfile, LibraryDrag, BuilderRequest, Edit, New, Open, Template, template() (+14 more)

### Community 2 - "VizView"
Cohesion: 0.15
Nodes (13): GizmoDrag, Missing, Mode, Build, Live, move_all(), Pick, Fixture (+5 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (12): UNIVERSE_SIZE, BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve() (+4 more)

### Community 4 - "fx_color.rs"
Cohesion: 0.15
Nodes (14): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+6 more)

### Community 5 - "programmer.rs"
Cohesion: 0.06
Nodes (47): ProgrammerMode, Blind, Edit, Live, kind_color(), Category, All, Beam (+39 more)

### Community 6 - "SoundBoard"
Cohesion: 0.11
Nodes (12): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), MIN_WIDTH, Pad, pad_grid_keeps_used_pads(), PAD_GRID_MAX (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.25
Nodes (18): all_presets_render_every_beam(), burst_width_widens_rings(), color_span(), ColorFx, grayscale_has_equal_channels(), hard_edges_use_palette_colors_only(), knight_rider(), knight_rider_lights_one_bar() (+10 more)

### Community 9 - "DmxFrame"
Cohesion: 0.05
Nodes (20): addresses_are_one_based(), DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound (+12 more)

### Community 10 - "ViewCtx"
Cohesion: 0.15
Nodes (17): ViewCtx, apply_placement(), renumber(), apply_texture(), aim_fixtures(), color_field(), duplicate_object(), floor_height() (+9 more)

### Community 11 - "DialogKind"
Cohesion: 0.20
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 12 - "server.rs"
Cohesion: 0.14
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
Nodes (39): Keyboard and MIDI mapping, Making new elements mappable, Mappable elements, Setting up MIDI controllers (Settings → MIDI), Turning mapping on, Window, AI assistant (MCP), Connecting (+31 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.21
Nodes (20): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+12 more)

### Community 20 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 21 - "super"
Cohesion: 0.11
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

### Community 27 - "src/mapping.rs"
Cohesion: 0.07
Nodes (34): Overlay, add_find_and_json(), channels_key(), Control, AudioFade, AudioMaster, AudioMute, BankStep (+26 more)

### Community 28 - "Settings"
Cohesion: 0.13
Nodes (6): cable_belongs_to_one_universe(), default_instance_name(), hidden_tabs_from_old_setting(), last_tab_stays(), RECENT_MAX, Settings

### Community 29 - "profile_builder.rs"
Cohesion: 0.06
Nodes (16): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+8 more)

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.21
Nodes (19): label_row(), toggle_row(), BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid() (+11 more)

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "profile_builder.rs"
Cohesion: 0.08
Nodes (43): capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H, full_combo(), general() (+35 more)

### Community 38 - "DMXCraft – UI guidelines"
Cohesion: 0.20
Nodes (9): 1. Principles, 2. Design tokens (`ui/theme.rs`), 3. Layout, 4. Components (`ui/widgets/`), 5. Interaction, 6. Text & icons, 7. UI architecture, 8. Checks before every commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.16
Nodes (11): run_updater(), Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "scene_grid.rs"
Cohesion: 0.08
Nodes (48): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+40 more)

### Community 43 - "duration"
Cohesion: 0.16
Nodes (8): Fullscreen, MAX_WAIT, Moving, SETTLE, STABLE_FRAMES, Step, Enter, Maximize

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DMXCraft – notes for AI assistants"
Cohesion: 0.40
Nodes (4): DMXCraft – notes for AI assistants, Orientation (save tokens), Rules, Structure

### Community 47 - "DmxCraftApp"
Cohesion: 0.17
Nodes (3): DmxCraftApp, fill_types(), keep()

### Community 48 - "settings_window.rs"
Cohesion: 0.23
Nodes (25): t(), ai(), audio(), AUTOSAVE_MINUTES, customize(), full_combo(), general(), HEIGHT (+17 more)

### Community 50 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (29): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+21 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 68 - "core/src/venue.rs"
Cohesion: 0.12
Nodes (12): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, moving_head_aim(), moving_head_direction(), mul(), Placement (+4 more)

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (25): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.11
Nodes (25): copy_look(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+17 more)

### Community 71 - "mcp.rs"
Cohesion: 0.18
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.22
Nodes (4): aim_moving_head_respects_limits(), .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.12
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.10
Nodes (21): along(), apply(), catmull(), circle_goes_round(), dist(), EaseScope, Cycle, Leg (+13 more)

### Community 77 - "engine.rs"
Cohesion: 0.18
Nodes (40): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+32 more)

### Community 78 - ".ui"
Cohesion: 0.27
Nodes (3): fmt_ms(), mark_clipboard(), TimelineView

### Community 79 - "egui"
Cohesion: 0.05
Nodes (32): DipSwitch, Knob, SIZE, SWEEP, Grip, Body, Corner, EdgeX (+24 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "range_slider.rs"
Cohesion: 0.16
Nodes (9): drag_number(), endless_delta(), raw_available(), from_pct(), HANDLE, pct(), RangeSlider, value_field() (+1 more)

### Community 87 - "fire"
Cohesion: 0.13
Nodes (17): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), is_value_control() (+9 more)

### Community 89 - "profile.rs"
Cohesion: 0.06
Nodes (31): MemoryProfiles, add_range_until_every_value_is_used(), beam_angle_order_gives_the_zoom_direction(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity() (+23 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.13
Nodes (25): GROUP_COLOR, AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), handles(), hit_y(), model_tex_id() (+17 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "textures.rs"
Cohesion: 0.13
Nodes (27): asphalt(), brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), checker(), cobblestone(), concrete() (+19 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.11
Nodes (14): apply_camera(), camera_json(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality (+6 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.10
Nodes (19): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+11 more)

### Community 101 - "autosave.rs"
Cohesion: 0.20
Nodes (7): AutoSave, autosave_only_when_due_dirty_and_saved_before(), now_secs(), off(), RECOVERY_EVERY, RecoveryInfo, without_folder_nothing_pending()

### Community 102 - "Obs"
Cohesion: 0.28
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 104 - "look.rs"
Cohesion: 0.28
Nodes (14): color_wheel_gets_nearest_slot(), is_color(), Look, LookTarget, Channel, VirtualDimmer, position_in_degrees(), rgbw_to_rgb_and_back() (+6 more)

### Community 105 - "updater.rs"
Cohesion: 0.15
Nodes (17): APP_NAME, client(), github_latest(), install_kind(), installed_by_msi(), InstallKind, Browser, Msi (+9 more)

### Community 106 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 107 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "outline.rs"
Cohesion: 0.09
Nodes (17): DragFixtures, DragObjects, FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place(), natural() (+9 more)

### Community 109 - "Rect"
Cohesion: 0.20
Nodes (9): audio_segment(), paint_bands(), paint_beats(), paint_clip(), paint_lead(), paint_segments(), paint_triggers(), track_header() (+1 more)

### Community 110 - "Theme"
Cohesion: 0.13
Nodes (12): Theme, .ALL, Contrast, Daylight, Ember, Midnight, Night, Paper (+4 more)

### Community 112 - "audio_trigger_ui.rs"
Cohesion: 0.21
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 113 - "Shape"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Venue"
Cohesion: 0.12
Nodes (11): load_file(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Texture, Object (+3 more)

### Community 118 - "View"
Cohesion: 0.17
Nodes (8): View, body(), caption(), FixtureSettings, hanging_of(), range_row(), SetupView, StageEditorView

### Community 119 - ".row"
Cohesion: 0.12
Nodes (19): drag_row(), drop_marker(), Renaming, Folder, Object, Tree, TreeAction, Delete (+11 more)

### Community 120 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.11
Nodes (32): angle_range(), capabilities(), channel_kind(), convert_fixture(), default_value(), degrees(), fine_aliases(), kinds() (+24 more)

### Community 123 - "Timeline"
Cohesion: 0.08
Nodes (24): media_track(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Group, Obs, Scene (+16 more)

### Community 127 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 128 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 129 - "run"
Cohesion: 0.27
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

### Community 130 - "app/src/obs.rs"
Cohesion: 0.13
Nodes (17): placeholders_match(), auth_matches_obs_websocket_example(), auth_response(), b64(), fire(), media_action(), media_settings(), names() (+9 more)

### Community 131 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 134 - "qlc-bundle.rs"
Cohesion: 0.23
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 136 - "control/timeline.rs"
Cohesion: 0.08
Nodes (25): add_audio(), add_audio_dialog(), add_video(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, drop_hint(), EDGE (+17 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.12
Nodes (13): all_texts_translated(), DE, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 138 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (55): accent_hover(), Active, ACTIVE, BAND_HIGH, BAND_LOW, BAND_MID, c(), canvas() (+47 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.12
Nodes (24): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+16 more)

### Community 142 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (11): SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "DeviceModel"
Cohesion: 0.12
Nodes (11): default_feedback(), APC_MK2_PALETTE, detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 145 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 147 - "RangeEffect"
Cohesion: 0.19
Nodes (9): ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other, Strobe (+1 more)

### Community 149 - "TouchView"
Cohesion: 0.15
Nodes (13): ControlViews, add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 150 - "mapping/mod.rs"
Cohesion: 0.12
Nodes (34): alt_click(), button(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels() (+26 more)

### Community 151 - "text_dim"
Cohesion: 0.16
Nodes (28): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_font() (+20 more)

### Community 152 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 155 - "text_field"
Cohesion: 0.15
Nodes (17): CANVAS_COLORS, canvas_swatches(), color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex() (+9 more)

### Community 158 - "app/src/profiles.rs"
Cohesion: 0.21
Nodes (9): AppProfiles, delete(), file(), load_all(), load_user(), save(), sort(), sort_user() (+1 more)

### Community 159 - "BankScope"
Cohesion: 0.11
Nodes (11): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank() (+3 more)

### Community 160 - "EffectKind"
Cohesion: 0.14
Nodes (13): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+5 more)

### Community 163 - "super"
Cohesion: 0.25
Nodes (10): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), bx(), find_box() (+2 more)

### Community 164 - "mcp/src/profiles.rs"
Cohesion: 0.27
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "views/touch.rs"
Cohesion: 0.17
Nodes (18): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), GAP, GRID_STEPS (+10 more)

### Community 167 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 168 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 169 - "menu.rs"
Cohesion: 0.17
Nodes (13): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+5 more)

### Community 170 - "show"
Cohesion: 0.20
Nodes (10): audio_control(), AudioInfo, load_meter(), AudioMaster, Mapping, ShowTab, missing_alert(), show() (+2 more)

### Community 171 - "Drag"
Cohesion: 0.29
Nodes (7): Drag, FadeIn, FadeOut, Lead, Left, Move, Right

### Community 173 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (56): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_clip_fields(), apply_scene_fields(), base64(), boolean() (+48 more)

### Community 177 - "show.rs"
Cohesion: 0.28
Nodes (13): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_axes_follow_a_mirrored_plan() (+5 more)

### Community 178 - "Easing"
Cohesion: 0.22
Nodes (8): Easing, .ALL, Custom, EaseIn, EaseInOut, EaseOut, EaseOutIn, Linear

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (17): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+9 more)

### Community 180 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 181 - "Transform"
Cohesion: 0.22
Nodes (8): normalized(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 182 - "fader.rs"
Cohesion: 0.08
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 183 - "scene_props.rs"
Cohesion: 0.06
Nodes (53): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+45 more)

### Community 184 - ".objects_tab"
Cohesion: 0.27
Nodes (5): heading_text(), fold_section(), section_header(), VizView, button()

### Community 185 - "Engine"
Cohesion: 0.06
Nodes (7): Engine, FadeState, In, Out, lerp(), PlayingScene, ChannelRef

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

### Community 193 - "TouchItem"
Cohesion: 0.34
Nodes (9): content_rect(), display_label(), fader_parts(), item_target(), mapping_of(), paint_cover(), paint_item(), target_entry() (+1 more)

### Community 195 - "stage_editor.rs"
Cohesion: 0.21
Nodes (8): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep()

### Community 196 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 197 - "apply_effect_hold"
Cohesion: 0.31
Nodes (10): apply(), apply_effect(), apply_effect_hold(), Beam, beams(), feature_levels(), luma(), to_rgb() (+2 more)

### Community 201 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 203 - "Purpose"
Cohesion: 0.20
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

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
Cohesion: 0.17
Nodes (8): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), RemoteLink, SettingsCx

### Community 213 - "Scene"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 218 - "AppState"
Cohesion: 0.08
Nodes (17): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+9 more)

### Community 227 - "ChannelDef"
Cohesion: 0.24
Nodes (5): ChannelDef, strobe_ranges(), ft(), nearest_slot(), shutter_value()

### Community 229 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 230 - "views/sound.rs"
Cohesion: 0.18
Nodes (19): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+11 more)

### Community 232 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 233 - "UpdateBadge"
Cohesion: 0.29
Nodes (7): UpdateBadge, Available, Busy, Error, Installed, InstallerStarted, None

### Community 234 - "qxf.rs"
Cohesion: 0.22
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

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

### Community 272 - "Physics"
Cohesion: 0.33
Nodes (4): Physics, ButtonMode, Toggle, Value

### Community 285 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 286 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 287 - "ok"
Cohesion: 0.53
Nodes (6): failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok()

### Community 288 - "RemoteStatus"
Cohesion: 0.50
Nodes (4): RemoteStatus, Error, Off, Running

## Knowledge Gaps
- **566 isolated node(s):** `Failed`, `Loading`, `VirtualDimmer`, `Add`, `Remove` (+561 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1387 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **139 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `ProfileBuilder`, `library_panel.rs`, `app/src/obs.rs`, `programmer.rs`, `control/timeline.rs`, `DmxFrame`, `ViewCtx`, `mcp/src/venue.rs`, `core/src/lib.rs`, `mapping/mod.rs`, `text_dim`, `rig.rs`, `app/src/audio.rs`, `profile_builder.rs`, `mcp/src/profiles.rs`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Snapshot`, `remote/mod.rs`, `scene_props.rs`, `TouchItem`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `engine.rs`, `.ui`, `egui`, `app.rs`, `fire`, `audio_triggers.rs`, `Obs`, `audio_trigger_ui.rs`, `View`, `AudioPlayer`, `Host`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Are the 185 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.autosave_tick()`) actually correct?**
  _`t()` has 185 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Failed`, `Loading`, `VirtualDimmer` to the rest of the system?**
  _566 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `library_panel.rs`, `VizView`, `SoundBoard`, `fx.rs`, `theme.rs`, `stage_canvas.rs`, `core/src/lib.rs`, `missing_audio.rs`, `ShowError`, `ObsSettings`, `TouchView`, `mapping/mod.rs`, `Scene`, `rig.rs`, `src/mapping.rs`, `.check_placement`, `BankScope`, `views/touch.rs`, `FixtureAction`, `scene_grid.rs`, `DmxCraftApp`, `tools.rs`, `show.rs`, `Snapshot`, `TouchLayout`, `scene_props.rs`, `Engine`, `scene_group.rs`, `TouchItem`, `apply_effect_hold`, `feature.rs`, `motion.rs`, `engine.rs`, `app.rs`, `Scene`, `fire`, `viz/mod.rs`, `autosave.rs`, `Venue`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Are the 6 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 6 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11932773109243698 - nodes in this community are weakly interconnected._