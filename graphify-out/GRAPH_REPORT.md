# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 68 files · ~39,721 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1141 nodes · 2389 edges · 94 communities (47 shown, 47 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 61 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d664ceb4`
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
- fixture_settings.rs
- DmxCraftApp
- host
- plan
- with
- ensure_linux_menu_entry
- patch_grid.rs
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
- stage.rs
- xy_pad.rs
- swatch.rs
- DvcGoldUsb
- scene_card.rs
- OutputStatus
- titlebar.rs
- Settings
- apply
- toggle.rs
- IconButton
- Arrange
- Grip
- Failing
- dip_switch.rs
- Stroke
- Toggle<'a>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 67 edges
3. `Icon` - 61 edges
4. `DmxFrame` - 36 edges
5. `ChannelKind` - 35 edges
6. `AppState` - 33 edges
7. `DmxCraftApp` - 30 edges
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
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (94 total, 47 thin omitted)

### Community 0 - "Show"
Cohesion: 0.05
Nodes (43): cv(), demo_show(), empty_show(), ensure_builtin_types(), UNIVERSE_SIZE, ChannelDef, FixtureType, builtin_fixture_types() (+35 more)

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (22): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+14 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): ACCENT, ACCENT_DIM, ACCENT_HOVER, BG, BORDER, DANGER, DIP_BODY, FIXTURE_BAR (+23 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.06
Nodes (34): Fader<'a>, FaderEvent, Changed, None, Release, truncate(), begin(), ICON_SLOT (+26 more)

### Community 6 - "ChannelKind"
Cohesion: 0.05
Nodes (48): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+40 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.15
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.11
Nodes (8): AppState, Mode, Control, Setup, PatchForm, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, Kind, Cancel, Primary (+2 more)

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
Cohesion: 0.19
Nodes (8): fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES, set_pos()

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (47): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+39 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

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
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (12): BOX, CanvasMode, Edit, Select, fixture_rect(), live_colors(), MenuFn, paint_fixture() (+4 more)

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
Cohesion: 0.18
Nodes (6): LockDialog, OUTPUT_HZ, PathDialog, Open, SaveAs, INPUT_H

### Community 37 - "plan.rs"
Cohesion: 0.08
Nodes (24): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+16 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "form"
Cohesion: 0.27
Nodes (3): DialogResult, Form, form()

### Community 40 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 41 - "ViewCtx"
Cohesion: 0.14
Nodes (8): View, ViewCtx, FixtureSettings, SetupView, ScenesView, StageEditorView, ZonesView, Windows

### Community 44 - "OutputKind"
Cohesion: 0.25
Nodes (4): OutputKind, .ALL, DvcGoldUsb, Simulation

### Community 45 - "caption"
Cohesion: 0.31
Nodes (3): body(), Segmented, Segmented<'a, T>

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "patch_grid.rs"
Cohesion: 0.16
Nodes (5): Fader, LABEL_H, VALUE_H, LimitPad, RangeSlider

### Community 48 - "stage.rs"
Cohesion: 0.27
Nodes (6): ControlViews, section_header(), ProgrammerView, group_bar(), selection_tools(), StageView

### Community 50 - "xy_pad.rs"
Cohesion: 0.22
Nodes (5): caption(), heading_font(), heading_text(), title(), XyPad

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.06
Nodes (43): HEIGHT, LEVELS, load_meter(), LOCK, main_menu(), MenuAction, About, CheckUpdates (+35 more)

### Community 73 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), style()

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 79 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **161 isolated node(s):** `Open`, `SaveAs`, `Setup`, `Control`, `None` (+156 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 439 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **47 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `theme.rs`, `app.rs`, `plan.rs`, `FixtureAction`, `fixture_menu.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.134) - this node is a cross-community bridge._
- **What connects `Open`, `SaveAs`, `Setup` to the rest of the system?**
  _161 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Show` be split into smaller, more focused modules?**
  _Cohesion score 0.05133599831685252 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Show`, `runtime.rs`, `app.rs`, `scene_grid.rs`, `ChannelKind`, `ViewCtx`, `AppState`, `stage.rs`, `fixture_menu.rs`, `fixture_settings.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.124) - this node is a cross-community bridge._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09180327868852459 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `scene_grid.rs`, `titlebar.rs`, `IconButton`, `dialog.rs`, `fixture_menu.rs`, `fixture_settings.rs`, `Stroke`, `patch_grid.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **Should `convert.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07077922077922078 - nodes in this community are weakly interconnected._