# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 64 files · ~32,325 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 968 nodes · 2040 edges · 98 communities (46 shown, 52 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 49 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `70a3925a`
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
- egui
- FixtureAction
- show.rs
- jobs
- steps
- DmxCraft
- core/src/lib.rs
- announce
- build-local-artifacts
- Icon
- .status_bar
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
- ViewCtx
- plan.rs
- DmxCraft – UI-Richtlinien
- ChannelValue
- scene_card.rs
- fixture_settings.rs
- DmxFrame
- Arrange
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- patch_grid.rs
- FixtureType
- swatch.rs
- .check_placement
- DvcGoldUsb
- FixtureLimits
- PlanShape
- OutputKind
- Settings
- .start
- IconButton
- ZoneId
- app.rs
- dip.rs
- dip_switch.rs
- toggle.rs
- Grip
- Toggle<'a>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 61 edges
3. `ChannelKind` - 36 edges
4. `DmxFrame` - 36 edges
5. `AppState` - 31 edges
6. `ViewCtx` - 30 edges
7. `DmxCraftApp` - 26 edges
8. `FixtureType` - 26 edges
9. `MenuAction` - 26 edges
10. `FixtureAction` - 23 edges

## Surprising Connections (you probably didn't know these)
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `setup()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/library.rs
- `defaults_are_rendered()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/library.rs
- `limits_cap_dimmer_and_position()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/library.rs
- `show()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/show.rs → crates/core/src/library.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (98 total, 52 thin omitted)

### Community 1 - "Engine"
Cohesion: 0.06
Nodes (42): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+34 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (8): OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (31): ACCENT, ACCENT_DIM, apply(), BG, body(), BORDER, DANGER, DIP_BODY (+23 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.18
Nodes (17): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+9 more)

### Community 6 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.09
Nodes (14): ICON_PNG, texture(), window_icon(), APP_NAME, Updater, UpdateState, Available, Checking (+6 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.12
Nodes (7): AppState, PatchForm, group_bar(), selection_tools(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 11 - "egui"
Cohesion: 0.15
Nodes (11): section_header(), CanvasMode, Edit, Select, apply(), MenuPlace, Patch, Stage (+3 more)

### Community 12 - "FixtureAction"
Cohesion: 0.10
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 13 - "show.rs"
Cohesion: 0.30
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DmxCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "core/src/lib.rs"
Cohesion: 0.17
Nodes (7): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.14
Nodes (14): Icon, Center, Close, LightOff, LightOn, Maximize, Menu, Minimize (+6 more)

### Community 24 - "fader.rs"
Cohesion: 0.13
Nodes (10): caption(), Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H (+2 more)

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
Cohesion: 0.12
Nodes (20): main_menu(), MenuAction, About, CheckUpdates, ClearProgrammer, DemoShow, NewShow, OpenShow (+12 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.27
Nodes (8): BOX, fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), StageCanvas, StageInteraction

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "titlebar.rs"
Cohesion: 0.19
Nodes (7): Mode, Control, Setup, HEIGHT, resize_handles(), show(), TitleInfo

### Community 36 - "ViewCtx"
Cohesion: 0.08
Nodes (24): View, ViewCtx, FixtureSettings, SetupView, clip(), Drag, GridMode, Output (+16 more)

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 41 - "fixture_settings.rs"
Cohesion: 0.23
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 44 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "patch_grid.rs"
Cohesion: 0.14
Nodes (8): LimitPad, RangeSlider, Segmented, XyPad, CELL_H_MAX, CELL_H_MIN, COLUMNS, ROWS

### Community 65 - "FixtureType"
Cohesion: 0.23
Nodes (4): ChannelDef, FixtureType, builtin_fixture_types(), ft()

### Community 67 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 71 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

### Community 77 - "app.rs"
Cohesion: 0.24
Nodes (8): ControlViews, OUTPUT_HZ, PathDialog, Open, SaveAs, ProgrammerView, SceneGridView, StageView

### Community 78 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 90 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **121 isolated node(s):** `Open`, `SaveAs`, `Play`, `Pause`, `Prev` (+116 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 360 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **52 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `ViewCtx`, `scene_grid.rs`, `fixture_settings.rs`, `AppState`, `egui`, `.start`, `app.rs`, `core/src/lib.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.130) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Play` to the rest of the system?**
  _121 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06414565826330532 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `FixtureType`, `.check_placement`, `theme.rs`, `runtime.rs`, `ChannelValue`, `egui`, `FixtureAction`, `app.rs`, `show.rs`, `ZoneId`, `ShowError`, `core/src/lib.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `ChannelKind` connect `ChannelKind` to `FixtureType`, `Engine`, `convert.rs`, `fixture_settings.rs`, `core/src/lib.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.061) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06892230576441102 - nodes in this community are weakly interconnected._