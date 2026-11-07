# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 107 files · ~115,028 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2405 nodes · 5723 edges · 200 communities (94 shown, 106 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 253 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8e72efce`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- venue.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- dip_switch.rs
- steps
- fx.rs
- OutputError
- Action
- Engine
- super
- t
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- BankScope
- rig.rs
- Box<dyn serialport::SerialPort>
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
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
- fixture_settings.rs
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
- scene_grid.rs
- plan
- Toggle<'a>
- Gpu
- viz/mod.rs
- ChannelKind
- limit_pad.rs
- mesh.rs
- Dialog<'a>
- MapPreset
- core/src/lib.rs
- EffectKind
- en.rs
- mcp.rs
- Rect
- OrbitCamera
- app.rs
- mapping.rs
- audio.rs
- AppState
- file_dialog.rs
- Step
- Category
- CurveShape
- ColorPreset
- bank_menu
- stage_editor.rs
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
- swatch.rs
- Quality
- Vec2
- Tab
- toggle.rs
- DialogKind
- ok
- FixtureLimits
- elide
- Scene
- tools.rs
- scene_card.rs
- Settings

## God Nodes (most connected - your core abstractions)
1. `Engine` - 162 edges
2. `Show` - 116 edges
3. `t()` - 107 edges
4. `Icon` - 100 edges
5. `ChannelKind` - 54 edges
6. `AppState` - 49 edges
7. `ChannelRef` - 43 edges
8. `ViewCtx` - 43 edges
9. `DmxFrame` - 42 edges
10. `DmxCraftApp` - 40 edges

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
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (200 total, 106 thin omitted)

### Community 0 - "venue.rs"
Cohesion: 0.06
Nodes (37): renumber(), apply(), CameraPreset, close(), Environment, argb(), DEFAULT_TRUSS_LEN, Ev2Fixture (+29 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (41): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+33 more)

### Community 5 - "programmer.rs"
Cohesion: 0.21
Nodes (20): color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+12 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, fbm(), feature_levels() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "Action"
Cohesion: 0.14
Nodes (25): select_all(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash, FocusBank (+17 more)

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (37): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), FadeState, In (+29 more)

### Community 12 - "super"
Cohesion: 0.07
Nodes (36): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password() (+28 more)

### Community 13 - "t"
Cohesion: 0.05
Nodes (67): t(), builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui() (+59 more)

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
Cohesion: 0.07
Nodes (47): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+39 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (76): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+68 more)

### Community 23 - "BankScope"
Cohesion: 0.14
Nodes (9): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), SceneSettings (+1 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (20): Beam, build(), cell_light(), cube(), Kind, kind(), Bar, Blinder (+12 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (15): grid_options(), Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat (+7 more)

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

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (9): Fader, FaderEvent, Changed, None, Release, ToggleFx, fine_mark(), paint_symbol() (+1 more)

### Community 45 - "render.rs"
Cohesion: 0.12
Nodes (15): DEPTH, Frame3d, Globals, HDR, Instance, Light, MAX_LIGHTS, MeshKey (+7 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.20
Nodes (19): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+11 more)

### Community 50 - "fixture_settings.rs"
Cohesion: 0.21
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+1 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "demo.rs"
Cohesion: 0.29
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 68 - "ViewCtx"
Cohesion: 0.20
Nodes (5): ViewCtx, fixture_props(), object_tree(), vec3_row(), VizView

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

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
Cohesion: 0.20
Nodes (14): address_map_marks_ranges(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names() (+6 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 89 - "scene_grid.rs"
Cohesion: 0.22
Nodes (12): SceneCard, programmer_values(), apply(), apply_bank(), arrow(), chip(), COLUMN_W, drag_ghost() (+4 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 92 - "Gpu"
Cohesion: 0.23
Nodes (3): Gpu, GpuMesh, Targets

### Community 93 - "viz/mod.rs"
Cohesion: 0.21
Nodes (14): hit_y(), Mode, Build, Live, Pick, Fixture, Object, ray_aabb_exit() (+6 more)

### Community 94 - "ChannelKind"
Cohesion: 0.07
Nodes (22): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+14 more)

### Community 95 - "limit_pad.rs"
Cohesion: 0.15
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

### Community 96 - "mesh.rs"
Cohesion: 0.33
Nodes (9): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+1 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (14): LoopMode, Loop, Once, old_show_files_without_new_fields_load(), PLAN_SIZE_RANGE, SceneKind, Fx, Static (+6 more)

### Community 104 - "EffectKind"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 108 - "mcp.rs"
Cohesion: 0.20
Nodes (5): McpLink, McpStatus, Error, Off, Running

### Community 109 - "Rect"
Cohesion: 0.26
Nodes (5): paint_fixture(), paint_shape_icon(), fmt_ms(), paint_clip(), TimelineView

### Community 113 - "app.rs"
Cohesion: 0.08
Nodes (16): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ICON_PNG, texture(), window_icon() (+8 more)

### Community 117 - "mapping.rs"
Cohesion: 0.20
Nodes (10): every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level(), transform() (+2 more)

### Community 118 - "audio.rs"
Cohesion: 0.06
Nodes (25): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+17 more)

### Community 119 - "AppState"
Cohesion: 0.11
Nodes (12): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+4 more)

### Community 121 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

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

### Community 136 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 137 - "stage_editor.rs"
Cohesion: 0.21
Nodes (8): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep()

### Community 140 - "server.rs"
Cohesion: 0.20
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (17): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+9 more)

### Community 143 - "DmxFrame"
Cohesion: 0.16
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (20): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete (+12 more)

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
Cohesion: 0.36
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 157 - "Quality"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "Vec2"
Cohesion: 0.11
Nodes (17): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, ICON_FAMILY (+9 more)

### Community 160 - "Tab"
Cohesion: 0.25
Nodes (7): Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 165 - "DialogKind"
Cohesion: 0.50
Nodes (4): DialogKind, ImportEv2, LoadVenue, SaveVenue

### Community 170 - "ok"
Cohesion: 0.67
Nodes (4): full_workflow(), helpful_errors(), host(), ok()

### Community 176 - "tools.rs"
Cohesion: 0.14
Nodes (49): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+41 more)

### Community 195 - "scene_card.rs"
Cohesion: 0.31
Nodes (4): CARD_H, clip(), SceneCardResponse, STRIP_W

### Community 196 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

## Knowledge Gaps
- **359 isolated node(s):** `De`, `En`, `Setup`, `Control`, `Visual` (+354 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 864 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **106 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `venue.rs`, `runtime.rs`, `theme.rs`, `bank_menu`, `fx.rs`, `Action`, `Engine`, `super`, `stage_canvas.rs`, `BankScope`, `rig.rs`, `Scene`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `feature.rs`, `motion.rs`, `show.rs`, `scene_grid.rs`, `viz/mod.rs`, `ChannelKind`, `core/src/lib.rs`, `app.rs`?**
  _High betweenness centrality (0.164) - this node is a cross-community bridge._
- **Are the 86 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 86 INFERRED edges - model-reasoned connections that need verification._
- **What connects `De`, `En`, `Setup` to the rest of the system?**
  _359 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.060655737704918035 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Category`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `Action`, `super`, `t`, `DmxFrame`, `control/timeline.rs`, `ProgrammerMode`, `rig.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `fixture_settings.rs`, `ViewCtx`, `stage.rs`, `Show`, `Host`, `scene_grid.rs`, `core/src/lib.rs`, `mcp.rs`, `Rect`, `app.rs`, `audio.rs`, `AppState`?**
  _High betweenness centrality (0.139) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13768115942028986 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `programmer.rs`, `stage_editor.rs`, `t`, `control/timeline.rs`, `titlebar.rs`, `IconButton`, `menu.rs`, `Vec2`, `Tab`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `settings_window.rs`, `fixture_settings.rs`, `scene_card.rs`, `.paint`, `stage.rs`, `patch_grid.rs`, `dialog.rs`, `scene_grid.rs`, `viz/mod.rs`?**
  _High betweenness centrality (0.085) - this node is a cross-community bridge._