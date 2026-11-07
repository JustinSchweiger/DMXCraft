# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 76 files · ~54,103 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1407 nodes · 3157 edges · 144 communities (71 shown, 73 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 129 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f716b47d`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- sense
- engine.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- widgets/mod.rs
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
- stage_canvas.rs
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
- MenuAction
- Fullscreen
- convert.rs
- OutputKind
- show.rs
- patch_grid.rs
- plan.rs
- MenuItem<'a>
- OpenDmx
- qxf.rs
- fixture_settings.rs
- Swatch
- Stroke
- Toggle<'a>
- egui
- lock.rs
- Theme
- Settings
- LoopMode
- Show
- PlanShape
- ZoneId
- Pos2
- ShowError
- swatch.rs
- en.rs
- DmxFrame
- FixtureLimits
- scene_card.rs
- .grid
- limit_pad.rs
- Category
- i18n/mod.rs
- TitleInfo
- Dialog<'a>
- .show
- Box<dyn serialport::SerialPort>
- resize_handles

## God Nodes (most connected - your core abstractions)
1. `Show` - 78 edges
2. `Engine` - 77 edges
3. `Icon` - 71 edges
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

## Communities (144 total, 73 thin omitted)

### Community 0 - "sense"
Cohesion: 0.27
Nodes (4): accent(), bg(), DipSwitch, RangeSlider

### Community 1 - "engine.rs"
Cohesion: 0.14
Nodes (19): cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), limits_cap_dimmer_and_position(), MAX_STEP_ADVANCES_PER_TICK, MS (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): accent_hover(), ACTIVE, apply(), c(), DANGER, DIP_BODY, fixture_off(), FS_BODY (+23 more)

### Community 5 - "widgets/mod.rs"
Cohesion: 0.25
Nodes (5): CanvasMode, Edit, Select, ShapeButton, StageCanvas

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
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.06
Nodes (60): select_all(), color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), programmer_values() (+52 more)

### Community 11 - "dialog.rs"
Cohesion: 0.11
Nodes (10): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+2 more)

### Community 12 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.21
Nodes (9): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+1 more)

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

### Community 24 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 25 - "app.rs"
Cohesion: 0.11
Nodes (13): ControlViews, LockDialog, OUTPUT_HZ, text_field(), ProgrammerView, StageView, View, ViewCtx (+5 more)

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

### Community 31 - "stage_canvas.rs"
Cohesion: 0.27
Nodes (8): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), paint_fixture(), paint_shape_icon(), WORLD

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
Cohesion: 0.14
Nodes (11): section_header(), ZOOM_MAX, ZOOM_MIN, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 37 - "super"
Cohesion: 0.25
Nodes (3): History, LIMIT, undo_redo_roundtrip()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.05
Nodes (40): AppState, Mode, Control, Setup, PatchForm, group_pill(), GroupAction, AddSelection (+32 more)

### Community 41 - "FixtureType"
Cohesion: 0.20
Nodes (5): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

### Community 44 - ".show"
Cohesion: 0.14
Nodes (9): Fader, Fader<'a>, FaderEvent, Changed, None, Release, LABEL_H, truncate() (+1 more)

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 50 - "menu.rs"
Cohesion: 0.18
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 65 - "Settings"
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 66 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 71 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 73 - "convert.rs"
Cohesion: 0.13
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.29
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.13
Nodes (11): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, COLUMNS, Drag, GridMode, Output (+3 more)

### Community 78 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "qxf.rs"
Cohesion: 0.21
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): text_dim(), text_faint(), caption(), Quick, quick_action(), Center, Off, On (+1 more)

### Community 89 - "Swatch"
Cohesion: 0.50
Nodes (4): Swatch, Auto, Color, Custom

### Community 93 - "egui"
Cohesion: 0.18
Nodes (4): ICON_PNG, texture(), window_icon(), Toggle

### Community 94 - "lock.rs"
Cohesion: 0.23
Nodes (11): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+3 more)

### Community 96 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 98 - "LoopMode"
Cohesion: 0.67
Nodes (3): LoopMode, Loop, Once

### Community 99 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 100 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "Pos2"
Cohesion: 0.40
Nodes (3): Camera, MenuFn, StageInteraction

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 108 - "DmxFrame"
Cohesion: 0.20
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 112 - ".grid"
Cohesion: 0.15
Nodes (12): load_meter(), show(), accent_dim(), body(), caption(), fixture_bar(), heading_font(), heading_text() (+4 more)

### Community 117 - "limit_pad.rs"
Cohesion: 0.24
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 118 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 133 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

## Knowledge Gaps
- **184 isolated node(s):** `All`, `Delete`, `New`, `Show`, `None` (+179 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 530 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **73 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Icon` connect `Icon` to `egui`, `widgets/mod.rs`, `titlebar.rs`, `FixtureAction`, `t`, `dialog.rs`, `color_picker.rs`, `patch_grid.rs`, `MenuItem<'a>`, `settings_window.rs`, `scene_card.rs`, `menu.rs`, `fixture_settings.rs`, `Tab`, `Stroke`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 50 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 50 INFERRED edges - model-reasoned connections that need verification._
- **What connects `All`, `Delete`, `New` to the rest of the system?**
  _184 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `engine.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13704994192799072 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `t` to `engine.rs`, `runtime.rs`, `Show`, `FixtureAction`, `FixtureType`, `DmxCraftApp`, `settings_window.rs`, `fixture_settings.rs`, `app.rs`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `engine.rs`, `.check_placement`, `runtime.rs`, `theme.rs`, `widgets/mod.rs`, `super`, `ZoneId`, `FixtureAction`, `FixtureType`, `t`, `ShowError`, `show.rs`, `ChannelValue`, `stage_canvas.rs`?**
  _High betweenness centrality (0.078) - this node is a cross-community bridge._