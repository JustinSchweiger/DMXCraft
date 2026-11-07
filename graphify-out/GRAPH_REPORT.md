# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~51,303 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1377 nodes · 3092 edges · 125 communities (66 shown, 59 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 129 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `391bbf81`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- text_dim
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- Show
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- ofl/src/lib.rs
- send_frame
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- .show
- ViewCtx
- host
- plan
- with
- ensure_linux_menu_entry
- show.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- egui
- FixtureLimits
- DmxCraft – UI-Richtlinien
- FixtureAction
- patch_grid.rs
- FixtureType
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- accent
- general
- MenuItem<'a>
- app.rs
- show
- DvcGoldUsb
- OpenDmx
- titlebar.rs
- convert.rs
- OutputKind
- output/src/lib.rs
- sense
- ChannelValue
- settings_window.rs
- ShowError
- qxf.rs
- Category
- scenes.rs
- Stroke
- Toggle<'a>
- Arrange
- super
- Theme
- IconButton
- Scene
- .check_placement
- ZoneId
- arc
- en.rs
- fixture_settings.rs
- Dialog<'a>
- Tab
- scene_card.rs
- swatch.rs
- form
- .show
- limit_pad.rs
- Source
- i18n/mod.rs
- text_field

## God Nodes (most connected - your core abstractions)
1. `Engine` - 81 edges
2. `Show` - 73 edges
3. `t()` - 72 edges
4. `Icon` - 64 edges
5. `ChannelKind` - 38 edges
6. `DmxFrame` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 30 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/views/control/scene_grid.rs
- `save_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `save_bar()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/dialog.rs

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

## Communities (125 total, 59 thin omitted)

### Community 0 - "text_dim"
Cohesion: 0.25
Nodes (6): caption(), fixture_bar(), text_dim(), text_faint(), caption(), range_row()

### Community 1 - "Engine"
Cohesion: 0.06
Nodes (36): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), save_bar(), set_group() (+28 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (26): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_off(), FS_BODY, FS_CAPTION (+18 more)

### Community 5 - ".show"
Cohesion: 0.18
Nodes (6): Fader<'a>, FaderEvent, Changed, None, Release, truncate()

### Community 6 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.05
Nodes (54): t(), AppState, Mode, Control, Setup, PatchForm, Action, Bank (+46 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+2 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+1 more)

### Community 13 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

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

### Community 24 - ".show"
Cohesion: 0.22
Nodes (7): sidebar(), body(), heading_font(), heading_text(), surface(), text(), DialogResult

### Community 25 - "ViewCtx"
Cohesion: 0.17
Nodes (8): View, ViewCtx, FixtureSettings, SetupView, ScenesView, StageEditorView, ZonesView, Windows

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.16
Nodes (13): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "egui"
Cohesion: 0.13
Nodes (14): section_header(), CanvasMode, Edit, Select, StageCanvas, StageInteraction, ZOOM_MAX, ZOOM_MIN (+6 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "patch_grid.rs"
Cohesion: 0.20
Nodes (11): Segmented, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output (+3 more)

### Community 44 - "FixtureType"
Cohesion: 0.19
Nodes (3): ChannelDef, FixtureType, ft()

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.18
Nodes (13): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+5 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "accent"
Cohesion: 0.35
Nodes (4): accent(), bg(), border(), DipSwitch

### Community 48 - "general"
Cohesion: 0.24
Nodes (11): set_lang(), main(), full_combo(), general(), hint(), midi(), outputs(), project() (+3 more)

### Community 65 - "app.rs"
Cohesion: 0.05
Nodes (22): ControlViews, DmxCraftApp, LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs (+14 more)

### Community 66 - "show"
Cohesion: 0.24
Nodes (7): load_meter(), show(), accent_dim(), apply(), install_fonts(), raised(), raised_hover()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 71 - "titlebar.rs"
Cohesion: 0.05
Nodes (56): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+48 more)

### Community 73 - "convert.rs"
Cohesion: 0.14
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "output/src/lib.rs"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 77 - "sense"
Cohesion: 0.16
Nodes (7): Fader, LABEL_H, VALUE_H, LimitPad, RangeSlider, Toggle, XyPad

### Community 78 - "ChannelValue"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 79 - "settings_window.rs"
Cohesion: 0.20
Nodes (7): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, UsbDmxPort

### Community 80 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 86 - "qxf.rs"
Cohesion: 0.35
Nodes (6): channel_def(), child(), kind_from_group(), kind_from_preset(), SAMPLE, text()

### Community 87 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 88 - "scenes.rs"
Cohesion: 0.22
Nodes (11): programmer_values(), apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete, Down (+3 more)

### Community 93 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 94 - "super"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 96 - "Theme"
Cohesion: 0.29
Nodes (5): active(), Theme, .ALL, Gray, Navy

### Community 98 - "Scene"
Cohesion: 0.21
Nodes (6): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, UNIVERSES

### Community 100 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 108 - "fixture_settings.rs"
Cohesion: 0.24
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 110 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 117 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 118 - "Source"
Cohesion: 0.50
Nodes (3): Source, Ofl, Qlc

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

## Knowledge Gaps
- **181 isolated node(s):** `Setup`, `Control`, `Dimmer`, `Color`, `Position` (+176 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 509 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **59 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `text_dim`, `app.rs`, `show`, `Engine`, `egui`, `.show`, `titlebar.rs`, `FixtureAction`, `sense`, `general`, `i18n/mod.rs`, `.show`, `ViewCtx`, `scenes.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **Are the 58 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 58 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Dimmer` to the rest of the system?**
  _181 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06347962382445141 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `Scene`, `theme.rs`, `egui`, `ZoneId`, `.check_placement`, `FixtureAction`, `t`, `FixtureType`, `ChannelValue`, `ShowError`, `show.rs`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14130434782608695 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `app.rs`, `Scene`, `runtime.rs`, `egui`, `Show`, `t`, `fixture_settings.rs`, `settings_window.rs`, `general`, `scenes.rs`, `ViewCtx`, `stage_canvas.rs`?**
  _High betweenness centrality (0.083) - this node is a cross-community bridge._