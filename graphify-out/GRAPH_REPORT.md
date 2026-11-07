# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~56,814 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1436 nodes · 3203 edges · 135 communities (60 shown, 75 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 133 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `fa9a3182`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- AppState
- Settings
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
- ViewCtx
- DmxCraft – UI-Richtlinien
- AppState
- OutputStatus
- fader.rs
- ProgrammerMode
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- Category
- Fullscreen
- DvcGoldUsb
- programmer.rs
- convert.rs
- OutputKind
- show.rs
- Dialog<'a>
- OpenDmx
- IconButton
- icons.rs
- Toggle<'a>
- arc
- lock.rs
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
- T
- Grip
- Ui
- i18n/mod.rs
- FixtureType

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
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `color_bar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `toolbar()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/i18n/mod.rs
- `toolbar()` --calls--> `quick_action()`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/views/quick.rs

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

## Communities (135 total, 75 thin omitted)

### Community 0 - "AppState"
Cohesion: 0.17
Nodes (6): edit_preview(), AppState, Mode, Control, Setup, PatchForm

### Community 1 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.05
Nodes (54): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+46 more)

### Community 5 - "Engine"
Cohesion: 0.09
Nodes (24): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+16 more)

### Community 6 - "titlebar.rs"
Cohesion: 0.05
Nodes (56): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+48 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (55): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+47 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (9): Button, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel, Primary (+1 more)

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
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 25 - "app.rs"
Cohesion: 0.16
Nodes (5): LockDialog, OUTPUT_HZ, Dialog, ZonesView, Windows

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

### Community 31 - "egui"
Cohesion: 0.16
Nodes (5): SETTLE, HANDLE, LimitPad, Segmented, XyPad

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
Cohesion: 0.12
Nodes (15): section_header(), CanvasMode, Edit, Select, StageCanvas, ZOOM_MAX, ZOOM_MIN, Arrange (+7 more)

### Community 37 - "ViewCtx"
Cohesion: 0.06
Nodes (27): RangeSlider, View, ViewCtx, Quick, quick_action(), Center, Off, On (+19 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "AppState"
Cohesion: 0.07
Nodes (38): ControlViews, ProgrammerView, group_pill(), GroupAction, AddSelection, Delete, New, Rename (+30 more)

### Community 41 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+6 more)

### Community 45 - "ProgrammerMode"
Cohesion: 0.40
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.16
Nodes (18): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+10 more)

### Community 50 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 65 - "Fullscreen"
Cohesion: 0.29
Nodes (4): Fullscreen, Step, Enter, Maximize

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.31
Nodes (15): category_list(), color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks() (+7 more)

### Community 73 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.24
Nodes (12): UNIVERSE_SIZE, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "lock.rs"
Cohesion: 0.23
Nodes (11): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+3 more)

### Community 98 - "core/src/lib.rs"
Cohesion: 0.22
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 99 - "Show"
Cohesion: 0.23
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 100 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (12): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+4 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

### Community 137 - "FixtureType"
Cohesion: 0.19
Nodes (3): ChannelDef, FixtureType, ft()

## Knowledge Gaps
- **188 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+183 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 544 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **75 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `egui`, `stage_canvas.rs`, `Engine`, `AppState`, `FixtureType`, `t`, `ZoneId`, `History`, `show.rs`, `ShowError`, `ChannelValue`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _188 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09090909090909091 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `DmxCraftApp` to `Settings`, `file_dialog.rs`, `core/src/lib.rs`, `theme.rs`, `programmer.rs`, `titlebar.rs`, `stage_canvas.rs`, `AppState`, `ViewCtx`, `t`, `dialog.rs`, `fader.rs`, `ProgrammerMode`, `Dialog<'a>`, `settings_window.rs`, `egui`, `i18n/mod.rs`, `app.rs`?**
  _High betweenness centrality (0.085) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05388151174668029 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `core/src/lib.rs`, `Show`, `runtime.rs`, `programmer.rs`, `ViewCtx`, `AppState`, `t`, `DmxCraftApp`, `settings_window.rs`, `app.rs`?**
  _High betweenness centrality (0.066) - this node is a cross-community bridge._