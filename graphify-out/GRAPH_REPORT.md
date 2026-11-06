# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 74 files · ~47,042 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 9 file(s) not represented in the graph (top: (none) 3, .ico 2, .ttf 1)

## Summary
- 1308 nodes · 2987 edges · 119 communities (65 shown, 54 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 122 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `4e664dd8`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- file_dialog.rs
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- Show
- steps
- DmxCraftApp
- OutputError
- AppState
- dialog.rs
- app.rs
- plan.rs
- jobs
- steps
- DmxCraft
- ViewCtx
- announce
- build-local-artifacts
- Icon
- ChannelKind
- fixture_settings.rs
- t
- host
- plan
- with
- ensure_linux_menu_entry
- patch_grid.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- show.rs
- MenuItem<'a>
- DmxCraft – UI-Richtlinien
- ChannelValue
- egui
- Scene
- ofl/src/lib.rs
- ShowError
- DmxCraft – Hinweise für KI-Assistenten
- sense
- settings_window.rs
- menu.rs
- Settings
- swatch.rs
- DvcGoldUsb
- OpenDmx
- lock.rs
- titlebar.rs
- FixtureLimits
- convert.rs
- OutputKind
- dip.rs
- IconButton
- accent_dim
- Segmented
- LibraryPanel
- Dialog<'a>
- DmxFrame
- Stroke
- Toggle<'a>
- Arrange
- super
- ZoneId
- FixtureType
- Value
- Tab
- Category
- arc
- text_dim
- Matrix
- MenuPlace
- en.rs
- .check_placement
- ofl-bundle.rs
- Grip
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 80 edges
2. `Show` - 65 edges
3. `Icon` - 63 edges
4. `t()` - 56 edges
5. `DmxFrame` - 38 edges
6. `text_dim()` - 37 edges
7. `ChannelKind` - 36 edges
8. `DmxCraftApp` - 34 edges
9. `AppState` - 33 edges
10. `ViewCtx` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `general()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs
- `midi()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs
- `outputs()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (119 total, 54 thin omitted)

### Community 0 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 1 - "Engine"
Cohesion: 0.06
Nodes (43): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values(), save_bar() (+35 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (31): accent_hover(), active(), ACTIVE, c(), DANGER, DIP_BODY, fixture_off(), FS_BODY (+23 more)

### Community 5 - ".show"
Cohesion: 0.20
Nodes (6): Fader<'a>, FaderEvent, Changed, None, Release, truncate()

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "AppState"
Cohesion: 0.16
Nodes (6): AppState, Mode, Control, Setup, PatchForm, SIZES

### Community 11 - "dialog.rs"
Cohesion: 0.09
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H (+4 more)

### Community 12 - "app.rs"
Cohesion: 0.31
Nodes (6): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView

### Community 13 - "plan.rs"
Cohesion: 0.31
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

### Community 19 - "ViewCtx"
Cohesion: 0.15
Nodes (10): apply(), View, ViewCtx, FixtureSettings, SetupView, ScenesView, set_pos(), StageEditorView (+2 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (47): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+39 more)

### Community 23 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "fixture_settings.rs"
Cohesion: 0.21
Nodes (7): caption(), Quick, quick_action(), Center, Off, On, range_row()

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "patch_grid.rs"
Cohesion: 0.18
Nodes (11): fixture_bar(), CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output (+3 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (13): Segmented, BOX, CanvasMode, Edit, Select, fixture_pos(), fixture_rect(), live_colors() (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.15
Nodes (10): ShapeButton, PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead (+2 more)

### Community 36 - "show.rs"
Cohesion: 0.30
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 40 - "egui"
Cohesion: 0.29
Nodes (3): section_header(), group_bar(), selection_tools()

### Community 43 - "Scene"
Cohesion: 0.16
Nodes (8): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, UNIVERSES

### Community 44 - "ofl/src/lib.rs"
Cohesion: 0.19
Nodes (8): OflError, Invalid, Json, Skip, EMBEDDED, embedded_library_loads(), Library, LibraryFixture

### Community 45 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "sense"
Cohesion: 0.17
Nodes (5): Fader, LABEL_H, VALUE_H, clip(), SceneCard

### Community 48 - "settings_window.rs"
Cohesion: 0.16
Nodes (17): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+9 more)

### Community 50 - "menu.rs"
Cohesion: 0.06
Nodes (48): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+40 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 70 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 71 - "titlebar.rs"
Cohesion: 0.05
Nodes (48): Fullscreen, SETTLE, Step, Enter, Maximize, delete(), HEIGHT, k() (+40 more)

### Community 73 - "convert.rs"
Cohesion: 0.26
Nodes (10): convert_fixture(), default_value(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), redirects_are_skipped(), simple_par_with_modes_defaults_and_null() (+2 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 75 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 77 - "accent_dim"
Cohesion: 0.20
Nodes (5): accent_dim(), apply(), install_fonts(), raised_hover(), Toggle

### Community 80 - "LibraryPanel"
Cohesion: 0.23
Nodes (3): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch()

### Community 93 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 94 - "super"
Cohesion: 0.25
Nodes (3): History, LIMIT, undo_redo_roundtrip()

### Community 97 - "FixtureType"
Cohesion: 0.26
Nodes (4): ChannelDef, FixtureType, builtin_fixture_types(), ft()

### Community 98 - "Value"
Cohesion: 0.36
Nodes (7): capabilities(), channel_kind(), fine_aliases(), Lookup, Lookup<'a>, Resolved, wheel_kind()

### Community 100 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 101 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 102 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 104 - "text_dim"
Cohesion: 0.11
Nodes (19): sidebar(), load_meter(), show(), accent(), bg(), body(), caption(), heading_font() (+11 more)

### Community 105 - "Matrix"
Cohesion: 0.29
Nodes (4): Matrix, natural_cmp(), natural_sort(), Pixel

### Community 106 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

### Community 109 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 111 - "ofl-bundle.rs"
Cohesion: 0.38
Nodes (3): git(), main(), run()

### Community 117 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

## Knowledge Gaps
- **172 isolated node(s):** `OpenShow`, `SaveShowAs`, `De`, `En`, `General` (+167 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 492 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **54 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `runtime.rs`, `Show`, `text_dim`, `egui`, `AppState`, `Scene`, `app.rs`, `settings_window.rs`, `LibraryPanel`, `menu.rs`, `ViewCtx`, `fixture_settings.rs`, `t`?**
  _High betweenness centrality (0.105) - this node is a cross-community bridge._
- **Are the 54 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 54 INFERRED edges - model-reasoned connections that need verification._
- **What connects `OpenShow`, `SaveShowAs`, `De` to the rest of the system?**
  _172 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06275946275946276 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `ZoneId`, `Engine`, `FixtureType`, `theme.rs`, `show.rs`, `ChannelValue`, `AppState`, `Scene`, `.check_placement`, `ShowError`, `menu.rs`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `Tab`, `MenuItem<'a>`, `titlebar.rs`, `AppState`, `dialog.rs`, `IconButton`, `settings_window.rs`, `menu.rs`, `ViewCtx`, `fixture_settings.rs`, `Stroke`, `patch_grid.rs`, `stage_canvas.rs`?**
  _High betweenness centrality (0.079) - this node is a cross-community bridge._