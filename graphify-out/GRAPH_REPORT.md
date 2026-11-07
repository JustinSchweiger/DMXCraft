# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~61,956 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1517 nodes · 3462 edges · 148 communities (72 shown, 76 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 146 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `979aa5ae`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- app.rs
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- text_dim
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- stage_editor.rs
- super
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- MenuAction
- stage.rs
- ChannelKind
- host
- plan
- with
- ensure_linux_menu_entry
- Value
- swatch.rs
- env
- dmxcraft
- menu.rs
- engine.rs
- color_picker.rs
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- ChannelRef
- fader.rs
- Snapshot
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- DmxFrame
- show
- DvcGoldUsb
- programmer.rs
- Show
- knob.rs
- OutputKind
- ofl/src/lib.rs
- resize_handles
- OpenDmx
- show.rs
- icons.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- arc
- core/src/lib.rs
- scene_card.rs
- PatchGrid
- t
- PlanShape
- Category
- ShowError
- ZoneId
- stage_canvas.rs
- FixtureLimits
- show
- en.rs
- stage.rs
- qxf.rs
- Vec2
- limit_pad.rs
- sidebar
- qlc-bundle.rs
- Theme
- time_field.rs
- IconButton
- scene_props.rs
- .ui
- fixture_settings.rs
- Pos2
- Tab
- Box<dyn serialport::SerialPort>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 111 edges
2. `Show` - 83 edges
3. `Icon` - 74 edges
4. `t()` - 72 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 31 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `properties()` --calls--> `time_field()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/time_field.rs
- `header()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/i18n/mod.rs
- `properties()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/i18n/mod.rs
- `properties()` --calls--> `form()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/dialog.rs

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

## Communities (148 total, 76 thin omitted)

### Community 0 - "app.rs"
Cohesion: 0.27
Nodes (7): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, ScenePropsView, StageView

### Community 1 - "AppState"
Cohesion: 0.09
Nodes (13): edit_preview(), AppState, LibraryDrag, Mode, Control, Setup, PatchForm, drag_ghost() (+5 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): accent_dim(), accent_hover(), ACTIVE, apply(), c(), DANGER, DIP_BODY, fixture_bar() (+23 more)

### Community 5 - "Engine"
Cohesion: 0.10
Nodes (3): Engine, lerp(), PlayingScene

### Community 6 - "text_dim"
Cohesion: 0.30
Nodes (8): accent(), bg(), border(), caption(), text(), text_dim(), text_faint(), DipSwitch

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.13
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.10
Nodes (41): select_all(), programmer_values(), Action, Bank, Delete, DuplicateScene, Jump, Mark (+33 more)

### Community 11 - "dialog.rs"
Cohesion: 0.07
Nodes (14): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+6 more)

### Community 12 - "stage_editor.rs"
Cohesion: 0.11
Nodes (16): StageInteraction, ZOOM_MAX, ZOOM_MIN, MenuPlace, Patch, Stage, StageEditor, Arrange (+8 more)

### Community 13 - "super"
Cohesion: 0.05
Nodes (35): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+27 more)

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
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (52): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+44 more)

### Community 23 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 24 - "stage.rs"
Cohesion: 0.26
Nodes (9): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+1 more)

### Community 25 - "ChannelKind"
Cohesion: 0.09
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "Value"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "menu.rs"
Cohesion: 0.20
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 36 - "engine.rs"
Cohesion: 0.25
Nodes (23): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+15 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (22): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+14 more)

### Community 41 - "ChannelRef"
Cohesion: 0.11
Nodes (9): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On (+1 more)

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+5 more)

### Community 45 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.06
Nodes (17): DmxCraftApp, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension() (+9 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (19): t(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+11 more)

### Community 66 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.18
Nodes (22): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+14 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "knob.rs"
Cohesion: 0.20
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "core/src/lib.rs"
Cohesion: 0.16
Nodes (8): SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneSettings, UNIVERSES

### Community 95 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "t"
Cohesion: 0.18
Nodes (6): View, ViewCtx, FixtureSettings, SetupView, ZonesView, Windows

### Community 98 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 99 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Show, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (10): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+2 more)

### Community 104 - "show"
Cohesion: 0.24
Nodes (7): load_meter(), show(), body(), heading_font(), heading_text(), raised(), title()

### Community 108 - "stage.rs"
Cohesion: 0.29
Nodes (6): section_header(), CanvasMode, Edit, Select, ShapeButton, StageCanvas

### Community 110 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 111 - "Vec2"
Cohesion: 0.24
Nodes (3): HANDLE, RangeSlider, XyPad

### Community 112 - "limit_pad.rs"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 118 - "sidebar"
Cohesion: 0.25
Nodes (7): sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 123 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.36
Nodes (7): format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 137 - "scene_props.rs"
Cohesion: 0.47
Nodes (5): bar_slider(), header(), properties(), single_scene(), TABS_W

### Community 143 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Properties, Second, Third

## Knowledge Gaps
- **194 isolated node(s):** `Properties`, `Second`, `Third`, `Round`, `Square` (+189 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 561 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **76 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `ZoneId` to `app.rs`, `AppState`, `runtime.rs`, `theme.rs`, `engine.rs`, `stage_canvas.rs`, `Engine`, `FixtureAction`, `Show`, `t`, `ShowError`, `stage.rs`, `Snapshot`, `DmxCraftApp`, `show.rs`, `ChannelValue`, `core/src/lib.rs`?**
  _High betweenness centrality (0.120) - this node is a cross-community bridge._
- **Are the 55 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 55 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Properties`, `Second`, `Third` to the rest of the system?**
  _194 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.08974358974358974 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `app.rs`, `AppState`, `t`, `runtime.rs`, `engine.rs`, `programmer.rs`, `ZoneId`, `FixtureAction`, `scene_props.rs`, `t`, `ChannelRef`, `stage.rs`, `Snapshot`, `DmxCraftApp`, `settings_window.rs`, `DmxFrame`, `stage.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.119) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `settings_window.rs` to `app.rs`, `AppState`, `text_dim`, `scene_props.rs`, `t`, `dialog.rs`, `fixture_settings.rs`, `super`, `stage_editor.rs`, `titlebar.rs`, `stage.rs`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `DmxCraftApp`, `programmer.rs`, `PatchGrid`, `t`, `stage_canvas.rs`, `show`, `stage.rs`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._