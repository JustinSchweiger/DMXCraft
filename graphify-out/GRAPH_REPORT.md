# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 61 files · ~30,390 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 931 nodes · 1977 edges · 88 communities (46 shown, 42 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 49 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1c93e1a1`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Show
- Engine
- runtime.rs
- convert.rs
- theme.rs
- scene_grid.rs
- programmer.rs
- steps
- DmxCraftApp
- OutputError
- ChannelKind
- egui
- FixtureAction
- show.rs
- jobs
- steps
- DmxCraft
- core/src/lib.rs
- announce
- build-local-artifacts
- icons.rs
- .ui
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
- ChannelValue
- scene_card.rs
- patch_grid.rs
- DmxFrame
- Arrange
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- fader.rs
- arc
- swatch.rs
- .check_placement
- DvcGoldUsb
- FixtureLimits
- PlanShape
- OutputKind
- Settings
- .start
- ZoneId
- PathDialog

## God Nodes (most connected - your core abstractions)
1. `Engine` - 74 edges
2. `Show` - 61 edges
3. `ChannelKind` - 36 edges
4. `DmxFrame` - 36 edges
5. `AppState` - 31 edges
6. `ViewCtx` - 30 edges
7. `DmxCraftApp` - 26 edges
8. `MenuAction` - 26 edges
9. `FixtureType` - 26 edges
10. `FixtureAction` - 23 edges

## Surprising Connections (you probably didn't know these)
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `SetupView` --references--> `StageEditorView`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/stage_editor.rs
- `setup()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/library.rs
- `defaults_are_rendered()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/library.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (88 total, 42 thin omitted)

### Community 0 - "Show"
Cohesion: 0.20
Nodes (3): Fixture, Group, Show

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (22): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+14 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (8): OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (30): ACCENT, ACCENT_DIM, BG, body(), BORDER, DANGER, DIP_BODY, FIXTURE_BAR (+22 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.18
Nodes (17): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+9 more)

### Community 6 - "programmer.rs"
Cohesion: 0.10
Nodes (30): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+22 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "ChannelKind"
Cohesion: 0.06
Nodes (27): AppState, PatchForm, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), ChannelDef, ChannelKind, .ALL (+19 more)

### Community 11 - "egui"
Cohesion: 0.14
Nodes (13): section_header(), CanvasMode, Edit, Select, fixture_pos(), apply(), MenuPlace, Patch (+5 more)

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
Cohesion: 0.20
Nodes (4): LoopMode, Loop, Once, Scene

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "icons.rs"
Cohesion: 0.09
Nodes (12): Icon, Close, Maximize, Menu, Minimize, Next, Pause, Play (+4 more)

### Community 23 - ".ui"
Cohesion: 0.28
Nodes (3): update_widget(), resize_handles(), apply()

### Community 24 - "fader.rs"
Cohesion: 0.14
Nodes (8): Fader, Fader<'a>, FaderEvent, Changed, None, Release, truncate(), Segmented

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
Nodes (16): MenuAction, About, CheckUpdates, ClearProgrammer, DemoShow, NewShow, OpenShow, Quit (+8 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.28
Nodes (7): BOX, fixture_rect(), live_colors(), MenuFn, paint_fixture(), StageCanvas, StageInteraction

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "titlebar.rs"
Cohesion: 0.19
Nodes (10): Mode, Control, Setup, HEIGHT, main_menu(), Output, SetMode, Zoom (+2 more)

### Community 36 - "app.rs"
Cohesion: 0.10
Nodes (17): ControlViews, OUTPUT_HZ, ProgrammerView, SceneGridView, group_bar(), selection_tools(), StageView, View (+9 more)

### Community 37 - "plan.rs"
Cohesion: 0.14
Nodes (17): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), addresses_are_one_based(), UNIVERSE_SIZE (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 41 - "patch_grid.rs"
Cohesion: 0.20
Nodes (11): caption(), CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output (+3 more)

### Community 44 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "fader.rs"
Cohesion: 0.20
Nodes (4): DipSwitch, LABEL_H, VALUE_H, XyPad

### Community 65 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 67 - ".check_placement"
Cohesion: 0.48
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 71 - "OutputKind"
Cohesion: 0.29
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

### Community 77 - "PathDialog"
Cohesion: 0.67
Nodes (3): PathDialog, Open, SaveAs

## Knowledge Gaps
- **111 isolated node(s):** `Open`, `SaveAs`, `Setup`, `Control`, `NewShow` (+106 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 342 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **42 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `app.rs`, `scene_grid.rs`, `programmer.rs`, `.start`, `ChannelKind`, `egui`, `core/src/lib.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.129) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Setup` to the rest of the system?**
  _111 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.10213032581453634 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `.check_placement`, `app.rs`, `theme.rs`, `ChannelValue`, `ChannelKind`, `egui`, `FixtureAction`, `show.rs`, `ZoneId`, `ShowError`, `core/src/lib.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._
- **Why does `ChannelKind` connect `ChannelKind` to `Engine`, `convert.rs`, `app.rs`, `programmer.rs`, `core/src/lib.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.058) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0655367231638418 - nodes in this community are weakly interconnected._