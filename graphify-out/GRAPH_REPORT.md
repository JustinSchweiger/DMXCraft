# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 115 files · ~139,691 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2861 nodes · 6912 edges · 237 communities (125 shown, 112 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 339 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b31facec`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Venue
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
- pal
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
- mapping.rs
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- Profile
- FixtureAction
- drag_number
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
- Transform
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
- View
- Snapshot
- knob.rs
- mcp.rs
- EffectKind
- convert.rs
- model.rs
- Capability
- CurveShape
- ColorPreset
- outline.rs
- stage_editor.rs
- Step
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
- Vec2
- core/src/lib.rs
- ZoneId
- SlotKind
- Problem
- .check_placement
- OrbitCamera
- ProfileChannel
- .show
- tools.rs
- file_dialog.rs
- Category
- Settings
- qlc-bundle.rs
- lock.rs
- ShowError
- Lang
- arc
- Settings
- Drag
- dip.rs
- FixtureLimits
- plan
- Tab
- DmxFrame
- .fade_state
- ChaserPreset
- text
- Theme
- Timeline
- ChannelValue
- ok
- FooterButton<'a>
- venue_object_schema
- hashmap
- T

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 147 edges
3. `Show` - 118 edges
4. `Icon` - 104 edges
5. `ViewCtx` - 60 edges
6. `ChannelKind` - 60 edges
7. `AppState` - 55 edges
8. `FixtureType` - 43 edges
9. `Profile` - 42 edges
10. `DmxFrame` - 42 edges

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

## Communities (237 total, 112 thin omitted)

### Community 0 - "Venue"
Cohesion: 0.05
Nodes (42): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, argb(), DEFAULT_TRUSS_LEN, Ev2Fixture (+34 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (13): LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH (+5 more)

### Community 2 - "runtime.rs"
Cohesion: 0.10
Nodes (13): AudioDuration, DEFAULT_PORT, engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running (+5 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (29): accent_dim(), accent_hover(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+21 more)

### Community 5 - "programmer.rs"
Cohesion: 0.10
Nodes (31): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+23 more)

### Community 6 - "AppState"
Cohesion: 0.11
Nodes (15): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (22): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, grayscale_has_equal_channels(), hard_edges_use_palette_colors_only(), knight_rider() (+14 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig (+4 more)

### Community 10 - "Action"
Cohesion: 0.13
Nodes (26): Action, Bank, CreateSuper, Delete, DuplicateScene, Flash, FocusBank, HideScenes (+18 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.17
Nodes (8): all_texts_translated(), EN, fill(), fill_replaces_names(), LANG, tr!, texts_in_source(), ACTIVE

### Community 13 - "pal"
Cohesion: 0.21
Nodes (8): accent(), bg(), border(), caption(), raised(), raised_hover(), text_dim(), text_faint()

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
Cohesion: 0.06
Nodes (48): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+40 more)

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
Cohesion: 0.13
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.07
Nodes (31): linear_to_srgb(), load(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart, smooth_normals() (+23 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.07
Nodes (46): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+38 more)

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
Cohesion: 0.12
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "audio.rs"
Cohesion: 0.07
Nodes (21): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+13 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "mapping.rs"
Cohesion: 0.18
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 36 - "Timeline"
Cohesion: 0.18
Nodes (9): ClipContent, Audio, Scene, GridMode, Beat, Time, one(), super_scene_from_scenes_keeps_fades() (+1 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.14
Nodes (14): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+6 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "Profile"
Cohesion: 0.16
Nodes (5): fine_names(), import_real_ofl_with_16bit_ranges(), ofl_round_trip(), Profile, rename_and_resolution_update_modes()

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

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
Cohesion: 0.08
Nodes (20): DialogKind, ImportEv2, ImportModel, LoadVenue, SaveVenue, Missing, Mode, Build (+12 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - ".row"
Cohesion: 0.16
Nodes (14): drag_row(), DragObjects, Tree, TreeAction, Delete, Drop, Duplicate, GroupSelection (+6 more)

### Community 68 - "ViewCtx"
Cohesion: 0.20
Nodes (7): fold_section(), ViewCtx, renumber(), drop_marker(), Folder, VizView, VizView

### Community 69 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.18
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), Fixture, paste_fixtures_finds_free_addresses_and_names(), Show

### Community 73 - "feature.rs"
Cohesion: 0.15
Nodes (19): apply_effect(), apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx (+11 more)

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

### Community 79 - "scene_grid.rs"
Cohesion: 0.19
Nodes (9): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W, arrow(), chip(), COLUMN_W (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 89 - "profile.rs"
Cohesion: 0.11
Nodes (16): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), INTENSITY_COLORS, KEY_PREFIX, OFL_SCHEMA (+8 more)

### Community 90 - "t"
Cohesion: 0.13
Nodes (28): t(), text_field(), capability_card(), capability_fields(), category_label(), check(), color_label(), FOOT_H (+20 more)

### Community 92 - "profile_builder.rs"
Cohesion: 0.09
Nodes (26): CardAction, End, None, Remove, list_header(), list_row(), ListAction, Add (+18 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.17
Nodes (17): AXIS_COLORS, GizmoDrag, Handles, handles(), hit_y(), move_all(), project(), QualityParams (+9 more)

### Community 94 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 95 - "panels.rs"
Cohesion: 0.25
Nodes (8): aim_selected(), apply_placement(), Pick, Fixture, Object, sel_frame(), targets(), Placement

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.14
Nodes (3): Dialog<'a>, DialogResult, Form

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "app.rs"
Cohesion: 0.18
Nodes (10): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, programmer_values(), ProgrammerView, apply() (+2 more)

### Community 104 - "qxf.rs"
Cohesion: 0.23
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "profiles.rs"
Cohesion: 0.26
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 109 - "Rect"
Cohesion: 0.29
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "View"
Cohesion: 0.17
Nodes (10): section_header(), View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView (+2 more)

### Community 113 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 118 - "mcp.rs"
Cohesion: 0.15
Nodes (9): McpLink, McpStatus, Error, Off, Running, AppEvent, Saved, SelectScene (+1 more)

### Community 119 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 123 - "model.rs"
Cohesion: 0.11
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 128 - "Capability"
Cohesion: 0.21
Nodes (3): cap_for_kind(), Capability, last_matching()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "outline.rs"
Cohesion: 0.17
Nodes (10): deg_quat(), euler_matches_core_rotation(), quat_deg(), rotate_all(), DragFixtures, move_objects(), moving_into_rotated_group_keeps_world_place(), natural() (+2 more)

### Community 137 - "stage_editor.rs"
Cohesion: 0.16
Nodes (11): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 140 - "Step"
Cohesion: 0.18
Nodes (7): Fullscreen, SETTLE, Step, Enter, Maximize, drag_ghost(), mark_clipboard()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (16): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+8 more)

### Community 143 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (20): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, grid_options(), HEADER_W, HeaderAction, Delete, Duplicate (+12 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.16
Nodes (17): Host, call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list() (+9 more)

### Community 152 - "bridge.rs"
Cohesion: 0.24
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 153 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.17
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, vec3_row()

### Community 158 - "Vec2"
Cohesion: 0.09
Nodes (19): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+11 more)

### Community 160 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (10): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+2 more)

### Community 161 - "ZoneId"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 162 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 172 - "ProfileChannel"
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 174 - ".show"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 176 - "tools.rs"
Cohesion: 0.14
Nodes (47): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+39 more)

### Community 177 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 195 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.27
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 201 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.22
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 210 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 218 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 219 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 220 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 221 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 222 - "text"
Cohesion: 0.33
Nodes (5): body(), heading_font(), heading_text(), text(), title()

### Community 223 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 224 - "Timeline"
Cohesion: 0.18
Nodes (7): add_audio(), add_audio_dialog(), Clip, fades(), paste_finds_free_place_and_keeps_gaps(), split_and_length(), Timeline

### Community 226 - "ok"
Cohesion: 0.33
Nodes (5): full_workflow(), helpful_errors(), host(), ok(), scene_kind()

### Community 229 - "venue_object_schema"
Cohesion: 0.67
Nodes (3): props!, obj(), venue_object_schema()

### Community 230 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

## Knowledge Gaps
- **433 isolated node(s):** `Cancel`, `Secondary`, `Primary`, `Open`, `New` (+428 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 998 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **112 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `outline.rs`, `stage_editor.rs`, `Action`, `i18n/mod.rs`, `pal`, `bank_menu`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `.viewport`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `file_dialog.rs`, `VizView`, `.row`, `Settings`, `ViewCtx`, `stage.rs`, `Lang`, `patch_grid.rs`, `fx_ui.rs`, `scene_grid.rs`, `dialog.rs`, `profile_builder.rs`, `viz/mod.rs`, `panels.rs`, `Timeline`, `Dialog<'a>`, `app.rs`, `.show`, `profiles.rs`, `Rect`, `View`?**
  _High betweenness centrality (0.155) - this node is a cross-community bridge._
- **Are the 119 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 119 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Cancel`, `Secondary`, `Primary` to the rest of the system?**
  _433 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Venue` be split into smaller, more focused modules?**
  _Cohesion score 0.05333333333333334 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `Action`, `control/timeline.rs`, `server.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `audio.rs`, `core/src/lib.rs`, `FixtureAction`, `.show`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `ViewCtx`, `engine.rs`, `stage.rs`, `Show`, `scene_grid.rs`, `t`, `DmxFrame`, `.fade_state`, `app.rs`, `hashmap`, `Rect`, `Snapshot`, `mcp.rs`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13911290322580644 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Venue`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `Action`, `Engine`, `Step`, `stage_canvas.rs`, `bank_menu`, `.add_scene`, `rig.rs`, `core/src/lib.rs`, `ZoneId`, `FixtureAction`, `.check_placement`, `DmxCraftApp`, `tools.rs`, `engine.rs`, `feature.rs`, `ShowError`, `motion.rs`, `scene_grid.rs`, `show.rs`, `viz/mod.rs`, `panels.rs`, `app.rs`, `Snapshot`, `EffectKind`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._