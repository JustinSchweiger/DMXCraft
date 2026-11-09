# Graph Report - dmxcraft-src  (2026-11-09)

## Corpus Check
- 143 files · ~213,275 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3985 nodes · 10035 edges · 275 communities (157 shown, 118 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 520 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e011badf`
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
- Settings
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
- pal
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
- Ui
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
- dialog.rs
- Deserialize
- fx/mapping.rs
- en.rs
- Control
- TimelineView
- app.rs
- server.rs
- ev2.rs
- Venue
- Dialog<'a>
- outline.rs
- convert.rs
- Timeline
- Timeline
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- scene_card.rs
- theme.rs
- stage_canvas.rs
- R
- missing_audio.rs
- patch_grid.rs
- Mapper
- EffectKind
- menu.rs
- Rect
- ProfileChannel
- target_shape
- icon_library.rs
- mcp/src/venue.rs
- audio_trigger_ui.rs
- ShutterEffect
- core/src/sound.rs
- Scene
- show.rs
- Tab
- color_picker.rs
- Problem
- app/src/profiles.rs
- Profile
- gltf_import.rs
- BankScope
- Drag
- Fullscreen
- midi.rs
- core/src/obs.rs
- tools.rs
- PlanShape
- RemoteLink
- ClipAction
- demo.rs
- qxf.rs
- Category
- egui
- TouchKind
- Obs
- bridge.rs
- super
- ProfileBuilder
- MenuItem<'a>
- apply
- call
- Value
- DeckState
- qlc-bundle.rs
- lock.rs
- DmxFrame
- dip.rs
- ShowError
- ViewCtx
- Purpose
- arc
- HeaderAction
- FixtureLimits
- IconButton
- ObsSettings
- Grip
- Cmd
- BuilderRequest
- OrbitCamera
- ListAction
- Knob<'a>
- fx_color.rs
- .fade_state
- FooterButton<'a>
- resize_handles
- Physics
- ObsAction
- plan
- AudioPlayer
- run
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
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

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

## Communities (275 total, 118 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.19
Nodes (9): LibraryDrag, drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch() (+1 more)

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (13): active(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.10
Nodes (34): button(), ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current() (+26 more)

### Community 6 - "AppState"
Cohesion: 0.09
Nodes (17): presets(), show(), table(), AppState, Focus, Bank, Effect, Scenes (+9 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, Transfer, Failing

### Community 10 - "Deserialize"
Cohesion: 0.10
Nodes (8): TouchImage, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "app/src/obs.rs"
Cohesion: 0.20
Nodes (8): auth_matches_obs_websocket_example(), auth_response(), b64(), open(), read_json(), send_json(), VideoPlay, Ws

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

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
Cohesion: 0.15
Nodes (25): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+17 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (84): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+76 more)

### Community 23 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.07
Nodes (52): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+44 more)

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
Cohesion: 0.14
Nodes (13): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_details(), fill_physical(), Library (+5 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.18
Nodes (21): canvas(), canvas_line(), label_row(), slider_row(), BodyCtx, combo(), curve_graph(), effect_body() (+13 more)

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
Cohesion: 0.13
Nodes (26): t(), kind_color(), capability_fields(), category_label(), check(), color_label(), FOOT_H, full_combo() (+18 more)

### Community 37 - "pal"
Cohesion: 0.17
Nodes (18): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), heading_font() (+10 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (33): MenuItem, apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete (+25 more)

### Community 41 - "drag.rs"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.19
Nodes (4): DmxCraftApp, fill_types(), moved_out(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (23): ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint(), midi() (+15 more)

### Community 50 - "super"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.12
Nodes (20): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+12 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchLayout, TouchPage

### Community 65 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 68 - "ViewCtx"
Cohesion: 0.13
Nodes (13): fold_section(), DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, vec3_row() (+5 more)

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (21): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+13 more)

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Ui"
Cohesion: 0.17
Nodes (15): card_row(), list_header(), list_row(), ModeOp, Add, Down, Remove, Up (+7 more)

### Community 72 - "Show"
Cohesion: 0.17
Nodes (9): .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+1 more)

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.20
Nodes (5): OutputConfig, OutputKind, .ALL, OpenDmx, Simulation

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
Cohesion: 0.09
Nodes (20): DipSwitch, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+12 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "views/sound.rs"
Cohesion: 0.19
Nodes (18): AUDIO_EXT, audio_files(), BAR_H, clock(), crossfade_bar(), deck_color(), DECK_H, deck_name() (+10 more)

### Community 87 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (21): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+13 more)

### Community 90 - "toolbar.rs"
Cohesion: 0.24
Nodes (8): chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width(), VizView

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.07
Nodes (41): aim_selected(), apply_placement(), AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles (+33 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "Action"
Cohesion: 0.07
Nodes (48): programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene, Flash (+40 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.19
Nodes (3): camera_json(), OrbitCamera, ray_through_center_hits_target()

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.15
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "dialog.rs"
Cohesion: 0.12
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 104 - "Deserialize"
Cohesion: 0.09
Nodes (19): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+11 more)

### Community 106 - "fx/mapping.rs"
Cohesion: 0.19
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 108 - "Control"
Cohesion: 0.07
Nodes (34): add_find_and_json(), ButtonMode, Toggle, channels_key(), Control, AudioFade, AudioMaster, AudioMute (+26 more)

### Community 110 - "app.rs"
Cohesion: 0.14
Nodes (12): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, UpdateBadge, Available, Busy (+4 more)

### Community 112 - "server.rs"
Cohesion: 0.22
Nodes (7): bridge_forwards_to_server(), header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 113 - "ev2.rs"
Cohesion: 0.11
Nodes (23): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+15 more)

### Community 117 - "Venue"
Cohesion: 0.08
Nodes (19): renumber(), aim_at_points_beam_to_target(), apply(), close(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss() (+11 more)

### Community 118 - "Dialog<'a>"
Cohesion: 0.12
Nodes (3): Dialog<'a>, DialogResult, Form

### Community 119 - "outline.rs"
Cohesion: 0.08
Nodes (28): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+20 more)

### Community 121 - "convert.rs"
Cohesion: 0.10
Nodes (29): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+21 more)

### Community 123 - "Timeline"
Cohesion: 0.13
Nodes (14): audio_grid_sets_tempo_and_offset(), ClipContent, Audio, Obs, Scene, Video, fades(), GridMode (+6 more)

### Community 128 - "Timeline"
Cohesion: 0.22
Nodes (3): Clip, Timeline, Track

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (27): add_audio(), add_audio_dialog(), add_video(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms() (+19 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "scene_card.rs"
Cohesion: 0.14
Nodes (8): Mapping, CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W, COLUMN_W

### Community 140 - "theme.rs"
Cohesion: 0.08
Nodes (40): accent_hover(), BAND_HIGH, BAND_LOW, BAND_MID, BLACK, c(), canvas_dim(), canvas_faint() (+32 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.06
Nodes (42): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+34 more)

### Community 142 - "R"
Cohesion: 0.26
Nodes (16): add_clips(), add_effect(), apply_scene_fields(), build_steps(), changed_scene(), create_scene(), create_super_scene(), existing_fixtures() (+8 more)

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "patch_grid.rs"
Cohesion: 0.12
Nodes (12): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+4 more)

### Community 146 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

### Community 147 - "EffectKind"
Cohesion: 0.14
Nodes (13): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+5 more)

### Community 149 - "menu.rs"
Cohesion: 0.18
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MIN_W (+3 more)

### Community 150 - "Rect"
Cohesion: 0.38
Nodes (5): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 151 - "ProfileChannel"
Cohesion: 0.24
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 152 - "target_shape"
Cohesion: 0.10
Nodes (42): alt_click(), bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), fader(), fader_channels(), fire(), fires_id() (+34 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.29
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "audio_trigger_ui.rs"
Cohesion: 0.11
Nodes (19): channel_name(), describe(), describe_mapping(), t_deleted(), apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction (+11 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - "core/src/sound.rs"
Cohesion: 0.18
Nodes (7): Deck, DECKS, default_board_is_not_saved_and_old_shows_load(), file_name(), Pad, PADS, Track

### Community 159 - "Scene"
Cohesion: 0.14
Nodes (4): free_scenes_get_a_bank(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 161 - "Tab"
Cohesion: 0.14
Nodes (14): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+6 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.12
Nodes (14): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+6 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "app/src/profiles.rs"
Cohesion: 0.23
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 167 - "Profile"
Cohesion: 0.11
Nodes (15): AppProfiles, import(), template(), MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format() (+7 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "BankScope"
Cohesion: 0.12
Nodes (9): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), move_scenes_between_banks() (+1 more)

### Community 170 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 171 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 173 - "midi.rs"
Cohesion: 0.14
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 174 - "core/src/obs.rs"
Cohesion: 0.48
Nodes (4): bx(), find_box(), mp4_duration_ms(), reads_mp4_length()

### Community 176 - "tools.rs"
Cohesion: 0.13
Nodes (33): add_fixtures(), any_obj(), boolean(), camera_schema(), channel_kind(), clip_schema(), collect_values(), dmx_output() (+25 more)

### Community 177 - "PlanShape"
Cohesion: 0.12
Nodes (11): FixtureType, ft(), PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix (+3 more)

### Community 178 - "RemoteLink"
Cohesion: 0.18
Nodes (7): layout_json(), RemoteLink, RemoteStatus, Error, Off, Running, Running

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_segment(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "demo.rs"
Cohesion: 0.24
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 183 - "egui"
Cohesion: 0.17
Nodes (8): View, body(), caption(), FixtureSettings, range_row(), SetupView, ZonesView, Windows

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Obs"
Cohesion: 0.29
Nodes (6): connects_and_plays_video_block(), engine(), fake_obs(), Obs, wait(), wrong_password_is_reported()

### Community 186 - "bridge.rs"
Cohesion: 0.29
Nodes (6): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 187 - "super"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 189 - "ProfileBuilder"
Cohesion: 0.12
Nodes (14): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, Tab (+6 more)

### Community 192 - "apply"
Cohesion: 0.39
Nodes (7): apply(), state_json(), channels_of(), current_color(), current_position(), set_color(), set_position()

### Community 193 - "call"
Cohesion: 0.39
Nodes (9): base64(), call(), failed_super_scene_leaves_nothing(), full_workflow(), helpful_errors(), host(), obs_action_clips(), ok() (+1 more)

### Community 195 - "Value"
Cohesion: 0.39
Nodes (5): fire(), media_action(), media_settings(), names(), seek()

### Community 196 - "DeckState"
Cohesion: 0.25
Nodes (7): DeckState, Empty, Ended, Fading, Paused, Playing, Ready

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 199 - "DmxFrame"
Cohesion: 0.16
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 200 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ViewCtx"
Cohesion: 0.08
Nodes (39): section_header(), ViewCtx, add_page(), add_tile(), apply_pick(), COLORS, content_rect(), display_label() (+31 more)

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
Cohesion: 0.12
Nodes (4): ICON_FILL_FAMILY, IconButton, paint_alert(), Toggle

### Community 209 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 210 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 211 - "BuilderRequest"
Cohesion: 0.40
Nodes (5): BuilderRequest, Edit, New, Open, Template

### Community 213 - "OrbitCamera"
Cohesion: 0.14
Nodes (13): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, Quality, .ALL (+5 more)

### Community 217 - "ListAction"
Cohesion: 0.50
Nodes (4): ListAction, Add, Duplicate, Remove

### Community 223 - "fx_color.rs"
Cohesion: 0.17
Nodes (15): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+7 more)

### Community 224 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 233 - "ObsAction"
Cohesion: 0.67
Nodes (3): ObsAction, Scene, Source

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 251 - "AudioPlayer"
Cohesion: 0.13
Nodes (8): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade(), WaveState, Failed, Loading, Ready

### Community 266 - "run"
Cohesion: 0.27
Nodes (8): run(), session(), Shared, Status, Connected, Connecting, Failed, Off

## Knowledge Gaps
- **545 isolated node(s):** `Loading`, `Empty`, `Ready`, `Playing`, `Paused` (+540 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1280 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **118 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `SoundPlayer`, `runtime.rs`, `fx.rs`, `scene_card.rs`, `Engine`, `app/src/obs.rs`, `theme.rs`, `stage_canvas.rs`, `missing_audio.rs`, `R`, `target_shape`, `scene_props.rs`, `rig.rs`, `audio_trigger_ui.rs`, `Scene`, `show.rs`, `FixtureAction`, `BankScope`, `DmxCraftApp`, `tools.rs`, `PlanShape`, `demo.rs`, `TouchLayout`, `super`, `apply`, `feature.rs`, `ViewCtx`, `ShowError`, `motion.rs`, `engine.rs`, `ObsSettings`, `viz/mod.rs`, `Action`, `Deserialize`, `Control`, `app.rs`, `Venue`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 176 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 176 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Empty`, `Ready` to the rest of the system?**
  _545 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `SoundPlayer` be split into smaller, more focused modules?**
  _Cohesion score 0.1431451612903226 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Timeline`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `AppState`, `control/timeline.rs`, `scene_card.rs`, `app/src/obs.rs`, `R`, `Mapper`, `target_shape`, `scene_props.rs`, `rig.rs`, `audio_trigger_ui.rs`, `mcp/src/venue.rs`, `app/src/audio.rs`, `t`, `Profile`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `RemoteLink`, `remote/mod.rs`, `egui`, `super`, `ProfileBuilder`, `apply`, `Host`, `call`, `Value`, `window.rs`, `stage.rs`, `DmxFrame`, `Show`, `ViewCtx`, `engine.rs`, `hashmap`, `mcp.rs`, `Action`, `.fade_state`, `audio_triggers.rs`, `Deserialize`, `TimelineView`, `app.rs`, `AudioPlayer`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Are the 5 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 5 INFERRED edges - model-reasoned connections that need verification._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11330049261083744 - nodes in this community are weakly interconnected._