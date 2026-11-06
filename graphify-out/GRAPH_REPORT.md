# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 55 files · ~23,341 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 782 nodes · 1647 edges · 66 communities (37 shown, 29 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 40 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4dedad50`
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
- OutputError
- app.rs
- section_header
- DmxFrame
- jobs
- steps
- views/library.rs
- DmxCraft
- DmxOutput
- announce
- build-local-artifacts
- icons.rs
- DvcGoldUsb
- fader.rs
- scenes.rs
- host
- plan
- with
- egui
- stage_canvas.rs
- env
- dmxcraft
- AppState
- ViewCtx
- plan.rs
- DmxCraft – UI-Richtlinien
- SetupTab
- scene_card.rs
- monitor.rs
- patch.rs
- stage.rs
- Arrange
- .start
- DmxCraft – Hinweise für KI-Assistenten
- xy_pad.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 78 edges
2. `Show` - 54 edges
3. `DmxFrame` - 38 edges
4. `ChannelKind` - 34 edges
5. `AppState` - 28 edges
6. `ViewCtx` - 25 edges
7. `convert_fixture()` - 22 edges
8. `DmxCraftApp` - 18 edges
9. `FixtureType` - 17 edges
10. `ChannelRef` - 17 edges

## Surprising Connections (you probably didn't know these)
- `live_color()` --references--> `val`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → .github/workflows/release.yml
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `scene_header()` --calls--> `zone_combo()`  [INFERRED]
  crates/app/src/views/setup/scenes.rs → crates/app/src/views/control/programmer.rs
- `import_file()` --calls--> `convert_fixture()`  [INFERRED]
  crates/app/src/views/library.rs → crates/ofl/src/convert.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (66 total, 29 thin omitted)

### Community 0 - "Show"
Cohesion: 0.06
Nodes (36): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelDef, FixtureType, ft(), ChannelValue (+28 more)

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (22): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), MAX_STEP_ADVANCES_PER_TICK (+14 more)

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (28): ACCENT, ACCENT_DIM, BG, body(), BORDER, caption(), DANGER, FS_BODY (+20 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.19
Nodes (16): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+8 more)

### Community 6 - "ChannelKind"
Cohesion: 0.07
Nodes (37): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+29 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.07
Nodes (19): DmxCraftApp, file_menu(), update_widget(), apply(), APP_NAME, Updater, UpdateState, Available (+11 more)

### Community 9 - "runtime.rs"
Cohesion: 0.13
Nodes (8): OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, Runtime, runtime_sends_rendered_frames()

### Community 10 - "OutputError"
Cohesion: 0.18
Nodes (6): NullOutput, RecordingOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 11 - "app.rs"
Cohesion: 0.36
Nodes (8): ControlViews, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView, View, SetupView, ScenesView

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 16 - "views/library.rs"
Cohesion: 0.26
Nodes (4): import_file(), LibraryWindow, MAX_RESULTS, take_type()

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DmxCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "DmxOutput"
Cohesion: 0.22
Nodes (3): DVC_GOLD_UNIVERSES, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "icons.rs"
Cohesion: 0.13
Nodes (7): Icon, Next, Pause, Play, Plus, Prev, IconButton

### Community 24 - "fader.rs"
Cohesion: 0.14
Nodes (9): Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H, truncate() (+1 more)

### Community 25 - "scenes.rs"
Cohesion: 0.24
Nodes (11): programmer_values(), apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete, Down (+3 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "egui"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 31 - "stage_canvas.rs"
Cohesion: 0.22
Nodes (7): BOX, CanvasMode, Edit, Select, fixture_pos(), live_color(), StageCanvas

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "AppState"
Cohesion: 0.21
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 36 - "ViewCtx"
Cohesion: 0.21
Nodes (4): StageInteraction, ViewCtx, set_pos(), StageEditorView

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "SetupTab"
Cohesion: 0.25
Nodes (8): SetupTab, Monitor, Patch, Scenes, Stage, Zones, Segmented, TABS

### Community 41 - "monitor.rs"
Cohesion: 0.38
Nodes (4): COLUMNS, monitor_ui(), MonitorView, UNIVERSE_SIZE

### Community 42 - "patch.rs"
Cohesion: 0.48
Nodes (3): add_fixtures(), patch_ui(), PatchView

### Community 44 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

## Knowledge Gaps
- **68 isolated node(s):** `Setup`, `Control`, `Patch`, `Stage`, `Scenes` (+63 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 268 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **29 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `theme.rs`, `runtime.rs`, `app.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.193) - this node is a cross-community bridge._
- **What connects `Setup`, `Control`, `Patch` to the rest of the system?**
  _68 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05871725383920506 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Show`, `ViewCtx`, `scene_grid.rs`, `ChannelKind`, `DmxCraftApp`, `monitor.rs`, `patch.rs`, `app.rs`, `stage.rs`, `runtime.rs`, `.start`, `views/library.rs`, `scenes.rs`?**
  _High betweenness centrality (0.184) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09265536723163842 - nodes in this community are weakly interconnected._
- **Why does `live_color()` connect `stage_canvas.rs` to `Show`, `host`, `DmxFrame`?**
  _High betweenness centrality (0.142) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07142857142857142 - nodes in this community are weakly interconnected._