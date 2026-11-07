# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 110 files · ~126,393 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2649 nodes · 6344 edges · 215 communities (108 shown, 107 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 300 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c7d3c31b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- venue.rs
- FixtureType
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- pal
- steps
- fx.rs
- OutputError
- scene_grid.rs
- Engine
- super
- fx_ui.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- BankScope
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- Scene
- env
- dmxcraft
- knob.rs
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
- hashmap
- ClipAction
- PlanShape
- demo.rs
- ViewCtx
- .paint
- stage.rs
- Transform
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- patch_grid.rs
- Host
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- plan
- Toggle<'a>
- t
- viz/mod.rs
- ChannelKind
- Grip
- CapabilityKind
- Dialog<'a>
- MapPreset
- core/src/lib.rs
- EffectKind
- en.rs
- Profile
- Rect
- OrbitCamera
- text_field
- mapping.rs
- i18n/mod.rs
- AppState
- convert.rs
- Step
- Category
- CurveShape
- ColorPreset
- .modes_tab
- View
- server.rs
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- DvcGoldUsb
- ProgrammerMode
- Drag
- IconButton
- bridge.rs
- ShutterEffect
- Quality
- Vec2
- .add_scene
- call
- MenuAction
- Problem
- ev2.rs
- FixtureLimits
- Capability
- .show
- tools.rs
- scene_card.rs
- deserialize
- qlc-bundle.rs
- time_field.rs
- show
- app.rs
- ShowError
- Shape
- arc
- Settings
- .check_placement
- collect_values

## God Nodes (most connected - your core abstractions)
1. `Engine` - 165 edges
2. `t()` - 136 edges
3. `Show` - 116 edges
4. `Icon` - 103 edges
5. `ChannelKind` - 60 edges
6. `AppState` - 55 edges
7. `ViewCtx` - 43 edges
8. `ChannelRef` - 43 edges
9. `Profile` - 42 edges
10. `DmxFrame` - 42 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `DmxCraftApp` --references--> `McpLink`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/mcp.rs
- `DmxCraftApp` --references--> `VizView`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/viz/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (215 total, 107 thin omitted)

### Community 0 - "venue.rs"
Cohesion: 0.11
Nodes (14): renumber(), apply(), CameraPreset, close(), Environment, IDENTITY, Material, Object (+6 more)

### Community 1 - "FixtureType"
Cohesion: 0.08
Nodes (20): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+12 more)

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (26): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+18 more)

### Community 5 - "programmer.rs"
Cohesion: 0.20
Nodes (20): color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks() (+12 more)

### Community 6 - "pal"
Cohesion: 0.22
Nodes (13): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+5 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (25): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+17 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "scene_grid.rs"
Cohesion: 0.09
Nodes (49): select_all(), CARD_H, programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene (+41 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (43): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+35 more)

### Community 12 - "super"
Cohesion: 0.06
Nodes (31): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+23 more)

### Community 13 - "fx_ui.rs"
Cohesion: 0.11
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

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
Cohesion: 0.21
Nodes (19): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+11 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 24 - "rig.rs"
Cohesion: 0.13
Nodes (20): Beam, build(), cell_light(), cube(), Kind, kind(), Bar, Blinder (+12 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.09
Nodes (35): Show, advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping() (+27 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.14
Nodes (14): heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.09
Nodes (24): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+16 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

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
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "render.rs"
Cohesion: 0.07
Nodes (27): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+19 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (26): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+18 more)

### Community 50 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "demo.rs"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 68 - "ViewCtx"
Cohesion: 0.18
Nodes (9): ViewCtx, DialogKind, ImportEv2, LoadVenue, SaveVenue, fixture_props(), object_tree(), vec3_row() (+1 more)

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (13): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+5 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "Host"
Cohesion: 0.16
Nodes (17): AppEvent, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host, call_tool() (+9 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.10
Nodes (20): slug(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), file_slug(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+12 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 92 - "t"
Cohesion: 0.16
Nodes (26): t(), capability_card(), capability_fields(), category_label(), check(), color_label(), full_combo(), general() (+18 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.21
Nodes (14): hit_y(), Mode, Build, Live, Pick, Fixture, Object, ray_aabb_exit() (+6 more)

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (19): FxTarget, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+11 more)

### Community 95 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "core/src/lib.rs"
Cohesion: 0.15
Nodes (11): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 104 - "EffectKind"
Cohesion: 0.13
Nodes (13): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+5 more)

### Community 108 - "Profile"
Cohesion: 0.12
Nodes (12): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+4 more)

### Community 109 - "Rect"
Cohesion: 0.29
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 113 - "text_field"
Cohesion: 0.14
Nodes (4): text_field(), section_header(), ZonesView, Windows

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "i18n/mod.rs"
Cohesion: 0.05
Nodes (26): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), all_texts_translated() (+18 more)

### Community 119 - "AppState"
Cohesion: 0.15
Nodes (11): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.16
Nodes (20): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+12 more)

### Community 123 - "Step"
Cohesion: 0.20
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, mark_clipboard()

### Community 128 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - ".modes_tab"
Cohesion: 0.12
Nodes (22): list_header(), list_row(), ListAction, Add, Duplicate, Remove, ModeOp, Add (+14 more)

### Community 137 - "View"
Cohesion: 0.11
Nodes (16): apply(), View, body(), caption(), FixtureSettings, range_row(), SetupView, Arrange (+8 more)

### Community 140 - "server.rs"
Cohesion: 0.24
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.16
Nodes (19): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+11 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.10
Nodes (22): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, grid_options(), HEADER_W, HeaderAction (+14 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 150 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 152 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 154 - "bridge.rs"
Cohesion: 0.24
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "Vec2"
Cohesion: 0.10
Nodes (16): DipSwitch, ICON_FAMILY, ICON_FILL_FAMILY, SIZE, SWEEP, LimitPad, from_pct(), HANDLE (+8 more)

### Community 160 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 161 - "call"
Cohesion: 0.30
Nodes (21): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), create_scene(), create_super_scene(), dmx_output() (+13 more)

### Community 162 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "ev2.rs"
Cohesion: 0.18
Nodes (14): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+6 more)

### Community 172 - "Capability"
Cohesion: 0.20
Nodes (3): cap_for_kind(), Capability, ProfileChannel

### Community 174 - ".show"
Cohesion: 0.27
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 176 - "tools.rs"
Cohesion: 0.14
Nodes (26): any_obj(), apply_scene_fields(), boolean(), clip_schema(), fixture_json(), full_workflow(), helpful_errors(), host() (+18 more)

### Community 195 - "scene_card.rs"
Cohesion: 0.33
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 196 - "deserialize"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Segmented<'a, T>

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.24
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 199 - "show"
Cohesion: 0.18
Nodes (9): load_meter(), show(), TitleInfo, UpdateBadge, Available, Busy, Error, Installed (+1 more)

### Community 200 - "app.rs"
Cohesion: 0.26
Nodes (7): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView, StageView

### Community 201 - "ShowError"
Cohesion: 0.21
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Shape"
Cohesion: 0.22
Nodes (9): Shape, Box, Cylinder, Group, Plane, Truss, TrussProfile, Square (+1 more)

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 206 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 207 - "collect_values"
Cohesion: 0.70
Nodes (4): channel_kind(), collect_values(), existing_fixtures(), resolve_channel()

## Knowledge Gaps
- **424 isolated node(s):** `Setup`, `Control`, `Visual`, `Edit`, `Blind` (+419 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 953 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **107 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Category`, `FixtureType`, `runtime.rs`, `programmer.rs`, `View`, `scene_grid.rs`, `super`, `DmxFrame`, `control/timeline.rs`, `ProgrammerMode`, `rig.rs`, `scene_props.rs`, `call`, `FixtureAction`, `.show`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `hashmap`, `ViewCtx`, `stage.rs`, `app.rs`, `Show`, `Host`, `t`, `core/src/lib.rs`, `Profile`, `Rect`, `i18n/mod.rs`?**
  _High betweenness centrality (0.196) - this node is a cross-community bridge._
- **Are the 111 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 111 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Visual` to the rest of the system?**
  _424 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11229946524064172 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `venue.rs`, `FixtureType`, `runtime.rs`, `theme.rs`, `fx.rs`, `scene_grid.rs`, `Engine`, `stage_canvas.rs`, `BankScope`, `rig.rs`, `Scene`, `.add_scene`, `call`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `app.rs`, `feature.rs`, `ShowError`, `motion.rs`, `.check_placement`, `collect_values`, `show.rs`, `viz/mod.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.123) - this node is a cross-community bridge._
- **Should `FixtureType` be split into smaller, more focused modules?**
  _Cohesion score 0.0821256038647343 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `Category`, `FixtureType`, `programmer.rs`, `pal`, `.modes_tab`, `View`, `scene_grid.rs`, `fx_ui.rs`, `control/timeline.rs`, `titlebar.rs`, `ProgrammerMode`, `scene_props.rs`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `ViewCtx`, `stage.rs`, `show`, `app.rs`, `Settings`, `patch_grid.rs`, `dialog.rs`, `Dialog<'a>`, `Profile`, `Rect`, `text_field`, `i18n/mod.rs`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._