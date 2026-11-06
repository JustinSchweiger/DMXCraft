# Graph Report - dmxcraft-src  (2026-11-06)

## Corpus Check
- 70 files · ~40,413 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1168 nodes · 2411 edges · 117 communities (63 shown, 54 thin omitted)
- Extraction: 97% EXTRACTED · 3% INFERRED · 0% AMBIGUOUS · INFERRED: 63 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bb3c7eef`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- super
- Engine
- runtime.rs
- Value
- theme.rs
- scene_grid.rs
- Show
- steps
- DmxCraftApp
- OutputError
- AppState
- dialog.rs
- FixtureAction
- jobs
- steps
- DmxCraft
- ViewCtx
- announce
- build-local-artifacts
- Icon
- ChannelKind
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
- core/src/lib.rs
- show.rs
- super
- DmxCraft – UI-Richtlinien
- ChannelValue
- arc
- windows.rs
- Scene
- ofl/src/lib.rs
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- Vec2
- MenuAction
- menu.rs
- swatch.rs
- DvcGoldUsb
- Matrix
- OutputStatus
- titlebar.rs
- FixtureLimits
- Mode
- .show
- scene_card.rs
- Arrange
- Segmented
- LibraryPanel
- library_panel.rs
- Stroke
- Toggle<'a>
- egui
- plan.rs
- convert_fixture
- FixtureType
- fader.rs
- Dialog<'a>
- ofl-bundle.rs
- dip.rs
- UpdateBadge
- body
- dip_switch.rs
- toggle.rs
- .check_placement
- section_header
- apply
- MenuPlace

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
10. `FixtureAction` - 24 edges

## Surprising Connections (you probably didn't know these)
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (117 total, 54 thin omitted)

### Community 0 - "super"
Cohesion: 0.21
Nodes (4): History, LIMIT, undo_redo_roundtrip(), UNIVERSE_SIZE

### Community 1 - "Engine"
Cohesion: 0.07
Nodes (36): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values(), save_bar() (+28 more)

### Community 2 - "runtime.rs"
Cohesion: 0.18
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "Value"
Cohesion: 0.36
Nodes (7): capabilities(), channel_kind(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): ACCENT, ACCENT_DIM, ACCENT_HOVER, BG, BORDER, DANGER, DIP_BODY, FIXTURE_BAR (+23 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.07
Nodes (39): ControlViews, Category, All, Beam, Color, Dimmer, Gobo, .ITEMS (+31 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.18
Nodes (5): AppState, PatchForm, group_bar(), selection_tools(), StageView

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+2 more)

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

### Community 19 - "ViewCtx"
Cohesion: 0.20
Nodes (7): apply(), View, ViewCtx, FixtureSettings, SetupView, set_pos(), StageEditorView

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
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 25 - "DmxCraftApp"
Cohesion: 0.06
Nodes (20): DmxCraftApp, LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension() (+12 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 29 - "ensure_linux_menu_entry"
Cohesion: 0.50
Nodes (3): ensure_linux_menu_entry(), register_mime(), main()

### Community 30 - "patch_grid.rs"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 31 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (13): BOX, CanvasMode, Edit, Select, fixture_pos(), fixture_rect(), live_colors(), MenuFn (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (13): SIZES, Group, PLAN_SIZE_RANGE, PlanShape, .ALL, BarHorizontal, BarVertical, Effect (+5 more)

### Community 36 - "show.rs"
Cohesion: 0.30
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 37 - "super"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 40 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 41 - "windows.rs"
Cohesion: 0.38
Nodes (3): ScenesView, ZonesView, Windows

### Community 43 - "Scene"
Cohesion: 0.19
Nodes (5): LoopMode, Loop, Once, Scene, Zone

### Community 44 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "Vec2"
Cohesion: 0.17
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, RangeSlider

### Community 48 - "MenuAction"
Cohesion: 0.12
Nodes (16): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenShow, Quit (+8 more)

### Community 50 - "menu.rs"
Cohesion: 0.08
Nodes (18): Fader<'a>, FaderEvent, Changed, None, Release, begin(), context(), dropdown() (+10 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.20
Nodes (3): DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "Matrix"
Cohesion: 0.29
Nodes (4): Matrix, natural_cmp(), natural_sort(), Pixel

### Community 70 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.18
Nodes (15): HEIGHT, LEVELS, LOCK, MOD, NEW, OPEN, REDO, resize_handles() (+7 more)

### Community 73 - "Mode"
Cohesion: 0.23
Nodes (9): Mode, Control, Setup, load_meter(), main_menu(), OpenRecent, Zoom, show() (+1 more)

### Community 74 - ".show"
Cohesion: 0.27
Nodes (3): DialogResult, Form, form()

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 93 - "egui"
Cohesion: 0.14
Nodes (6): SETTLE, XyPad, CELL_H_MAX, CELL_H_MIN, COLUMNS, ROWS

### Community 94 - "plan.rs"
Cohesion: 0.31
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 96 - "convert_fixture"
Cohesion: 0.24
Nodes (10): convert_fixture(), default_value(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), redirects_are_skipped(), simple_par_with_modes_defaults_and_null() (+2 more)

### Community 97 - "FixtureType"
Cohesion: 0.15
Nodes (4): RECENT_MAX, ChannelDef, FixtureType, ft()

### Community 98 - "fader.rs"
Cohesion: 0.29
Nodes (4): Fader, LABEL_H, truncate(), VALUE_H

### Community 100 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 101 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 102 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 103 - "body"
Cohesion: 0.33
Nodes (4): body(), caption(), heading_font(), heading_text()

### Community 109 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 111 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), style()

### Community 112 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

## Knowledge Gaps
- **163 isolated node(s):** `Enter`, `Maximize`, `Stage`, `StageEditor`, `Patch` (+158 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 448 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **54 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `super`, `Engine`, `core/src/lib.rs`, `FixtureType`, `theme.rs`, `show.rs`, `runtime.rs`, `ChannelValue`, `Scene`, `FixtureAction`, `.check_placement`, `ShowError`, `DmxCraftApp`, `stage_canvas.rs`?**
  _High betweenness centrality (0.149) - this node is a cross-community bridge._
- **What connects `Enter`, `Maximize`, `Stage` to the rest of the system?**
  _163 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06722689075630252 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `core/src/lib.rs`, `runtime.rs`, `scene_grid.rs`, `Show`, `AppState`, `LibraryPanel`, `ViewCtx`, `fixture_settings.rs`, `DmxCraftApp`, `library_panel.rs`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09243697478991597 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `core/src/lib.rs`, `scene_grid.rs`, `titlebar.rs`, `dialog.rs`, `menu.rs`, `ViewCtx`, `fixture_settings.rs`, `Stroke`, `egui`, `stage_canvas.rs`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Should `scene_grid.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06868686868686869 - nodes in this community are weakly interconnected._