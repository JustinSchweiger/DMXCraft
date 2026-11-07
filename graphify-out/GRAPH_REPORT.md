# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~65,422 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1575 nodes · 3600 edges · 145 communities (75 shown, 70 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 152 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9bb0b650`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Tab
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
- ofl/src/lib.rs
- egui
- i18n/mod.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- qxf.rs
- stage.rs
- engine.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- patch_grid.rs
- env
- dmxcraft
- qlc-bundle.rs
- color_picker.rs
- file_dialog.rs
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- .ui
- fader.rs
- super
- DmxCraft – Hinweise für KI-Assistenten
- t
- settings_window.rs
- DmxFrame
- core/src/lib.rs
- DvcGoldUsb
- programmer.rs
- Settings
- Show
- knob.rs
- OutputKind
- Snapshot
- Fullscreen
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- arc
- FixtureLimits
- app.rs
- PatchGrid
- ViewCtx
- PlanShape
- Category
- ShowError
- ZoneId
- file_dialog.rs
- show
- EffectKind
- en.rs
- .fade_state
- Arrange
- limit_pad.rs
- sidebar
- super
- Theme
- time_field.rs
- plan.rs
- scene_props.rs
- dip.rs
- stage_canvas.rs
- IconButton
- ChannelKind

## God Nodes (most connected - your core abstractions)
1. `Engine` - 113 edges
2. `Show` - 83 edges
3. `Icon` - 77 edges
4. `t()` - 76 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 33 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `color_swatches()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/color_picker.rs
- `column()` --calls--> `t()`  [INFERRED]
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

## Communities (145 total, 70 thin omitted)

### Community 0 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 1 - "AppState"
Cohesion: 0.10
Nodes (12): edit_preview(), AppState, LibraryDrag, Mode, Control, Setup, PatchForm, drag_ghost() (+4 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+19 more)

### Community 5 - "Engine"
Cohesion: 0.08
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 6 - "widgets/mod.rs"
Cohesion: 0.19
Nodes (11): accent(), apply(), bg(), border(), caption(), install_fonts(), raised(), raised_hover() (+3 more)

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
Nodes (43): select_all(), programmer_values(), Action, Bank, Delete, DuplicateScene, Flash, Jump (+35 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 12 - "egui"
Cohesion: 0.14
Nodes (8): RangeSlider, section_header(), ZOOM_MAX, ZOOM_MIN, Toggle, set_pos(), StageEditorView, zoom_bar()

### Community 13 - "i18n/mod.rs"
Cohesion: 0.10
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
Nodes (55): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+47 more)

### Community 23 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 24 - "stage.rs"
Cohesion: 0.24
Nodes (9): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+1 more)

### Community 25 - "engine.rs"
Cohesion: 0.24
Nodes (27): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+19 more)

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

### Community 31 - "patch_grid.rs"
Cohesion: 0.15
Nodes (10): Dialog, DipSwitch, LimitPad, Segmented, Swatch, XyPad, CELL_H_MAX, CELL_H_MIN (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 36 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 37 - "file_dialog.rs"
Cohesion: 0.38
Nodes (4): Pending, Purpose, OpenShow, SaveShowAs

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.08
Nodes (30): CanvasMode, Edit, Select, StageCanvas, apply(), FixtureAction, AddToGroup, CopyLayout (+22 more)

### Community 44 - "fader.rs"
Cohesion: 0.11
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+6 more)

### Community 45 - "super"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.17
Nodes (18): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+10 more)

### Community 66 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (16): BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode, Loop (+8 more)

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.11
Nodes (30): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+22 more)

### Community 70 - "Settings"
Cohesion: 0.16
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 77 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.05
Nodes (18): Button, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+10 more)

### Community 90 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 95 - "app.rs"
Cohesion: 0.31
Nodes (5): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, StageView

### Community 96 - "PatchGrid"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 97 - "ViewCtx"
Cohesion: 0.16
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
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 102 - "file_dialog.rs"
Cohesion: 0.40
Nodes (3): save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 103 - "show"
Cohesion: 0.28
Nodes (6): load_meter(), show(), accent_dim(), body(), heading_font(), heading_text()

### Community 104 - "EffectKind"
Cohesion: 0.17
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 108 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 111 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 112 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 118 - "sidebar"
Cohesion: 0.22
Nodes (8): sidebar(), Tab, .ALL, General, Midi, Outputs, Project, surface()

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.29
Nodes (8): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 136 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 137 - "scene_props.rs"
Cohesion: 0.18
Nodes (15): advanced(), bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog, header(), properties() (+7 more)

### Community 140 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (13): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+5 more)

### Community 149 - "ChannelKind"
Cohesion: 0.08
Nodes (21): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+13 more)

## Knowledge Gaps
- **212 isolated node(s):** `All`, `Delete`, `New`, `Show`, `Properties` (+207 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 584 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **70 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `AppState`, `widgets/mod.rs`, `scene_props.rs`, `t`, `egui`, `i18n/mod.rs`, `stage_canvas.rs`, `titlebar.rs`, `stage.rs`, `patch_grid.rs`, `color_picker.rs`, `file_dialog.rs`, `FixtureAction`, `.ui`, `fader.rs`, `settings_window.rs`, `programmer.rs`, `Settings`, `dialog.rs`, `app.rs`, `PatchGrid`, `ViewCtx`, `show`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Are the 58 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 58 INFERRED edges - model-reasoned connections that need verification._
- **What connects `All`, `Delete`, `New` to the rest of the system?**
  _212 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.10160427807486631 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `ViewCtx`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `FixtureAction`, `scene_props.rs`, `t`, `Show`, `Snapshot`, `.fade_state`, `t`, `settings_window.rs`, `stage.rs`, `engine.rs`, `app.rs`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `Engine`, `ZoneId`, `ShowError`, `FixtureAction`, `t`, `Snapshot`, `stage_canvas.rs`, `t`, `ChannelKind`, `show.rs`, `engine.rs`, `ChannelValue`, `app.rs`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._