# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~57,703 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1457 nodes · 3281 edges · 148 communities (67 shown, 81 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 134 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1543a9aa`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- AppState
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- menu.rs
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- stage.rs
- super
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- stage.rs
- ChannelRef
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- egui
- env
- dmxcraft
- titlebar.rs
- Arrange
- core/src/lib.rs
- DmxCraft – UI-Richtlinien
- FixtureAction
- Settings
- fader.rs
- color_picker.rs
- DmxCraft – Hinweise für KI-Assistenten
- t
- settings_window.rs
- ViewCtx
- View
- DvcGoldUsb
- programmer.rs
- convert.rs
- OutputKind
- show.rs
- MenuAction
- app.rs
- swatch.rs
- OpenDmx
- library_panel.rs
- IconButton
- MenuItem<'a>
- toggle.rs
- Toggle<'a>
- arc
- FixtureType
- file_dialog.rs
- fixture_settings.rs
- DmxFrame
- Show
- PlanShape
- ZoneId
- stage_canvas.rs
- ShowError
- Fullscreen
- en.rs
- .ui
- FixtureLimits
- Tab
- scene_card.rs
- T
- TitleInfo
- .show
- i18n/mod.rs
- Category
- windows.rs
- resize_handles
- engine.rs
- Grip

## God Nodes (most connected - your core abstractions)
1. `Engine` - 99 edges
2. `Show` - 74 edges
3. `Icon` - 73 edges
4. `t()` - 67 edges
5. `ChannelKind` - 43 edges
6. `DmxFrame` - 37 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `ChannelRef` - 29 edges
10. `ViewCtx` - 28 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `toolbar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `toolbar()` --calls--> `quick_action()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/views/quick.rs
- `mode_switch()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs

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

## Communities (148 total, 81 thin omitted)

### Community 0 - "AppState"
Cohesion: 0.19
Nodes (6): edit_preview(), AppState, Mode, Control, Setup, PatchForm

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (52): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+44 more)

### Community 5 - "Engine"
Cohesion: 0.10
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 6 - "menu.rs"
Cohesion: 0.20
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.09
Nodes (46): color_picker(), color_swatches(), select_all(), text_field(), CARD_H, programmer_values(), Action, Bank (+38 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 12 - "stage.rs"
Cohesion: 0.16
Nodes (13): CanvasMode, Edit, Select, fit(), fixture_pos(), ZOOM_MAX, ZOOM_MIN, apply() (+5 more)

### Community 13 - "super"
Cohesion: 0.08
Nodes (27): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password() (+19 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (52): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+44 more)

### Community 23 - "ChannelKind"
Cohesion: 0.07
Nodes (23): Fader, FaderEvent, Changed, None, Release, ChannelKind, .ALL, Amber (+15 more)

### Community 24 - "stage.rs"
Cohesion: 0.26
Nodes (9): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+1 more)

### Community 25 - "ChannelRef"
Cohesion: 0.12
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 31 - "egui"
Cohesion: 0.16
Nodes (11): DipSwitch, HANDLE, LimitPad, RangeSlider, Segmented, paint_fixture(), XyPad, CELL_H_MAX (+3 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 36 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 37 - "core/src/lib.rs"
Cohesion: 0.20
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 44 - "fader.rs"
Cohesion: 0.25
Nodes (8): DOT_H, FADER_W, fine_mark(), NUM_H, paint_symbol(), SYMBOL_H, VALUE_H, WHEEL_STEP

### Community 45 - "color_picker.rs"
Cohesion: 0.15
Nodes (11): hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch, Auto (+3 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.17
Nodes (17): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+9 more)

### Community 50 - "ViewCtx"
Cohesion: 0.27
Nodes (7): ViewCtx, clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 65 - "View"
Cohesion: 0.27
Nodes (3): View, SetupView, StageEditorView

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.22
Nodes (18): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+10 more)

### Community 73 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 77 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 78 - "app.rs"
Cohesion: 0.31
Nodes (5): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, StageView

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "FixtureType"
Cohesion: 0.19
Nodes (3): ChannelDef, FixtureType, ft()

### Community 96 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 97 - "fixture_settings.rs"
Cohesion: 0.23
Nodes (4): section_header(), caption(), FixtureSettings, range_row()

### Community 99 - "Show"
Cohesion: 0.20
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 100 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.19
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Show, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.20
Nodes (9): BOX, Camera, fixture_rect(), live_colors(), MenuFn, paint_shape_icon(), StageCanvas, StageInteraction (+1 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 104 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 110 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "scene_card.rs"
Cohesion: 0.29
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 118 - ".show"
Cohesion: 0.14
Nodes (3): Dialog<'a>, DialogResult, Form

### Community 119 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 123 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 138 - "engine.rs"
Cohesion: 0.27
Nodes (21): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets(), MAX_STEP_ADVANCES_PER_TICK (+13 more)

### Community 140 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

## Knowledge Gaps
- **190 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+185 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 547 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **81 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `ZoneId` to `runtime.rs`, `Show`, `theme.rs`, `Engine`, `stage_canvas.rs`, `core/src/lib.rs`, `FixtureAction`, `ShowError`, `t`, `engine.rs`, `stage.rs`, `super`, `show.rs`, `FixtureType`, `ChannelValue`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _190 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `View`, `Engine`, `runtime.rs`, `programmer.rs`, `ZoneId`, `core/src/lib.rs`, `t`, `engine.rs`, `stage.rs`, `super`, `app.rs`, `t`, `settings_window.rs`, `ViewCtx`, `library_panel.rs`, `stage.rs`, `ChannelRef`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09090909090909091 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `Engine`, `theme.rs`, `windows.rs`, `t`, `dialog.rs`, `stage.rs`, `ChannelKind`, `stage.rs`, `egui`, `titlebar.rs`, `core/src/lib.rs`, `FixtureAction`, `Settings`, `fader.rs`, `color_picker.rs`, `settings_window.rs`, `ViewCtx`, `programmer.rs`, `app.rs`, `library_panel.rs`, `file_dialog.rs`, `fixture_settings.rs`, `stage_canvas.rs`, `.ui`, `.show`, `i18n/mod.rs`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._