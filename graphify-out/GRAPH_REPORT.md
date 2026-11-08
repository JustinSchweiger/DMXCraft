# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 116 files · ~144,257 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2912 nodes · 7043 edges · 236 communities (125 shown, 111 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 360 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `346105b3`
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
- Action
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
- super
- env
- dmxcraft
- Venue
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- MenuAction
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- VizView
- ClipAction
- PlanShape
- .row
- ViewCtx
- engine.rs
- stage.rs
- gltf_import.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- fx_ui.rs
- scene_grid.rs
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- t
- Toggle<'a>
- profile_builder.rs
- viz/mod.rs
- ChannelKind
- panels.rs
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
- Capability
- CurveShape
- ColorPreset
- FixtureType
- stage_editor.rs
- Scene
- stage_canvas.rs
- bank_menu
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- server.rs
- bridge.rs
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- widgets/mod.rs
- core/src/lib.rs
- protocol.rs
- fixture_menu.rs
- Problem
- track_header
- OrbitCamera
- Shape
- .show
- tools.rs
- file_dialog.rs
- DmxLine
- send_frame
- demo.rs
- palettes.rs
- Category
- swatch.rs
- qlc-bundle.rs
- Tab
- ShowError
- Lang
- arc
- Settings
- Drag
- UpdateBadge
- FixtureLimits
- plan
- Tab
- DmxFrame
- paint_symbol
- GridMode
- scene_brief
- Theme
- Timeline
- ChannelValue
- ok
- FooterButton<'a>
- T

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 148 edges
3. `Show` - 120 edges
4. `Icon` - 104 edges
5. `ChannelKind` - 69 edges
6. `ViewCtx` - 61 edges
7. `AppState` - 55 edges
8. `FixtureType` - 43 edges
9. `DmxFrame` - 43 edges
10. `Profile` - 42 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `fader()` --calls--> `kind_color()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/fader.rs
- `paint_range_block()` --calls--> `kind_color()`  [INFERRED]
  crates/app/src/views/profile_builder.rs → crates/app/src/ui/widgets/fader.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/control/programmer.rs -> crates/app/src/views/control/programmer.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (236 total, 111 thin omitted)

### Community 0 - "core/src/venue.rs"
Cohesion: 0.12
Nodes (13): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, hang_below_nearest_truss(), IDENTITY, Material (+5 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.13
Nodes (15): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (43): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+35 more)

### Community 5 - "programmer.rs"
Cohesion: 0.13
Nodes (27): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+19 more)

### Community 6 - "AppState"
Cohesion: 0.14
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (25): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, fbm(), grayscale_has_equal_channels(), hard_edges_use_palette_colors_only() (+17 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig (+4 more)

### Community 10 - "Action"
Cohesion: 0.12
Nodes (27): programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash, FocusBank (+19 more)

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (10): Layer, Live, Programmer, Engine, FadeState, In, Out, lerp() (+2 more)

### Community 12 - "i18n/mod.rs"
Cohesion: 0.16
Nodes (8): all_texts_translated(), EN, fill(), fill_replaces_names(), LANG, tr!, texts_in_source(), ACTIVE

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
Cohesion: 0.18
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Show, steps_insert_remove_and_normalize(), Zone

### Community 24 - "rig.rs"
Cohesion: 0.13
Nodes (22): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), Kind, kind() (+14 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (49): Step, Enter, Maximize, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+41 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.17
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "super"
Cohesion: 0.07
Nodes (26): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+18 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.16
Nodes (6): fixture_folders_are_exclusive(), FixtureFolder, mul(), Object, room_and_tree(), Venue

### Community 36 - "Timeline"
Cohesion: 0.23
Nodes (8): ClipContent, Audio, Scene, fades(), one(), paste_finds_free_place_and_keeps_gaps(), split_and_length(), super_scene_from_scenes_keeps_fades()

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
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.29
Nodes (4): SETTLE, drag_number(), endless_delta(), raw_available()

### Community 43 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 44 - "fader.rs"
Cohesion: 0.16
Nodes (6): Fader, FaderEvent, Changed, None, Release, ToggleFx

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (18): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+10 more)

### Community 50 - "VizView"
Cohesion: 0.09
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadVenue, SaveVenue, hit_y(), Missing, Mode (+6 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - ".row"
Cohesion: 0.10
Nodes (23): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+15 more)

### Community 68 - "ViewCtx"
Cohesion: 0.20
Nodes (7): fold_section(), ViewCtx, renumber(), vec3_row(), Folder, VizView, VizView

### Community 69 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 70 - "stage.rs"
Cohesion: 0.20
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

### Community 71 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart, smooth_normals() (+1 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Placement

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (25): apply_effect(), apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.13
Nodes (16): Dialog, section_header(), View, SetupView, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS (+8 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.12
Nodes (24): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+16 more)

### Community 79 - "scene_grid.rs"
Cohesion: 0.09
Nodes (20): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, ICON_FAMILY (+12 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (12): Button, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "t"
Cohesion: 0.14
Nodes (24): t(), capability_fields(), category_label(), check(), color_label(), FOOT_H, general(), hex_field() (+16 more)

### Community 92 - "profile_builder.rs"
Cohesion: 0.12
Nodes (22): kind_color(), SceneCardResponse, card_row(), full_combo(), list_header(), list_row(), ListAction, Add (+14 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.16
Nodes (20): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles(), project(), proposals_for_easy_view_names() (+12 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "panels.rs"
Cohesion: 0.16
Nodes (11): aim_selected(), apply_placement(), move_all(), Pick, Fixture, Object, sel_frame(), targets() (+3 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.06
Nodes (38): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+30 more)

### Community 100 - "app.rs"
Cohesion: 0.17
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Fullscreen, ProgrammerView, SceneGridView

### Community 104 - "qxf.rs"
Cohesion: 0.23
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "profiles.rs"
Cohesion: 0.17
Nodes (11): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+3 more)

### Community 109 - "Rect"
Cohesion: 0.30
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+1 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "mcp.rs"
Cohesion: 0.13
Nodes (12): McpLink, McpStatus, Error, Off, Running, AppEvent, Saved, SelectScene (+4 more)

### Community 119 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 123 - "model.rs"
Cohesion: 0.10
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 128 - "Capability"
Cohesion: 0.13
Nodes (8): add_range_until_every_value_is_used(), cap_for_kind(), Capability, last_matching(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "FixtureType"
Cohesion: 0.21
Nodes (3): ChannelDef, FixtureType, ft()

### Community 137 - "stage_editor.rs"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 140 - "Scene"
Cohesion: 0.21
Nodes (5): Scene, SceneKind, Fx, Static, Super

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (20): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+12 more)

### Community 143 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.14
Nodes (14): drag_ghost(), add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H (+6 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.24
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 152 - "bridge.rs"
Cohesion: 0.22
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.39
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.20
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 158 - "widgets/mod.rs"
Cohesion: 0.19
Nodes (7): from_pct(), pct(), RangeSlider, value_field(), VALUE_H, Toggle, XyPad

### Community 160 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (14): SceneProgress, BankScope, All, Bank, Banks, Off, OutsideBank, Group (+6 more)

### Community 161 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 162 - "fixture_menu.rs"
Cohesion: 0.18
Nodes (6): apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "track_header"
Cohesion: 0.24
Nodes (9): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, row_h(), track_h() (+1 more)

### Community 172 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 174 - ".show"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 176 - "tools.rs"
Cohesion: 0.13
Nodes (50): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+42 more)

### Community 177 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 179 - "send_frame"
Cohesion: 0.39
Nodes (5): frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 195 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.27
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.17
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 210 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 218 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 219 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 221 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 223 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 224 - "Timeline"
Cohesion: 0.21
Nodes (4): grid_options(), Clip, Timeline, Track

### Community 226 - "ok"
Cohesion: 0.67
Nodes (4): full_workflow(), helpful_errors(), host(), ok()

### Community 233 - "T"
Cohesion: 0.15
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

## Knowledge Gaps
- **439 isolated node(s):** `None`, `Release`, `ToggleFx`, `Select`, `Edit` (+434 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1021 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **111 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `Action`, `control/timeline.rs`, `.add_scene`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `super`, `core/src/lib.rs`, `fixture_menu.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `ViewCtx`, `engine.rs`, `stage.rs`, `scene_grid.rs`, `t`, `DmxFrame`, `scene_brief`, `app.rs`, `Rect`, `fixture_settings.rs`, `mcp.rs`?**
  _High betweenness centrality (0.125) - this node is a cross-community bridge._
- **Are the 125 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 125 INFERRED edges - model-reasoned connections that need verification._
- **What connects `None`, `Release`, `ToggleFx` to the rest of the system?**
  _439 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `core/src/venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1225071225071225 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `.add_scene` to `runtime.rs`, `theme.rs`, `fx.rs`, `FixtureType`, `Action`, `Engine`, `Scene`, `stage_canvas.rs`, `bank_menu`, `control/timeline.rs`, `rig.rs`, `super`, `core/src/lib.rs`, `fixture_menu.rs`, `Venue`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `engine.rs`, `Show`, `feature.rs`, `ShowError`, `motion.rs`, `scene_grid.rs`, `show.rs`, `viz/mod.rs`, `panels.rs`, `app.rs`, `EffectKind`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `theme.rs`, `programmer.rs`, `stage_editor.rs`, `Action`, `i18n/mod.rs`, `viz/library.rs`, `bank_menu`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `.viewport`, `fixture_menu.rs`, `color_picker.rs`, `FixtureAction`, `track_header`, `fader.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `file_dialog.rs`, `VizView`, `palettes.rs`, `.row`, `ViewCtx`, `stage.rs`, `Lang`, `Settings`, `fx_ui.rs`, `scene_grid.rs`, `patch_grid.rs`, `dialog.rs`, `profile_builder.rs`, `Timeline`, `Dialog<'a>`, `app.rs`, `profiles.rs`, `Rect`, `fixture_settings.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._