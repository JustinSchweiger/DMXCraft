# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~53,383 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1393 nodes · 3127 edges · 135 communities (68 shown, 67 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 126 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `babb04c0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- text_dim
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
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
- .show
- library_panel.rs
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- stage_canvas.rs
- env
- dmxcraft
- file_dialog.rs
- stage_editor.rs
- super
- DmxCraft – UI-Richtlinien
- FixtureAction
- PatchGrid
- t
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Settings
- IconButton
- DvcGoldUsb
- t
- Category
- convert.rs
- OutputKind
- show.rs
- egui
- plan.rs
- MenuItem<'a>
- OpenDmx
- qxf.rs
- fixture_settings.rs
- Stroke
- Toggle<'a>
- arc
- lock.rs
- Theme
- Settings
- core/src/lib.rs
- Show
- PlanShape
- ZoneId
- app.rs
- ShowError
- swatch.rs
- fixture_menu.rs
- en.rs
- DmxFrame
- FixtureLimits
- scene_card.rs
- patch_grid.rs
- limit_pad.rs
- Arrange
- i18n/mod.rs
- MenuPlace

## God Nodes (most connected - your core abstractions)
1. `Show` - 78 edges
2. `Engine` - 77 edges
3. `Icon` - 70 edges
4. `t()` - 65 edges
5. `DmxFrame` - 38 edges
6. `ChannelKind` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 31 edges
9. `ViewCtx` - 28 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `SetupView` --references--> `PatchGrid`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/patch_grid.rs
- `ensure_builtin_types()` --calls--> `builtin_fixture_types()`  [INFERRED]
  crates/app/src/demo.rs → crates/core/src/library.rs

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

## Communities (135 total, 67 thin omitted)

### Community 0 - "text_dim"
Cohesion: 0.22
Nodes (15): sidebar(), load_meter(), show(), accent(), accent_dim(), bg(), body(), border() (+7 more)

### Community 1 - "Engine"
Cohesion: 0.07
Nodes (35): XyPad, color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values() (+27 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+19 more)

### Community 5 - ".show"
Cohesion: 0.22
Nodes (6): Fader<'a>, FaderEvent, Changed, None, Release, truncate()

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
Nodes (45): Fullscreen, SETTLE, Step, Enter, Maximize, Action, Bank, Delete (+37 more)

### Community 11 - "dialog.rs"
Cohesion: 0.07
Nodes (11): Button, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+3 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.14
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

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
Nodes (51): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronLeft (+43 more)

### Community 23 - "ChannelKind"
Cohesion: 0.09
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 24 - ".show"
Cohesion: 0.22
Nodes (6): apply(), heading_font(), install_fonts(), surface(), DialogResult, style()

### Community 25 - "library_panel.rs"
Cohesion: 0.20
Nodes (5): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

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
Cohesion: 0.18
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), Fader, ChannelValue, Step

### Community 31 - "stage_canvas.rs"
Cohesion: 0.20
Nodes (12): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+4 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 36 - "stage_editor.rs"
Cohesion: 0.21
Nodes (7): CanvasMode, Edit, Select, ZOOM_MAX, ZOOM_MIN, set_pos(), zoom_bar()

### Community 37 - "super"
Cohesion: 0.17
Nodes (5): History, LIMIT, undo_redo_roundtrip(), addresses_are_one_based(), UNIVERSE_SIZE

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "PatchGrid"
Cohesion: 0.29
Nodes (5): Drag, GridMode, Output, Patch, PatchGrid

### Community 44 - "t"
Cohesion: 0.20
Nodes (5): AppState, Mode, Control, Setup, PatchForm

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.12
Nodes (24): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+16 more)

### Community 50 - "menu.rs"
Cohesion: 0.18
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 65 - "Settings"
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "t"
Cohesion: 0.14
Nodes (10): t(), Dialog, text_field(), RangeSlider, section_header(), group_bar(), selection_tools(), ViewCtx (+2 more)

### Community 71 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 73 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 77 - "egui"
Cohesion: 0.17
Nodes (3): LABEL_H, VALUE_H, Toggle

### Community 78 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.21
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

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
Cohesion: 0.22
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 99 - "Show"
Cohesion: 0.23
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 100 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "app.rs"
Cohesion: 0.19
Nodes (10): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView, View, FixtureSettings (+2 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 112 - "patch_grid.rs"
Cohesion: 0.18
Nodes (7): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, ROWS

### Community 117 - "limit_pad.rs"
Cohesion: 0.29
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 118 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 119 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 127 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

## Knowledge Gaps
- **183 isolated node(s):** `Enter`, `Maximize`, `Delete`, `New`, `Show` (+178 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 527 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **67 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `super`, `ZoneId`, `ShowError`, `FixtureAction`, `fixture_menu.rs`, `t`, `show.rs`, `library_panel.rs`, `ChannelValue`, `stage_canvas.rs`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Enter`, `Maximize`, `Delete` to the rest of the system?**
  _183 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.07492507492507493 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `IconButton`, `stage_editor.rs`, `t`, `titlebar.rs`, `fixture_menu.rs`, `t`, `dialog.rs`, `color_picker.rs`, `MenuItem<'a>`, `settings_window.rs`, `scene_card.rs`, `menu.rs`, `patch_grid.rs`, `fixture_settings.rs`, `Stroke`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12535612535612536 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `core/src/lib.rs`, `Show`, `runtime.rs`, `t`, `app.rs`, `fixture_menu.rs`, `t`, `DmxCraftApp`, `settings_window.rs`, `fixture_settings.rs`, `library_panel.rs`?**
  _High betweenness centrality (0.089) - this node is a cross-community bridge._