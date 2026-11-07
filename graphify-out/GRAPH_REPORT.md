# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~53,001 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1390 nodes · 3123 edges · 132 communities (63 shown, 69 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 127 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9178609d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .grid
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- widgets/mod.rs
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
- send_frame
- DmxCraft – UI-Richtlinien
- FixtureAction
- patch_grid.rs
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
- stage.rs
- egui
- heading_text
- MenuItem<'a>
- OpenDmx
- qxf.rs
- fixture_settings.rs
- Stroke
- Toggle<'a>
- arc
- super
- Theme
- Dialog
- settings.rs
- Fader<'a>
- apply
- Show
- app.rs
- Swatch
- swatch.rs
- en.rs
- Pos2
- scene_card.rs
- limit_pad.rs
- i18n/mod.rs
- Ui

## God Nodes (most connected - your core abstractions)
1. `Show` - 78 edges
2. `Engine` - 77 edges
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
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `color_swatches()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/color_picker.rs
- `playback_controls()` --calls--> `t()`  [INFERRED]
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

## Communities (132 total, 69 thin omitted)

### Community 0 - ".grid"
Cohesion: 0.28
Nodes (12): sidebar(), accent(), accent_dim(), bg(), border(), caption(), raised(), raised_hover() (+4 more)

### Community 1 - "Engine"
Cohesion: 0.07
Nodes (36): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values(), save_bar() (+28 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+19 more)

### Community 5 - "widgets/mod.rs"
Cohesion: 0.14
Nodes (12): Fader, FaderEvent, Changed, None, Release, LABEL_H, truncate(), VALUE_H (+4 more)

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
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.10
Nodes (43): CARD_H, Action, Bank, Delete, DuplicateScene, Jump, Mark, MoveScenes (+35 more)

### Community 11 - "dialog.rs"
Cohesion: 0.15
Nodes (5): FOOTER_H, FooterButton, HEADER_H, INPUT_H, LABEL_W

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.19
Nodes (9): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+1 more)

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
Cohesion: 0.23
Nodes (7): load_meter(), show(), body(), heading_font(), surface(), DialogResult, title()

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
Cohesion: 0.19
Nodes (11): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+3 more)

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
Cohesion: 0.16
Nodes (10): ZOOM_MAX, ZOOM_MIN, Arrange, AlignH, AlignV, Grid, Reset, Row (+2 more)

### Community 37 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (35): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+27 more)

### Community 41 - "patch_grid.rs"
Cohesion: 0.14
Nodes (12): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+4 more)

### Community 44 - "t"
Cohesion: 0.18
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (17): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+9 more)

### Community 50 - "menu.rs"
Cohesion: 0.17
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 65 - "Settings"
Cohesion: 0.21
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), Settings

### Community 66 - "t"
Cohesion: 0.24
Nodes (3): t(), ZonesView, Windows

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "ViewCtx"
Cohesion: 0.27
Nodes (5): View, ViewCtx, FixtureSettings, SetupView, StageEditorView

### Community 71 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 73 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "stage.rs"
Cohesion: 0.43
Nodes (3): section_header(), group_bar(), selection_tools()

### Community 77 - "egui"
Cohesion: 0.17
Nodes (3): LimitPad, Toggle, XyPad

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.19
Nodes (8): RangeSlider, caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "super"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 96 - "Theme"
Cohesion: 0.29
Nodes (5): active(), Theme, .ALL, Gray, Navy

### Community 97 - "Dialog"
Cohesion: 0.29
Nodes (5): Button, Dialog, Kind, Cancel, Primary

### Community 100 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), style()

### Community 101 - "Show"
Cohesion: 0.05
Nodes (42): cv(), demo_show(), empty_show(), ensure_builtin_types(), UNIVERSE_SIZE, address_map_marks_ranges(), ChannelValue, duplicate_zone_copies_scenes_behind_original() (+34 more)

### Community 102 - "app.rs"
Cohesion: 0.14
Nodes (12): ControlViews, LockDialog, OUTPUT_HZ, SettingsWindow, Tab, .ALL, General, Midi (+4 more)

### Community 103 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 111 - "scene_card.rs"
Cohesion: 0.33
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

## Knowledge Gaps
- **182 isolated node(s):** `All`, `Delete`, `OpenBanksWindow`, `New`, `Show` (+177 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 524 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **69 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Icon` connect `Icon` to `egui`, `widgets/mod.rs`, `app.rs`, `titlebar.rs`, `FixtureAction`, `patch_grid.rs`, `t`, `dialog.rs`, `color_picker.rs`, `MenuItem<'a>`, `settings_window.rs`, `scene_card.rs`, `menu.rs`, `fixture_settings.rs`, `Stroke`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._
- **Are the 52 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 52 INFERRED edges - model-reasoned connections that need verification._
- **What connects `All`, `Delete`, `OpenBanksWindow` to the rest of the system?**
  _182 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06621067031463748 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `theme.rs`, `widgets/mod.rs`, `FixtureAction`, `t`, `ChannelKind`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `runtime.rs`, `ViewCtx`, `app.rs`, `Show`, `FixtureAction`, `t`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `fixture_settings.rs`, `library_panel.rs`?**
  _High betweenness centrality (0.072) - this node is a cross-community bridge._