# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~55,165 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1422 nodes · 3183 edges · 146 communities (68 shown, 78 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 133 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a0db8e49`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .grid
- MenuAction
- runtime.rs
- open_dmx.rs
- Color32
- Engine
- t
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- ofl/src/lib.rs
- scene_grid.rs
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- Tab
- app.rs
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- egui
- env
- dmxcraft
- file_dialog.rs
- egui
- super
- DmxCraft – UI-Richtlinien
- FixtureAction
- theme.rs
- .show
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Fullscreen
- .check_placement
- DvcGoldUsb
- programmer.rs
- convert.rs
- OutputKind
- show.rs
- patch_grid.rs
- Dialog<'a>
- .group_bar
- OpenDmx
- qxf.rs
- IconButton
- ViewCtx
- icons.rs
- Toggle<'a>
- arc
- Settings
- MenuItem<'a>
- .paint
- core/src/lib.rs
- Show
- PlanShape
- ZoneId
- stage_canvas.rs
- ShowError
- swatch.rs
- en.rs
- DmxFrame
- FixtureLimits
- widgets/mod.rs
- scene_card.rs
- T
- Grip
- Ui
- i18n/mod.rs
- .paint_stage
- Swatch
- MenuPlace
- FixtureType
- sidebar
- Arrange

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 78 edges
3. `Icon` - 73 edges
4. `t()` - 63 edges
5. `ChannelKind` - 39 edges
6. `DmxFrame` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 32 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `toolbar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `mode_switch()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (146 total, 78 thin omitted)

### Community 0 - ".grid"
Cohesion: 0.12
Nodes (15): accent_dim(), body(), caption(), fixture_bar(), heading_font(), heading_text(), raised(), text() (+7 more)

### Community 1 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Color32"
Cohesion: 0.11
Nodes (18): accent_hover(), active(), c(), DANGER, DIP_BODY, fixture_off(), GRAY, NAVY (+10 more)

### Community 5 - "Engine"
Cohesion: 0.09
Nodes (24): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+16 more)

### Community 6 - "t"
Cohesion: 0.17
Nodes (22): t(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+14 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.09
Nodes (42): ControlViews, ProgrammerView, Action, Bank, Delete, DuplicateScene, Jump, Mark (+34 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (9): Button, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel, Primary (+1 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 13 - "scene_grid.rs"
Cohesion: 0.14
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

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
Nodes (52): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+44 more)

### Community 23 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 25 - "app.rs"
Cohesion: 0.13
Nodes (11): LockDialog, OUTPUT_HZ, Mode, Control, Setup, UpdateBadge, Available, Busy (+3 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ChannelValue"
Cohesion: 0.23
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), programmer_values(), ChannelValue, Step

### Community 31 - "egui"
Cohesion: 0.24
Nodes (5): LABEL_H, VALUE_H, LimitPad, Segmented, Toggle

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 36 - "egui"
Cohesion: 0.31
Nodes (4): ZOOM_MAX, ZOOM_MIN, set_pos(), zoom_bar()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (21): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+13 more)

### Community 41 - "theme.rs"
Cohesion: 0.12
Nodes (13): ACTIVE, FS_BODY, FS_CAPTION, FS_HEADING, FS_TITLE, HEADING_FAMILY, R_M, R_S (+5 more)

### Community 44 - ".show"
Cohesion: 0.18
Nodes (8): border(), Fader<'a>, FaderEvent, Changed, None, Release, truncate(), separator()

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.15
Nodes (18): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+10 more)

### Community 50 - "menu.rs"
Cohesion: 0.21
Nodes (8): begin(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem, MIN_W, sub()

### Community 65 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 66 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.06
Nodes (39): edit_preview(), AppState, PatchForm, ProgrammerMode, Blind, Edit, Live, Fader (+31 more)

### Community 73 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.25
Nodes (9): CELL_H_MAX, CELL_H_MIN, COLUMNS, Drag, GridMode, Output, Patch, PatchGrid (+1 more)

### Community 79 - ".group_bar"
Cohesion: 0.26
Nodes (9): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.21
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 89 - "ViewCtx"
Cohesion: 0.11
Nodes (11): Dialog, section_header(), View, ViewCtx, caption(), FixtureSettings, range_row(), SetupView (+3 more)

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "Settings"
Cohesion: 0.06
Nodes (27): LIMIT, digest(), hash_and_verify(), hash_password(), hex(), verify(), cable_belongs_to_one_universe(), default_instance_name() (+19 more)

### Community 98 - "core/src/lib.rs"
Cohesion: 0.22
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 100 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.21
Nodes (10): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+2 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 110 - "widgets/mod.rs"
Cohesion: 0.40
Nodes (5): CanvasMode, Edit, Select, ShapeButton, StageCanvas

### Community 111 - "scene_card.rs"
Cohesion: 0.27
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 133 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 135 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

### Community 137 - "FixtureType"
Cohesion: 0.21
Nodes (3): ChannelDef, FixtureType, ft()

### Community 138 - "sidebar"
Cohesion: 0.23
Nodes (9): sidebar(), accent(), apply(), bg(), install_fonts(), raised_hover(), surface(), DipSwitch (+1 more)

### Community 140 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

## Knowledge Gaps
- **188 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+183 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 538 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **78 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `.grid`, `ViewCtx`, `core/src/lib.rs`, `Show`, `runtime.rs`, `programmer.rs`, `FixtureAction`, `t`, `scene_grid.rs`, `DmxCraftApp`, `settings_window.rs`, `.group_bar`, `app.rs`, `ChannelValue`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _188 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `.grid` be split into smaller, more focused modules?**
  _Cohesion score 0.1206896551724138 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `core/src/lib.rs`, `.check_placement`, `Color32`, `super`, `stage_canvas.rs`, `Engine`, `FixtureAction`, `theme.rs`, `t`, `FixtureType`, `show.rs`, `scene_grid.rs`, `widgets/mod.rs`, `ShowError`, `runtime.rs`, `ChannelValue`, `ZoneId`, `Settings`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._
- **Should `MenuAction` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `MenuItem<'a>`, `egui`, `programmer.rs`, `t`, `FixtureAction`, `t`, `dialog.rs`, `scene_grid.rs`, `widgets/mod.rs`, `scene_card.rs`, `settings_window.rs`, `patch_grid.rs`, `menu.rs`, `IconButton`, `Tab`, `ViewCtx`, `icons.rs`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._