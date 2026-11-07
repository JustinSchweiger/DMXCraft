# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~64,641 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1609 nodes · 3650 edges · 153 communities (81 shown, 72 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 148 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `0f698ebd`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- text
- steps
- DmxCraftApp
- OutputError
- t
- ofl/src/lib.rs
- fixture_settings.rs
- AppState
- jobs
- steps
- DmxCraft
- menu.rs
- announce
- build-local-artifacts
- Icon
- qxf.rs
- stage.rs
- scene_card.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- egui
- env
- dmxcraft
- qlc-bundle.rs
- BankScope
- settings_window.rs
- DmxCraft – UI-Richtlinien
- limit_pad.rs
- FixtureAction
- widgets/mod.rs
- fader.rs
- super
- DmxCraft – Hinweise für KI-Assistenten
- t
- general
- arc
- Pos2
- core/src/lib.rs
- programmer.rs
- Settings
- titlebar.rs
- Show
- knob.rs
- OutputKind
- .check_placement
- Fullscreen
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- MenuAction
- FixtureLimits
- ViewCtx
- PatchGrid
- Snapshot
- PlanShape
- ShowError
- ZoneId
- EffectKind
- en.rs
- Dialog<'a>
- show
- stage_editor.rs
- Category
- DvcGoldUsb
- sidebar
- time_field.rs
- Theme
- scene_props.rs
- app.rs
- ProgrammerMode
- stage_canvas.rs
- quick.rs
- DmxFrame
- .ui
- IconButton
- ChannelKind
- Ui
- body
- PickTarget
- Tab
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Engine` - 116 edges
2. `Icon` - 84 edges
3. `Show` - 83 edges
4. `t()` - 78 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 35 edges
9. `ViewCtx` - 33 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `form()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `general()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `outputs()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (153 total, 72 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.10
Nodes (6): Layer, Live, Programmer, Engine, lerp(), ChannelRef

### Community 1 - "AppState"
Cohesion: 0.13
Nodes (7): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (26): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+18 more)

### Community 5 - "Engine"
Cohesion: 0.12
Nodes (32): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), FadeState, In, Out, first_step_fades_intensity_in() (+24 more)

### Community 6 - "text"
Cohesion: 0.21
Nodes (17): load_meter(), show(), accent(), accent_dim(), apply(), bg(), border(), caption() (+9 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (58): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+50 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 12 - "fixture_settings.rs"
Cohesion: 0.21
Nodes (3): section_header(), caption(), range_row()

### Community 13 - "AppState"
Cohesion: 0.13
Nodes (10): edit_preview(), AppState, Focus, Bank, Effect, Scenes, Mode, Control (+2 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (62): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+54 more)

### Community 23 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 24 - "stage.rs"
Cohesion: 0.23
Nodes (10): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+2 more)

### Community 25 - "scene_card.rs"
Cohesion: 0.21
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

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
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - "egui"
Cohesion: 0.13
Nodes (5): ICON_FAMILY, ICON_FILL_FAMILY, RangeSlider, Swatch, XyPad

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 36 - "BankScope"
Cohesion: 0.29
Nodes (6): BankScope, All, Bank, Banks, Off, OutsideBank

### Community 37 - "settings_window.rs"
Cohesion: 0.22
Nodes (7): HEIGHT, SCALES, SettingsCx, SIDEBAR_W, TAB_H, WIDTH, UsbDmxPort

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "limit_pad.rs"
Cohesion: 0.15
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+18 more)

### Community 41 - "widgets/mod.rs"
Cohesion: 0.50
Nodes (4): CanvasMode, Edit, Select, StageCanvas

### Community 44 - "fader.rs"
Cohesion: 0.11
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+6 more)

### Community 45 - "super"
Cohesion: 0.05
Nodes (35): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+27 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "t"
Cohesion: 0.12
Nodes (8): DmxCraftApp, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 48 - "general"
Cohesion: 0.36
Nodes (8): full_combo(), general(), hint(), midi(), outputs(), project(), section(), SettingsChanges

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 66 - "core/src/lib.rs"
Cohesion: 0.15
Nodes (10): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx, Static (+2 more)

### Community 69 - "programmer.rs"
Cohesion: 0.25
Nodes (20): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+12 more)

### Community 70 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 72 - "Show"
Cohesion: 0.25
Nodes (3): Fixture, Group, Show

### Community 73 - "knob.rs"
Cohesion: 0.21
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 77 - "Fullscreen"
Cohesion: 0.20
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 95 - "ViewCtx"
Cohesion: 0.17
Nodes (3): ViewCtx, ZonesView, Windows

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 98 - "PlanShape"
Cohesion: 0.13
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 104 - "EffectKind"
Cohesion: 0.18
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 109 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 111 - "stage_editor.rs"
Cohesion: 0.16
Nodes (10): ZOOM_MAX, ZOOM_MIN, Arrange, AlignH, AlignV, Grid, Reset, Row (+2 more)

### Community 118 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 128 - "sidebar"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 133 - "time_field.rs"
Cohesion: 0.29
Nodes (8): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 136 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 137 - "scene_props.rs"
Cohesion: 0.16
Nodes (21): t(), advanced(), banks_of(), bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog (+13 more)

### Community 138 - "app.rs"
Cohesion: 0.22
Nodes (9): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, ScenePropsView, View, FixtureSettings, SetupView (+1 more)

### Community 140 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (11): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+3 more)

### Community 142 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 149 - "ChannelKind"
Cohesion: 0.08
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 151 - "body"
Cohesion: 0.25
Nodes (4): body(), heading_font(), heading_text(), DialogResult

### Community 152 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 153 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

## Knowledge Gaps
- **225 isolated node(s):** `General`, `Outputs`, `Midi`, `Project`, `Dimmer` (+220 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 606 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **72 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `AppState`, `runtime.rs`, `theme.rs`, `Engine`, `app.rs`, `t`, `stage_canvas.rs`, `stage.rs`, `FixtureAction`, `widgets/mod.rs`, `t`, `core/src/lib.rs`, `.check_placement`, `show.rs`, `ChannelValue`, `Snapshot`, `ShowError`, `ZoneId`?**
  _High betweenness centrality (0.120) - this node is a cross-community bridge._
- **Are the 59 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 59 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Outputs`, `Midi` to the rest of the system?**
  _225 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09682539682539683 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Snapshot`, `AppState`, `core/src/lib.rs`, `runtime.rs`, `settings_window.rs`, `programmer.rs`, `Engine`, `FixtureAction`, `scene_props.rs`, `app.rs`, `t`, `Show`, `AppState`, `quick.rs`, `t`, `general`, `stage.rs`, `ViewCtx`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.1339031339031339 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `scene_props.rs` to `AppState`, `text`, `app.rs`, `t`, `fixture_settings.rs`, `stage_canvas.rs`, `body`, `stage.rs`, `settings_window.rs`, `FixtureAction`, `fader.rs`, `super`, `t`, `general`, `programmer.rs`, `Settings`, `titlebar.rs`, `dialog.rs`, `ViewCtx`, `PatchGrid`, `PlanShape`, `stage_editor.rs`?**
  _High betweenness centrality (0.082) - this node is a cross-community bridge._