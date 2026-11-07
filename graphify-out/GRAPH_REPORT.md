# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~92,683 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2040 nodes · 4738 edges · 169 communities (92 shown, 77 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 224 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `1af7a97c`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- AppState
- runtime.rs
- open_dmx.rs
- theme.rs
- ChannelKind
- pal
- steps
- fx.rs
- OutputError
- t
- ofl/src/lib.rs
- file_dialog.rs
- fx_ui.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- super
- scene_card.rs
- host
- qxf.rs
- with
- ensure_linux_menu_entry
- convert.rs
- .add_scene
- env
- dmxcraft
- super
- Timeline
- ProgrammerMode
- DmxCraft – UI-Richtlinien
- audio.rs
- stage.rs
- drag_number
- Fader
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- arc
- egui
- PlanShape
- core/src/lib.rs
- qlc-bundle.rs
- programmer.rs
- Tab
- Dialog
- Show
- feature.rs
- OutputKind
- motion.rs
- OflError
- widgets/mod.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- Layer
- FixtureLimits
- limit_pad.rs
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- Category
- en.rs
- Tab
- .ui
- ViewCtx
- mapping.rs
- control/timeline.rs
- DvcGoldUsb
- BankScope
- FixtureType
- hashmap
- CurveShape
- ColorPreset
- t
- app.rs
- swatch.rs
- stage_canvas.rs
- knob.rs
- DmxFrame
- ClipAction
- Theme
- IconButton
- egui
- plan.rs
- PickTarget
- sense
- Pos2
- Arrange
- MenuPlace
- Drag
- HeaderAction
- Step

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `t()` - 100 edges
3. `Show` - 98 edges
4. `Icon` - 97 edges
5. `ChannelKind` - 52 edges
6. `AppState` - 47 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 38 edges
10. `Timeline` - 34 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `ProgrammerView` --references--> `TimelineView`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/views/control/timeline.rs
- `grid_options()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/timeline.rs → crates/app/src/i18n/mod.rs
- `track_header()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/timeline.rs → crates/app/src/i18n/mod.rs

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

## Communities (169 total, 77 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.06
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 1 - "AppState"
Cohesion: 0.05
Nodes (32): AppState, Focus, Bank, Effect, Scenes, Timeline, LibraryDrag, Mode (+24 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (27): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+19 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (20): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+12 more)

### Community 6 - "pal"
Cohesion: 0.22
Nodes (17): load_meter(), show(), accent(), accent_dim(), apply(), bg(), border(), caption() (+9 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.16
Nodes (26): FxPreview, all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels() (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.06
Nodes (57): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+49 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.20
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.10
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

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
Nodes (55): delete(), HEIGHT, k(), levels(), lock(), main_menu(), MenuAction, About (+47 more)

### Community 20 - "announce"
Cohesion: 0.22
Nodes (10): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+2 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (74): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+66 more)

### Community 23 - "super"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 25 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.20
Nodes (10): if, needs, outputs, runs-on, host, publishing, tag, tag-flag (+2 more)

### Community 27 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "super"
Cohesion: 0.10
Nodes (24): digest(), hash_and_verify(), hash_password(), hex(), verify(), COMPACT_W, FINE_STEP_MS, format() (+16 more)

### Community 36 - "Timeline"
Cohesion: 0.11
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "stage.rs"
Cohesion: 0.10
Nodes (24): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+16 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "Fader"
Cohesion: 0.13
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, fine_mark(), paint_symbol()

### Community 45 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "egui"
Cohesion: 0.17
Nodes (4): t(), section_header(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.16
Nodes (10): paint_fixture(), paint_shape_icon(), PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix (+2 more)

### Community 66 - "core/src/lib.rs"
Cohesion: 0.16
Nodes (7): ChannelValue, Scene, SceneKind, Fx, Static, Super, Step

### Community 68 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 69 - "programmer.rs"
Cohesion: 0.26
Nodes (12): elide(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock, fx_target_id() (+4 more)

### Community 70 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 71 - "Dialog"
Cohesion: 0.33
Nodes (6): Button, Dialog, Kind, Cancel, Primary, Secondary

### Community 72 - "Show"
Cohesion: 0.18
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (24): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+16 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "OflError"
Cohesion: 0.40
Nodes (4): OflError, Invalid, Json, Skip

### Community 78 - "widgets/mod.rs"
Cohesion: 0.24
Nodes (7): CanvasMode, Edit, Select, StageCanvas, Underlay, set_pos(), tool_sep()

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.15
Nodes (7): FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H, LABEL_W

### Community 90 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 93 - "Layer"
Cohesion: 0.23
Nodes (7): color_bar(), current(), set_group(), xy_pad(), Layer, Live, Programmer

### Community 95 - "limit_pad.rs"
Cohesion: 0.15
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 101 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 104 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 106 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 108 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 109 - ".ui"
Cohesion: 0.34
Nodes (4): fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 113 - "ViewCtx"
Cohesion: 0.19
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.16
Nodes (12): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, RULER_H (+4 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 121 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 123 - "FixtureType"
Cohesion: 0.14
Nodes (9): SceneProgress, ChannelDef, FixtureType, ft(), LoopMode, Loop, Once, PLAN_SIZE_RANGE (+1 more)

### Community 128 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 137 - "t"
Cohesion: 0.16
Nodes (23): advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping(), FX_EXT (+15 more)

### Community 138 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 141 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (11): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, StageInteraction, strobe_hz() (+3 more)

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 153 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "egui"
Cohesion: 0.10
Nodes (17): SETTLE, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+9 more)

### Community 155 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 165 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 169 - "MenuPlace"
Cohesion: 0.50
Nodes (4): MenuPlace, Patch, Stage, StageEditor

### Community 170 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 171 - "HeaderAction"
Cohesion: 0.24
Nodes (9): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, row_h(), track_h() (+1 more)

### Community 172 - "Step"
Cohesion: 0.16
Nodes (7): Fullscreen, Step, Enter, Maximize, FxFile, import_fx(), mark_clipboard()

## Knowledge Gaps
- **314 isolated node(s):** `Move`, `Left`, `Right`, `FadeIn`, `FadeOut` (+309 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 756 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **77 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `AppState`, `runtime.rs`, `theme.rs`, `fx.rs`, `app.rs`, `t`, `stage_canvas.rs`, `.add_scene`, `stage.rs`, `DmxCraftApp`, `core/src/lib.rs`, `feature.rs`, `motion.rs`, `widgets/mod.rs`, `show.rs`, `demo.rs`, `ShowError`, `EffectKind`, `BankScope`, `FixtureType`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Move`, `Left`, `Right` to the rest of the system?**
  _314 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.057133985405992856 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `egui` to `AppState`, `pal`, `t`, `app.rs`, `t`, `file_dialog.rs`, `fx_ui.rs`, `titlebar.rs`, `super`, `egui`, `ProgrammerMode`, `stage.rs`, `HeaderAction`, `Fader`, `i18n/mod.rs`, `Step`, `DmxCraftApp`, `settings_window.rs`, `programmer.rs`, `widgets/mod.rs`, `dialog.rs`, `Layer`, `PatchGrid`, `Dialog<'a>`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`?**
  _High betweenness centrality (0.120) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.0532724505327245 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `hashmap`, `AppState`, `runtime.rs`, `ChannelKind`, `t`, `app.rs`, `t`, `DmxFrame`, `Timeline`, `ProgrammerMode`, `audio.rs`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `programmer.rs`, `Show`, `Layer`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`, `FixtureType`?**
  _High betweenness centrality (0.108) - this node is a cross-community bridge._