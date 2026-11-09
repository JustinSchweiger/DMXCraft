# Graph Report - dmxcraft-src  (2026-11-09)

## Corpus Check
- 143 files · ~213,401 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3985 nodes · 10031 edges · 275 communities (153 shown, 122 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 519 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `af716a44`
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
- Deserialize
- Engine
- app/src/obs.rs
- viz/library.rs
- jobs
- steps
- Tastatur- und MIDI-Zuordnung
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- cable_belongs_to_one_universe
- rig.rs
- scene_props.rs
- host
- core/src/audio.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- app/src/audio.rs
- t
- scene_grid.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag.rs
- textures.rs
- Fader
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- super
- remote/mod.rs
- TouchLayout
- Host
- MenuAction
- ViewCtx
- window.rs
- stage.rs
- Tab
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- hashmap
- Vec2
- OpenDmx
- views/sound.rs
- mcp.rs
- profile.rs
- toolbar.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- Action
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- panels.rs
- Deserialize
- fx/mapping.rs
- en.rs
- Control
- Rect
- app.rs
- server.rs
- ev2.rs
- Venue
- Dialog<'a>
- outline.rs
- convert.rs
- Timeline
- widgets/mod.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- scene_card.rs
- theme.rs
- stage_canvas.rs
- core/src/venue.rs
- missing_audio.rs
- patch_grid.rs
- Mapper
- EffectKind
- menu.rs
- fire
- bank_menu
- target_shape
- icon_library.rs
- mcp/src/venue.rs
- audio_trigger_ui.rs
- ShutterEffect
- core/src/sound.rs
- .add_scene
- show.rs
- Tab
- color_picker.rs
- Problem
- TouchItem
- Profile
- gltf_import.rs
- core/src/lib.rs
- Drag
- Fullscreen
- midi.rs
- Scene
- tools.rs
- PlanShape
- RemoteStatus
- ClipAction
- demo.rs
- qxf.rs
- protocol.rs
- egui
- TouchKind
- AudioTrigger
- .page_bar
- super
- ProfileBuilder
- MenuItem<'a>
- plan.rs
- text_field
- ChaserPreset
- DeckState
- qlc-bundle.rs
- Pos2
- Kind
- dip.rs
- ShowError
- ViewCtx
- Purpose
- arc
- HeaderAction
- FixtureLimits
- IconButton
- WaveState
- limit_pad.rs
- channel_name
- UpdateBadge
- Quality
- .check_placement
- Knob<'a>
- palettes.rs
- EnvState
- FooterButton<'a>
- plan
- AudioPlayer
- Box<dyn serialport::SerialPort>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 227 edges
2. `t()` - 206 edges
3. `Show` - 150 edges
4. `Icon` - 115 edges
5. `AppState` - 102 edges
6. `ViewCtx` - 88 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 57 edges
9. `Profile` - 53 edges
10. `FixtureType` - 50 edges

## Surprising Connections (you probably didn't know these)
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (275 total, 122 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (19): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (13): active(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.08
Nodes (43): button(), ProgrammerMode, Blind, Edit, Live, Category, All, Beam (+35 more)

### Community 6 - "AppState"
Cohesion: 0.09
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.09
Nodes (8): DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer

### Community 10 - "Deserialize"
Cohesion: 0.08
Nodes (9): RECENT_MAX, addresses_are_one_based(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart (+1 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (8): item_value(), Engine, FadeState, In, Out, lerp(), PlayingScene, ChannelRef

### Community 12 - "app/src/obs.rs"
Cohesion: 0.07
Nodes (41): digest(), hash_and_verify(), hash_password(), hex(), verify(), auth_matches_obs_websocket_example(), auth_response(), b64() (+33 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (18): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+10 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.04
Nodes (38): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten, Aufbau, Einschalten (+30 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.21
Nodes (19): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+11 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 24 - "rig.rs"
Cohesion: 0.18
Nodes (17): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), kind() (+9 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.07
Nodes (53): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+45 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.06
Nodes (45): analyze(), Analyzer, AudioAnalysis, AudioGrid, AudioSegment, bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), Biquad (+37 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.14
Nodes (27): canvas(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore (+19 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.15
Nodes (12): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+4 more)

### Community 36 - "t"
Cohesion: 0.09
Nodes (44): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+36 more)

### Community 37 - "scene_grid.rs"
Cohesion: 0.17
Nodes (12): programmer_values(), MoveScenes, apply(), apply_bank(), chip(), COLUMN_W, drag_ghost(), drop_target() (+4 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag.rs"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.18
Nodes (17): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+9 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.16
Nodes (26): Settings, ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint() (+18 more)

### Community 50 - "super"
Cohesion: 0.12
Nodes (12): default_feedback(), APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic (+4 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (34): addresses(), APP_JS, apply(), CLIENT_TIMEOUT, Cmd, Color, Press, Value (+26 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "Host"
Cohesion: 0.21
Nodes (9): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+1 more)

### Community 66 - "MenuAction"
Cohesion: 0.07
Nodes (32): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+24 more)

### Community 68 - "ViewCtx"
Cohesion: 0.19
Nodes (7): fold_section(), ViewCtx, vec3_row(), drop_marker(), Folder, VizView, VizView

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.20
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 72 - "Show"
Cohesion: 0.18
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.20
Nodes (36): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+28 more)

### Community 78 - "hashmap"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 79 - "Vec2"
Cohesion: 0.07
Nodes (18): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, from_pct() (+10 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.20
Nodes (18): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+10 more)

### Community 87 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (26): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+18 more)

### Community 90 - "toolbar.rs"
Cohesion: 0.22
Nodes (9): select_all(), chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width() (+1 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.09
Nodes (36): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, handles() (+28 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

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
Cohesion: 0.18
Nodes (9): compute(), env_key(), EnvKey, EventKey, Events, fire(), is_fader(), mapping() (+1 more)

### Community 101 - "panels.rs"
Cohesion: 0.10
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, apply_texture(), OutlineTab (+6 more)

### Community 104 - "Deserialize"
Cohesion: 0.10
Nodes (14): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+6 more)

### Community 106 - "fx/mapping.rs"
Cohesion: 0.15
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 108 - "Control"
Cohesion: 0.07
Nodes (35): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, AudioFade, AudioMaster (+27 more)

### Community 109 - "Rect"
Cohesion: 0.21
Nodes (7): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView, track_header()

### Community 110 - "app.rs"
Cohesion: 0.20
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), ProgrammerView

### Community 112 - "server.rs"
Cohesion: 0.13
Nodes (14): Running, dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server() (+6 more)

### Community 113 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "Venue"
Cohesion: 0.12
Nodes (10): renumber(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree() (+2 more)

### Community 118 - "Dialog<'a>"
Cohesion: 0.07
Nodes (14): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+6 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (25): drag_row(), DragFixtures, DragObjects, FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (17): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video, fades() (+9 more)

### Community 128 - "widgets/mod.rs"
Cohesion: 0.13
Nodes (13): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+5 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.10
Nodes (26): add_audio(), add_audio_dialog(), add_video(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms() (+18 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "scene_card.rs"
Cohesion: 0.20
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 140 - "theme.rs"
Cohesion: 0.06
Nodes (61): accent(), accent_dim(), accent_hover(), apply(), BAND_HIGH, BAND_LOW, BAND_MID, bg() (+53 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, mul(), Placement, rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.18
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 147 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 149 - "menu.rs"
Cohesion: 0.19
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 150 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 151 - "bank_menu"
Cohesion: 0.24
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+5 more)

### Community 152 - "target_shape"
Cohesion: 0.14
Nodes (29): alt_click(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id(), Pick (+21 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "audio_trigger_ui.rs"
Cohesion: 0.26
Nodes (8): apply_trigger_pick(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx, triggers_ui()

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - "core/src/sound.rs"
Cohesion: 0.18
Nodes (8): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), Pad, PADS, SoundBoard, Track

### Community 159 - ".add_scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 160 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 161 - "Tab"
Cohesion: 0.15
Nodes (12): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+4 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.12
Nodes (20): fine_names(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 166 - "TouchItem"
Cohesion: 0.34
Nodes (9): content_rect(), display_label(), fader_parts(), item_target(), mapping_of(), paint_cover(), paint_item(), target_entry() (+1 more)

### Community 167 - "Profile"
Cohesion: 0.18
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 169 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (15): UNIVERSE_SIZE, SceneProgress, BankScope, All, Bank, Banks, Off, OutsideBank (+7 more)

### Community 170 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 171 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 173 - "midi.rs"
Cohesion: 0.14
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 174 - "Scene"
Cohesion: 0.21
Nodes (5): Scene, SceneKind, Fx, Static, Super

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (63): ChannelValue, Step, add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64() (+55 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 178 - "RemoteStatus"
Cohesion: 0.50
Nodes (4): RemoteStatus, Error, Off, Running

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 183 - "egui"
Cohesion: 0.15
Nodes (9): View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView, ZonesView (+1 more)

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "AudioTrigger"
Cohesion: 0.22
Nodes (6): AudioTrigger, TriggerBand, .ALL, High, Low, Mid

### Community 186 - ".page_bar"
Cohesion: 0.33
Nodes (5): Bank, FocusBank, RenameBank, arrow(), hidden_banks_menu()

### Community 187 - "super"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 189 - "ProfileBuilder"
Cohesion: 0.09
Nodes (16): AppProfiles, BuilderRequest, Edit, New, Open, Template, FileJob, Export (+8 more)

### Community 192 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 195 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 196 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 199 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 200 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.09
Nodes (28): section_header(), add_page(), add_tile(), apply_pick(), COLORS, Drag, Move, Resize (+20 more)

### Community 203 - "Purpose"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 207 - "IconButton"
Cohesion: 0.13
Nodes (4): ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 208 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 209 - "limit_pad.rs"
Cohesion: 0.22
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 210 - "channel_name"
Cohesion: 0.47
Nodes (5): channel_name(), describe(), describe_mapping(), t_deleted(), trigger_cx()

### Community 211 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 213 - "Quality"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 217 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 221 - "Knob<'a>"
Cohesion: 0.19
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "EnvState"
Cohesion: 0.50
Nodes (4): EnvState, Failed, Loading, Ready

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 251 - "AudioPlayer"
Cohesion: 0.17
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

## Knowledge Gaps
- **545 isolated node(s):** `ImportEv2`, `ImportModel`, `LoadTexture`, `LoadVenue`, `SaveVenue` (+540 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1283 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **122 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `app/src/obs.rs`, `Mapper`, `fire`, `target_shape`, `scene_props.rs`, `rig.rs`, `audio_trigger_ui.rs`, `mcp/src/venue.rs`, `app/src/audio.rs`, `t`, `scene_grid.rs`, `TouchItem`, `Profile`, `FixtureAction`, `core/src/lib.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `egui`, `.page_bar`, `super`, `ProfileBuilder`, `Host`, `ViewCtx`, `window.rs`, `stage.rs`, `Show`, `ViewCtx`, `engine.rs`, `hashmap`, `channel_name`, `mcp.rs`, `Action`, `audio_triggers.rs`, `Timeline`, `Rect`, `app.rs`, `AudioPlayer`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Are the 176 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 176 INFERRED edges - model-reasoned connections that need verification._
- **What connects `ImportEv2`, `ImportModel`, `LoadTexture` to the rest of the system?**
  _545 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.1477832512315271 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `widgets/mod.rs`, `runtime.rs`, `fx.rs`, `Engine`, `app/src/obs.rs`, `theme.rs`, `stage_canvas.rs`, `missing_audio.rs`, `EffectKind`, `fire`, `bank_menu`, `target_shape`, `scene_props.rs`, `rig.rs`, `core/src/sound.rs`, `.add_scene`, `show.rs`, `scene_grid.rs`, `TouchItem`, `FixtureAction`, `core/src/lib.rs`, `Scene`, `DmxCraftApp`, `tools.rs`, `remote/mod.rs`, `demo.rs`, `TouchLayout`, `.page_bar`, `super`, `feature.rs`, `ViewCtx`, `ShowError`, `motion.rs`, `engine.rs`, `channel_name`, `.check_placement`, `viz/mod.rs`, `Deserialize`, `Control`, `app.rs`, `Venue`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10953058321479374 - nodes in this community are weakly interconnected._