# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~49,572 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1348 nodes · 3041 edges · 115 communities (64 shown, 51 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 125 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a478a134`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- egui
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- Show
- steps
- DmxCraftApp
- OutputError
- AppState
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
- core/src/lib.rs
- DmxCraft – UI-Richtlinien
- FixtureAction
- file_dialog.rs
- FixtureType
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- text_dim
- settings_window.rs
- menu.rs
- Settings
- text
- DvcGoldUsb
- OpenDmx
- titlebar.rs
- convert.rs
- OutputKind
- output/src/lib.rs
- sense
- ChannelValue
- app.rs
- ShowError
- qxf.rs
- t
- DmxFrame
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
- limit_pad.rs
- i18n/mod.rs
- DmxCraftApp

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 66 edges
3. `Icon` - 64 edges
4. `t()` - 63 edges
5. `ChannelKind` - 38 edges
6. `DmxFrame` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `text_dim()` - 31 edges
10. `ViewCtx` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `run()` --calls--> `convert_qxf()`  [INFERRED]
  crates/ofl/src/bin/qlc-bundle.rs → crates/ofl/src/qxf.rs
- `run()` --calls--> `convert_fixture()`  [INFERRED]
  crates/ofl/src/bin/ofl-bundle.rs → crates/ofl/src/convert.rs

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

## Communities (115 total, 51 thin omitted)

### Community 0 - "egui"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (23): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+15 more)

### Community 2 - "runtime.rs"
Cohesion: 0.17
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (28): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+20 more)

### Community 5 - ".show"
Cohesion: 0.16
Nodes (7): accent_dim(), Fader<'a>, FaderEvent, Changed, None, Release, truncate()

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
Cohesion: 0.12
Nodes (9): AppState, Mode, Control, Setup, PatchForm, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch() (+1 more)

### Community 11 - "dialog.rs"
Cohesion: 0.09
Nodes (11): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind (+3 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

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
Cohesion: 0.18
Nodes (6): heading_font(), heading_text(), DialogResult, Form, form(), title()

### Community 25 - "ViewCtx"
Cohesion: 0.06
Nodes (27): text_faint(), Segmented<'a, T>, View, ViewCtx, caption(), FixtureSettings, Quick, quick_action() (+19 more)

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
Cohesion: 0.30
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (13): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+5 more)

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
Cohesion: 0.13
Nodes (14): section_header(), CanvasMode, Edit, Select, StageCanvas, ZOOM_MAX, ZOOM_MIN, MenuPlace (+6 more)

### Community 37 - "core/src/lib.rs"
Cohesion: 0.16
Nodes (4): FixtureLimits, Group, PLAN_SIZE_RANGE, UNIVERSES

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 44 - "FixtureType"
Cohesion: 0.21
Nodes (3): ChannelDef, FixtureType, ft()

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "text_dim"
Cohesion: 0.19
Nodes (6): accent(), bg(), caption(), text_dim(), DipSwitch, LimitPad

### Community 48 - "settings_window.rs"
Cohesion: 0.10
Nodes (27): set_lang(), main(), full_combo(), general(), HEIGHT, hint(), midi(), outputs() (+19 more)

### Community 50 - "menu.rs"
Cohesion: 0.07
Nodes (27): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+19 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 66 - "text"
Cohesion: 0.13
Nodes (12): apply(), body(), border(), install_fonts(), raised(), raised_hover(), text(), style() (+4 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 71 - "titlebar.rs"
Cohesion: 0.07
Nodes (45): delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu(), MenuAction (+37 more)

### Community 73 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "output/src/lib.rs"
Cohesion: 0.17
Nodes (6): OutputConfig, OutputStatus, Connecting, Error, Running, Stopped

### Community 77 - "sense"
Cohesion: 0.15
Nodes (8): Fader, LABEL_H, VALUE_H, RangeSlider, SceneCard, Segmented, Swatch, XyPad

### Community 78 - "ChannelValue"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 79 - "app.rs"
Cohesion: 0.31
Nodes (6): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView

### Community 80 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 86 - "qxf.rs"
Cohesion: 0.21
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "t"
Cohesion: 0.46
Nodes (3): t(), group_bar(), selection_tools()

### Community 88 - "DmxFrame"
Cohesion: 0.07
Nodes (33): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+25 more)

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
Cohesion: 0.32
Nodes (4): LoopMode, Loop, Once, Scene

### Community 100 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 117 - "limit_pad.rs"
Cohesion: 0.33
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (12): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+4 more)

## Knowledge Gaps
- **175 isolated node(s):** `Json`, `Ofl`, `Qlc`, `Enter`, `Maximize` (+170 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 501 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **51 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `text`, `runtime.rs`, `egui`, `core/src/lib.rs`, `Show`, `AppState`, `app.rs`, `settings_window.rs`, `menu.rs`, `t`, `DmxCraftApp`, `ViewCtx`, `DmxFrame`, `stage_canvas.rs`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Are the 55 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 55 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Json`, `Ofl`, `Qlc` to the rest of the system?**
  _175 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09344262295081968 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `Settings`, `text`, `egui`, `.show`, `titlebar.rs`, `FixtureAction`, `file_dialog.rs`, `AppState`, `app.rs`, `settings_window.rs`, `text_dim`, `menu.rs`, `i18n/mod.rs`, `DmxCraftApp`, `.show`, `DmxFrame`, `ViewCtx`, `stage_canvas.rs`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09047619047619047 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `IconButton`, `egui`, `titlebar.rs`, `dialog.rs`, `settings_window.rs`, `menu.rs`, `ViewCtx`, `Stroke`?**
  _High betweenness centrality (0.063) - this node is a cross-community bridge._