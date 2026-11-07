# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~52,268 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1384 nodes · 3094 edges · 127 communities (65 shown, 62 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 126 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f8772a48`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- text_dim
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- MenuAction
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
- PlanShape
- egui
- send_frame
- DmxCraft – UI-Richtlinien
- FixtureAction
- patch_grid.rs
- AppState
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Settings
- egui
- DvcGoldUsb
- t
- titlebar.rs
- convert.rs
- OutputKind
- file_dialog.rs
- sense
- show
- MenuItem<'a>
- OpenDmx
- qxf.rs
- fixture_settings.rs
- Stroke
- Toggle<'a>
- arc
- super
- Theme
- widgets/mod.rs
- OutputStatus
- T
- Show
- app.rs
- .ui
- text_field
- en.rs
- set_lang
- resize_handles
- Tab
- scene_card.rs
- limit_pad.rs
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 77 edges
2. `Show` - 75 edges
3. `Icon` - 67 edges
4. `t()` - 66 edges
5. `DmxFrame` - 38 edges
6. `ChannelKind` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 31 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `ensure_builtin_types()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/app/src/demo.rs → crates/core/src/library.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `save_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs

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

## Communities (127 total, 62 thin omitted)

### Community 0 - "text_dim"
Cohesion: 0.27
Nodes (8): accent(), bg(), caption(), text_dim(), text_faint(), DipSwitch, caption(), range_row()

### Community 1 - "Engine"
Cohesion: 0.05
Nodes (46): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+38 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (33): accent_dim(), accent_hover(), ACTIVE, apply(), border(), c(), DANGER, DIP_BODY (+25 more)

### Community 5 - ".show"
Cohesion: 0.15
Nodes (9): Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H, truncate() (+1 more)

### Community 6 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

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
Cohesion: 0.11
Nodes (38): Action, Bank, Delete, DuplicateScene, Jump, Mark, NewScene, OpenBanksWindow (+30 more)

### Community 11 - "dialog.rs"
Cohesion: 0.16
Nodes (7): FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H, LABEL_W

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.15
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
Nodes (49): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+41 more)

### Community 23 - "ChannelKind"
Cohesion: 0.08
Nodes (21): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+13 more)

### Community 24 - ".show"
Cohesion: 0.21
Nodes (7): body(), heading_font(), heading_text(), surface(), text(), DialogResult, title()

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

### Community 30 - "Dialog<'a>"
Cohesion: 0.15
Nodes (6): Button, Dialog, Dialog<'a>, Kind, Cancel, Primary

### Community 31 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (11): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+3 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "egui"
Cohesion: 0.16
Nodes (10): ZOOM_MAX, ZOOM_MIN, Arrange, AlignH, AlignV, Grid, Reset, Row (+2 more)

### Community 37 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (25): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+17 more)

### Community 41 - "patch_grid.rs"
Cohesion: 0.15
Nodes (13): fixture_bar(), raised_hover(), apply(), CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag (+5 more)

### Community 44 - "AppState"
Cohesion: 0.20
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.11
Nodes (11): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, git(), main(), run(), dedupe_keys(), git() (+3 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "menu.rs"
Cohesion: 0.21
Nodes (10): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+2 more)

### Community 66 - "egui"
Cohesion: 0.20
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "t"
Cohesion: 0.20
Nodes (6): t(), section_header(), group_bar(), selection_tools(), ZonesView, Windows

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 73 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 77 - "sense"
Cohesion: 0.13
Nodes (5): LimitPad, RangeSlider, Segmented, Swatch, XyPad

### Community 78 - "show"
Cohesion: 0.20
Nodes (9): load_meter(), show(), TitleInfo, UpdateBadge, Available, Busy, Error, Installed (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.24
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "super"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 96 - "Theme"
Cohesion: 0.29
Nodes (5): active(), Theme, .ALL, Gray, Navy

### Community 97 - "widgets/mod.rs"
Cohesion: 0.16
Nodes (6): IconButton, CanvasMode, Edit, Select, ShapeButton, StageCanvas

### Community 98 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 101 - "Show"
Cohesion: 0.05
Nodes (41): cv(), demo_show(), empty_show(), ensure_builtin_types(), UNIVERSE_SIZE, address_map_marks_ranges(), ChannelValue, duplicate_zone_copies_scenes_behind_original() (+33 more)

### Community 102 - "app.rs"
Cohesion: 0.22
Nodes (9): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, StageView, View, FixtureSettings, SetupView (+1 more)

### Community 108 - "set_lang"
Cohesion: 0.50
Nodes (3): set_lang(), main(), set_theme()

### Community 110 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

## Knowledge Gaps
- **182 isolated node(s):** `Setup`, `Control`, `Play`, `Pause`, `Prev` (+177 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 522 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **62 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `runtime.rs`, `t`, `app.rs`, `Show`, `FixtureAction`, `patch_grid.rs`, `t`, `DmxCraftApp`, `settings_window.rs`, `fixture_settings.rs`, `library_panel.rs`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **Are the 52 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 52 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Play` to the rest of the system?**
  _182 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.0516404581634634 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `text_dim`, `Engine`, `.show`, `t`, `color_picker.rs`, `.show`, `library_panel.rs`, `stage_canvas.rs`, `egui`, `FixtureAction`, `patch_grid.rs`, `AppState`, `DmxCraftApp`, `settings_window.rs`, `Settings`, `titlebar.rs`, `file_dialog.rs`, `show`, `app.rs`, `text_field`, `i18n/mod.rs`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08478513356562137 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `widgets/mod.rs`, `egui`, `titlebar.rs`, `FixtureAction`, `patch_grid.rs`, `t`, `dialog.rs`, `color_picker.rs`, `Tab`, `scene_card.rs`, `settings_window.rs`, `menu.rs`, `fixture_settings.rs`, `Stroke`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._