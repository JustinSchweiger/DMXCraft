# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 60 files · ~29,211 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 915 nodes · 1943 edges · 77 communities (47 shown, 30 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 46 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `386cf912`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Show
- Engine
- runtime.rs
- convert.rs
- theme.rs
- scene_grid.rs
- Engine
- steps
- DmxCraftApp
- OutputError
- ChannelKind
- egui
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
- DmxCraftApp
- host
- plan
- with
- MenuAction
- stage_canvas.rs
- env
- dmxcraft
- titlebar.rs
- app.rs
- plan.rs
- DmxCraft – UI-Richtlinien
- ofl/src/lib.rs
- scene_card.rs
- monitor.rs
- DmxFrame
- Arrange
- ofl-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- fader.rs
- Value
- stage_editor.rs
- FixtureType
- DvcGoldUsb
- Category
- PlanShape
- OutputKind
- Settings
- .start

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 61 edges
3. `DmxFrame` - 36 edges
4. `ChannelKind` - 34 edges
5. `AppState` - 31 edges
6. `ViewCtx` - 27 edges
7. `DmxCraftApp` - 26 edges
8. `MenuAction` - 24 edges
9. `FixtureAction` - 23 edges
10. `convert_fixture()` - 22 edges

## Surprising Connections (you probably didn't know these)
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `show()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/show.rs → crates/core/src/library.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs
- `SetupView` --references--> `MonitorView`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/monitor.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (77 total, 30 thin omitted)

### Community 0 - "Show"
Cohesion: 0.06
Nodes (36): cv(), demo_show(), empty_show(), ensure_builtin_types(), address_map_marks_ranges(), ChannelValue, Fixture, Group (+28 more)

### Community 1 - "Engine"
Cohesion: 0.17
Nodes (18): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), MAX_STEP_ADVANCES_PER_TICK, MS, once_mode_stops_on_last_step() (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (8): OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.19
Nodes (14): convert_fixture(), default_value(), kinds(), Matrix, matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), natural_cmp() (+6 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (28): ACCENT, ACCENT_DIM, apply(), BG, BORDER, DANGER, DIP_BODY, FIXTURE_BAR (+20 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.18
Nodes (18): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+10 more)

### Community 6 - "Engine"
Cohesion: 0.06
Nodes (35): AppState, Mode, Control, Setup, PatchForm, color_bar(), current(), fader() (+27 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.09
Nodes (14): ICON_PNG, texture(), window_icon(), APP_NAME, Updater, UpdateState, Available, Checking (+6 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 11 - "egui"
Cohesion: 0.24
Nodes (5): MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 12 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

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
Cohesion: 0.09
Nodes (12): Icon, Close, Maximize, Menu, Minimize, Next, Pause, Play (+4 more)

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
Cohesion: 0.13
Nodes (19): main_menu(), MenuAction, About, CheckUpdates, ClearProgrammer, DemoShow, NewShow, OpenShow (+11 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.28
Nodes (7): BOX, fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), StageInteraction

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "titlebar.rs"
Cohesion: 0.14
Nodes (11): SetupTab, Monitor, Patch, Scenes, Stage, Zones, HEIGHT, resize_handles() (+3 more)

### Community 36 - "app.rs"
Cohesion: 0.15
Nodes (13): ControlViews, OUTPUT_HZ, PathDialog, Open, SaveAs, ProgrammerView, StageView, View (+5 more)

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 41 - "monitor.rs"
Cohesion: 0.14
Nodes (14): body(), caption(), section_header(), apply(), CELL_H, clip(), COLUMNS, details() (+6 more)

### Community 43 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 44 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 45 - "ofl-bundle.rs"
Cohesion: 0.21
Nodes (3): git(), main(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "fader.rs"
Cohesion: 0.16
Nodes (3): DipSwitch, Swatch, XyPad

### Community 65 - "Value"
Cohesion: 0.27
Nodes (7): capabilities(), channel_kind(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 66 - "stage_editor.rs"
Cohesion: 0.27
Nodes (6): CanvasMode, Edit, Select, StageCanvas, set_pos(), shape_and_size()

### Community 67 - "FixtureType"
Cohesion: 0.24
Nodes (3): ChannelDef, FixtureType, ft()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 70 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, MovingHead, Round, Square

### Community 71 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

## Knowledge Gaps
- **113 isolated node(s):** `Open`, `SaveAs`, `NewShow`, `DemoShow`, `OpenShow` (+108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 330 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **30 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Show`, `Engine`, `runtime.rs`, `app.rs`, `scene_grid.rs`, `monitor.rs`, `.start`, `egui`, `views/library.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.169) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `NewShow` to the rest of the system?**
  _113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05533279871692061 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `stage_editor.rs`, `FixtureType`, `app.rs`, `theme.rs`, `Engine`, `runtime.rs`, `egui`, `FixtureAction`, `stage_canvas.rs`?**
  _High betweenness centrality (0.129) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `ChannelKind` connect `ChannelKind` to `Show`, `Value`, `FixtureType`, `convert.rs`, `Category`, `Engine`, `ofl-bundle.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.059) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09269162210338681 - nodes in this community are weakly interconnected._