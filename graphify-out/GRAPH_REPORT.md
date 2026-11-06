# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 64 files · ~33,256 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 8 file(s) not represented in the graph (top: (none) 3, .ico 2, .wxs 1)

## Summary
- 988 nodes · 2073 edges · 95 communities (52 shown, 43 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 50 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f3c72551`
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
- library_panel.rs
- stage.rs
- FixtureAction
- scenes.rs
- jobs
- steps
- DmxCraft
- programmer.rs
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
- AppState
- ViewCtx
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
- FixtureType
- swatch.rs
- Category
- DvcGoldUsb
- TitleInfo
- egui
- OutputKind
- Settings
- ofl-bundle.rs
- Quick
- IconButton
- app.rs
- dip.rs
- dip_switch.rs
- Toggle<'a>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 60 edges
3. `DmxFrame` - 36 edges
4. `ChannelKind` - 34 edges
5. `AppState` - 32 edges
6. `ViewCtx` - 30 edges
7. `Icon` - 28 edges
8. `MenuAction` - 27 edges
9. `FixtureType` - 26 edges
10. `DmxCraftApp` - 25 edges

## Surprising Connections (you probably didn't know these)
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `menu()` --calls--> `shape_menu_row()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs

## Import Cycles
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (95 total, 43 thin omitted)

### Community 0 - "Show"
Cohesion: 0.06
Nodes (38): cv(), demo_show(), empty_show(), ensure_builtin_types(), SceneProgress, address_map_marks_ranges(), ChannelValue, Fixture (+30 more)

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (22): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+14 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (10): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+2 more)

### Community 3 - "convert.rs"
Cohesion: 0.19
Nodes (14): convert_fixture(), default_value(), kinds(), Matrix, matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), natural_cmp() (+6 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (29): ACCENT, ACCENT_DIM, apply(), BG, BORDER, DANGER, DIP_BODY, FIXTURE_BAR (+21 more)

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
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "library_panel.rs"
Cohesion: 0.25
Nodes (3): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 11 - "stage.rs"
Cohesion: 0.29
Nodes (3): section_header(), group_bar(), selection_tools()

### Community 12 - "FixtureAction"
Cohesion: 0.08
Nodes (26): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+18 more)

### Community 13 - "scenes.rs"
Cohesion: 0.24
Nodes (11): programmer_values(), apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete, Down (+3 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DmxCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "programmer.rs"
Cohesion: 0.36
Nodes (10): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), save_bar(), set_group() (+2 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.11
Nodes (19): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Center, Close, LightOff (+11 more)

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
Cohesion: 0.11
Nodes (22): main_menu(), MenuAction, About, CheckUpdates, ClearProgrammer, DemoShow, InstallUpdate, NewShow (+14 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.07
Nodes (31): BOX, CanvasMode, Edit, Select, fixture_pos(), fixture_rect(), live_colors(), MenuFn (+23 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "AppState"
Cohesion: 0.18
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 36 - "ViewCtx"
Cohesion: 0.16
Nodes (8): View, ViewCtx, FixtureSettings, SetupView, ScenesView, StageEditorView, ZonesView, Windows

### Community 37 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 41 - "fixture_settings.rs"
Cohesion: 0.24
Nodes (3): caption(), quick_action(), range_row()

### Community 43 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 44 - "Value"
Cohesion: 0.27
Nodes (7): capabilities(), channel_kind(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 45 - "caption"
Cohesion: 0.24
Nodes (4): body(), caption(), Segmented, Segmented<'a, T>

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DmxCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "patch_grid.rs"
Cohesion: 0.11
Nodes (9): HEIGHT, Grip, Body, Corner, EdgeX, EdgeY, LimitPad, RangeSlider (+1 more)

### Community 65 - "FixtureType"
Cohesion: 0.17
Nodes (3): ChannelDef, FixtureType, ft()

### Community 67 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 70 - "egui"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 71 - "OutputKind"
Cohesion: 0.29
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

### Community 72 - "Settings"
Cohesion: 0.25
Nodes (4): PathDialog, Open, SaveAs, Settings

### Community 73 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 74 - "Quick"
Cohesion: 0.50
Nodes (4): Quick, Center, Off, On

### Community 77 - "app.rs"
Cohesion: 0.36
Nodes (5): ControlViews, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView

### Community 78 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

## Knowledge Gaps
- **132 isolated node(s):** `Open`, `SaveAs`, `Setup`, `Control`, `None` (+127 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 376 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **43 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `ViewCtx`, `scene_grid.rs`, `fixture_settings.rs`, `library_panel.rs`, `stage.rs`, `FixtureAction`, `scenes.rs`, `app.rs`, `programmer.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Setup` to the rest of the system?**
  _132 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05586741512964448 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `FixtureType`, `Engine`, `runtime.rs`, `theme.rs`, `FixtureAction`, `stage_canvas.rs`?**
  _High betweenness centrality (0.097) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09818819403857393 - nodes in this community are weakly interconnected._
- **Why does `DmxFrame` connect `DmxFrame` to `Show`, `Engine`, `runtime.rs`, `ViewCtx`, `DvcGoldUsb`, `OutputError`, `programmer.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.050) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._