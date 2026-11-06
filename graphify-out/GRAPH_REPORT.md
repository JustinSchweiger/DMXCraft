# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 66 files · ~36,516 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1057 nodes · 2226 edges · 92 communities (43 shown, 49 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 58 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8c1ae7e9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Show
- Engine
- runtime.rs
- convert.rs
- theme.rs
- scene_grid.rs
- ChannelKind
- steps
- DmxCraftApp
- OutputError
- AppState
- dialog.rs
- FixtureAction
- jobs
- steps
- DmxCraft
- fixture_menu.rs
- announce
- build-local-artifacts
- Icon
- .show
- fader.rs
- DmxCraftApp
- host
- plan
- with
- MenuAction
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- app.rs
- plan.rs
- DmxCraft – UI-Richtlinien
- form
- arc
- ViewCtx
- DmxFrame
- OutputKind
- caption
- DmxCraft – Hinweise für KI-Assistenten
- patch_grid.rs
- text_field
- swatch.rs
- DvcGoldUsb
- OutputStatus
- titlebar.rs
- Settings
- toggle.rs
- IconButton
- Arrange
- Grip
- Failing
- dip_switch.rs
- Toggle<'a>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 61 edges
3. `DmxFrame` - 36 edges
4. `ChannelKind` - 34 edges
5. `AppState` - 32 edges
6. `Icon` - 31 edges
7. `ViewCtx` - 30 edges
8. `DmxCraftApp` - 29 edges
9. `MenuAction` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `save_bar()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/dialog.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `scene_header()` --calls--> `zone_combo()`  [INFERRED]
  crates/app/src/views/setup/scenes.rs → crates/app/src/views/control/programmer.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (92 total, 49 thin omitted)

### Community 0 - "Show"
Cohesion: 0.05
Nodes (41): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelDef, FixtureType, ft(), address_map_marks_ranges() (+33 more)

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (23): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+15 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (37): ACCENT, ACCENT_DIM, ACCENT_HOVER, apply(), BG, body(), BORDER, caption() (+29 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.18
Nodes (17): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+9 more)

### Community 6 - "ChannelKind"
Cohesion: 0.06
Nodes (37): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+29 more)

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
Cohesion: 0.10
Nodes (10): AppState, Mode, Control, Setup, PatchForm, group_bar(), selection_tools(), LibraryPanel (+2 more)

### Community 11 - "dialog.rs"
Cohesion: 0.14
Nodes (9): Button, Dialog, FOOTER_H, HEADER_H, INPUT_H, Kind, Cancel, Primary (+1 more)

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
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DmxCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "fixture_menu.rs"
Cohesion: 0.17
Nodes (10): section_header(), fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES (+2 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.10
Nodes (21): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Center, Close, LightOff (+13 more)

### Community 24 - "fader.rs"
Cohesion: 0.15
Nodes (9): Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H, truncate() (+1 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "MenuAction"
Cohesion: 0.10
Nodes (22): main_menu(), MenuAction, About, CheckUpdates, ClearProgrammer, DemoShow, InstallUpdate, NewShow (+14 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (13): BOX, CanvasMode, Edit, Select, fixture_rect(), live_colors(), MenuFn, paint_fixture() (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "app.rs"
Cohesion: 0.20
Nodes (9): ControlViews, LockDialog, OUTPUT_HZ, PathDialog, Open, SaveAs, ProgrammerView, SceneGridView (+1 more)

### Community 37 - "plan.rs"
Cohesion: 0.10
Nodes (22): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+14 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "form"
Cohesion: 0.31
Nodes (3): heading_font(), Form, form()

### Community 40 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 41 - "ViewCtx"
Cohesion: 0.05
Nodes (32): heading_text(), programmer_values(), View, ViewCtx, caption(), FixtureSettings, Quick, quick_action() (+24 more)

### Community 44 - "OutputKind"
Cohesion: 0.29
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "patch_grid.rs"
Cohesion: 0.14
Nodes (8): LimitPad, RangeSlider, Segmented, XyPad, CELL_H_MAX, CELL_H_MIN, COLUMNS, ROWS

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.16
Nodes (11): HEIGHT, load_meter(), resize_handles(), show(), TitleInfo, UpdateBadge, Available, Busy (+3 more)

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 79 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **137 isolated node(s):** `Open`, `SaveAs`, `Cancel`, `Primary`, `Dimmer` (+132 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 399 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **49 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `app.rs`, `theme.rs`, `FixtureAction`, `fixture_menu.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Cancel` to the rest of the system?**
  _137 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05284831846259437 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `app.rs`, `scene_grid.rs`, `ChannelKind`, `ViewCtx`, `AppState`, `fixture_menu.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.119) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09095716552088842 - nodes in this community are weakly interconnected._
- **Why does `FixtureType` connect `Show` to `Engine`, `PlanShape`, `convert.rs`, `ChannelKind`, `AppState`?**
  _High betweenness centrality (0.045) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06662770309760374 - nodes in this community are weakly interconnected._