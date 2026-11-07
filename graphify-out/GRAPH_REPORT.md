# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~95,092 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2058 nodes · 4774 edges · 194 communities (89 shown, 105 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ebea3e58`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- stage.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- pal
- steps
- fx.rs
- OutputError
- t
- Engine
- file_dialog.rs
- fx_ui.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Scene
- ofl/src/lib.rs
- AppState
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- BankScope
- env
- dmxcraft
- knob.rs
- Timeline
- FixtureType
- DmxCraft – UI-Richtlinien
- audio.rs
- FixtureAction
- drag_number
- fader.rs
- qxf.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- hashmap
- t
- PlanShape
- Scene
- DvcGoldUsb
- .paint
- app.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- ChannelValue
- OpenDmx
- show.rs
- dialog.rs
- time_field.rs
- plan
- Toggle<'a>
- .ui
- ChannelKind
- T
- .of
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- Grip
- en.rs
- qlc-bundle.rs
- Rect
- ViewCtx
- mapping.rs
- super
- stage_editor.rs
- core/src/lib.rs
- fixture_menu.rs
- CurveShape
- ColorPreset
- Settings
- t
- scene_card.rs
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Category
- Step
- Drag
- IconButton
- swatch.rs
- PickTarget
- egui
- ProgrammerMode
- FixtureLimits
- arc
- Tab
- Box<dyn serialport::SerialPort>
- elide

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `Show` - 102 edges
3. `t()` - 102 edges
4. `Icon` - 98 edges
5. `ChannelKind` - 52 edges
6. `AppState` - 48 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 38 edges
10. `ViewCtx` - 34 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `fit_mapping()` --references--> `Show`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/views/control/scene_grid.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
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

## Communities (194 total, 105 thin omitted)

### Community 0 - "stage.rs"
Cohesion: 0.20
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (25): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+17 more)

### Community 5 - "programmer.rs"
Cohesion: 0.26
Nodes (18): color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+10 more)

### Community 6 - "pal"
Cohesion: 0.25
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.06
Nodes (59): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+51 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (43): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+35 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.10
Nodes (31): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+23 more)

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
Cohesion: 0.06
Nodes (51): Fullscreen, Mode, Control, Setup, clip(), delete(), HEIGHT, k() (+43 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (75): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+67 more)

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 25 - "AppState"
Cohesion: 0.13
Nodes (8): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, super_scene()

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.13
Nodes (14): heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - "BankScope"
Cohesion: 0.25
Nodes (7): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "FixtureType"
Cohesion: 0.27
Nodes (3): ChannelDef, FixtureType, ft()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.29
Nodes (4): SETTLE, drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.13
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, fine_mark(), paint_symbol()

### Community 45 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.13
Nodes (23): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+15 more)

### Community 50 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 51 - "t"
Cohesion: 0.26
Nodes (3): t(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "Scene"
Cohesion: 0.27
Nodes (3): duplicate_zone_copies_scenes_behind_original(), Show, Zone

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "app.rs"
Cohesion: 0.36
Nodes (4): edit_preview(), load_step(), LockDialog, OUTPUT_HZ

### Community 71 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.17
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.13
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.23
Nodes (15): address_map_marks_ranges(), free_scenes_get_a_bank(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, move_scenes_between_banks(), next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names() (+7 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (15): Button, Dialog, DialogResult, FOOTER_H, FooterButton, Form, form(), HEADER_H (+7 more)

### Community 89 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (20): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+12 more)

### Community 95 - "T"
Cohesion: 0.27
Nodes (3): body(), Segmented, Segmented<'a, T>

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
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 106 - "Grip"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 108 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 109 - "Rect"
Cohesion: 0.22
Nodes (7): paint_fixture(), paint_shape_icon(), arrow(), fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 113 - "ViewCtx"
Cohesion: 0.22
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "super"
Cohesion: 0.05
Nodes (36): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+28 more)

### Community 121 - "stage_editor.rs"
Cohesion: 0.21
Nodes (8): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep()

### Community 123 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (10): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+2 more)

### Community 128 - "fixture_menu.rs"
Cohesion: 0.18
Nodes (6): apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 137 - "t"
Cohesion: 0.16
Nodes (22): advanced(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping(), FX_EXT, FxDialog (+14 more)

### Community 140 - "scene_card.rs"
Cohesion: 0.29
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (17): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+9 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.08
Nodes (28): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, ClipAction, Copy, Cut, Delete, Duplicate (+20 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.16
Nodes (13): ControlViews, Category, All, Beam, Color, Dimmer, Gobo, .ITEMS (+5 more)

### Community 150 - "Step"
Cohesion: 0.29
Nodes (5): Step, Enter, Maximize, FxFile, import_fx()

### Community 152 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 158 - "egui"
Cohesion: 0.11
Nodes (18): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, ICON_FAMILY (+10 more)

### Community 170 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 174 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

## Knowledge Gaps
- **320 isolated node(s):** `All`, `Delete`, `Range`, `New`, `Auto` (+315 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 784 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **105 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Scene` to `fixture_menu.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `Engine`, `stage_canvas.rs`, `Scene`, `FixtureType`, `FixtureAction`, `DmxCraftApp`, `app.rs`, `Show`, `feature.rs`, `motion.rs`, `ChannelValue`, `show.rs`, `ShowError`, `core/src/lib.rs`?**
  _High betweenness centrality (0.168) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `All`, `Delete`, `Range` to the rest of the system?**
  _320 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `stage.rs`, `fixture_menu.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `DmxFrame`, `control/timeline.rs`, `Category`, `AppState`, `audio.rs`, `ProgrammerMode`, `DmxCraftApp`, `settings_window.rs`, `hashmap`, `Scene`, `app.rs`, `ChannelKind`, `Rect`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `stage.rs`, `fixture_menu.rs`, `library_panel.rs`, `theme.rs`, `programmer.rs`, `pal`, `Settings`, `t`, `t`, `file_dialog.rs`, `fx_ui.rs`, `control/timeline.rs`, `titlebar.rs`, `Category`, `Step`, `egui`, `FixtureAction`, `ProgrammerMode`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `app.rs`, `patch_grid.rs`, `dialog.rs`, `Rect`, `ViewCtx`, `super`, `stage_editor.rs`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._