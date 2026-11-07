# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~53,336 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1393 nodes · 3129 edges · 134 communities (69 shown, 65 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 126 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `82cddf07`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .grid
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- titlebar.rs
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- ofl/src/lib.rs
- color_picker.rs
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- .show
- library_panel.rs
- host
- plan
- with
- ensure_linux_menu_entry
- Dialog<'a>
- stage_canvas.rs
- env
- dmxcraft
- file_dialog.rs
- egui
- Show
- DmxCraft – UI-Richtlinien
- FixtureAction
- PatchGrid
- t
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Settings
- t
- DvcGoldUsb
- ViewCtx
- Category
- convert.rs
- OutputKind
- show.rs
- patch_grid.rs
- text_dim
- MenuItem<'a>
- OpenDmx
- qxf.rs
- fixture_settings.rs
- Stroke
- Toggle<'a>
- arc
- super
- Theme
- settings_window.rs
- core/src/lib.rs
- FixtureId
- PlanShape
- ZoneId
- app.rs
- ShowError
- swatch.rs
- Tab
- en.rs
- DmxFrame
- FixtureLimits
- scene_card.rs
- .show
- limit_pad.rs
- Arrange
- i18n/mod.rs
- Ui
- .check_placement

## God Nodes (most connected - your core abstractions)
1. `Show` - 78 edges
2. `Engine` - 77 edges
3. `Icon` - 70 edges
4. `t()` - 65 edges
5. `DmxFrame` - 38 edges
6. `ChannelKind` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 31 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `column()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `scene_card()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs

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

## Communities (134 total, 65 thin omitted)

### Community 0 - ".grid"
Cohesion: 0.17
Nodes (11): sidebar(), accent(), accent_dim(), apply(), bg(), border(), install_fonts(), raised_hover() (+3 more)

### Community 1 - "Engine"
Cohesion: 0.07
Nodes (39): cv(), demo_show(), empty_show(), ensure_builtin_types(), color_bar(), current(), fader(), fader_groups() (+31 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (26): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_off(), FS_BODY, FS_CAPTION (+18 more)

### Community 5 - ".show"
Cohesion: 0.18
Nodes (6): Fader<'a>, FaderEvent, Changed, None, Release, truncate()

### Community 6 - "titlebar.rs"
Cohesion: 0.05
Nodes (48): Fullscreen, SETTLE, Step, Enter, Maximize, delete(), HEIGHT, k() (+40 more)

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
Cohesion: 0.08
Nodes (45): color_picker(), color_swatches(), text_field(), CARD_H, Action, Bank, Delete, DuplicateScene (+37 more)

### Community 11 - "dialog.rs"
Cohesion: 0.11
Nodes (9): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, Kind, Cancel, Primary (+1 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.14
Nodes (11): hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch, Auto (+3 more)

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
Nodes (51): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+43 more)

### Community 23 - "ChannelKind"
Cohesion: 0.09
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 24 - ".show"
Cohesion: 0.23
Nodes (7): load_meter(), show(), body(), heading_font(), raised(), DialogResult, title()

### Community 25 - "library_panel.rs"
Cohesion: 0.26
Nodes (4): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 31 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (11): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+3 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "file_dialog.rs"
Cohesion: 0.20
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 36 - "egui"
Cohesion: 0.13
Nodes (15): section_header(), CanvasMode, Edit, Select, fixture_pos(), ZOOM_MAX, ZOOM_MIN, apply() (+7 more)

### Community 37 - "Show"
Cohesion: 0.15
Nodes (5): History, LIMIT, undo_redo_roundtrip(), FixtureType, Show

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "PatchGrid"
Cohesion: 0.40
Nodes (5): Drag, GridMode, Output, Patch, PatchGrid

### Community 44 - "t"
Cohesion: 0.22
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.24
Nodes (12): set_lang(), main(), full_combo(), general(), hint(), midi(), outputs(), project() (+4 more)

### Community 50 - "menu.rs"
Cohesion: 0.19
Nodes (10): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+2 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 66 - "t"
Cohesion: 0.39
Nodes (3): t(), group_bar(), selection_tools()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "ViewCtx"
Cohesion: 0.16
Nodes (7): View, ViewCtx, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 71 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

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
Cohesion: 0.13
Nodes (12): DipSwitch, Fader, LABEL_H, VALUE_H, LimitPad, Segmented, Toggle, XyPad (+4 more)

### Community 78 - "text_dim"
Cohesion: 0.20
Nodes (9): caption(), fixture_bar(), heading_text(), text(), text_dim(), text_faint(), caption(), range_row() (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.21
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.22
Nodes (6): RangeSlider, Quick, quick_action(), Center, Off, On

### Community 90 - "Stroke"
Cohesion: 0.16
Nodes (3): file_icons(), IconButton, ShapeButton

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "super"
Cohesion: 0.10
Nodes (22): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+14 more)

### Community 96 - "Theme"
Cohesion: 0.29
Nodes (5): active(), Theme, .ALL, Gray, Navy

### Community 97 - "settings_window.rs"
Cohesion: 0.20
Nodes (8): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, INPUT_H, UsbDmxPort

### Community 98 - "core/src/lib.rs"
Cohesion: 0.20
Nodes (7): SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 100 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "app.rs"
Cohesion: 0.31
Nodes (5): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, StageView

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 105 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "scene_card.rs"
Cohesion: 0.33
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 118 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

### Community 132 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

## Knowledge Gaps
- **183 isolated node(s):** `Play`, `Pause`, `Prev`, `Next`, `Plus` (+178 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 525 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **65 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `Engine`, `.show`, `titlebar.rs`, `t`, `color_picker.rs`, `.show`, `library_panel.rs`, `stage_canvas.rs`, `file_dialog.rs`, `egui`, `FixtureAction`, `t`, `DmxCraftApp`, `settings_window.rs`, `Settings`, `ViewCtx`, `text_dim`, `app.rs`, `i18n/mod.rs`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Play`, `Pause`, `Prev` to the rest of the system?**
  _183 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06655974338412189 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `settings_window.rs`, `t`, `core/src/lib.rs`, `egui`, `ViewCtx`, `app.rs`, `Show`, `runtime.rs`, `t`, `DmxCraftApp`, `settings_window.rs`, `fixture_settings.rs`, `library_panel.rs`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `core/src/lib.rs`, `FixtureId`, `theme.rs`, `egui`, `ZoneId`, `.check_placement`, `FixtureAction`, `ShowError`, `t`, `runtime.rs`, `show.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._