# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 71 files · ~42,415 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1227 nodes · 2519 edges · 115 communities (57 shown, 58 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 62 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7660380c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- super
- Engine
- runtime.rs
- open_dmx.rs
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
- ensure_linux_menu_entry
- patch_grid.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- show.rs
- send_frame
- DmxCraft – UI-Richtlinien
- ChannelValue
- egui
- windows.rs
- Scene
- convert.rs
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- Vec2
- MenuAction
- menu.rs
- app.rs
- swatch.rs
- DvcGoldUsb
- OpenDmx
- OutputStatus
- titlebar.rs
- FixtureLimits
- Mode
- widgets/mod.rs
- scene_card.rs
- IconButton
- Arrange
- Segmented
- library_panel.rs
- DmxFrame
- Stroke
- Toggle<'a>
- Fullscreen
- plan.rs
- ZoneId
- FixtureType
- resize_handles
- UpdateBadge
- dip_switch.rs
- toggle.rs
- .check_placement

## God Nodes (most connected - your core abstractions)
1. `Engine` - 76 edges
2. `Show` - 65 edges
3. `Icon` - 62 edges
4. `DmxFrame` - 38 edges
5. `DmxCraftApp` - 34 edges
6. `ChannelKind` - 34 edges
7. `AppState` - 33 edges
8. `ViewCtx` - 30 edges
9. `FixtureType` - 26 edges
10. `FixtureAction` - 24 edges

## Surprising Connections (you probably didn't know these)
- `runtime_sends_rendered_frames()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/output/src/runtime.rs → crates/core/src/library.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `color_bar()` --references--> `Swatch`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/swatch.rs
- `save_bar()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/dialog.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (115 total, 58 thin omitted)

### Community 1 - "Engine"
Cohesion: 0.09
Nodes (23): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+15 more)

### Community 2 - "runtime.rs"
Cohesion: 0.16
Nodes (5): engine_keeps_running_without_device(), RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (12): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+4 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (38): ACCENT, ACCENT_DIM, ACCENT_HOVER, apply(), BG, body(), BORDER, caption() (+30 more)

### Community 5 - "scene_grid.rs"
Cohesion: 0.11
Nodes (28): programmer_values(), Action, Delete, Edit, Jump, NewScene, Pause, Toggle (+20 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.11
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "AppState"
Cohesion: 0.17
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

### Community 23 - "ChannelKind"
Cohesion: 0.06
Nodes (39): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+31 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 25 - "DmxCraftApp"
Cohesion: 0.10
Nodes (13): DmxCraftApp, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension() (+5 more)

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

### Community 36 - "show.rs"
Cohesion: 0.27
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 37 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.31
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 40 - "egui"
Cohesion: 0.16
Nodes (6): ICON_PNG, texture(), window_icon(), section_header(), group_bar(), selection_tools()

### Community 41 - "windows.rs"
Cohesion: 0.22
Nodes (4): ViewCtx, ScenesView, ZonesView, Windows

### Community 43 - "Scene"
Cohesion: 0.32
Nodes (4): LoopMode, Loop, Once, Scene

### Community 44 - "convert.rs"
Cohesion: 0.07
Nodes (32): git(), main(), run(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases() (+24 more)

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "Vec2"
Cohesion: 0.15
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, XyPad

### Community 48 - "MenuAction"
Cohesion: 0.12
Nodes (16): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenShow, Quit (+8 more)

### Community 50 - "menu.rs"
Cohesion: 0.07
Nodes (22): Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H, truncate() (+14 more)

### Community 65 - "app.rs"
Cohesion: 0.25
Nodes (10): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView, View, FixtureSettings (+2 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 70 - "OutputStatus"
Cohesion: 0.17
Nodes (6): OutputConfig, OutputStatus, Connecting, Error, Running, Stopped

### Community 71 - "titlebar.rs"
Cohesion: 0.21
Nodes (14): HEIGHT, LEVELS, LOCK, MOD, NEW, OPEN, REDO, SAVE (+6 more)

### Community 73 - "Mode"
Cohesion: 0.33
Nodes (6): load_meter(), main_menu(), OpenRecent, Zoom, show(), TitleInfo

### Community 74 - "widgets/mod.rs"
Cohesion: 0.24
Nodes (5): RangeSlider, CanvasMode, Edit, Select, StageCanvas

### Community 77 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 80 - "library_panel.rs"
Cohesion: 0.18
Nodes (4): text_field(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 93 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 94 - "plan.rs"
Cohesion: 0.06
Nodes (26): LIMIT, digest(), hash_and_verify(), hash_password(), hex(), verify(), cable_belongs_to_one_universe(), RECENT_MAX (+18 more)

### Community 97 - "FixtureType"
Cohesion: 0.20
Nodes (4): FixtureType, Group, PLAN_SIZE_RANGE, UNIVERSES

### Community 102 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 109 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

## Knowledge Gaps
- **164 isolated node(s):** `NotFound`, `NotImplemented`, `Transfer`, `Simulation`, `OpenDmx` (+159 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 465 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **58 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `app.rs`, `FixtureType`, `runtime.rs`, `scene_grid.rs`, `Show`, `egui`, `windows.rs`, `library_panel.rs`, `fixture_menu.rs`, `ChannelKind`, `fixture_settings.rs`, `DmxCraftApp`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **What connects `NotFound`, `NotImplemented`, `Transfer` to the rest of the system?**
  _164 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09095716552088842 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `super`, `Engine`, `FixtureType`, `ZoneId`, `theme.rs`, `show.rs`, `ChannelValue`, `widgets/mod.rs`, `Scene`, `FixtureAction`, `.check_placement`, `ShowError`, `fixture_menu.rs`, `plan.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.07171717171717172 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `app.rs`, `scene_grid.rs`, `titlebar.rs`, `widgets/mod.rs`, `dialog.rs`, `IconButton`, `menu.rs`, `fixture_menu.rs`, `fixture_settings.rs`, `Stroke`, `patch_grid.rs`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._
- **Should `scene_grid.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10609756097560975 - nodes in this community are weakly interconnected._