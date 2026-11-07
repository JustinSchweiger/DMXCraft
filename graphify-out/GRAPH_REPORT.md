# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~59,072 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1472 nodes · 3331 edges · 142 communities (67 shown, 75 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 138 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bc947707`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- app.rs
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- pal
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- stage_editor.rs
- super
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- MenuAction
- stage.rs
- ChannelKind
- host
- plan
- with
- ensure_linux_menu_entry
- Value
- swatch.rs
- env
- dmxcraft
- menu.rs
- Settings
- Context
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- quick.rs
- fader.rs
- form
- DmxCraft – Hinweise für KI-Assistenten
- t
- settings_window.rs
- DmxFrame
- show
- DvcGoldUsb
- programmer.rs
- Show
- convert.rs
- OutputKind
- ofl/src/lib.rs
- resize_handles
- OpenDmx
- show.rs
- icons.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- arc
- core/src/lib.rs
- scene_card.rs
- PatchGrid
- t
- PlanShape
- Category
- ShowError
- ZoneId
- stage_canvas.rs
- FixtureLimits
- Arrange
- en.rs
- .check_placement
- qxf.rs
- egui
- limit_pad.rs
- Dialog<'a>
- qlc-bundle.rs
- Theme
- ProgrammerMode
- IconButton
- settings.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 103 edges
2. `Show` - 80 edges
3. `Icon` - 73 edges
4. `t()` - 68 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ChannelRef` - 29 edges
10. `ViewCtx` - 28 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `SetupView` --references--> `StageEditorView`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/stage_editor.rs

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

## Communities (142 total, 75 thin omitted)

### Community 0 - "app.rs"
Cohesion: 0.15
Nodes (9): LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT (+1 more)

### Community 1 - "AppState"
Cohesion: 0.08
Nodes (15): edit_preview(), AppState, LibraryDrag, Mode, Control, Setup, PatchForm, drag_ghost() (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (26): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+18 more)

### Community 5 - "Engine"
Cohesion: 0.06
Nodes (32): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered() (+24 more)

### Community 6 - "pal"
Cohesion: 0.17
Nodes (21): sidebar(), load_meter(), show(), accent(), accent_dim(), apply(), bg(), body() (+13 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.07
Nodes (56): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+48 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 12 - "stage_editor.rs"
Cohesion: 0.14
Nodes (11): section_header(), CanvasMode, Edit, Select, StageCanvas, StageInteraction, ZOOM_MAX, ZOOM_MIN (+3 more)

### Community 13 - "super"
Cohesion: 0.05
Nodes (35): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+27 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (52): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+44 more)

### Community 23 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 24 - "stage.rs"
Cohesion: 0.20
Nodes (12): ControlViews, ProgrammerView, group_pill(), GroupAction, AddSelection, Delete, New, Rename (+4 more)

### Community 25 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "Value"
Cohesion: 0.17
Nodes (12): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Matrix, natural_cmp() (+4 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "menu.rs"
Cohesion: 0.20
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 36 - "Settings"
Cohesion: 0.19
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), Settings

### Community 37 - "Context"
Cohesion: 0.25
Nodes (4): Fullscreen, Step, Enter, Maximize

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (27): fixture_pos(), apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH (+19 more)

### Community 41 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+5 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.12
Nodes (24): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+16 more)

### Community 66 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.25
Nodes (19): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+11 more)

### Community 72 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 73 - "convert.rs"
Cohesion: 0.18
Nodes (13): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), OflError, Invalid, Json (+5 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "core/src/lib.rs"
Cohesion: 0.24
Nodes (5): LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 95 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "t"
Cohesion: 0.14
Nodes (8): View, ViewCtx, caption(), FixtureSettings, range_row(), SetupView, ZonesView, Windows

### Community 98 - "PlanShape"
Cohesion: 0.13
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 99 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.25
Nodes (10): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+2 more)

### Community 104 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 108 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 110 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 111 - "egui"
Cohesion: 0.13
Nodes (6): SETTLE, HANDLE, LimitPad, RangeSlider, Toggle, XyPad

### Community 112 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 123 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "ProgrammerMode"
Cohesion: 0.29
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

## Knowledge Gaps
- **191 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+186 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 548 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **75 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `app.rs`, `AppState`, `runtime.rs`, `theme.rs`, `Engine`, `stage_canvas.rs`, `ZoneId`, `FixtureAction`, `ShowError`, `t`, `stage_editor.rs`, `.check_placement`, `t`, `show.rs`, `ChannelValue`, `core/src/lib.rs`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 52 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 52 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _191 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.07878787878787878 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `app.rs`, `AppState`, `t`, `runtime.rs`, `programmer.rs`, `FixtureAction`, `quick.rs`, `t`, `Show`, `t`, `settings_window.rs`, `stage.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.097) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `PatchGrid`, `t`, `menu.rs`, `programmer.rs`, `IconButton`, `FixtureAction`, `t`, `dialog.rs`, `stage_editor.rs`, `settings_window.rs`, `titlebar.rs`, `icons.rs`, `stage.rs`, `MenuItem<'a>`, `scene_card.rs`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._