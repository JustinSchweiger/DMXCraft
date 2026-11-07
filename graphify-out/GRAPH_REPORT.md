# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~93,400 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2041 nodes · 4741 edges · 171 communities (92 shown, 79 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 225 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `876cf52b`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
- FixtureType
- runtime.rs
- open_dmx.rs
- theme.rs
- ChannelKind
- text_dim
- steps
- fx.rs
- OutputError
- t
- stage.rs
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
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- .add_scene
- env
- dmxcraft
- time_field.rs
- Timeline
- ProgrammerMode
- DmxCraft – UI-Richtlinien
- audio.rs
- stage.rs
- drag_number
- Fader
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- AppState
- MenuAction
- PlanShape
- demo.rs
- qxf.rs
- programmer.rs
- ofl/src/lib.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- Vec2
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- plan
- Toggle<'a>
- Scene
- FixtureLimits
- strokekind
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
- qlc-bundle.rs
- ViewCtx
- mapping.rs
- control/timeline.rs
- egui
- BankScope
- core/src/lib.rs
- hashmap
- CurveShape
- ColorPreset
- body
- t
- Step
- swatch.rs
- stage_canvas.rs
- Effect
- DmxFrame
- Fullscreen
- ClipAction
- Theme
- UpdateBadge
- Drag
- elide
- IconButton
- Vec2
- PickTarget
- fullscreen.rs
- .show
- resize_handles
- HeaderAction
- arc

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `t()` - 100 edges
3. `Icon` - 98 edges
4. `Show` - 98 edges
5. `ChannelKind` - 52 edges
6. `AppState` - 47 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 38 edges
10. `ViewCtx` - 34 edges

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

## Communities (171 total, 79 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (43): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+35 more)

### Community 1 - "FixtureType"
Cohesion: 0.14
Nodes (7): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

### Community 2 - "runtime.rs"
Cohesion: 0.18
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (31): accent_dim(), accent_hover(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+23 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (19): FxTarget, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+11 more)

### Community 6 - "text_dim"
Cohesion: 0.19
Nodes (9): load_meter(), show(), accent(), bg(), caption(), heading_font(), text_dim(), DipSwitch (+1 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.09
Nodes (10): NullOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputError, NotFound (+2 more)

### Community 10 - "t"
Cohesion: 0.06
Nodes (59): ControlViews, color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS (+51 more)

### Community 11 - "stage.rs"
Cohesion: 0.16
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.14
Nodes (11): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Pending, Purpose, OpenShow, SaveShowAs (+3 more)

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
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (75): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+67 more)

### Community 23 - "super"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 25 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.19
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - ".add_scene"
Cohesion: 0.22
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Show, Zone

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.14
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "stage.rs"
Cohesion: 0.08
Nodes (29): fixture_pos(), apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH (+21 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "Fader"
Cohesion: 0.16
Nodes (6): Fader, FaderEvent, Changed, None, Release, ToggleFx

### Community 45 - "super"
Cohesion: 0.05
Nodes (36): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+28 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.13
Nodes (24): t(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+16 more)

### Community 50 - "AppState"
Cohesion: 0.12
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 51 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 68 - "qxf.rs"
Cohesion: 0.17
Nodes (12): ChannelDef, ft(), channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset() (+4 more)

### Community 69 - "programmer.rs"
Cohesion: 0.22
Nodes (19): color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+11 more)

### Community 70 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

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
Cohesion: 0.11
Nodes (11): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation, OutputStatus, Connecting (+3 more)

### Community 76 - "motion.rs"
Cohesion: 0.13
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "Vec2"
Cohesion: 0.11
Nodes (14): DOT_H, FADER_W, fine_mark(), HANDLE, NUM_H, paint_symbol(), SYMBOL_H, VALUE_H (+6 more)

### Community 78 - "stage_editor.rs"
Cohesion: 0.16
Nodes (11): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (14): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H (+6 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 95 - "strokekind"
Cohesion: 0.20
Nodes (7): text_faint(), Grip, Body, Corner, EdgeX, EdgeY, LimitPad

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
Cohesion: 0.24
Nodes (7): fmt_ms(), grid_options(), paint_clip(), row_h(), TimelineView, track_h(), track_header()

### Community 110 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 113 - "ViewCtx"
Cohesion: 0.19
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.15
Nodes (12): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, RULER_H (+4 more)

### Community 119 - "egui"
Cohesion: 0.22
Nodes (3): section_header(), ZonesView, Windows

### Community 121 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 123 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (12): ChannelValue, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+4 more)

### Community 128 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "body"
Cohesion: 0.31
Nodes (3): body(), Segmented, Segmented<'a, T>

### Community 137 - "t"
Cohesion: 0.17
Nodes (22): advanced(), banks_of(), bar_slider(), block_props(), effect_tiles(), fit_mapping(), FX_EXT, FxDialog (+14 more)

### Community 138 - "Step"
Cohesion: 0.29
Nodes (5): Step, Enter, Maximize, FxFile, import_fx()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.17
Nodes (15): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+7 more)

### Community 142 - "Effect"
Cohesion: 0.29
Nodes (4): Show, effect_header(), FxPreview, Effect

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "UpdateBadge"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 151 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 154 - "Vec2"
Cohesion: 0.33
Nodes (5): from_pct(), pct(), RangeSlider, value_field(), VALUE_H

### Community 156 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 158 - ".show"
Cohesion: 0.17
Nodes (5): raised(), raised_hover(), Knob, Knob<'a>, Toggle

### Community 171 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

## Knowledge Gaps
- **315 isolated node(s):** `Play`, `Pause`, `Prev`, `Next`, `Plus` (+310 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 756 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **79 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `.add_scene` to `Engine`, `FixtureType`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `stage.rs`, `file_dialog.rs`, `stage_canvas.rs`, `stage.rs`, `DmxCraftApp`, `demo.rs`, `Show`, `feature.rs`, `motion.rs`, `show.rs`, `Scene`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.133) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Play`, `Pause`, `Prev` to the rest of the system?**
  _315 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.05322128851540616 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `hashmap`, `FixtureType`, `runtime.rs`, `t`, `t`, `stage.rs`, `file_dialog.rs`, `.add_scene`, `ProgrammerMode`, `audio.rs`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `programmer.rs`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.109) - this node is a cross-community bridge._
- **Should `FixtureType` be split into smaller, more focused modules?**
  _Cohesion score 0.14153846153846153 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `PatchGrid`, `programmer.rs`, `stage.rs`, `t`, `t`, `stage.rs`, `Vec2`, `stage_editor.rs`, `settings_window.rs`, `ViewCtx`, `titlebar.rs`, `control/timeline.rs`, `dialog.rs`, `IconButton`, `menu.rs`, `scene_card.rs`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._