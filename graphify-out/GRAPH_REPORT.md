# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 72 files · ~43,509 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1265 nodes · 2615 edges · 124 communities (61 shown, 63 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 66 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6c2591c4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- engine.rs
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- scenes.rs
- Show
- steps
- DmxCraftApp
- OutputError
- AppState
- dialog.rs
- FixtureAction
- jobs
- steps
- DmxCraft
- .ui
- announce
- build-local-artifacts
- Icon
- ChannelKind
- fixture_settings.rs
- app.rs
- host
- plan
- with
- ensure_linux_menu_entry
- patch_grid.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- show.rs
- send_frame
- DmxCraft – UI-Richtlinien
- ChannelValue
- egui
- text_field
- Scene
- ofl/src/lib.rs
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- Vec2
- settings_window.rs
- menu.rs
- Settings
- swatch.rs
- DvcGoldUsb
- OpenDmx
- OutputStatus
- titlebar.rs
- FixtureLimits
- convert.rs
- widgets/mod.rs
- scene_card.rs
- IconButton
- stage_editor.rs
- Segmented
- LibraryPanel
- Dialog<'a>
- DmxFrame
- Stroke
- Toggle<'a>
- programmer.rs
- plan.rs
- ZoneId
- FixtureType
- Value
- Tab
- Category
- arc
- dip_switch.rs
- Matrix
- .check_placement
- library_panel.rs
- ofl-bundle.rs
- Ui
- Grip
- .ui

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 65 edges
3. `Icon` - 63 edges
4. `DmxFrame` - 38 edges
5. `DmxCraftApp` - 34 edges
6. `ChannelKind` - 34 edges
7. `AppState` - 33 edges
8. `ViewCtx` - 30 edges
9. `FixtureType` - 26 edges
10. `FixtureAction` - 24 edges

## Surprising Connections (you probably didn't know these)
- `general()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `project()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `save_bar()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/dialog.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (124 total, 63 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.26
Nodes (18): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), limits_cap_dimmer_and_position(), MAX_STEP_ADVANCES_PER_TICK, MS (+10 more)

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 2 - "runtime.rs"
Cohesion: 0.16
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (38): ACCENT, ACCENT_DIM, ACCENT_HOVER, apply(), BG, body(), BORDER, caption() (+30 more)

### Community 5 - "scenes.rs"
Cohesion: 0.20
Nodes (12): programmer_values(), zone_combo(), apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "AppState"
Cohesion: 0.19
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 11 - "dialog.rs"
Cohesion: 0.13
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+2 more)

### Community 12 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (47): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+39 more)

### Community 23 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 25 - "app.rs"
Cohesion: 0.05
Nodes (31): ControlViews, DmxCraftApp, LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs (+23 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "patch_grid.rs"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (9): BOX, fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon(), StageCanvas (+1 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "show.rs"
Cohesion: 0.30
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 37 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 40 - "egui"
Cohesion: 0.23
Nodes (6): section_header(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 43 - "Scene"
Cohesion: 0.19
Nodes (7): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, UNIVERSES

### Community 44 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "Vec2"
Cohesion: 0.16
Nodes (5): LABEL_H, VALUE_H, LimitPad, RangeSlider, XyPad

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (17): full_combo(), general(), HEIGHT, hint(), lock(), midi(), outputs(), project() (+9 more)

### Community 50 - "menu.rs"
Cohesion: 0.06
Nodes (36): Fader<'a>, FaderEvent, Changed, None, Release, truncate(), begin(), context() (+28 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 70 - "OutputStatus"
Cohesion: 0.17
Nodes (6): OutputConfig, OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.06
Nodes (43): HEIGHT, LEVELS, load_meter(), LOCK, main_menu(), MenuAction, About, CheckUpdates (+35 more)

### Community 73 - "convert.rs"
Cohesion: 0.25
Nodes (9): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), redirects_are_skipped(), simple_par_with_modes_defaults_and_null(), slug() (+1 more)

### Community 74 - "widgets/mod.rs"
Cohesion: 0.22
Nodes (5): Fader, CanvasMode, Edit, Select, ShapeButton

### Community 77 - "stage_editor.rs"
Cohesion: 0.17
Nodes (8): Toggle, Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos()

### Community 88 - "DmxFrame"
Cohesion: 0.16
Nodes (4): ViewCtx, addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 93 - "programmer.rs"
Cohesion: 0.42
Nodes (9): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), save_bar(), set_group() (+1 more)

### Community 94 - "plan.rs"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 97 - "FixtureType"
Cohesion: 0.21
Nodes (3): ChannelDef, FixtureType, ft()

### Community 98 - "Value"
Cohesion: 0.31
Nodes (8): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 100 - "Tab"
Cohesion: 0.22
Nodes (9): SettingsWindow, sidebar(), Tab, .ALL, General, Lock, Midi, Outputs (+1 more)

### Community 101 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 105 - "Matrix"
Cohesion: 0.29
Nodes (4): Matrix, natural_cmp(), natural_sort(), Pixel

### Community 109 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 111 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 117 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **169 isolated node(s):** `General`, `Outputs`, `Midi`, `Project`, `Lock` (+164 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 486 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **63 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `engine.rs`, `runtime.rs`, `scenes.rs`, `Show`, `egui`, `Scene`, `library_panel.rs`, `settings_window.rs`, `LibraryPanel`, `menu.rs`, `.ui`, `.ui`, `DmxFrame`, `app.rs`, `programmer.rs`, `fixture_settings.rs`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **What connects `General`, `Outputs`, `Midi` to the rest of the system?**
  _169 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.10037878787878787 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `Tab`, `titlebar.rs`, `egui`, `widgets/mod.rs`, `dialog.rs`, `IconButton`, `stage_editor.rs`, `settings_window.rs`, `menu.rs`, `fixture_settings.rs`, `Stroke`, `patch_grid.rs`?**
  _High betweenness centrality (0.121) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07171717171717172 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `engine.rs`, `Engine`, `FixtureType`, `ZoneId`, `theme.rs`, `show.rs`, `ChannelValue`, `egui`, `Scene`, `FixtureAction`, `.check_placement`, `ShowError`, `plan.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.084) - this node is a cross-community bridge._
- **Should `steps` be split into smaller, more focused modules?**
  _Cohesion score 0.08695652173913043 - nodes in this community are weakly interconnected._