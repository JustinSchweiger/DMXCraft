# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 65 files · ~35,228 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1014 nodes · 2134 edges · 97 communities (51 shown, 46 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 51 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `884c153b`
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
- stage.rs
- FixtureAction
- scenes.rs
- jobs
- steps
- DmxCraft
- fixture_menu.rs
- announce
- build-local-artifacts
- Icon
- patch_grid.rs
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
- ofl/src/lib.rs
- scene_card.rs
- fixture_settings.rs
- DmxFrame
- Value
- caption
- DmxCraft – Hinweise für KI-Assistenten
- patch_grid.rs
- widgets/mod.rs
- FixtureType
- swatch.rs
- lock.rs
- DvcGoldUsb
- TitleInfo
- OutputStatus
- titlebar.rs
- Settings
- ofl-bundle.rs
- toggle.rs
- IconButton
- Arrange
- dip.rs
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
6. `Icon` - 30 edges
7. `ViewCtx` - 30 edges
8. `DmxCraftApp` - 29 edges
9. `MenuAction` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `show()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/core/src/show.rs → crates/core/src/library.rs
- `runtime_sends_rendered_frames()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/output/src/runtime.rs → crates/core/src/library.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (97 total, 46 thin omitted)

### Community 0 - "Show"
Cohesion: 0.05
Nodes (38): cv(), demo_show(), empty_show(), ensure_builtin_types(), address_map_marks_ranges(), ChannelValue, Fixture, FixtureLimits (+30 more)

### Community 1 - "Engine"
Cohesion: 0.07
Nodes (43): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+35 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.19
Nodes (14): convert_fixture(), default_value(), kinds(), Matrix, matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), natural_cmp() (+6 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (35): ACCENT, ACCENT_DIM, apply(), BG, body(), BORDER, DANGER, DIP_BODY (+27 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.18
Nodes (17): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+9 more)

### Community 6 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.09
Nodes (14): ICON_PNG, texture(), window_icon(), APP_NAME, Updater, UpdateState, Available, Checking (+6 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.09
Nodes (8): AppState, PatchForm, group_bar(), selection_tools(), ViewCtx, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 12 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 13 - "scenes.rs"
Cohesion: 0.26
Nodes (10): apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete, Down, Load (+2 more)

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
Cohesion: 0.20
Nodes (9): fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES, set_pos() (+1 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.10
Nodes (21): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Center, Close, LightOff (+13 more)

### Community 23 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

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
Cohesion: 0.30
Nodes (8): BOX, fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon(), StageCanvas, StageInteraction

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
Cohesion: 0.15
Nodes (14): ControlViews, LockDialog, OUTPUT_HZ, PathDialog, Open, SaveAs, ProgrammerView, SceneGridView (+6 more)

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 40 - "scene_card.rs"
Cohesion: 0.36
Nodes (3): caption(), clip(), SceneCard

### Community 41 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (8): caption(), FixtureSettings, Quick, quick_action(), Center, Off, On, range_row()

### Community 43 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 44 - "Value"
Cohesion: 0.27
Nodes (7): capabilities(), channel_kind(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "patch_grid.rs"
Cohesion: 0.18
Nodes (3): LimitPad, RangeSlider, XyPad

### Community 50 - "widgets/mod.rs"
Cohesion: 0.24
Nodes (5): CanvasMode, Edit, Select, shape_menu_row(), ShapeButton

### Community 65 - "FixtureType"
Cohesion: 0.17
Nodes (3): ChannelDef, FixtureType, ft()

### Community 67 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "TitleInfo"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 70 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.13
Nodes (12): Mode, Control, Setup, HEIGHT, load_meter(), resize_handles(), show(), TitleInfo (+4 more)

### Community 73 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 78 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 79 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **135 isolated node(s):** `Open`, `SaveAs`, `Setup`, `Control`, `None` (+130 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 383 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **46 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `FixtureType`, `runtime.rs`, `app.rs`, `theme.rs`, `FixtureAction`, `fixture_menu.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Setup` to the rest of the system?**
  _135 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05343320848938826 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `app.rs`, `scene_grid.rs`, `fixture_settings.rs`, `AppState`, `stage.rs`, `scenes.rs`, `fixture_menu.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06626506024096386 - nodes in this community are weakly interconnected._
- **Why does `DmxFrame` connect `DmxFrame` to `Show`, `Engine`, `runtime.rs`, `DvcGoldUsb`, `OutputError`, `AppState`, `Failing`, `stage_canvas.rs`?**
  _High betweenness centrality (0.049) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07419712070874862 - nodes in this community are weakly interconnected._