# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 117 files · ~149,448 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2988 nodes · 7222 edges · 238 communities (125 shown, 113 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 365 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a01a6bc9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- core/src/venue.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- FixtureType
- Engine
- i18n/mod.rs
- viz/library.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- .add_scene
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- audio.rs
- env
- dmxcraft
- Venue
- call
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- textures.rs
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- VizView
- ClipAction
- core/src/lib.rs
- .row
- ViewCtx
- engine.rs
- stage.rs
- Profile
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- fx_ui.rs
- Vec2
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- t
- Toggle<'a>
- t
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- Dialog<'a>
- MapPreset
- app.rs
- qxf.rs
- en.rs
- profiles.rs
- Rect
- fixture_settings.rs
- ev2.rs
- knob.rs
- mcp.rs
- EffectKind
- convert.rs
- model.rs
- Snapshot
- CurveShape
- ColorPreset
- MenuAction
- stage_editor.rs
- ProfileBuilder
- stage_canvas.rs
- mapping.rs
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- server.rs
- Timeline
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- .modes_tab
- ZoneId
- protocol.rs
- SlotKind
- Problem
- DvcGoldUsb
- Vec2
- Shape
- lock.rs
- tools.rs
- file_dialog.rs
- dip.rs
- PlanShape
- demo.rs
- .default
- Theme
- Category
- swatch.rs
- qlc-bundle.rs
- Tab
- Transform
- ShowError
- Capability
- arc
- Settings
- Drag
- panels.rs
- FixtureLimits
- plan
- .check_placement
- toggle.rs
- Grip
- paint_symbol
- .fade_state
- Response
- Timeline
- ChannelValue
- Box<dyn serialport::SerialPort>
- FooterButton<'a>
- Mode
- T

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 157 edges
3. `Show` - 121 edges
4. `Icon` - 104 edges
5. `ChannelKind` - 71 edges
6. `ViewCtx` - 61 edges
7. `AppState` - 55 edges
8. `FixtureType` - 45 edges
9. `DmxFrame` - 43 edges
10. `Profile` - 42 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `DmxCraftApp` --references--> `McpLink`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/mcp.rs

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

## Communities (238 total, 113 thin omitted)

### Community 0 - "core/src/venue.rs"
Cohesion: 0.11
Nodes (15): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, hang_below_nearest_truss(), IDENTITY, Material (+7 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (15): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (42): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+34 more)

### Community 5 - "programmer.rs"
Cohesion: 0.11
Nodes (30): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+22 more)

### Community 6 - "AppState"
Cohesion: 0.13
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, fbm(), feature_levels() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (10): DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented (+2 more)

### Community 10 - "FixtureType"
Cohesion: 0.10
Nodes (14): ChannelDef, ChannelRange, FixtureType, Physics, RangeEffect, Closed, Color, Gobo (+6 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.08
Nodes (22): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+14 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.16
Nodes (22): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+14 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - ".add_scene"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (50): Step, Enter, Maximize, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+42 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.16
Nodes (5): fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

### Community 36 - "call"
Cohesion: 0.25
Nodes (25): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), channel_kind(), collect_values(), create_scene() (+17 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.09
Nodes (26): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+18 more)

### Community 44 - "fader.rs"
Cohesion: 0.13
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fader_ranges()

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (26): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+18 more)

### Community 50 - "VizView"
Cohesion: 0.18
Nodes (14): aim_selected(), apply_placement(), decode_texture(), GizmoDrag, move_all(), Pick, Fixture, Object (+6 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (11): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 66 - ".row"
Cohesion: 0.09
Nodes (27): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+19 more)

### Community 68 - "ViewCtx"
Cohesion: 0.21
Nodes (5): ViewCtx, renumber(), vec3_row(), VizView, VizView

### Community 69 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Profile"
Cohesion: 0.18
Nodes (4): fine_names(), Profile, ProfileChannel, rename_and_resolution_update_modes()

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.11
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

### Community 79 - "Vec2"
Cohesion: 0.07
Nodes (22): DipSwitch, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+14 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.11
Nodes (18): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX (+10 more)

### Community 90 - "t"
Cohesion: 0.16
Nodes (7): View, body(), caption(), FixtureSettings, range_row(), ZonesView, Windows

### Community 92 - "t"
Cohesion: 0.13
Nodes (30): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+22 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.13
Nodes (21): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), Handles, handles(), hit_y(), Missing, model_tex_id() (+13 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.06
Nodes (53): ControlViews, Fullscreen, SETTLE, select_all(), programmer_values(), ProgrammerView, Action, Bank (+45 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "app.rs"
Cohesion: 0.18
Nodes (10): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, UpdateBadge, Available, Busy, Error (+2 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 108 - "profiles.rs"
Cohesion: 0.23
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 109 - "Rect"
Cohesion: 0.32
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "fixture_settings.rs"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "mcp.rs"
Cohesion: 0.21
Nodes (5): McpLink, McpStatus, Error, Off, Running

### Community 119 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "model.rs"
Cohesion: 0.11
Nodes (9): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Model, ModelPart, part_roundtrip_through_json() (+1 more)

### Community 128 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 137 - "stage_editor.rs"
Cohesion: 0.12
Nodes (13): CanvasMode, Edit, Select, SetupView, Arrange, AlignH, AlignV, Grid (+5 more)

### Community 140 - "ProfileBuilder"
Cohesion: 0.17
Nodes (8): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (20): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+12 more)

### Community 143 - "mapping.rs"
Cohesion: 0.20
Nodes (10): every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level(), transform() (+2 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.12
Nodes (18): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, grid_options(), HEADER_W, HeaderAction, Delete, Duplicate (+10 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.12
Nodes (13): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), header() (+5 more)

### Community 152 - "Timeline"
Cohesion: 0.22
Nodes (4): add_audio(), add_audio_dialog(), Clip, Timeline

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - ".modes_tab"
Cohesion: 0.19
Nodes (12): list_header(), ListAction, Add, Duplicate, Remove, ModeOp, Add, Down (+4 more)

### Community 160 - "ZoneId"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 161 - "protocol.rs"
Cohesion: 0.17
Nodes (17): AppEvent, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host, call_tool() (+9 more)

### Community 162 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 172 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.11
Nodes (31): any_obj(), apply_scene_fields(), boolean(), clip_schema(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+23 more)

### Community 177 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 178 - "dip.rs"
Cohesion: 0.29
Nodes (7): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), addresses_are_one_based()

### Community 179 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - ".default"
Cohesion: 0.36
Nodes (4): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ranges(), ranges_always_cover_0_to_255()

### Community 183 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 195 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 199 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.16
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 210 - "panels.rs"
Cohesion: 0.08
Nodes (18): SceneCard, fold_section(), section_header(), Segmented, DialogKind, ImportEv2, ImportModel, LoadTexture (+10 more)

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 218 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 220 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 222 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 223 - "Response"
Cohesion: 0.33
Nodes (4): clip(), SceneCardResponse, list_row(), wide_button()

### Community 224 - "Timeline"
Cohesion: 0.16
Nodes (12): ClipContent, Audio, Scene, fades(), GridMode, Beat, Time, one() (+4 more)

### Community 230 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

## Knowledge Gaps
- **449 isolated node(s):** `None`, `Release`, `ToggleFx`, `Select`, `Edit` (+444 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1047 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **113 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Snapshot`, `runtime.rs`, `theme.rs`, `fx.rs`, `FixtureType`, `Engine`, `stage_canvas.rs`, `.add_scene`, `rig.rs`, `ZoneId`, `Venue`, `call`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `VizView`, `demo.rs`, `core/src/lib.rs`, `engine.rs`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `.check_placement`, `viz/mod.rs`, `scene_grid.rs`, `app.rs`, `EffectKind`?**
  _High betweenness centrality (0.122) - this node is a cross-community bridge._
- **Are the 128 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 128 INFERRED edges - model-reasoned connections that need verification._
- **What connects `None`, `Release`, `ToggleFx` to the rest of the system?**
  _449 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `core/src/venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `ProfileBuilder`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `audio.rs`, `protocol.rs`, `call`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `core/src/lib.rs`, `ViewCtx`, `engine.rs`, `stage.rs`, `Show`, `t`, `t`, `.fade_state`, `scene_grid.rs`, `app.rs`, `Rect`, `fixture_settings.rs`, `mcp.rs`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14039408866995073 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `theme.rs`, `programmer.rs`, `stage_editor.rs`, `i18n/mod.rs`, `ProfileBuilder`, `viz/library.rs`, `control/timeline.rs`, `titlebar.rs`, `Timeline`, `scene_props.rs`, `.modes_tab`, `color_picker.rs`, `FixtureAction`, `textures.rs`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `file_dialog.rs`, `VizView`, `.row`, `ViewCtx`, `stage.rs`, `Settings`, `fx_ui.rs`, `Vec2`, `patch_grid.rs`, `panels.rs`, `dialog.rs`, `t`, `viz/mod.rs`, `scene_grid.rs`, `Dialog<'a>`, `app.rs`, `profiles.rs`, `Rect`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._