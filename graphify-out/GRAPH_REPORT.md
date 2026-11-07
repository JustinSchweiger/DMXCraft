# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 81 files · ~64,765 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1623 nodes · 3736 edges · 147 communities (73 shown, 74 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 143 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c9a8364b`
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
- convert.rs
- jobs
- steps
- DmxCraft
- menu.rs
- announce
- build-local-artifacts
- Icon
- FixtureType
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
- general
- arc
- PlanShape
- core/src/lib.rs
- programmer.rs
- Settings
- titlebar.rs
- Show
- knob.rs
- OutputKind
- .check_placement
- Fullscreen
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
- Grip
- Knob<'a>
- ShowError
- ZoneId
- EffectKind
- en.rs
- Dialog<'a>
- show
- DvcGoldUsb
- sidebar
- time_field.rs
- scene_props.rs
- app.rs
- stage_canvas.rs
- quick.rs
- DmxFrame
- fixture_settings.rs
- IconButton
- Ui
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
9. `ViewCtx` - 33 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `properties()` --calls--> `time_field()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/time_field.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs

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

## Communities (147 total, 74 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.06
Nodes (40): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered() (+32 more)

### Community 1 - "AppState"
Cohesion: 0.08
Nodes (15): AppState, Focus, Bank, Effect, Scenes, LibraryDrag, Mode, Control (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.06
Nodes (34): accent_hover(), active(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+26 more)

### Community 5 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 6 - "text_dim"
Cohesion: 0.25
Nodes (17): load_meter(), show(), accent(), accent_dim(), apply(), bg(), body(), border() (+9 more)

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
Cohesion: 0.06
Nodes (58): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+50 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.13
Nodes (8): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), Segmented<'a, T>

### Community 13 - "convert.rs"
Cohesion: 0.20
Nodes (13): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), OflError, Invalid, Json (+5 more)

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
Nodes (65): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+57 more)

### Community 23 - "FixtureType"
Cohesion: 0.15
Nodes (4): RECENT_MAX, ChannelDef, FixtureType, ft()

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 25 - "scene_card.rs"
Cohesion: 0.25
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
Cohesion: 0.17
Nodes (12): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Matrix, natural_cmp() (+4 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "stage.rs"
Cohesion: 0.16
Nodes (10): CanvasMode, Edit, Select, StageCanvas, StageInteraction, MenuPlace, Patch, Stage (+2 more)

### Community 36 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 37 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "egui"
Cohesion: 0.13
Nodes (8): DipSwitch, LimitPad, section_header(), Segmented, CELL_H_MAX, CELL_H_MIN, COLUMNS, ROWS

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+24 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+7 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.05
Nodes (35): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+27 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "general"
Cohesion: 0.18
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 65 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.09
Nodes (16): BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode, Loop (+8 more)

### Community 69 - "programmer.rs"
Cohesion: 0.11
Nodes (37): ProgrammerMode, Blind, Edit, Live, XyPad, Category, All, Beam (+29 more)

### Community 70 - "Settings"
Cohesion: 0.19
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), Settings

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 72 - "Show"
Cohesion: 0.25
Nodes (3): Fixture, Group, Show

### Community 73 - "knob.rs"
Cohesion: 0.29
Nodes (3): Knob, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 77 - "Fullscreen"
Cohesion: 0.20
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, drag_ghost()

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
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 95 - "ViewCtx"
Cohesion: 0.12
Nodes (10): apply(), View, ViewCtx, FixtureSettings, SetupView, set_pos(), StageEditorView, tool_sep() (+2 more)

### Community 96 - "PatchGrid"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 97 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.16
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 104 - "EffectKind"
Cohesion: 0.18
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 109 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 128 - "sidebar"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 133 - "time_field.rs"
Cohesion: 0.27
Nodes (7): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS

### Community 137 - "scene_props.rs"
Cohesion: 0.13
Nodes (24): advanced(), banks_of(), bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog, header() (+16 more)

### Community 138 - "app.rs"
Cohesion: 0.16
Nodes (10): ControlViews, edit_preview(), LockDialog, OUTPUT_HZ, ProgrammerView, ScenePropsView, Tab, Effects (+2 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.20
Nodes (13): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+5 more)

### Community 142 - "quick.rs"
Cohesion: 0.23
Nodes (7): Quick, quick_action(), Center, Off, On, caption(), range_row()

### Community 146 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 150 - "Ui"
Cohesion: 0.31
Nodes (3): DialogResult, Form, form()

## Knowledge Gaps
- **227 isolated node(s):** `Navy`, `Gray`, `None`, `Release`, `Select` (+222 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 612 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **74 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `AppState`, `stage.rs`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `FixtureAction`, `scene_props.rs`, `app.rs`, `t`, `Show`, `quick.rs`, `t`, `general`, `DmxFrame`, `ViewCtx`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Are the 56 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 56 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Navy`, `Gray`, `None` to the rest of the system?**
  _227 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.05676704742125303 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `stage.rs`, `core/src/lib.rs`, `theme.rs`, `ZoneId`, `ShowError`, `runtime.rs`, `FixtureAction`, `app.rs`, `t`, `.check_placement`, `stage_canvas.rs`, `Fullscreen`, `t`, `show.rs`, `FixtureType`, `ChannelValue`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.08333333333333333 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `sidebar`, `stage.rs`, `programmer.rs`, `titlebar.rs`, `egui`, `scene_props.rs`, `t`, `general`, `fixture_settings.rs`, `IconButton`, `menu.rs`, `dialog.rs`, `MenuItem<'a>`, `scene_card.rs`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._