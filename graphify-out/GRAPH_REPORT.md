# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 84 files · ~73,558 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1733 nodes · 4015 edges · 145 communities (73 shown, 72 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 167 edges (avg confidence: 0.83)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `9bbc9f21`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- ChannelKind
- text_dim
- steps
- fx.rs
- OutputError
- t
- ofl/src/lib.rs
- file_dialog.rs
- fx_color.rs
- jobs
- steps
- DmxCraft
- menu.rs
- GH_TOKEN
- build-local-artifacts
- Icon
- super
- scene_card.rs
- host
- qxf.rs
- with
- ensure_linux_menu_entry
- convert.rs
- duplicate_zone_copies_scenes_behind_original
- env
- dmxcraft
- super
- titlebar.rs
- DmxCraft – UI-Richtlinien
- sense
- stage.rs
- drag_number
- fader.rs
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- app.rs
- settings_window.rs
- arc
- egui
- PlanShape
- core/src/lib.rs
- qlc-bundle.rs
- programmer.rs
- MenuAction
- Box<dyn serialport::SerialPort>
- Show
- OutputKind
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
- EffectKind
- en.rs
- Step
- Tab
- BankScope
- DvcGoldUsb
- egui
- scene_props.rs
- Arrange
- fixture_menu.rs
- stage_canvas.rs
- knob.rs
- DmxFrame
- resize_handles
- IconButton
- PickTarget

## God Nodes (most connected - your core abstractions)
1. `Engine` - 122 edges
2. `Show` - 89 edges
3. `Icon` - 87 edges
4. `t()` - 85 edges
5. `ChannelKind` - 44 edges
6. `DmxFrame` - 40 edges
7. `DmxCraftApp` - 38 edges
8. `AppState` - 38 edges
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
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs

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

## Communities (145 total, 72 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.07
Nodes (37): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), Engine, fades_between_steps_and_loops(), FadeState, In (+29 more)

### Community 1 - "AppState"
Cohesion: 0.06
Nodes (28): AppState, Focus, Bank, Effect, Scenes, LibraryDrag, Mode, Control (+20 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (31): accent_hover(), active(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+23 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 6 - "text_dim"
Cohesion: 0.19
Nodes (20): load_meter(), show(), accent(), accent_dim(), apply(), bg(), body(), border() (+12 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.09
Nodes (40): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, ColorPreset, .ALL, Burst (+32 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.05
Nodes (62): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+54 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_color.rs"
Cohesion: 0.16
Nodes (17): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+9 more)

### Community 14 - "jobs"
Cohesion: 0.14
Nodes (13): env, if, needs, runs-on, jobs, announce, name, on (+5 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.20
Nodes (9): Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, Neue Version veröffentlichen, Oberfläche, Show-Format (+1 more)

### Community 19 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 20 - "GH_TOKEN"
Cohesion: 0.29
Nodes (8): env, needs, runs-on, env, BUILD_MANIFEST_NAME, GH_TOKEN, build-global-artifacts, env

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (65): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, Builder, Center, ChevronDown (+57 more)

### Community 23 - "super"
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 25 - "scene_card.rs"
Cohesion: 0.21
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.18
Nodes (11): env, if, needs, outputs, runs-on, host, publishing, tag (+3 more)

### Community 27 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - "duplicate_zone_copies_scenes_behind_original"
Cohesion: 0.21
Nodes (3): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks()

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "super"
Cohesion: 0.07
Nodes (33): digest(), hash_and_verify(), hash_password(), hex(), verify(), COMPACT_W, FINE_STEP_MS, format() (+25 more)

### Community 37 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "sense"
Cohesion: 0.13
Nodes (5): LimitPad, HANDLE, Swatch, Toggle, XyPad

### Community 40 - "stage.rs"
Cohesion: 0.11
Nodes (22): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+14 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.09
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, fine_mark() (+7 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "app.rs"
Cohesion: 0.05
Nodes (24): ControlViews, DmxCraftApp, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Fullscreen, History (+16 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.10
Nodes (25): Settings, full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+17 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "egui"
Cohesion: 0.22
Nodes (3): section_header(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (10): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx, Static (+2 more)

### Community 68 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 69 - "programmer.rs"
Cohesion: 0.13
Nodes (31): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), fader() (+23 more)

### Community 70 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 77 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 78 - "stage_editor.rs"
Cohesion: 0.22
Nodes (8): CanvasMode, Edit, Select, StageCanvas, set_pos(), tool_sep(), plan, runs-on

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 90 - "demo.rs"
Cohesion: 0.18
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step, steps_insert_remove_and_normalize()

### Community 93 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (11): t(), CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output (+3 more)

### Community 99 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 104 - "EffectKind"
Cohesion: 0.14
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 108 - "Step"
Cohesion: 0.20
Nodes (6): SETTLE, Step, Enter, Maximize, FxFile, import_fx()

### Community 109 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 113 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 136 - "egui"
Cohesion: 0.16
Nodes (9): single_scene(), View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView (+1 more)

### Community 137 - "scene_props.rs"
Cohesion: 0.16
Nodes (21): advanced(), banks_of(), bar_slider(), effect_header(), effect_tiles(), FX_EXT, FxDialog, header() (+13 more)

### Community 138 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 140 - "fixture_menu.rs"
Cohesion: 0.22
Nodes (7): fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (13): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+5 more)

### Community 142 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 146 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 147 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

## Knowledge Gaps
- **239 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+234 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 642 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **72 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `AppState`, `core/src/lib.rs`, `runtime.rs`, `theme.rs`, `ShowError`, `stage.rs`, `fx.rs`, `t`, `EffectKind`, `fixture_menu.rs`, `stage_canvas.rs`, `stage_editor.rs`, `app.rs`, `BankScope`, `show.rs`, `demo.rs`, `duplicate_zone_copies_scenes_behind_original`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Are the 65 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 65 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _239 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.06543235851041447 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `core/src/lib.rs`, `runtime.rs`, `programmer.rs`, `egui`, `scene_props.rs`, `t`, `Show`, `fixture_menu.rs`, `app.rs`, `settings_window.rs`, `DmxFrame`?**
  _High betweenness centrality (0.110) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.05794556628621598 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `PatchGrid` to `AppState`, `text_dim`, `egui`, `scene_props.rs`, `t`, `file_dialog.rs`, `fx_color.rs`, `fixture_menu.rs`, `titlebar.rs`, `stage.rs`, `fader.rs`, `i18n/mod.rs`, `app.rs`, `settings_window.rs`, `egui`, `programmer.rs`, `stage_editor.rs`, `dialog.rs`, `Dialog<'a>`, `Step`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._