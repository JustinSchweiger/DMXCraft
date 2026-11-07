# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 84 files · ~72,916 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1728 nodes · 4010 edges · 157 communities (78 shown, 79 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 164 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d78e7661`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- ChannelKind
- text_dim
- steps
- fx.rs
- OutputError
- t
- UpdateState
- file_dialog.rs
- fx_color.rs
- jobs
- steps
- DmxCraft
- menu.rs
- announce
- build-local-artifacts
- Icon
- super
- stage.rs
- scene_card.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- swatch.rs
- env
- dmxcraft
- super
- AppState
- titlebar.rs
- DmxCraft – UI-Richtlinien
- egui
- stage.rs
- drag_number
- fader.rs
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- arc
- app.rs
- PlanShape
- core/src/lib.rs
- programmer.rs
- MenuAction
- Fader
- Show
- form
- OutputKind
- toggle.rs
- TitleInfo
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- Category
- FixtureLimits
- T
- PatchGrid
- Dialog<'a>
- Grip
- ShowError
- Layer
- EffectKind
- en.rs
- Fullscreen
- Tab
- Scene
- range_slider.rs
- Tab
- DvcGoldUsb
- quick.rs
- ProgrammerMode
- time_field.rs
- egui
- scene_props.rs
- Arrange
- MenuPlace
- stage_canvas.rs
- dip_switch.rs
- DmxFrame
- resize_handles
- IconButton
- PickTarget

## God Nodes (most connected - your core abstractions)
1. `Engine` - 122 edges
2. `Show` - 89 edges
3. `Icon` - 87 edges
4. `t()` - 84 edges
5. `ChannelKind` - 43 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 36 edges
9. `ViewCtx` - 34 edges
10. `ChannelRef` - 33 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `builtin()` --calls--> `t()`  [INFERRED]
  crates/app/src/palettes.rs → crates/app/src/i18n/mod.rs
- `save_user()` --calls--> `t()`  [INFERRED]
  crates/app/src/palettes.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs

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

## Communities (157 total, 79 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.06
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.16
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (34): accent_hover(), active(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+26 more)

### Community 5 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 6 - "text_dim"
Cohesion: 0.24
Nodes (17): load_meter(), show(), accent_dim(), bg(), body(), border(), caption(), heading_font() (+9 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.09
Nodes (39): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, ColorPreset, .ALL, Burst (+31 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.05
Nodes (61): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+53 more)

### Community 11 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_color.rs"
Cohesion: 0.16
Nodes (17): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), label_row() (+9 more)

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
Cohesion: 0.20
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (65): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+57 more)

### Community 23 - "super"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 24 - "stage.rs"
Cohesion: 0.22
Nodes (11): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+3 more)

### Community 25 - "scene_card.rs"
Cohesion: 0.21
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

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
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "super"
Cohesion: 0.10
Nodes (22): digest(), hash_and_verify(), hash_password(), hex(), verify(), known_addresses(), on_switches(), roundtrip() (+14 more)

### Community 36 - "AppState"
Cohesion: 0.15
Nodes (9): AppState, Focus, Bank, Effect, Scenes, Mode, Control, Setup (+1 more)

### Community 37 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "egui"
Cohesion: 0.10
Nodes (13): SETTLE, ICON_FAMILY, ICON_FILL_FAMILY, Knob, SIZE, SWEEP, LimitPad, HANDLE (+5 more)

### Community 40 - "stage.rs"
Cohesion: 0.10
Nodes (23): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+15 more)

### Community 41 - "drag_number"
Cohesion: 0.38
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.14
Nodes (13): DOT_H, FADER_W, FaderEvent, Changed, None, Release, fine_mark(), HANDLE (+5 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "app.rs"
Cohesion: 0.17
Nodes (8): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Dialog, ProgrammerView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (11): ChannelDef, FixtureType, ft(), LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind (+3 more)

### Community 69 - "programmer.rs"
Cohesion: 0.30
Nodes (13): fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock, kind_color(), merge_groups() (+5 more)

### Community 70 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "form"
Cohesion: 0.27
Nodes (3): DialogResult, Form, form()

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 77 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 78 - "stage_editor.rs"
Cohesion: 0.23
Nodes (7): CanvasMode, Edit, Select, StageCanvas, set_pos(), StageEditorView, tool_sep()

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.21
Nodes (14): address_map_marks_ranges(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), next_free_address_skips_used(), patch_rejects_overlap_and_range() (+6 more)

### Community 87 - "dialog.rs"
Cohesion: 0.12
Nodes (10): Button, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel, Primary (+2 more)

### Community 90 - "demo.rs"
Cohesion: 0.24
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 96 - "PatchGrid"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 99 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "Layer"
Cohesion: 0.39
Nodes (7): color_bar(), current(), set_group(), xy_pad(), Layer, Live, Programmer

### Community 104 - "EffectKind"
Cohesion: 0.14
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 108 - "Fullscreen"
Cohesion: 0.18
Nodes (6): Fullscreen, Step, Enter, Maximize, FxFile, import_fx()

### Community 109 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 113 - "Scene"
Cohesion: 0.10
Nodes (10): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), Scene (+2 more)

### Community 117 - "range_slider.rs"
Cohesion: 0.27
Nodes (7): from_pct(), pct(), RangeSlider, value_field(), VALUE_H, paint_fixture(), paint_shape_icon()

### Community 118 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 123 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 128 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 133 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 136 - "egui"
Cohesion: 0.28
Nodes (6): ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView

### Community 137 - "scene_props.rs"
Cohesion: 0.17
Nodes (22): advanced(), banks_of(), bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog, header() (+14 more)

### Community 138 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 140 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (13): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, StageInteraction (+5 more)

### Community 142 - "dip_switch.rs"
Cohesion: 0.27
Nodes (3): accent(), DipSwitch, Knob<'a>

### Community 147 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

## Knowledge Gaps
- **240 isolated node(s):** `De`, `En`, `Setup`, `Control`, `Edit` (+235 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 642 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **79 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `ShowError`, `stage.rs`, `fx.rs`, `t`, `EffectKind`, `stage_canvas.rs`, `stage_editor.rs`, `DmxCraftApp`, `Scene`, `app.rs`, `show.rs`, `demo.rs`?**
  _High betweenness centrality (0.160) - this node is a cross-community bridge._
- **Are the 64 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 64 INFERRED edges - model-reasoned connections that need verification._
- **What connects `De`, `En`, `Setup` to the rest of the system?**
  _240 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.05678893178893179 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `text_dim`, `Layer`, `stage.rs`, `scene_props.rs`, `t`, `egui`, `Show`, `stage_canvas.rs`, `DmxCraftApp`, `settings_window.rs`, `app.rs`, `stage.rs`, `quick.rs`?**
  _High betweenness centrality (0.113) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11965811965811966 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `DmxCraftApp` to `library_panel.rs`, `text_dim`, `egui`, `scene_props.rs`, `t`, `file_dialog.rs`, `fx_color.rs`, `stage_canvas.rs`, `super`, `stage.rs`, `titlebar.rs`, `egui`, `stage.rs`, `fader.rs`, `i18n/mod.rs`, `settings_window.rs`, `app.rs`, `programmer.rs`, `form`, `stage_editor.rs`, `dialog.rs`, `PatchGrid`, `Layer`, `Fullscreen`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._