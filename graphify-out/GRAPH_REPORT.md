# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 77 files · ~52,854 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1398 nodes · 3146 edges · 121 communities (65 shown, 56 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 132 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c7dd2032`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- text_dim
- Engine
- runtime.rs
- open_dmx.rs
- theme.rs
- .show
- Show
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
- AppState
- host
- plan
- with
- ensure_linux_menu_entry
- show.rs
- stage_canvas.rs
- env
- dmxcraft
- PlanShape
- egui
- FixtureLimits
- DmxCraft – UI-Richtlinien
- FixtureAction
- patch_grid.rs
- FixtureType
- qlc-bundle.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- menu.rs
- Settings
- apply
- DvcGoldUsb
- t
- titlebar.rs
- convert.rs
- OutputKind
- file_dialog.rs
- sense
- ChannelValue
- programmer.rs
- ShowError
- qxf.rs
- Category
- scenes.rs
- Stroke
- Toggle<'a>
- Arrange
- super
- Theme
- IconButton
- core/src/lib.rs
- DmxFrame
- ZoneId
- app.rs
- en.rs
- Pos2
- Tab
- scene_card.rs
- swatch.rs
- limit_pad.rs
- i18n/mod.rs

## God Nodes (most connected - your core abstractions)
1. `Engine` - 81 edges
2. `Show` - 74 edges
3. `t()` - 72 edges
4. `Icon` - 66 edges
5. `DmxFrame` - 38 edges
6. `ChannelKind` - 38 edges
7. `DmxCraftApp` - 34 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 30 edges
10. `FixtureType` - 26 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `color_picker()` --calls--> `text_field()`  [INFERRED]
  crates/app/src/ui/widgets/color_picker.rs → crates/app/src/ui/widgets/dialog.rs
- `color_swatches()` --calls--> `t()`  [INFERRED]
  crates/app/src/ui/widgets/color_picker.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `color_swatches()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/color_picker.rs
- `scene_card()` --calls--> `color_swatches()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/color_picker.rs

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

## Communities (121 total, 56 thin omitted)

### Community 0 - "text_dim"
Cohesion: 0.16
Nodes (11): load_meter(), show(), accent(), accent_dim(), bg(), caption(), raised(), raised_hover() (+3 more)

### Community 1 - "Engine"
Cohesion: 0.10
Nodes (21): cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), first_step_fades_intensity_in(), htp_highest_wins_ltp_latest_wins(), lerp(), limits_cap_dimmer_and_position() (+13 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.07
Nodes (21): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+13 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (27): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), fixture_off(), FS_BODY (+19 more)

### Community 5 - ".show"
Cohesion: 0.16
Nodes (8): border(), Fader<'a>, FaderEvent, Changed, None, Release, truncate(), separator()

### Community 6 - "Show"
Cohesion: 0.19
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "DmxCraftApp"
Cohesion: 0.09
Nodes (14): ICON_PNG, texture(), window_icon(), APP_NAME, Updater, UpdateState, Available, Checking (+6 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.10
Nodes (39): Action, Bank, Delete, DuplicateScene, Edit, Jump, Mark, NewScene (+31 more)

### Community 11 - "dialog.rs"
Cohesion: 0.07
Nodes (12): Button, Dialog, Dialog<'a>, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H (+4 more)

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
Nodes (48): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, Close (+40 more)

### Community 23 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 24 - ".show"
Cohesion: 0.20
Nodes (8): sidebar(), body(), heading_font(), heading_text(), surface(), text(), DialogResult, title()

### Community 25 - "AppState"
Cohesion: 0.06
Nodes (21): AppState, Mode, Control, Setup, PatchForm, View, ViewCtx, caption() (+13 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 31 - "stage_canvas.rs"
Cohesion: 0.12
Nodes (17): BOX, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect(), live_colors() (+9 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 36 - "egui"
Cohesion: 0.17
Nodes (9): section_header(), ZOOM_MAX, ZOOM_MIN, MenuPlace, Patch, Stage, StageEditor, set_pos() (+1 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "patch_grid.rs"
Cohesion: 0.15
Nodes (12): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+4 more)

### Community 44 - "FixtureType"
Cohesion: 0.26
Nodes (4): ChannelDef, FixtureType, builtin_fixture_types(), ft()

### Community 45 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (17): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+9 more)

### Community 50 - "menu.rs"
Cohesion: 0.12
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 65 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 66 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), style()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "t"
Cohesion: 0.22
Nodes (6): t(), group_bar(), selection_tools(), ScenesView, ZonesView, Windows

### Community 71 - "titlebar.rs"
Cohesion: 0.05
Nodes (48): Fullscreen, SETTLE, Step, Enter, Maximize, delete(), HEIGHT, k() (+40 more)

### Community 73 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 77 - "sense"
Cohesion: 0.19
Nodes (7): Fader, LABEL_H, VALUE_H, LimitPad, RangeSlider, Toggle, XyPad

### Community 78 - "ChannelValue"
Cohesion: 0.24
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelRef, ChannelValue, Step

### Community 79 - "programmer.rs"
Cohesion: 0.42
Nodes (9): color_bar(), current(), fader(), fader_groups(), FaderGroup, kind_color(), save_bar(), set_group() (+1 more)

### Community 80 - "ShowError"
Cohesion: 0.38
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 86 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 87 - "Category"
Cohesion: 0.20
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 88 - "scenes.rs"
Cohesion: 0.19
Nodes (12): programmer_values(), zone_combo(), apply(), scene_header(), scenes_ui(), seconds(), StepAction, Delete (+4 more)

### Community 93 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 94 - "super"
Cohesion: 0.09
Nodes (23): History, LIMIT, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password(), hex(), verify() (+15 more)

### Community 96 - "Theme"
Cohesion: 0.29
Nodes (5): active(), Theme, .ALL, Gray, Navy

### Community 98 - "core/src/lib.rs"
Cohesion: 0.16
Nodes (7): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 101 - "ZoneId"
Cohesion: 0.22
Nodes (4): duplicate_zone_copies_scenes_behind_original(), Scene, Show, Zone

### Community 102 - "app.rs"
Cohesion: 0.31
Nodes (6): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView, StageView

### Community 110 - "Tab"
Cohesion: 0.25
Nodes (7): SettingsWindow, Tab, .ALL, General, Midi, Outputs, Project

### Community 111 - "scene_card.rs"
Cohesion: 0.29
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 117 - "limit_pad.rs"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 119 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

## Knowledge Gaps
- **181 isolated node(s):** `Auto`, `Color`, `Custom`, `All`, `Free` (+176 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 517 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **56 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `ZoneId` to `Engine`, `core/src/lib.rs`, `theme.rs`, `Show`, `FixtureAction`, `t`, `FixtureType`, `ChannelValue`, `ShowError`, `show.rs`, `super`, `stage_canvas.rs`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Are the 57 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 57 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Auto`, `Color`, `Custom` to the rest of the system?**
  _181 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.10064935064935066 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `core/src/lib.rs`, `runtime.rs`, `DmxFrame`, `t`, `app.rs`, `ZoneId`, `t`, `ChannelValue`, `programmer.rs`, `settings_window.rs`, `DmxCraftApp`, `scenes.rs`, `AppState`, `stage_canvas.rs`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11965811965811966 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `text_dim`, `.show`, `t`, `color_picker.rs`, `.show`, `AppState`, `stage_canvas.rs`, `egui`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `Settings`, `titlebar.rs`, `file_dialog.rs`, `programmer.rs`, `scenes.rs`, `core/src/lib.rs`, `app.rs`, `i18n/mod.rs`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._