# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 74 files · ~47,917 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1322 nodes · 2988 edges · 109 communities (59 shown, 50 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 123 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `092de93d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- file_dialog.rs
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
- app.rs
- plan.rs
- jobs
- steps
- DmxCraft
- ViewCtx
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
- send_frame
- egui
- settings_window.rs
- OutputStatus
- ofl/src/lib.rs
- Camera
- DmxCraft – Hinweise für KI-Assistenten
- scene_card.rs
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
- Segmented
- LibraryPanel
- Dialog<'a>
- DmxFrame
- Stroke
- Toggle<'a>
- Arrange
- super
- Value
- Tab
- arc
- Matrix
- en.rs
- ofl-bundle.rs
- Grip
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 66 edges
3. `Icon` - 64 edges
4. `t()` - 60 edges
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
- `selection_tools()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/stage.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (109 total, 50 thin omitted)

### Community 0 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (23): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+15 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.06
Nodes (61): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+53 more)

### Community 5 - "fader.rs"
Cohesion: 0.29
Nodes (4): Fader, LABEL_H, truncate(), VALUE_H

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
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.16
Nodes (7): AppState, Mode, Control, Setup, PatchForm, group_bar(), selection_tools()

### Community 11 - "dialog.rs"
Cohesion: 0.11
Nodes (9): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, LABEL_W (+1 more)

### Community 12 - "app.rs"
Cohesion: 0.31
Nodes (6): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView

### Community 13 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "ViewCtx"
Cohesion: 0.16
Nodes (8): View, ViewCtx, FixtureSettings, SetupView, ScenesView, StageEditorView, ZonesView, Windows

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
Cohesion: 0.08
Nodes (21): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+13 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.22
Nodes (6): RangeSlider, Quick, quick_action(), Center, Off, On

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
Cohesion: 0.22
Nodes (9): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+1 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.15
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 36 - "stage_editor.rs"
Cohesion: 0.17
Nodes (9): CanvasMode, Edit, Select, StageCanvas, StageInteraction, ZOOM_MAX, ZOOM_MIN, set_pos() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 40 - "egui"
Cohesion: 0.18
Nodes (7): section_header(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 41 - "settings_window.rs"
Cohesion: 0.18
Nodes (8): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, INPUT_H, UsbDmxPort

### Community 43 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 44 - "ofl/src/lib.rs"
Cohesion: 0.20
Nodes (4): EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "general"
Cohesion: 0.24
Nodes (11): set_lang(), main(), full_combo(), general(), hint(), midi(), outputs(), project() (+3 more)

### Community 50 - "menu.rs"
Cohesion: 0.06
Nodes (49): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+41 more)

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
Cohesion: 0.05
Nodes (48): Fullscreen, SETTLE, Step, Enter, Maximize, delete(), HEIGHT, k() (+40 more)

### Community 73 - "convert.rs"
Cohesion: 0.21
Nodes (13): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), OflError, Invalid, Json (+5 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 80 - "LibraryPanel"
Cohesion: 0.23
Nodes (3): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 87 - "Dialog<'a>"
Cohesion: 0.20
Nodes (4): Dialog<'a>, Kind, Cancel, Primary

### Community 88 - "DmxFrame"
Cohesion: 0.07
Nodes (33): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+25 more)

### Community 93 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 94 - "super"
Cohesion: 0.12
Nodes (14): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+6 more)

### Community 98 - "Value"
Cohesion: 0.31
Nodes (8): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 100 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 105 - "Matrix"
Cohesion: 0.29
Nodes (4): Matrix, natural_cmp(), natural_sort(), Pixel

### Community 111 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 117 - "Grip"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

## Knowledge Gaps
- **173 isolated node(s):** `Play`, `Pause`, `Prev`, `Next`, `Plus` (+168 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 496 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **50 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `runtime.rs`, `theme.rs`, `Show`, `egui`, `settings_window.rs`, `AppState`, `app.rs`, `general`, `LibraryPanel`, `menu.rs`, `ViewCtx`, `DmxFrame`, `t`, `fixture_settings.rs`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Are the 55 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 55 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Play`, `Pause`, `Prev` to the rest of the system?**
  _173 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09344262295081968 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `theme.rs`, `stage_editor.rs`, `egui`, `menu.rs`, `ChannelKind`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14130434782608695 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `file_dialog.rs`, `Settings`, `theme.rs`, `stage_editor.rs`, `titlebar.rs`, `egui`, `AppState`, `general`, `LibraryPanel`, `menu.rs`, `ViewCtx`, `i18n/mod.rs`, `DmxFrame`, `stage_canvas.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._