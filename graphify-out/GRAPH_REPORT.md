# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 74 files · ~47,985 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1322 nodes · 2983 edges · 105 communities (54 shown, 51 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 123 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7f49eeda`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- egui
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- fader.rs
- Show
- steps
- DmxCraftApp
- OutputError
- AppState
- dialog.rs
- Category
- library_panel.rs
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- fixture_settings.rs
- t
- host
- plan
- with
- ensure_linux_menu_entry
- patch_grid.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- stage_editor.rs
- MenuItem<'a>
- DmxCraft – UI-Richtlinien
- FixtureAction
- settings_window.rs
- FixtureType
- Camera
- DmxCraft – Hinweise für KI-Assistenten
- widgets/mod.rs
- general
- menu.rs
- Settings
- swatch.rs
- DvcGoldUsb
- OpenDmx
- titlebar.rs
- convert.rs
- OutputKind
- sense
- LibraryPanel
- Dialog<'a>
- DmxFrame
- Stroke
- Toggle<'a>
- Arrange
- super
- Tab
- arc
- en.rs
- Grip
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 66 edges
3. `Icon` - 64 edges
4. `t()` - 61 edges
5. `DmxFrame` - 38 edges
6. `ChannelKind` - 35 edges
7. `DmxCraftApp` - 34 edges
8. `text_dim()` - 34 edges
9. `AppState` - 33 edges
10. `ViewCtx` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `paint_fixture()` --calls--> `fill()`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → crates/app/src/i18n/mod.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (105 total, 51 thin omitted)

### Community 0 - "egui"
Cohesion: 0.20
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 1 - "Engine"
Cohesion: 0.06
Nodes (44): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values(), save_bar() (+36 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.06
Nodes (58): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+50 more)

### Community 5 - "fader.rs"
Cohesion: 0.18
Nodes (8): Fader, FaderEvent, Changed, None, Release, LABEL_H, truncate(), VALUE_H

### Community 6 - "Show"
Cohesion: 0.06
Nodes (38): cv(), demo_show(), empty_show(), ensure_builtin_types(), address_map_marks_ranges(), ChannelValue, Fixture, FixtureLimits (+30 more)

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
Cohesion: 0.17
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 11 - "dialog.rs"
Cohesion: 0.11
Nodes (9): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, LABEL_W (+1 more)

### Community 12 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

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
Nodes (48): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+40 more)

### Community 23 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.24
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 25 - "t"
Cohesion: 0.05
Nodes (27): ControlViews, DmxCraftApp, LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs (+19 more)

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
Cohesion: 0.23
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (10): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "stage_editor.rs"
Cohesion: 0.28
Nodes (7): CanvasMode, Edit, Select, StageCanvas, ZOOM_MAX, ZOOM_MIN, set_pos()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.08
Nodes (26): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+18 more)

### Community 41 - "settings_window.rs"
Cohesion: 0.18
Nodes (8): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, INPUT_H, UsbDmxPort

### Community 44 - "FixtureType"
Cohesion: 0.29
Nodes (3): ChannelDef, FixtureType, ft()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "widgets/mod.rs"
Cohesion: 0.25
Nodes (4): RangeSlider, Segmented, ShapeButton, XyPad

### Community 48 - "general"
Cohesion: 0.24
Nodes (11): set_lang(), main(), full_combo(), general(), hint(), midi(), outputs(), project() (+3 more)

### Community 50 - "menu.rs"
Cohesion: 0.10
Nodes (28): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+20 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 71 - "titlebar.rs"
Cohesion: 0.07
Nodes (43): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+35 more)

### Community 73 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 77 - "sense"
Cohesion: 0.15
Nodes (3): clip(), SceneCard, Toggle

### Community 87 - "Dialog<'a>"
Cohesion: 0.20
Nodes (4): Dialog<'a>, Kind, Cancel, Primary

### Community 88 - "DmxFrame"
Cohesion: 0.20
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 93 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 94 - "super"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 100 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 117 - "Grip"
Cohesion: 0.16
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented<'a, T>

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

## Knowledge Gaps
- **173 isolated node(s):** `Select`, `Edit`, `Stage`, `StageEditor`, `Patch` (+168 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 498 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **51 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Icon` connect `Icon` to `Tab`, `MenuItem<'a>`, `stage_editor.rs`, `titlebar.rs`, `FixtureAction`, `settings_window.rs`, `dialog.rs`, `widgets/mod.rs`, `menu.rs`, `fixture_settings.rs`, `Stroke`, `patch_grid.rs`?**
  _High betweenness centrality (0.123) - this node is a cross-community bridge._
- **Are the 55 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 55 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `Stage` to the rest of the system?**
  _173 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06258958432871477 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `runtime.rs`, `theme.rs`, `Show`, `FixtureAction`, `settings_window.rs`, `library_panel.rs`, `general`, `LibraryPanel`, `menu.rs`, `fixture_settings.rs`, `t`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11965811965811966 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `theme.rs`, `stage_editor.rs`, `FixtureAction`, `FixtureType`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._