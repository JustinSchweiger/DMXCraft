# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 111 files · ~132,174 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2725 nodes · 6573 edges · 240 communities (121 shown, 119 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 305 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `32a570bf`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Venue
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- widgets/mod.rs
- steps
- fx.rs
- OutputError
- scene_grid.rs
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
- super
- env
- dmxcraft
- mapping.rs
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- VizView
- ClipAction
- egui
- demo.rs
- ViewCtx
- Scene
- stage.rs
- Transform
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- patch_grid.rs
- fx_ui.rs
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- call
- Toggle<'a>
- t
- viz/mod.rs
- ChannelKind
- AppState
- CapabilityKind
- Dialog<'a>
- MapPreset
- app.rs
- qxf.rs
- en.rs
- Profile
- Rect
- View
- Action
- knob.rs
- mcp.rs
- EffectKind
- convert.rs
- bank_menu
- core/src/venue.rs
- CurveShape
- ColorPreset
- ev2.rs
- stage_editor.rs
- Step
- stage_canvas.rs
- ScenePropsView
- control/timeline.rs
- scene_card.rs
- MenuItem<'a>
- server.rs
- bridge.rs
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- Quality
- sense
- core/src/lib.rs
- Kind
- time_field.rs
- Problem
- .check_placement
- OrbitCamera
- Host
- .show
- tools.rs
- file_dialog.rs
- Category
- DvcGoldUsb
- qlc-bundle.rs
- Effect
- ShowError
- ProgrammerMode
- arc
- Settings
- Drag
- DmxFrame
- Tab
- Theme
- Timeline
- ChannelValue
- Shape
- BankScope
- hashmap
- ChaserPreset
- T
- body
- FxFile
- OflError
- PickTarget
- Physics
- elide

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 137 edges
3. `Show` - 117 edges
4. `Icon` - 103 edges
5. `ChannelKind` - 60 edges
6. `AppState` - 55 edges
7. `ViewCtx` - 46 edges
8. `FixtureType` - 43 edges
9. `Profile` - 42 edges
10. `DmxFrame` - 42 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `AppState` --references--> `Quality`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `AppState` --references--> `Fixture`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `DmxCraftApp` --references--> `VizView`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/viz/mod.rs

## Import Cycles
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

## Communities (240 total, 119 thin omitted)

### Community 0 - "Venue"
Cohesion: 0.19
Nodes (7): renumber(), Environment, hang_below_nearest_truss(), Material, Object, room_and_tree(), Venue

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (16): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+8 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (8): AudioDuration, DEFAULT_PORT, engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (24): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+16 more)

### Community 5 - "programmer.rs"
Cohesion: 0.21
Nodes (20): color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+12 more)

### Community 6 - "widgets/mod.rs"
Cohesion: 0.17
Nodes (7): section_header(), Segmented, CanvasMode, Edit, Select, Swatch, XyPad

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "scene_grid.rs"
Cohesion: 0.15
Nodes (18): select_all(), CARD_H, programmer_values(), Bank, FocusBank, MoveScenes, RenameBank, apply() (+10 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "pal"
Cohesion: 0.20
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

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
Cohesion: 0.22
Nodes (10): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+2 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.16
Nodes (15): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), kind(), linear() (+7 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.28
Nodes (12): advanced(), bar_slider(), block_props(), FX_EXT, pick_tile(), PickColumn, properties(), row() (+4 more)

### Community 26 - "host"
Cohesion: 0.20
Nodes (10): if, needs, outputs, runs-on, host, publishing, tag, tag-flag (+2 more)

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (11): best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_physical(), Library, LibraryFixture (+3 more)

### Community 31 - "super"
Cohesion: 0.05
Nodes (40): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+32 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 36 - "Timeline"
Cohesion: 0.14
Nodes (13): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+5 more)

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
Cohesion: 0.08
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "render.rs"
Cohesion: 0.07
Nodes (29): ADDITIVE, Bloom, BLOOM_LEVELS, buffer(), DEPTH, Frame3d, fullscreen_pipeline(), Globals (+21 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (18): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+10 more)

### Community 50 - "VizView"
Cohesion: 0.14
Nodes (11): DialogKind, ImportEv2, LoadVenue, SaveVenue, Missing, Mode, Build, Live (+3 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "egui"
Cohesion: 0.12
Nodes (14): paint_shape_icon(), MenuPlace, Patch, Stage, StageEditor, SIZES, PlanShape, .ALL (+6 more)

### Community 66 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 68 - "ViewCtx"
Cohesion: 0.34
Nodes (9): ViewCtx, aim_selected(), apply_placement(), object_tree(), Pick, Fixture, Object, targets() (+1 more)

### Community 70 - "stage.rs"
Cohesion: 0.19
Nodes (13): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+5 more)

### Community 71 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 73 - "feature.rs"
Cohesion: 0.14
Nodes (17): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+9 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.11
Nodes (11): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation, OutputStatus, Connecting (+3 more)

### Community 76 - "motion.rs"
Cohesion: 0.12
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.11
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.18
Nodes (13): address_map_marks_ranges(), FixtureLimits, groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range() (+5 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (23): cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+15 more)

### Community 90 - "call"
Cohesion: 0.30
Nodes (21): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), create_scene(), create_super_scene(), dmx_output() (+13 more)

### Community 92 - "t"
Cohesion: 0.08
Nodes (48): t(), text_field(), capability_card(), capability_fields(), category_label(), check(), color_label(), full_combo() (+40 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.21
Nodes (14): AXIS_COLORS, Handles, handles(), hit_y(), project(), QualityParams, ray_aabb_exit(), ray_box() (+6 more)

### Community 94 - "ChannelKind"
Cohesion: 0.07
Nodes (21): FxTarget, ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+13 more)

### Community 95 - "AppState"
Cohesion: 0.13
Nodes (12): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+4 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "app.rs"
Cohesion: 0.24
Nodes (6): ControlViews, edit_preview(), load_step(), OUTPUT_HZ, SceneGridView, View

### Community 104 - "qxf.rs"
Cohesion: 0.23
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "Profile"
Cohesion: 0.12
Nodes (13): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+5 more)

### Community 109 - "Rect"
Cohesion: 0.25
Nodes (6): fmt_ms(), grid_options(), mark_clipboard(), paint_clip(), TimelineView, toggle_play()

### Community 110 - "View"
Cohesion: 0.21
Nodes (6): apply(), body(), caption(), FixtureSettings, range_row(), SetupView

### Community 113 - "Action"
Cohesion: 0.20
Nodes (18): Action, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump, Mark (+10 more)

### Community 118 - "mcp.rs"
Cohesion: 0.15
Nodes (9): McpLink, McpStatus, Error, Off, Running, AppEvent, Saved, SelectScene (+1 more)

### Community 119 - "EffectKind"
Cohesion: 0.14
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 123 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 128 - "core/src/venue.rs"
Cohesion: 0.15
Nodes (10): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, rotation(), rotation_order_x_then_y_then_z(), Transform (+2 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "ev2.rs"
Cohesion: 0.18
Nodes (14): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+6 more)

### Community 137 - "stage_editor.rs"
Cohesion: 0.19
Nodes (9): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), StageEditorView (+1 more)

### Community 140 - "Step"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 141 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (15): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+7 more)

### Community 143 - "ScenePropsView"
Cohesion: 0.18
Nodes (11): FxDialog, header(), Picker, picker_window(), ScenePropsView, single_scene(), Tab, Effects (+3 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.13
Nodes (17): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete, Duplicate, ExpandAll (+9 more)

### Community 147 - "scene_card.rs"
Cohesion: 0.25
Nodes (5): clip(), SceneCard, SceneCardResponse, STRIP_W, PillResult

### Community 150 - "server.rs"
Cohesion: 0.20
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 152 - "bridge.rs"
Cohesion: 0.36
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 153 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.39
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "sense"
Cohesion: 0.10
Nodes (15): SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+7 more)

### Community 160 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (13): UNIVERSE_SIZE, SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind (+5 more)

### Community 161 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 162 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 172 - "Host"
Cohesion: 0.36
Nodes (11): Host, call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list() (+3 more)

### Community 174 - ".show"
Cohesion: 0.17
Nodes (9): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, ZonesView (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.11
Nodes (30): any_obj(), apply_scene_fields(), boolean(), clip_schema(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+22 more)

### Community 177 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 195 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 196 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Effect"
Cohesion: 0.24
Nodes (6): Show, effect_header(), effect_tiles(), fit_mapping(), FxPreview, Effect

### Community 201 - "ShowError"
Cohesion: 0.21
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 222 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 223 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 224 - "Timeline"
Cohesion: 0.28
Nodes (3): add_audio(), add_audio_dialog(), Timeline

### Community 225 - "ChannelValue"
Cohesion: 0.33
Nodes (6): ChannelValue, Step, channel_kind(), collect_values(), existing_fixtures(), resolve_channel()

### Community 226 - "Shape"
Cohesion: 0.22
Nodes (9): Shape, Box, Cylinder, Group, Plane, Truss, TrussProfile, Square (+1 more)

### Community 228 - "BankScope"
Cohesion: 0.25
Nodes (7): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank

### Community 230 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 232 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 234 - "body"
Cohesion: 0.33
Nodes (4): body(), heading_font(), heading_text(), title()

### Community 236 - "OflError"
Cohesion: 0.40
Nodes (4): OflError, Invalid, Json, Skip

### Community 237 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

## Knowledge Gaps
- **424 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+419 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 973 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **119 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Venue`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `scene_grid.rs`, `Engine`, `stage_canvas.rs`, `.add_scene`, `rig.rs`, `core/src/lib.rs`, `FixtureAction`, `.check_placement`, `DmxCraftApp`, `tools.rs`, `egui`, `demo.rs`, `Scene`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `call`, `viz/mod.rs`, `ChannelValue`, `app.rs`, `bank_menu`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Are the 111 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 111 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _424 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1350806451612903 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `scene_grid.rs`, `ScenePropsView`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `super`, `core/src/lib.rs`, `Host`, `.show`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `egui`, `Category`, `ViewCtx`, `stage.rs`, `Show`, `ProgrammerMode`, `DmxFrame`, `call`, `t`, `AppState`, `app.rs`, `hashmap`, `Profile`, `Rect`, `View`, `Action`, `mcp.rs`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `stage_editor.rs`, `scene_grid.rs`, `i18n/mod.rs`, `pal`, `ScenePropsView`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `file_dialog.rs`, `VizView`, `egui`, `Category`, `ViewCtx`, `Effect`, `stage.rs`, `ProgrammerMode`, `Settings`, `fx_ui.rs`, `patch_grid.rs`, `dialog.rs`, `viz/mod.rs`, `AppState`, `Timeline`, `Dialog<'a>`, `app.rs`, `FxFile`, `Profile`, `Rect`, `View`, `Action`, `bank_menu`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._