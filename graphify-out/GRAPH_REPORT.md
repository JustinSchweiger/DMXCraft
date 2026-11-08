# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 131 files · ~187,225 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3576 nodes · 9030 edges · 249 communities (152 shown, 97 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 467 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7b88b93d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- views/touch.rs
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
- Venue
- viz/library.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Ui
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
- Venue
- t
- mapping.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- textures.rs
- Fader
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- MenuAction
- remote/mod.rs
- TouchLayout
- BankScope
- PlanShape
- .object_props
- window.rs
- stage.rs
- Host
- Show
- feature.rs
- OutputKind
- motion.rs
- View
- engine.rs
- Vec2
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- app.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- ProfileBuilder
- MapPreset
- scene_card.rs
- DeviceModel
- qxf.rs
- file_dialog.rs
- en.rs
- VizView
- Rect
- super
- server.rs
- ev2.rs
- call
- EffectKind
- outline.rs
- convert.rs
- Snapshot
- gltf_import.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- ChannelDef
- theme.rs
- stage_canvas.rs
- RemoteLink
- core/src/lib.rs
- flush
- paintItem
- panels.rs
- MenuItem<'a>
- plan.rs
- DMXCraft
- .fade_state
- mcp/src/venue.rs
- .show
- ShutterEffect
- .viewport
- FixtureLimits
- protocol.rs
- HeaderAction
- Swatch
- Problem
- DvcGoldUsb
- plan
- Scene
- hashmap
- Category
- Vec2
- fixture_menu.rs
- src/mapping.rs
- TouchKind
- tools.rs
- dip.rs
- ClipAction
- demo.rs
- Settings
- Dialog<'a>
- Tab
- Grip
- midi.rs
- Drag
- Touch-Oberfläche
- ChannelValue
- apply
- mapping/mod.rs
- .show
- qlc-bundle.rs
- Fullscreen
- SlotKind
- ShowError
- ChaserPreset
- arc
- icon_library.rs
- Tab
- T
- draw
- StageCanvas
- Mapper
- apply
- Arrange
- UpdateBadge
- .check_placement
- Timeline
- FooterButton<'a>
- Cmd
- textOn
- deg_quat
- GridMode
- Tastatur- und MIDI-Zuordnung

## God Nodes (most connected - your core abstractions)
1. `Engine` - 201 edges
2. `t()` - 186 edges
3. `Show` - 140 edges
4. `Icon` - 107 edges
5. `AppState` - 88 edges
6. `ViewCtx` - 72 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 56 edges
9. `FixtureType` - 46 edges
10. `DmxCraftApp` - 43 edges

## Surprising Connections (you probably didn't know these)
- `Aufbau` --references--> `send()`  [INFERRED]
  README.md → crates/app/src/remote/web/app.js
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (249 total, 97 thin omitted)

### Community 0 - "views/touch.rs"
Cohesion: 0.08
Nodes (39): rgb(), text_field(), section_header(), ViewCtx, add_page(), apply_pick(), COLORS, content_rect() (+31 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (20): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+12 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Theme"
Cohesion: 0.12
Nodes (13): main(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.12
Nodes (33): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+25 more)

### Community 6 - "AppState"
Cohesion: 0.09
Nodes (16): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+8 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.10
Nodes (9): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer (+1 more)

### Community 10 - "Deserialize"
Cohesion: 0.10
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.12
Nodes (10): renumber(), Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree() (+2 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (18): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+10 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.14
Nodes (11): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+3 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.15
Nodes (23): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+15 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (78): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+70 more)

### Community 23 - "Ui"
Cohesion: 0.19
Nodes (13): ListAction, Add, Duplicate, Remove, ModeOp, Add, Down, Remove (+5 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.05
Nodes (50): Knob, Knob<'a>, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split() (+42 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.05
Nodes (41): analysis(), analyze_file(), AudioPlayer, cache_file(), wav(), waveform_and_duration(), WaveState, Failed (+33 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.11
Nodes (33): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+25 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.14
Nodes (11): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, mul(), Placement, rotation() (+3 more)

### Community 36 - "t"
Cohesion: 0.11
Nodes (29): kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H, full_combo() (+21 more)

### Community 37 - "mapping.rs"
Cohesion: 0.15
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.18
Nodes (17): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+9 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.17
Nodes (3): DmxCraftApp, fill_types(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.17
Nodes (25): t(), Settings, ai(), customize(), full_combo(), general(), HEIGHT, hint() (+17 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.13
Nodes (20): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+12 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 66 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 68 - ".object_props"
Cohesion: 0.29
Nodes (6): fold_section(), TexAction, Builtin, Load, Set, VizView

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Host"
Cohesion: 0.13
Nodes (15): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+7 more)

### Community 72 - "Show"
Cohesion: 0.18
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "View"
Cohesion: 0.12
Nodes (11): View, body(), caption(), FixtureSettings, range_row(), SetupView, set_pos(), StageEditorView (+3 more)

### Community 78 - "engine.rs"
Cohesion: 0.22
Nodes (32): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+24 more)

### Community 79 - "Vec2"
Cohesion: 0.06
Nodes (34): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+26 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.07
Nodes (14): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+6 more)

### Community 89 - "profile.rs"
Cohesion: 0.06
Nodes (28): import(), add_range_until_every_value_is_used(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys() (+20 more)

### Community 90 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 92 - "app.js"
Cohesion: 0.12
Nodes (21): buildTabs(), clamp(), contentRect(), cv, de, fieldMove(), g, hsToRgb() (+13 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.17
Nodes (17): AXIS_COLORS, Handles, handles(), hit_y(), model_tex_id(), project(), proposals_for_easy_view_names(), propose_type() (+9 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.06
Nodes (61): zone_color(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H (+53 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 101 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "VizView"
Cohesion: 0.10
Nodes (12): decode_texture(), DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing (+4 more)

### Community 109 - "Rect"
Cohesion: 0.17
Nodes (8): fmt_ms(), grid_options(), has_audio(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), TimelineView

### Community 110 - "super"
Cohesion: 0.14
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), verify(), addresses_are_one_based(), DmxFrame

### Community 112 - "server.rs"
Cohesion: 0.14
Nodes (12): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+4 more)

### Community 113 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "call"
Cohesion: 0.30
Nodes (21): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), create_scene(), create_super_scene(), dmx_output() (+13 more)

### Community 118 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.08
Nodes (28): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+20 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 128 - "gltf_import.rs"
Cohesion: 0.18
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.13
Nodes (15): duration_ms(), add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H (+7 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "ChannelDef"
Cohesion: 0.13
Nodes (12): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.08
Nodes (62): sidebar(), accent(), accent_dim(), accent_hover(), active(), apply(), BAND_HIGH, BAND_LOW (+54 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.16
Nodes (16): AMBER, BOX, fit(), fixture_pos(), live_colors(), MenuFn, paint_effects(), paint_fixture() (+8 more)

### Community 142 - "RemoteLink"
Cohesion: 0.18
Nodes (7): layout_json(), RemoteLink, RemoteStatus, Error, Off, Running, Running

### Community 143 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (13): UNIVERSE_SIZE, SceneProgress, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind (+5 more)

### Community 144 - "flush"
Cohesion: 0.32
Nodes (12): api(), askPin(), flush(), loadLayout(), message(), poll(), pressDown(), release() (+4 more)

### Community 146 - "paintItem"
Cohesion: 0.24
Nodes (12): colorOf(), cover(), faderParts(), font(), paintItem(), panel(), rgbToHs(), rr() (+4 more)

### Community 147 - "panels.rs"
Cohesion: 0.19
Nodes (15): aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object, rotate_all() (+7 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 151 - "DMXCraft"
Cohesion: 0.17
Nodes (12): send(), 3D-Visualisierer, Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, KI-Assistent (MCP) (+4 more)

### Community 152 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - ".show"
Cohesion: 0.38
Nodes (3): Camera, fixture_rect(), StageInteraction

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 160 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 161 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 162 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 168 - "Scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 169 - "hashmap"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 170 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 172 - "fixture_menu.rs"
Cohesion: 0.18
Nodes (6): apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 173 - "src/mapping.rs"
Cohesion: 0.06
Nodes (38): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, BankStep, Blackout (+30 more)

### Community 174 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (31): any_obj(), apply_scene_fields(), boolean(), clip_schema(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+23 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_grid(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "Settings"
Cohesion: 0.20
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 183 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 184 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 185 - "midi.rs"
Cohesion: 0.24
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "Touch-Oberfläche"
Cohesion: 0.40
Nodes (4): Bearbeiten, Bedienen, Elemente hinzufügen, Touch-Oberfläche

### Community 189 - "ChannelValue"
Cohesion: 0.29
Nodes (6): ChannelValue, Step, channel_kind(), collect_values(), existing_fixtures(), resolve_channel()

### Community 190 - "apply"
Cohesion: 0.39
Nodes (7): apply(), state_json(), channels_of(), current_color(), current_position(), set_color(), set_position()

### Community 192 - "mapping/mod.rs"
Cohesion: 0.09
Nodes (46): alt_click(), bank_step_wraps(), button(), button_rect(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping() (+38 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 199 - "SlotKind"
Cohesion: 0.25
Nodes (8): slot_label(), SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 204 - "arc"
Cohesion: 0.29
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 207 - "Tab"
Cohesion: 0.18
Nodes (10): Tab, Ai, .ALL, Customize, General, Midi, Outputs, Project (+2 more)

### Community 209 - "draw"
Cohesion: 0.40
Nodes (6): draw(), hit(), image(), isHot(), page(), rectOf()

### Community 210 - "StageCanvas"
Cohesion: 0.33
Nodes (5): CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 216 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

### Community 217 - "apply"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 219 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 221 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 223 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (12): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Scene, fades(), one(), paste_finds_free_place_and_keeps_gaps() (+4 more)

### Community 230 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 231 - "textOn"
Cohesion: 0.50
Nodes (4): applyTheme(), dim(), hexRgb(), textOn()

### Community 233 - "deg_quat"
Cohesion: 0.67
Nodes (3): deg_quat(), euler_matches_core_rotation(), quat_deg()

### Community 237 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **510 isolated node(s):** `Loading`, `Off`, `Running`, `Error`, `Press` (+505 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1180 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **97 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `views/touch.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `RemoteLink`, `core/src/lib.rs`, `rig.rs`, `scene_props.rs`, `.fade_state`, `core/src/audio.rs`, `mcp/src/venue.rs`, `t`, `hashmap`, `fixture_menu.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `apply`, `mapping/mod.rs`, `window.rs`, `stage.rs`, `Host`, `Show`, `View`, `engine.rs`, `Mapper`, `app.rs`, `scene_grid.rs`, `ProfileBuilder`, `Rect`, `call`, `Snapshot`?**
  _High betweenness centrality (0.119) - this node is a cross-community bridge._
- **Are the 159 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 159 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Off`, `Running` to the rest of the system?**
  _510 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `views/touch.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07869742198100407 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `views/touch.rs`, `library_panel.rs`, `runtime.rs`, `fx.rs`, `Engine`, `theme.rs`, `stage_canvas.rs`, `Venue`, `core/src/lib.rs`, `panels.rs`, `rig.rs`, `.show`, `FixtureAction`, `Scene`, `fixture_menu.rs`, `src/mapping.rs`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `TouchLayout`, `ChannelValue`, `apply`, `mapping/mod.rs`, `BankScope`, `window.rs`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `StageCanvas`, `show.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `.check_placement`, `call`, `EffectKind`, `Snapshot`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10853658536585366 - nodes in this community are weakly interconnected._