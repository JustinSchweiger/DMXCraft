# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 81 files · ~65,289 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1629 nodes · 3738 edges · 159 communities (81 shown, 78 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `767ff4bb`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- ChannelKind
- text_dim
- steps
- DmxCraftApp
- OutputError
- t
- Box<dyn serialport::SerialPort>
- file_dialog.rs
- color_picker.rs
- jobs
- steps
- DmxCraft
- menu.rs
- announce
- build-local-artifacts
- Icon
- AppState
- ofl/src/lib.rs
- scene_card.rs
- host
- plan
- with
- ensure_linux_menu_entry
- Value
- swatch.rs
- env
- dmxcraft
- stage.rs
- qxf.rs
- qlc-bundle.rs
- DmxCraft – UI-Richtlinien
- egui
- FixtureAction
- drag_number
- fader.rs
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- t
- settings_window.rs
- arc
- PlanShape
- core/src/lib.rs
- programmer.rs
- .group_bar
- titlebar.rs
- Show
- knob.rs
- OutputKind
- .check_placement
- zones.rs
- Arrange
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
- Dialog<'a>
- Knob<'a>
- ShowError
- ZoneId
- EffectKind
- en.rs
- Fullscreen
- show
- range_slider.rs
- .ui
- DvcGoldUsb
- lock.rs
- fixture_menu.rs
- time_field.rs
- plan.rs
- scene_props.rs
- app.rs
- Category
- stage_canvas.rs
- quick.rs
- DmxFrame
- T
- dip.rs
- IconButton
- PickTarget
- Tab
- BankScope
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Engine` - 116 edges
2. `Icon` - 87 edges
3. `Show` - 84 edges
4. `t()` - 74 edges
5. `ChannelKind` - 42 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 35 edges
9. `ViewCtx` - 34 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `body()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/setup/fixture_settings.rs → crates/app/src/i18n/mod.rs
- `body()` --calls--> `quick_action()`  [INFERRED]
  crates/app/src/views/setup/fixture_settings.rs → crates/app/src/views/quick.rs

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

## Communities (159 total, 78 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.06
Nodes (40): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered() (+32 more)

### Community 1 - "AppState"
Cohesion: 0.16
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (31): accent_hover(), active(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+23 more)

### Community 5 - "ChannelKind"
Cohesion: 0.07
Nodes (21): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+13 more)

### Community 6 - "text_dim"
Cohesion: 0.19
Nodes (22): load_meter(), show(), accent(), accent_dim(), apply(), bg(), body(), border() (+14 more)

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
Cohesion: 0.09
Nodes (45): select_all(), CARD_H, programmer_values(), Action, Bank, Delete, DuplicateScene, Flash (+37 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "color_picker.rs"
Cohesion: 0.13
Nodes (14): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+6 more)

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
Cohesion: 0.20
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (65): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+57 more)

### Community 23 - "AppState"
Cohesion: 0.17
Nodes (6): AppState, Focus, Bank, Effect, Scenes, PatchForm

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 25 - "scene_card.rs"
Cohesion: 0.23
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

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
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "stage.rs"
Cohesion: 0.27
Nodes (8): CanvasMode, Edit, Select, StageCanvas, MenuPlace, Patch, Stage, StageEditor

### Community 36 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 37 - "qlc-bundle.rs"
Cohesion: 0.18
Nodes (9): RECENT_MAX, git(), main(), run(), dedupe_keys(), git(), main(), qxf_files() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "egui"
Cohesion: 0.11
Nodes (14): DipSwitch, Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented (+6 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (22): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+14 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+7 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.08
Nodes (27): cable_belongs_to_one_universe(), default_instance_name(), Settings, full_combo(), general(), HEIGHT, hint(), midi() (+19 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (10): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx, Static (+2 more)

### Community 69 - "programmer.rs"
Cohesion: 0.24
Nodes (21): category_list(), color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks() (+13 more)

### Community 70 - ".group_bar"
Cohesion: 0.21
Nodes (10): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+2 more)

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 72 - "Show"
Cohesion: 0.22
Nodes (3): Fixture, Group, Show

### Community 73 - "knob.rs"
Cohesion: 0.29
Nodes (3): Knob, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 77 - "zones.rs"
Cohesion: 0.24
Nodes (4): t(), section_header(), ZonesView, Windows

### Community 78 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.12
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 95 - "ViewCtx"
Cohesion: 0.19
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 96 - "PatchGrid"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.16
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 104 - "EffectKind"
Cohesion: 0.17
Nodes (11): effect_tiles(), EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+3 more)

### Community 108 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 109 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 117 - "range_slider.rs"
Cohesion: 0.33
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 123 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 128 - "fixture_menu.rs"
Cohesion: 0.25
Nodes (5): Mode, Control, Setup, apply(), SIZES

### Community 133 - "time_field.rs"
Cohesion: 0.27
Nodes (7): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS

### Community 136 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 137 - "scene_props.rs"
Cohesion: 0.16
Nodes (20): advanced(), banks_of(), bar_slider(), effect_header(), FX_EXT, FxDialog, header(), pick_tile() (+12 more)

### Community 138 - "app.rs"
Cohesion: 0.25
Nodes (5): ControlViews, edit_preview(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 140 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+6 more)

### Community 142 - "quick.rs"
Cohesion: 0.15
Nodes (10): ProgrammerMode, Blind, Edit, Live, items_mode(), Quick, quick_action(), Center (+2 more)

### Community 143 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 145 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 146 - "IconButton"
Cohesion: 0.16
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 147 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 149 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 150 - "BankScope"
Cohesion: 0.29
Nodes (6): BankScope, All, Bank, Banks, Off, OutsideBank

## Knowledge Gaps
- **227 isolated node(s):** `Select`, `Edit`, `Bank`, `Effect`, `Scenes` (+222 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 616 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **78 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `fixture_menu.rs`, `AppState`, `stage.rs`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `.group_bar`, `Show`, `scene_props.rs`, `app.rs`, `t`, `quick.rs`, `t`, `settings_window.rs`, `ViewCtx`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Are the 56 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 56 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `Bank` to the rest of the system?**
  _227 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.057838123787691764 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `fixture_menu.rs`, `stage.rs`, `programmer.rs`, `titlebar.rs`, `egui`, `scene_props.rs`, `t`, `color_picker.rs`, `settings_window.rs`, `IconButton`, `menu.rs`, `dialog.rs`, `.paint`, `MenuItem<'a>`, `scene_card.rs`, `ViewCtx`?**
  _High betweenness centrality (0.112) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `fixture_menu.rs`, `stage.rs`, `core/src/lib.rs`, `theme.rs`, `ChannelKind`, `ZoneId`, `ShowError`, `FixtureAction`, `runtime.rs`, `app.rs`, `t`, `.check_placement`, `stage_canvas.rs`, `t`, `show.rs`, `ChannelValue`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._