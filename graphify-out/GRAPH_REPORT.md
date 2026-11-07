# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~56,992 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1437 nodes · 3209 edges · 146 communities (67 shown, 79 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 133 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `baae47ad`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- AppState
- Engine
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
- History
- plan.rs
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- stage.rs
- color_picker.rs
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- Vec2
- env
- dmxcraft
- menu.rs
- Arrange
- t
- DmxCraft – UI-Richtlinien
- FixtureAction
- MenuAction
- fader.rs
- ofl/src/lib.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- patch_grid.rs
- send_frame
- DvcGoldUsb
- programmer.rs
- convert.rs
- OutputKind
- show.rs
- Dialog<'a>
- qxf.rs
- OpenDmx
- qlc-bundle.rs
- IconButton
- MenuItem<'a>
- icons.rs
- Toggle<'a>
- arc
- lock.rs
- bank_menu
- TitleInfo
- core/src/lib.rs
- Show
- PlanShape
- ZoneId
- stage_canvas.rs
- ShowError
- dip.rs
- en.rs
- .check_placement
- FixtureLimits
- widgets/mod.rs
- scene_card.rs
- T
- Grip
- .show
- i18n/mod.rs
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Engine` - 83 edges
2. `Show` - 78 edges
3. `Icon` - 73 edges
4. `t()` - 67 edges
5. `ChannelKind` - 43 edges
6. `DmxFrame` - 39 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `color_picker()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/ui/widgets/color_picker.rs → crates/app/src/ui/widgets/dialog.rs
- `color_swatches()` --calls--> `t()`  [INFERRED]
  crates/app/src/ui/widgets/color_picker.rs → crates/app/src/i18n/mod.rs

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

## Communities (146 total, 79 thin omitted)

### Community 0 - "AppState"
Cohesion: 0.12
Nodes (10): ControlViews, edit_preview(), LockDialog, OUTPUT_HZ, AppState, Mode, Control, Setup (+2 more)

### Community 1 - "Engine"
Cohesion: 0.15
Nodes (6): text_field(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), Engine

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.06
Nodes (54): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+46 more)

### Community 5 - "Engine"
Cohesion: 0.12
Nodes (23): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), lerp(), limits_cap_dimmer_and_position(), MAX_STEP_ADVANCES_PER_TICK (+15 more)

### Community 6 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

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
Cohesion: 0.13
Nodes (31): select_all(), programmer_values(), Action, Bank, Delete, DuplicateScene, Jump, Mark (+23 more)

### Community 11 - "dialog.rs"
Cohesion: 0.13
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+2 more)

### Community 12 - "History"
Cohesion: 0.29
Nodes (3): History, LIMIT, undo_redo_roundtrip()

### Community 13 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

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
Nodes (21): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+13 more)

### Community 24 - "stage.rs"
Cohesion: 0.22
Nodes (11): StageInteraction, group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+3 more)

### Community 25 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

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
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 31 - "Vec2"
Cohesion: 0.17
Nodes (4): HANDLE, RangeSlider, Swatch, XyPad

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "menu.rs"
Cohesion: 0.20
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 36 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 37 - "t"
Cohesion: 0.08
Nodes (20): t(), section_header(), fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor (+12 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.10
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (11): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, NUM_H (+3 more)

### Community 45 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.09
Nodes (13): DmxCraftApp, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension() (+5 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.07
Nodes (28): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings, full_combo(), general(), HEIGHT, hint() (+20 more)

### Community 50 - "patch_grid.rs"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 65 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.07
Nodes (39): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+31 more)

### Community 73 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.24
Nodes (12): UNIVERSE_SIZE, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 79 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 90 - "icons.rs"
Cohesion: 0.27
Nodes (5): fine_mark(), paint_symbol(), file_icons(), paint_fixture(), paint_shape_icon()

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 96 - "bank_menu"
Cohesion: 0.31
Nodes (11): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+3 more)

### Community 97 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 98 - "core/src/lib.rs"
Cohesion: 0.22
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 100 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.18
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (10): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, ShapeButton, WORLD (+2 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 104 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 108 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 110 - "widgets/mod.rs"
Cohesion: 0.50
Nodes (4): CanvasMode, Edit, Select, StageCanvas

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "Grip"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 119 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

## Knowledge Gaps
- **188 isolated node(s):** `None`, `Release`, `Control`, `Setup`, `Auto` (+183 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 549 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **79 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `bank_menu`, `Engine`, `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `t`, `stage_canvas.rs`, `Engine`, `FixtureAction`, `ZoneId`, `t`, `ShowError`, `History`, `show.rs`, `widgets/mod.rs`, `.check_placement`, `ChannelKind`, `ChannelValue`?**
  _High betweenness centrality (0.099) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `None`, `Release`, `Control` to the rest of the system?**
  _188 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.11688311688311688 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `core/src/lib.rs`, `Show`, `runtime.rs`, `programmer.rs`, `t`, `Engine`, `t`, `DmxCraftApp`, `settings_window.rs`, `stage.rs`?**
  _High betweenness centrality (0.086) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `MenuItem<'a>`, `menu.rs`, `programmer.rs`, `titlebar.rs`, `t`, `t`, `dialog.rs`, `widgets/mod.rs`, `scene_card.rs`, `settings_window.rs`, `patch_grid.rs`, `IconButton`, `stage.rs`, `color_picker.rs`, `icons.rs`?**
  _High betweenness centrality (0.077) - this node is a cross-community bridge._