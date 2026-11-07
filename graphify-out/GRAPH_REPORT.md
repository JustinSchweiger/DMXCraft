# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~62,759 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1532 nodes · 3488 edges · 147 communities (75 shown, 72 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 146 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `fbbc4b20`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- app.rs
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- widgets/mod.rs
- steps
- DmxCraftApp
- OutputError
- t
- t
- stage_editor.rs
- i18n/mod.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Settings
- stage.rs
- engine.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- swatch.rs
- env
- dmxcraft
- AppState
- Dialog<'a>
- file_dialog.rs
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- quick.rs
- fader.rs
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- DmxFrame
- Scene
- DvcGoldUsb
- programmer.rs
- color_picker.rs
- Show
- knob.rs
- OutputKind
- Snapshot
- send_frame
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- arc
- FixtureLimits
- scene_card.rs
- PatchGrid
- ViewCtx
- PlanShape
- Category
- ShowError
- ZoneId
- stage_canvas.rs
- show
- Fullscreen
- en.rs
- .fade_state
- Vec2
- limit_pad.rs
- sidebar
- Theme
- time_field.rs
- scene_props.rs
- Arrange
- section_header
- .show
- .check_placement
- FixtureType

## God Nodes (most connected - your core abstractions)
1. `Engine` - 112 edges
2. `Show` - 83 edges
3. `Icon` - 75 edges
4. `t()` - 73 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 31 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `paint_fixture()` --calls--> `fill()`  [INFERRED]
  crates/app/src/ui/widgets/stage_canvas.rs → crates/app/src/i18n/mod.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (147 total, 72 thin omitted)

### Community 0 - "app.rs"
Cohesion: 0.19
Nodes (9): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, ScenePropsView, Tab, Properties, Second (+1 more)

### Community 1 - "AppState"
Cohesion: 0.18
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.14
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): accent_hover(), ACTIVE, apply(), c(), DANGER, DIP_BODY, fixture_bar(), fixture_off() (+23 more)

### Community 5 - "Engine"
Cohesion: 0.08
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 6 - "widgets/mod.rs"
Cohesion: 0.23
Nodes (12): accent(), bg(), border(), caption(), raised(), text(), text_dim(), text_faint() (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.10
Nodes (9): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer (+1 more)

### Community 10 - "t"
Cohesion: 0.10
Nodes (41): programmer_values(), Action, Bank, Delete, DuplicateScene, Jump, Mark, MoveScenes (+33 more)

### Community 11 - "t"
Cohesion: 0.23
Nodes (4): t(), Dialog, ZonesView, Windows

### Community 12 - "stage_editor.rs"
Cohesion: 0.21
Nodes (6): StageInteraction, ZOOM_MAX, ZOOM_MIN, set_pos(), StageEditorView, zoom_bar()

### Community 13 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.05
Nodes (56): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+48 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (53): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+45 more)

### Community 23 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 24 - "stage.rs"
Cohesion: 0.23
Nodes (11): select_all(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+3 more)

### Community 25 - "engine.rs"
Cohesion: 0.25
Nodes (24): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+16 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "AppState"
Cohesion: 0.17
Nodes (6): edit_preview(), AppState, Mode, Control, Setup, PatchForm

### Community 36 - "Dialog<'a>"
Cohesion: 0.13
Nodes (7): Button, Dialog<'a>, DialogResult, Kind, Cancel, Primary, Secondary

### Community 37 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (25): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+17 more)

### Community 41 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 44 - "fader.rs"
Cohesion: 0.06
Nodes (32): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+24 more)

### Community 45 - "super"
Cohesion: 0.10
Nodes (22): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+14 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.17
Nodes (18): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+10 more)

### Community 66 - "Scene"
Cohesion: 0.21
Nodes (7): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneSettings, UNIVERSES

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.14
Nodes (25): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+17 more)

### Community 70 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 72 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 73 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 77 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.08
Nodes (7): FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, LABEL_W, IconButton

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 95 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 96 - "PatchGrid"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "ViewCtx"
Cohesion: 0.18
Nodes (6): View, ViewCtx, caption(), FixtureSettings, range_row(), SetupView

### Community 98 - "PlanShape"
Cohesion: 0.13
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 99 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.18
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (11): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+3 more)

### Community 103 - "show"
Cohesion: 0.28
Nodes (6): load_meter(), show(), accent_dim(), body(), heading_font(), heading_text()

### Community 104 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 108 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 111 - "Vec2"
Cohesion: 0.18
Nodes (3): file_icons(), LimitPad, Segmented

### Community 112 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 118 - "sidebar"
Cohesion: 0.25
Nodes (7): sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.29
Nodes (8): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 137 - "scene_props.rs"
Cohesion: 0.38
Nodes (6): bar_slider(), header(), properties(), single_scene(), TABS_W, time_text()

### Community 138 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 141 - ".show"
Cohesion: 0.40
Nodes (5): Camera, CanvasMode, Edit, Select, StageCanvas

### Community 143 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 149 - "FixtureType"
Cohesion: 0.21
Nodes (3): ChannelDef, FixtureType, ft()

## Knowledge Gaps
- **195 isolated node(s):** `Select`, `Edit`, `All`, `Delete`, `New` (+190 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 565 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **72 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `app.rs`, `AppState`, `runtime.rs`, `scene_props.rs`, `t`, `stage.rs`, `engine.rs`, `AppState`, `FixtureAction`, `quick.rs`, `DmxCraftApp`, `settings_window.rs`, `DmxFrame`, `Scene`, `programmer.rs`, `Show`, `Snapshot`, `ViewCtx`, `stage_canvas.rs`, `.fade_state`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **Are the 55 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 55 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `All` to the rest of the system?**
  _195 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1368421052631579 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `PatchGrid`, `ViewCtx`, `programmer.rs`, `color_picker.rs`, `widgets/mod.rs`, `FixtureAction`, `scene_props.rs`, `t`, `stage_editor.rs`, `Vec2`, `settings_window.rs`, `titlebar.rs`, `sidebar`, `dialog.rs`, `stage.rs`, `MenuItem<'a>`, `scene_card.rs`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09246088193456614 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `app.rs`, `Scene`, `runtime.rs`, `theme.rs`, `Engine`, `stage_canvas.rs`, `ZoneId`, `FixtureAction`, `ShowError`, `t`, `Snapshot`, `.show`, `DmxCraftApp`, `.check_placement`, `FixtureType`, `show.rs`, `engine.rs`, `ChannelValue`?**
  _High betweenness centrality (0.086) - this node is a cross-community bridge._