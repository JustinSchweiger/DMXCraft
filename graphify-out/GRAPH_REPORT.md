# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 81 files · ~66,764 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1644 nodes · 3781 edges · 162 communities (82 shown, 80 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 148 edges (avg confidence: 0.84)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `65c779cc`
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
- engine.rs
- OutputError
- t
- ChannelRef
- convert.rs
- color_picker.rs
- jobs
- steps
- DmxCraft
- menu.rs
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
- fixture_menu.rs
- qxf.rs
- qlc-bundle.rs
- DmxCraft – UI-Richtlinien
- egui
- FixtureAction
- drag_number
- fader.rs
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- app.rs
- settings_window.rs
- arc
- AppState
- PlanShape
- core/src/lib.rs
- FxFile
- programmer.rs
- .group_bar
- titlebar.rs
- Show
- knob.rs
- OutputKind
- toggle.rs
- Snapshot
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- MenuAction
- FixtureLimits
- ViewCtx
- PatchGrid
- Dialog<'a>
- FixtureType
- ShowError
- ZoneId
- EffectKind
- en.rs
- Fullscreen
- show
- BankScope
- range_slider.rs
- Tab
- DvcGoldUsb
- Category
- Theme
- time_field.rs
- fixture_settings.rs
- scene_props.rs
- apply
- .fade_state
- stage_canvas.rs
- dip_switch.rs
- DmxFrame
- heading_text
- IconButton
- PickTarget
- resize_handles

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
- `steps_table()` --calls--> `time_field_compact()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/ui/widgets/time_field.rs
- `FxFile` --references--> `Step`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/fullscreen.rs
- `import_fx()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/i18n/mod.rs

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

## Communities (162 total, 80 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.09
Nodes (4): Engine, lerp(), PlayingScene, SceneProgress

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
Cohesion: 0.09
Nodes (25): accent_hover(), ACTIVE, c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+17 more)

### Community 5 - "ChannelKind"
Cohesion: 0.10
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 6 - "text_dim"
Cohesion: 0.28
Nodes (17): load_meter(), show(), accent_dim(), bg(), body(), border(), caption(), heading_font() (+9 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "engine.rs"
Cohesion: 0.24
Nodes (27): blind_keeps_output_but_monitor_shows_edits(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position(), live_overrides_scenes_and_resets() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.09
Nodes (45): select_all(), text_field(), programmer_values(), Action, Bank, Delete, DuplicateScene, Flash (+37 more)

### Community 11 - "ChannelRef"
Cohesion: 0.08
Nodes (14): ProgrammerMode, Blind, Edit, Live, items_mode(), Layer, Live, Programmer (+6 more)

### Community 12 - "convert.rs"
Cohesion: 0.20
Nodes (13): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), OflError, Invalid, Json (+5 more)

### Community 13 - "color_picker.rs"
Cohesion: 0.15
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
Cohesion: 0.16
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

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
Cohesion: 0.17
Nodes (12): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Matrix, natural_cmp() (+4 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "fixture_menu.rs"
Cohesion: 0.22
Nodes (5): MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 36 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 37 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "egui"
Cohesion: 0.10
Nodes (12): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T> (+4 more)

### Community 40 - "FixtureAction"
Cohesion: 0.11
Nodes (22): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+14 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+6 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.05
Nodes (34): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+26 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "app.rs"
Cohesion: 0.06
Nodes (25): ControlViews, DmxCraftApp, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Pending, Purpose (+17 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "AppState"
Cohesion: 0.15
Nodes (9): AppState, Focus, Bank, Effect, Scenes, Mode, Control, Setup (+1 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (12): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind (+4 more)

### Community 69 - "programmer.rs"
Cohesion: 0.21
Nodes (18): color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks() (+10 more)

### Community 70 - ".group_bar"
Cohesion: 0.22
Nodes (10): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName (+2 more)

### Community 71 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "knob.rs"
Cohesion: 0.16
Nodes (5): ICON_FAMILY, ICON_FILL_FAMILY, Knob, SIZE, SWEEP

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 77 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 78 - "stage_editor.rs"
Cohesion: 0.19
Nodes (9): apply(), Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos() (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.15
Nodes (8): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, LABEL_W

### Community 90 - "demo.rs"
Cohesion: 0.39
Nodes (5): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue

### Community 93 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 95 - "ViewCtx"
Cohesion: 0.15
Nodes (8): single_scene(), View, ViewCtx, FixtureSettings, SetupView, StageEditorView, ZonesView, Windows

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.13
Nodes (6): Dialog<'a>, DialogResult, Kind, Cancel, Primary, Secondary

### Community 99 - "FixtureType"
Cohesion: 0.19
Nodes (3): ChannelDef, FixtureType, ft()

### Community 100 - "ShowError"
Cohesion: 0.24
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "ZoneId"
Cohesion: 0.19
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Show, steps_insert_remove_and_normalize(), Zone

### Community 104 - "EffectKind"
Cohesion: 0.11
Nodes (15): effect_tiles(), Tab, Effects, Properties, Third, EffectKind, .ALL, ChaserFx (+7 more)

### Community 108 - "Fullscreen"
Cohesion: 0.16
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, section_header()

### Community 109 - "show"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 113 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 117 - "range_slider.rs"
Cohesion: 0.33
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 118 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 123 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 128 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 133 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 136 - "fixture_settings.rs"
Cohesion: 0.43
Nodes (3): body(), caption(), range_row()

### Community 137 - "scene_props.rs"
Cohesion: 0.20
Nodes (21): t(), advanced(), banks_of(), bar_slider(), effect_header(), FX_EXT, FxDialog, header() (+13 more)

### Community 138 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), surface()

### Community 140 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (15): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+7 more)

### Community 142 - "dip_switch.rs"
Cohesion: 0.27
Nodes (3): accent(), DipSwitch, Knob<'a>

### Community 147 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

## Knowledge Gaps
- **227 isolated node(s):** `Properties`, `Effects`, `Third`, `Jump`, `Release` (+222 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 620 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **80 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `fixture_menu.rs`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `text_dim`, `.group_bar`, `engine.rs`, `scene_props.rs`, `t`, `ChannelRef`, `.fade_state`, `Snapshot`, `stage_editor.rs`, `app.rs`, `settings_window.rs`, `ZoneId`, `ViewCtx`?**
  _High betweenness centrality (0.117) - this node is a cross-community bridge._
- **Are the 58 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 58 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Properties`, `Effects`, `Third` to the rest of the system?**
  _227 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.09041835357624832 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `fixture_settings.rs`, `scene_props.rs`, `t`, `color_picker.rs`, `IconButton`, `menu.rs`, `.paint`, `scene_card.rs`, `fixture_menu.rs`, `egui`, `settings_window.rs`, `programmer.rs`, `.group_bar`, `titlebar.rs`, `knob.rs`, `stage_editor.rs`, `dialog.rs`, `MenuItem<'a>`, `PatchGrid`, `Tab`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11965811965811966 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `ZoneId` to `Engine`, `fixture_menu.rs`, `core/src/lib.rs`, `theme.rs`, `FixtureType`, `ShowError`, `runtime.rs`, `FixtureAction`, `engine.rs`, `t`, `Show`, `Snapshot`, `stage_canvas.rs`, `app.rs`, `show.rs`, `demo.rs`?**
  _High betweenness centrality (0.091) - this node is a cross-community bridge._