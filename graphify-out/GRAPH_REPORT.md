# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~57,996 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1461 nodes · 3293 edges · 136 communities (62 shown, 74 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 134 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7891a4c7`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- AppState
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- qxf.rs
- steps
- DmxCraftApp
- OutputError
- t
- dialog.rs
- CanvasMode
- super
- jobs
- steps
- DmxCraft
- announce
- build-local-artifacts
- Icon
- ChannelKind
- stage.rs
- ofl/src/lib.rs
- host
- plan
- with
- ensure_linux_menu_entry
- ChannelValue
- Vec2
- env
- dmxcraft
- titlebar.rs
- Arrange
- send_frame
- DmxCraft – UI-Richtlinien
- FixtureAction
- qlc-bundle.rs
- fader.rs
- form
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- ViewCtx
- IconButton
- OutputStatus
- DvcGoldUsb
- programmer.rs
- convert.rs
- OutputKind
- show.rs
- .check_placement
- OpenDmx
- Stroke
- MenuItem<'a>
- Toggle<'a>
- arc
- FixtureType
- t
- Show
- PlanShape
- ZoneId
- stage_canvas.rs
- ShowError
- en.rs
- FixtureLimits
- Tab
- scene_card.rs
- limit_pad.rs
- Dialog<'a>
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 99 edges
2. `Show` - 76 edges
3. `Icon` - 73 edges
4. `t()` - 67 edges
5. `ChannelKind` - 43 edges
6. `DmxCraftApp` - 37 edges
7. `DmxFrame` - 37 edges
8. `AppState` - 33 edges
9. `ChannelRef` - 29 edges
10. `ViewCtx` - 28 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `project()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs
- `color_picker()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/ui/widgets/color_picker.rs → crates/app/src/ui/widgets/dialog.rs
- `scene_card()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/dialog.rs

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

## Communities (136 total, 74 thin omitted)

### Community 0 - "AppState"
Cohesion: 0.13
Nodes (10): ControlViews, edit_preview(), OUTPUT_HZ, AppState, Mode, Control, Setup, PatchForm (+2 more)

### Community 1 - "Engine"
Cohesion: 0.26
Nodes (4): LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.15
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.17
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.06
Nodes (54): sidebar(), load_meter(), show(), accent(), accent_dim(), accent_hover(), active(), ACTIVE (+46 more)

### Community 5 - "Engine"
Cohesion: 0.06
Nodes (30): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), cv(), defaults_are_rendered(), Engine (+22 more)

### Community 6 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

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
Cohesion: 0.07
Nodes (55): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+47 more)

### Community 11 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel, Primary (+2 more)

### Community 12 - "CanvasMode"
Cohesion: 0.67
Nodes (3): CanvasMode, Edit, Select

### Community 13 - "super"
Cohesion: 0.11
Nodes (21): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+13 more)

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
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - "stage.rs"
Cohesion: 0.22
Nodes (11): text_field(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+3 more)

### Community 25 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

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
Cohesion: 0.15
Nodes (12): cv(), demo_show(), empty_show(), ensure_builtin_types(), UNIVERSE_SIZE, ChannelValue, LoopMode, Loop (+4 more)

### Community 31 - "Vec2"
Cohesion: 0.15
Nodes (5): HANDLE, RangeSlider, Segmented, Swatch, XyPad

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "titlebar.rs"
Cohesion: 0.05
Nodes (56): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+48 more)

### Community 36 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 37 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+18 more)

### Community 41 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+5 more)

### Community 45 - "form"
Cohesion: 0.31
Nodes (3): DialogResult, Form, form()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.05
Nodes (18): DmxCraftApp, LockDialog, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT (+10 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 50 - "ViewCtx"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 66 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.08
Nodes (40): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+32 more)

### Community 73 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 77 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 93 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 94 - "FixtureType"
Cohesion: 0.23
Nodes (4): ChannelDef, FixtureType, builtin_fixture_types(), ft()

### Community 97 - "t"
Cohesion: 0.11
Nodes (13): t(), section_header(), View, ViewCtx, caption(), FixtureSettings, range_row(), SetupView (+5 more)

### Community 99 - "Show"
Cohesion: 0.26
Nodes (3): Fixture, Group, Show

### Community 100 - "PlanShape"
Cohesion: 0.20
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 101 - "ZoneId"
Cohesion: 0.17
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.16
Nodes (15): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+7 more)

### Community 103 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 110 - "Tab"
Cohesion: 0.29
Nodes (6): Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "scene_card.rs"
Cohesion: 0.26
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 112 - "limit_pad.rs"
Cohesion: 0.17
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented<'a, T>

### Community 119 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

## Knowledge Gaps
- **191 isolated node(s):** `Cancel`, `Secondary`, `Primary`, `Control`, `Setup` (+186 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 548 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **74 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `AppState`, `runtime.rs`, `theme.rs`, `Engine`, `stage_canvas.rs`, `ZoneId`, `FixtureAction`, `ShowError`, `t`, `show.rs`, `.check_placement`, `DmxCraftApp`, `FixtureType`, `ChannelValue`?**
  _High betweenness centrality (0.102) - this node is a cross-community bridge._
- **Are the 51 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 51 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Cancel`, `Secondary`, `Primary` to the rest of the system?**
  _191 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.12857142857142856 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `t`, `Engine`, `Show`, `runtime.rs`, `programmer.rs`, `FixtureAction`, `t`, `DmxCraftApp`, `settings_window.rs`, `stage.rs`, `ChannelValue`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06172839506172839 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `AppState`, `Engine`, `titlebar.rs`, `theme.rs`, `programmer.rs`, `stage_canvas.rs`, `FixtureAction`, `t`, `dialog.rs`, `fader.rs`, `form`, `DmxCraftApp`, `settings_window.rs`, `ViewCtx`, `i18n/mod.rs`, `stage.rs`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._