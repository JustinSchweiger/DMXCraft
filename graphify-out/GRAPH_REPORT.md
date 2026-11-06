# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 69 files · ~40,057 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1154 nodes · 2402 edges · 109 communities (55 shown, 54 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 62 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `6d1d5b4c`
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
- ChannelKind
- fixture_settings.rs
- DmxCraftApp
- host
- plan
- with
- patch_grid.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- app.rs
- super
- DmxCraft – UI-Richtlinien
- MenuAction
- arc
- ViewCtx
- DmxFrame
- ofl/src/lib.rs
- caption
- DmxCraft – Hinweise für KI-Assistenten
- egui
- stage.rs
- menu.rs
- swatch.rs
- DvcGoldUsb
- scene_card.rs
- OutputStatus
- titlebar.rs
- show
- MenuItem<'a>
- toggle.rs
- library_panel.rs
- Arrange
- Grip
- Failing
- dip_switch.rs
- Stroke
- Toggle<'a>
- widgets/mod.rs
- plan.rs
- convert_fixture
- FixtureType
- fader.rs
- LibraryPanel
- ofl-bundle.rs
- settings.rs
- Fader<'a>
- FaderEvent

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 67 edges
3. `Icon` - 61 edges
4. `DmxFrame` - 36 edges
5. `ChannelKind` - 34 edges
6. `AppState` - 33 edges
7. `DmxCraftApp` - 32 edges
8. `ViewCtx` - 30 edges
9. `FixtureType` - 26 edges
10. `MenuAction` - 23 edges

## Surprising Connections (you probably didn't know these)
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `apply()` --calls--> `programmer_values()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/views/control/programmer.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `StageEditorView`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/stage_editor.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (109 total, 54 thin omitted)

### Community 0 - "Show"
Cohesion: 0.05
Nodes (40): cv(), demo_show(), empty_show(), ensure_builtin_types(), History, undo_redo_roundtrip(), address_map_marks_ranges(), ChannelValue (+32 more)

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (23): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+15 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.17
Nodes (12): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Matrix, natural_cmp() (+4 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (38): ACCENT, ACCENT_DIM, ACCENT_HOVER, apply(), BG, body(), BORDER, caption() (+30 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.17
Nodes (17): Action, Delete, Edit, Jump, NewScene, Pause, Toggle, apply() (+9 more)

### Community 6 - "ChannelKind"
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

### Community 10 - "AppState"
Cohesion: 0.19
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 11 - "dialog.rs"
Cohesion: 0.08
Nodes (14): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, form() (+6 more)

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
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "fixture_menu.rs"
Cohesion: 0.18
Nodes (9): fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES, set_pos() (+1 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (47): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+39 more)

### Community 23 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 25 - "DmxCraftApp"
Cohesion: 0.09
Nodes (10): DmxCraftApp, Pending, Purpose, OpenShow, SaveShowAs, Settings, OutputKind, .ALL (+2 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "patch_grid.rs"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.25
Nodes (7): BOX, fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon(), StageInteraction

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.15
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 36 - "app.rs"
Cohesion: 0.31
Nodes (5): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView

### Community 37 - "super"
Cohesion: 0.14
Nodes (14): LIMIT, digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches() (+6 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 40 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 41 - "ViewCtx"
Cohesion: 0.18
Nodes (7): View, ViewCtx, FixtureSettings, SetupView, ScenesView, ZonesView, Windows

### Community 44 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "stage.rs"
Cohesion: 0.27
Nodes (4): section_header(), group_bar(), selection_tools(), StageView

### Community 50 - "menu.rs"
Cohesion: 0.23
Nodes (13): main_menu(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H (+5 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.16
Nodes (15): HEIGHT, LEVELS, LOCK, MOD, NEW, OPEN, REDO, resize_handles() (+7 more)

### Community 72 - "show"
Cohesion: 0.20
Nodes (9): load_meter(), show(), TitleInfo, UpdateBadge, Available, Busy, Error, Installed (+1 more)

### Community 75 - "library_panel.rs"
Cohesion: 0.20
Nodes (3): text_field(), MAX_FIXTURES_PER_SEARCH, patch()

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 79 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 93 - "widgets/mod.rs"
Cohesion: 0.24
Nodes (5): RangeSlider, CanvasMode, Edit, Select, StageCanvas

### Community 94 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 96 - "convert_fixture"
Cohesion: 0.24
Nodes (9): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), redirects_are_skipped(), simple_par_with_modes_defaults_and_null(), slug() (+1 more)

### Community 97 - "FixtureType"
Cohesion: 0.31
Nodes (3): ChannelDef, FixtureType, ft()

### Community 98 - "fader.rs"
Cohesion: 0.29
Nodes (4): Fader, LABEL_H, truncate(), VALUE_H

### Community 100 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 103 - "FaderEvent"
Cohesion: 0.50
Nodes (4): FaderEvent, Changed, None, Release

## Knowledge Gaps
- **161 isolated node(s):** `OpenShow`, `SaveShowAs`, `None`, `Available`, `Busy` (+156 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 446 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **54 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `LibraryPanel`, `app.rs`, `scene_grid.rs`, `ChannelKind`, `ViewCtx`, `library_panel.rs`, `stage.rs`, `fixture_menu.rs`, `fixture_settings.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **What connects `OpenShow`, `SaveShowAs`, `None` to the rest of the system?**
  _161 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05021929824561404 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `FixtureType`, `runtime.rs`, `theme.rs`, `app.rs`, `super`, `FixtureAction`, `fixture_menu.rs`, `widgets/mod.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.115) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09548022598870057 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `scene_grid.rs`, `titlebar.rs`, `MenuItem<'a>`, `dialog.rs`, `menu.rs`, `fixture_menu.rs`, `fixture_settings.rs`, `Stroke`, `widgets/mod.rs`, `patch_grid.rs`?**
  _High betweenness centrality (0.068) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07171717171717172 - nodes in this community are weakly interconnected._