# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 80 files · ~64,064 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 12 file(s) not represented in the graph (top: (none) 5, .ico 2, .gz 2)

## Summary
- 1560 nodes · 3548 edges · 147 communities (75 shown, 72 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 148 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `2781fb0a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Tab
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- Engine
- widgets/mod.rs
- steps
- DmxCraftApp
- OutputError
- t
- ofl/src/lib.rs
- stage_editor.rs
- i18n/mod.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- qxf.rs
- stage.rs
- engine.rs
- host
- plan
- with
- ensure_linux_menu_entry
- convert.rs
- swatch.rs
- env
- dmxcraft
- qlc-bundle.rs
- Dialog<'a>
- file_dialog.rs
- DmxCraft – UI-Richtlinien
- T
- FixtureAction
- fixture_settings.rs
- fader.rs
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- DmxFrame
- Scene
- DvcGoldUsb
- programmer.rs
- core/src/lib.rs
- Show
- knob.rs
- OutputKind
- Snapshot
- Box<dyn serialport::SerialPort>
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- ChannelValue
- Toggle<'a>
- arc
- FixtureLimits
- scene_card.rs
- PatchGrid
- ViewCtx
- PlanShape
- Category
- ShowError
- ZoneId
- stage_canvas.rs
- show
- EffectKind
- en.rs
- .fade_state
- Vec2
- limit_pad.rs
- sidebar
- fixture_menu.rs
- Theme
- time_field.rs
- plan.rs
- scene_props.rs
- ProgrammerMode
- dip.rs
- Pos2
- IconButton
- ChannelKind

## God Nodes (most connected - your core abstractions)
1. `Engine` - 112 edges
2. `Show` - 83 edges
3. `Icon` - 77 edges
4. `t()` - 75 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 37 edges
8. `AppState` - 33 edges
9. `ViewCtx` - 33 edges
10. `ChannelRef` - 30 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `column()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `scene_card()` --calls--> `t()`  [INFERRED]
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

## Communities (147 total, 72 thin omitted)

### Community 0 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 1 - "AppState"
Cohesion: 0.10
Nodes (13): edit_preview(), AppState, LibraryDrag, Mode, Control, Setup, PatchForm, drag_ghost() (+5 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (31): accent_hover(), ACTIVE, apply(), c(), DANGER, DIP_BODY, fixture_bar(), fixture_off() (+23 more)

### Community 5 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 6 - "widgets/mod.rs"
Cohesion: 0.22
Nodes (11): accent(), bg(), border(), caption(), text(), text_dim(), text_faint(), Dialog (+3 more)

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
Nodes (55): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+47 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 12 - "stage_editor.rs"
Cohesion: 0.13
Nodes (13): CanvasMode, Edit, Select, ZOOM_MAX, ZOOM_MIN, Arrange, AlignH, AlignV (+5 more)

### Community 13 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (13): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+5 more)

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
Nodes (56): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+48 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.04
Nodes (55): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+47 more)

### Community 23 - "qxf.rs"
Cohesion: 0.21
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 24 - "stage.rs"
Cohesion: 0.25
Nodes (10): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+2 more)

### Community 25 - "engine.rs"
Cohesion: 0.23
Nodes (25): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+17 more)

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
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 37 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (20): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+12 more)

### Community 41 - "fixture_settings.rs"
Cohesion: 0.18
Nodes (8): section_header(), Quick, quick_action(), Center, Off, On, caption(), range_row()

### Community 44 - "fader.rs"
Cohesion: 0.12
Nodes (13): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+5 more)

### Community 45 - "super"
Cohesion: 0.22
Nodes (7): digest(), hash_and_verify(), hash_password(), hex(), verify(), addresses_are_one_based(), UNIVERSE_SIZE

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.07
Nodes (11): DmxCraftApp, LockDialog, Fullscreen, SETTLE, Step, Enter, Maximize, cable_belongs_to_one_universe() (+3 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (19): t(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+11 more)

### Community 66 - "Scene"
Cohesion: 0.21
Nodes (5): Scene, SceneKind, Fx, Static, SceneSettings

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "programmer.rs"
Cohesion: 0.25
Nodes (19): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+11 more)

### Community 70 - "core/src/lib.rs"
Cohesion: 0.20
Nodes (6): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 72 - "Show"
Cohesion: 0.23
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (11): Button, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 90 - "ChannelValue"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "arc"
Cohesion: 0.15
Nodes (6): ControlViews, OUTPUT_HZ, ICON_PNG, texture(), window_icon(), ProgrammerView

### Community 95 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "ViewCtx"
Cohesion: 0.18
Nodes (7): View, ViewCtx, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 98 - "PlanShape"
Cohesion: 0.18
Nodes (9): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, MovingHead, Round (+1 more)

### Community 99 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.19
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Show, Zone

### Community 102 - "stage_canvas.rs"
Cohesion: 0.26
Nodes (6): BOX, MenuFn, paint_fixture(), paint_shape_icon(), ShapeButton, WORLD

### Community 103 - "show"
Cohesion: 0.18
Nodes (8): load_meter(), show(), accent_dim(), body(), heading_font(), heading_text(), raised(), Toggle

### Community 104 - "EffectKind"
Cohesion: 0.17
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 108 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 112 - "limit_pad.rs"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 118 - "sidebar"
Cohesion: 0.25
Nodes (7): sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 123 - "fixture_menu.rs"
Cohesion: 0.20
Nodes (8): fit(), fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.29
Nodes (8): FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS, time_field()

### Community 136 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 137 - "scene_props.rs"
Cohesion: 0.20
Nodes (12): bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog, header(), properties(), ScenePropsView (+4 more)

### Community 138 - "ProgrammerMode"
Cohesion: 0.29
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 140 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 141 - "Pos2"
Cohesion: 0.30
Nodes (6): Camera, fixture_rect(), live_colors(), StageCanvas, StageInteraction, strobe_hz()

### Community 149 - "ChannelKind"
Cohesion: 0.08
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

## Knowledge Gaps
- **207 isolated node(s):** `Play`, `Pause`, `Prev`, `Next`, `Plus` (+202 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 581 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **72 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `AppState`, `ViewCtx`, `runtime.rs`, `programmer.rs`, `ZoneId`, `core/src/lib.rs`, `scene_props.rs`, `t`, `fixture_settings.rs`, `Snapshot`, `.fade_state`, `DmxCraftApp`, `settings_window.rs`, `DmxFrame`, `stage.rs`, `engine.rs`, `fixture_menu.rs`, `arc`?**
  _High betweenness centrality (0.128) - this node is a cross-community bridge._
- **Are the 57 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 57 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Play`, `Pause`, `Prev` to the rest of the system?**
  _207 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.09672830725462304 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `settings_window.rs` to `AppState`, `widgets/mod.rs`, `scene_props.rs`, `t`, `ProgrammerMode`, `stage_editor.rs`, `i18n/mod.rs`, `Pos2`, `titlebar.rs`, `stage.rs`, `Dialog<'a>`, `file_dialog.rs`, `FixtureAction`, `fixture_settings.rs`, `fader.rs`, `DmxCraftApp`, `programmer.rs`, `core/src/lib.rs`, `dialog.rs`, `arc`, `PatchGrid`, `ViewCtx`, `stage_canvas.rs`, `show`, `fixture_menu.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `ZoneId` to `AppState`, `runtime.rs`, `theme.rs`, `Engine`, `t`, `Pos2`, `engine.rs`, `FixtureAction`, `DmxCraftApp`, `Scene`, `core/src/lib.rs`, `Show`, `Snapshot`, `show.rs`, `ChannelValue`, `arc`, `ShowError`, `stage_canvas.rs`, `fixture_menu.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._