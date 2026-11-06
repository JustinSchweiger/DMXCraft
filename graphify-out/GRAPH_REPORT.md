# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 58 files · ~26,257 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 839 nodes · 1788 edges · 64 communities (36 shown, 28 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 47 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b9ae2a9c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Show
- Engine
- swatch.rs
- convert.rs
- theme.rs
- scene_grid.rs
- ChannelKind
- steps
- DmxCraftApp
- runtime.rs
- ChannelKind
- fixture_menu.rs
- FixtureAction
- .show
- jobs
- steps
- views/library.rs
- DmxCraft
- dip.rs
- announce
- build-local-artifacts
- icons.rs
- fader.rs
- scenes.rs
- host
- plan
- with
- stage_canvas.rs
- env
- dmxcraft
- AppState
- ViewCtx
- plan.rs
- DmxCraft – UI-Richtlinien
- scene_card.rs
- monitor.rs
- stage.rs
- Arrange
- DmxCraft – Hinweise für KI-Assistenten
- fader.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 58 edges
3. `DmxFrame` - 36 edges
4. `ChannelKind` - 34 edges
5. `AppState` - 32 edges
6. `ViewCtx` - 26 edges
7. `convert_fixture()` - 22 edges
8. `FixtureAction` - 20 edges
9. `FixtureType` - 18 edges
10. `DmxCraftApp` - 18 edges

## Surprising Connections (you probably didn't know these)
- `live_color()` --references--> `val`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → .github/workflows/release.yml
- `SetupView` --references--> `MonitorView`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/monitor.rs
- `show()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/show.rs → crates/core/src/library.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs
- `ensure_builtin_types()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/app/src/demo.rs → crates/core/src/library.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (64 total, 28 thin omitted)

### Community 0 - "Show"
Cohesion: 0.06
Nodes (35): cv(), demo_show(), empty_show(), ensure_builtin_types(), FixtureType, address_map_marks_ranges(), ChannelValue, Fixture (+27 more)

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (22): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), MAX_STEP_ADVANCES_PER_TICK (+14 more)

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): ACCENT, ACCENT_DIM, BG, BORDER, DANGER, DIP_BODY, FIXTURE_BAR, FS_BODY (+19 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.17
Nodes (18): programmer_values(), Action, Delete, Edit, Jump, NewScene, Pause, Toggle (+10 more)

### Community 6 - "ChannelKind"
Cohesion: 0.10
Nodes (22): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+14 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.05
Nodes (25): ControlViews, DmxCraftApp, file_menu(), OUTPUT_HZ, update_widget(), ICON_PNG, texture(), window_icon() (+17 more)

### Community 9 - "runtime.rs"
Cohesion: 0.06
Nodes (20): NullOutput, RecordingOutput, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputError, NotFound (+12 more)

### Community 10 - "ChannelKind"
Cohesion: 0.09
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 11 - "fixture_menu.rs"
Cohesion: 0.18
Nodes (10): fixture_pos(), apply(), AddToGroup, RemoveFromGroup, menu(), MenuPlace, Patch, Stage (+2 more)

### Community 12 - "FixtureAction"
Cohesion: 0.12
Nodes (15): FixtureAction, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup, NextFreeAddress (+7 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 16 - "views/library.rs"
Cohesion: 0.23
Nodes (4): import_file(), LibraryWindow, MAX_RESULTS, take_type()

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DmxCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "icons.rs"
Cohesion: 0.14
Nodes (7): Icon, Next, Pause, Play, Plus, Prev, IconButton

### Community 24 - "fader.rs"
Cohesion: 0.15
Nodes (7): Fader, Fader<'a>, FaderEvent, Changed, None, Release, truncate()

### Community 25 - "scenes.rs"
Cohesion: 0.25
Nodes (10): apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete, Down, Load (+2 more)

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
Nodes (10): Segmented, BOX, CanvasMode, Edit, Select, live_color(), MenuFn, StageCanvas (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "AppState"
Cohesion: 0.12
Nodes (12): AppState, Mode, Control, Setup, PatchForm, SetupTab, Monitor, Patch (+4 more)

### Community 36 - "ViewCtx"
Cohesion: 0.14
Nodes (9): View, ViewCtx, SetupView, add_fixtures(), patch_ui(), PatchView, ScenesView, StageEditorView (+1 more)

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 41 - "monitor.rs"
Cohesion: 0.16
Nodes (13): body(), caption(), section_header(), CELL_H, clip(), COLUMNS, details(), Drag (+5 more)

### Community 44 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "fader.rs"
Cohesion: 0.18
Nodes (4): DipSwitch, LABEL_H, VALUE_H, XyPad

## Knowledge Gaps
- **87 isolated node(s):** `Setup`, `Control`, `Patch`, `Stage`, `Scenes` (+82 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 296 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **28 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `theme.rs`, `DmxCraftApp`, `runtime.rs`, `fixture_menu.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.198) - this node is a cross-community bridge._
- **What connects `Setup`, `Control`, `Patch` to the rest of the system?**
  _87 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05581395348837209 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Show`, `ViewCtx`, `scene_grid.rs`, `ChannelKind`, `DmxCraftApp`, `monitor.rs`, `runtime.rs`, `stage.rs`, `fixture_menu.rs`, `views/library.rs`, `scenes.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.189) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09739866908650938 - nodes in this community are weakly interconnected._
- **Why does `live_color()` connect `stage_canvas.rs` to `Show`, `host`, `ChannelKind`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07477288609364081 - nodes in this community are weakly interconnected._