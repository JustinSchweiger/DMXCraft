# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~54,959 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1419 nodes · 3188 edges · 149 communities (76 shown, 73 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 133 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `413d1f46`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- .grid
- engine.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- titlebar.rs
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- ofl/src/lib.rs
- color_picker.rs
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- Tab
- app.rs
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- egui
- env
- dmxcraft
- file_dialog.rs
- egui
- super
- DmxCraft – UI-Richtlinien
- FixtureAction
- FixtureType
- .show
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Settings
- .check_placement
- DvcGoldUsb
- programmer.rs
- AppState
- convert.rs
- OutputKind
- show.rs
- patch_grid.rs
- plan.rs
- .group_bar
- OpenDmx
- qxf.rs
- fixture_settings.rs
- t
- icons.rs
- Toggle<'a>
- arc
- lock.rs
- Theme
- Settings
- core/src/lib.rs
- Show
- PlanShape
- ZoneId
- stage_canvas.rs
- ShowError
- swatch.rs
- en.rs
- DmxFrame
- FixtureLimits
- scene_card.rs
- text_dim
- Grip
- Category
- i18n/mod.rs
- quick_action
- FixtureType
- sidebar
- send_frame
- Arrange
- heading_text

## God Nodes (most connected - your core abstractions)
1. `Engine` - 78 edges
2. `Show` - 78 edges
3. `Icon` - 73 edges
4. `t()` - 68 edges
5. `ChannelKind` - 39 edges
6. `DmxFrame` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `save_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `save_bar()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/ui/widgets/dialog.rs
- `toolbar()` --calls--> `t()`  [INFERRED]
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

## Communities (149 total, 73 thin omitted)

### Community 0 - ".grid"
Cohesion: 0.24
Nodes (7): accent(), bg(), caption(), raised(), text(), text_faint(), DipSwitch

### Community 1 - "engine.rs"
Cohesion: 0.28
Nodes (18): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), limits_cap_dimmer_and_position(), MAX_STEP_ADVANCES_PER_TICK, MS (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+19 more)

### Community 5 - "Engine"
Cohesion: 0.09
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 6 - "titlebar.rs"
Cohesion: 0.07
Nodes (43): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+35 more)

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
Cohesion: 0.07
Nodes (49): Fullscreen, SETTLE, Step, Enter, Maximize, color_picker(), color_swatches(), select_all() (+41 more)

### Community 11 - "dialog.rs"
Cohesion: 0.07
Nodes (12): Button, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+4 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.15
Nodes (11): hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H, Swatch, Auto (+3 more)

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
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 25 - "app.rs"
Cohesion: 0.22
Nodes (9): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView, View, FixtureSettings (+1 more)

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
Cohesion: 0.36
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 31 - "egui"
Cohesion: 0.18
Nodes (7): Dialog, Fader, LABEL_H, VALUE_H, LimitPad, paint_fixture(), SIZES

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 36 - "egui"
Cohesion: 0.16
Nodes (9): section_header(), CanvasMode, Edit, Select, ZOOM_MAX, ZOOM_MIN, set_pos(), StageEditorView (+1 more)

### Community 37 - "super"
Cohesion: 0.29
Nodes (3): History, LIMIT, undo_redo_roundtrip()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "FixtureType"
Cohesion: 0.26
Nodes (4): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 44 - ".show"
Cohesion: 0.16
Nodes (8): border(), Fader<'a>, FaderEvent, Changed, None, Release, truncate(), separator()

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.24
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (17): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+9 more)

### Community 50 - "menu.rs"
Cohesion: 0.11
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 65 - "Settings"
Cohesion: 0.20
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 66 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.27
Nodes (14): color_bar(), current(), fader(), fader_groups(), FaderGroup, items_mode(), kind_color(), mode_switch() (+6 more)

### Community 71 - "AppState"
Cohesion: 0.15
Nodes (9): AppState, Mode, Control, Setup, PatchForm, ProgrammerMode, Blind, Edit (+1 more)

### Community 73 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.24
Nodes (12): UNIVERSE_SIZE, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.15
Nodes (14): MenuPlace, Patch, Stage, StageEditor, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS (+6 more)

### Community 78 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 79 - ".group_bar"
Cohesion: 0.26
Nodes (9): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.11
Nodes (6): IconButton, RangeSlider, Segmented, ShapeButton, Toggle, XyPad

### Community 89 - "t"
Cohesion: 0.26
Nodes (3): t(), ZonesView, Windows

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "lock.rs"
Cohesion: 0.23
Nodes (11): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+3 more)

### Community 96 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 98 - "core/src/lib.rs"
Cohesion: 0.24
Nodes (7): ChannelValue, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Step, UNIVERSES

### Community 99 - "Show"
Cohesion: 0.25
Nodes (3): Fixture, Group, Show

### Community 100 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (12): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_shape_icon() (+4 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 108 - "DmxFrame"
Cohesion: 0.16
Nodes (3): ViewCtx, addresses_are_one_based(), DmxFrame

### Community 111 - "scene_card.rs"
Cohesion: 0.31
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 112 - "text_dim"
Cohesion: 0.19
Nodes (9): load_meter(), show(), accent_dim(), body(), heading_font(), raised_hover(), text_dim(), title() (+1 more)

### Community 117 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 118 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 119 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 133 - "quick_action"
Cohesion: 0.25
Nodes (7): Quick, quick_action(), Center, Off, On, caption(), range_row()

### Community 137 - "FixtureType"
Cohesion: 0.24
Nodes (3): ChannelDef, FixtureType, ft()

### Community 138 - "sidebar"
Cohesion: 0.33
Nodes (5): sidebar(), apply(), install_fonts(), surface(), style()

### Community 139 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 140 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

## Knowledge Gaps
- **188 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+183 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 539 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **73 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `engine.rs`, `core/src/lib.rs`, `.check_placement`, `theme.rs`, `super`, `stage_canvas.rs`, `Engine`, `FixtureAction`, `FixtureType`, `t`, `ZoneId`, `show.rs`, `ShowError`, `runtime.rs`, `ChannelValue`, `egui`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Are the 52 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 52 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _188 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `engine.rs`, `core/src/lib.rs`, `Show`, `runtime.rs`, `programmer.rs`, `stage_canvas.rs`, `quick_action`, `FixtureType`, `t`, `DmxFrame`, `DmxCraftApp`, `settings_window.rs`, `.group_bar`, `app.rs`, `egui`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09848484848484848 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `egui`, `programmer.rs`, `titlebar.rs`, `t`, `dialog.rs`, `color_picker.rs`, `patch_grid.rs`, `scene_card.rs`, `settings_window.rs`, `menu.rs`, `fixture_settings.rs`, `Tab`, `icons.rs`, `egui`?**
  _High betweenness centrality (0.075) - this node is a cross-community bridge._