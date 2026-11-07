# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 81 files · ~67,015 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1644 nodes · 3788 edges · 159 communities (74 shown, 85 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 147 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d974b173`
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
- OutputError
- t
- UpdateState
- file_dialog.rs
- output/src/lib.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- super
- ofl/src/lib.rs
- scene_card.rs
- host
- plan
- with
- ensure_linux_menu_entry
- Value
- swatch.rs
- env
- dmxcraft
- plan.rs
- qxf.rs
- qlc-bundle.rs
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
- FxFile
- programmer.rs
- lock.rs
- Settings
- Show
- Knob<'a>
- OutputKind
- toggle.rs
- dip.rs
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- Dialog
- FixtureLimits
- ViewCtx
- PatchGrid
- Dialog<'a>
- Grip
- ShowError
- ZoneId
- EffectKind
- en.rs
- Fullscreen
- Tab
- BankScope
- range_slider.rs
- Tab
- DvcGoldUsb
- Box<dyn serialport::SerialPort>
- Theme
- time_field.rs
- egui
- scene_props.rs
- stage_canvas.rs
- dip_switch.rs
- DmxFrame
- IconButton
- PickTarget
- ScenePropsView

## God Nodes (most connected - your core abstractions)
1. `Engine` - 118 edges
2. `Icon` - 87 edges
3. `Show` - 85 edges
4. `t()` - 76 edges
5. `ChannelKind` - 42 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 35 edges
9. `ViewCtx` - 34 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `properties()` --calls--> `time_field()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/time_field.rs
- `import_fx()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/i18n/mod.rs
- `steps_table()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/i18n/mod.rs
- `steps_table()` --calls--> `time_field_compact()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/time_field.rs

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

## Communities (159 total, 85 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (49): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), Layer, Live, Programmer (+41 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.08
Nodes (16): AppState, Focus, Bank, Effect, Scenes, LibraryDrag, Mode, Control (+8 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (18): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (30): accent_dim(), accent_hover(), ACTIVE, apply(), c(), DANGER, DIP_BODY, fixture_bar() (+22 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 6 - "text_dim"
Cohesion: 0.26
Nodes (16): load_meter(), show(), bg(), body(), border(), caption(), heading_font(), heading_text() (+8 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (57): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+49 more)

### Community 11 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "output/src/lib.rs"
Cohesion: 0.17
Nodes (6): OutputConfig, OutputStatus, Connecting, Error, Running, Stopped

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
Nodes (54): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+46 more)

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
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

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

### Community 30 - "Value"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 36 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 37 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "egui"
Cohesion: 0.13
Nodes (6): SIZE, SWEEP, LimitPad, Segmented, Segmented<'a, T>, XyPad

### Community 40 - "stage.rs"
Cohesion: 0.06
Nodes (40): select_all(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+32 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+6 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

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
Cohesion: 0.36
Nodes (4): edit_preview(), load_step(), LockDialog, OUTPUT_HZ

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (13): UNIVERSE_SIZE, ChannelValue, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind (+5 more)

### Community 69 - "programmer.rs"
Cohesion: 0.09
Nodes (34): ControlViews, ProgrammerMode, Blind, Edit, Live, Category, All, Beam (+26 more)

### Community 70 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 77 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 78 - "stage_editor.rs"
Cohesion: 0.18
Nodes (11): CanvasMode, Edit, Select, StageCanvas, Arrange, AlignH, AlignV, Grid (+3 more)

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.15
Nodes (7): FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H, LABEL_W

### Community 90 - "demo.rs"
Cohesion: 0.43
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 93 - "Dialog"
Cohesion: 0.33
Nodes (6): Button, Dialog, Kind, Cancel, Primary, Secondary

### Community 95 - "ViewCtx"
Cohesion: 0.14
Nodes (5): single_scene(), ViewCtx, clip(), ZonesView, Windows

### Community 96 - "PatchGrid"
Cohesion: 0.23
Nodes (9): CELL_H_MAX, CELL_H_MIN, COLUMNS, Drag, GridMode, Output, Patch, PatchGrid (+1 more)

### Community 99 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.18
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings

### Community 104 - "EffectKind"
Cohesion: 0.17
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 108 - "Fullscreen"
Cohesion: 0.20
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, steps_table()

### Community 109 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 113 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 117 - "range_slider.rs"
Cohesion: 0.33
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 118 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 136 - "egui"
Cohesion: 0.19
Nodes (8): section_header(), View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 137 - "scene_props.rs"
Cohesion: 0.27
Nodes (15): t(), advanced(), banks_of(), bar_slider(), FX_EXT, header(), pick_tile(), PickColumn (+7 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+6 more)

### Community 146 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 147 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 149 - "ScenePropsView"
Cohesion: 0.39
Nodes (6): effect_header(), effect_tiles(), FxDialog, Picker, ScenePropsView, Effect

## Knowledge Gaps
- **227 isolated node(s):** `Properties`, `Effects`, `Third`, `Jump`, `Release` (+222 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 620 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **85 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `library_panel.rs`, `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `ZoneId`, `ShowError`, `stage.rs`, `t`, `stage_canvas.rs`, `stage_editor.rs`, `DmxCraftApp`, `BankScope`, `app.rs`, `show.rs`, `demo.rs`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Are the 58 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 58 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Properties`, `Effects`, `Third` to the rest of the system?**
  _227 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.05201886236368995 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `text_dim`, `stage.rs`, `scene_props.rs`, `t`, `Show`, `DmxCraftApp`, `settings_window.rs`, `DmxFrame`, `app.rs`, `ViewCtx`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.08305647840531562 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `PatchGrid`, `programmer.rs`, `stage.rs`, `scene_props.rs`, `t`, `egui`, `stage_editor.rs`, `settings_window.rs`, `IconButton`, `titlebar.rs`, `Tab`, `dialog.rs`, `MenuItem<'a>`, `scene_card.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._