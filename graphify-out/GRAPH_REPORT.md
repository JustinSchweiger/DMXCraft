# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~94,415 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2054 nodes · 4766 edges · 185 communities (90 shown, 95 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `15a02d99`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
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
- engine.rs
- file_dialog.rs
- fx_ui.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- i18n/mod.rs
- ofl/src/lib.rs
- AppState
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- .add_scene
- env
- dmxcraft
- qxf.rs
- Timeline
- qlc-bundle.rs
- DmxCraft – UI-Richtlinien
- audio.rs
- stage.rs
- drag_number
- Fader
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- Layer
- MenuAction
- PlanShape
- Scene
- DvcGoldUsb
- segmented.rs
- app.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- Knob<'a>
- Arrange
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- plan
- Toggle<'a>
- FixtureType
- ChannelKind
- egui
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- scene_card.rs
- en.rs
- Tab
- control/timeline.rs
- Step
- ViewCtx
- mapping.rs
- time_field.rs
- Ui
- Effect
- core/src/lib.rs
- .ui
- CurveShape
- ColorPreset
- settings.rs
- t
- demo.rs
- stage_canvas.rs
- DmxFrame
- ClipAction
- Theme
- Category
- Settings
- IconButton
- Step
- swatch.rs
- PickTarget
- sense
- BankScope
- HeaderAction
- arc
- .fade_state
- Box<dyn serialport::SerialPort>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `Show` - 102 edges
3. `t()` - 101 edges
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
- `mapping_handles()` --references--> `Camera`  [INFERRED]
  crates/app/src/views/control/stage.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `paint_mapping()` --references--> `Camera`  [INFERRED]
  crates/app/src/views/control/stage.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `apply()` --calls--> `fixture_pos()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs
- `menu()` --calls--> `paint_shape_icon()`  [INFERRED]
  crates/app/src/views/fixture_menu.rs → crates/app/src/ui/widgets/stage_canvas.rs

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

## Communities (185 total, 95 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.08
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

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
Nodes (27): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+19 more)

### Community 5 - "programmer.rs"
Cohesion: 0.17
Nodes (24): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+16 more)

### Community 6 - "pal"
Cohesion: 0.23
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, apply_levels(), feature_levels() (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.06
Nodes (57): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+49 more)

### Community 11 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.10
Nodes (32): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+24 more)

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
Cohesion: 0.18
Nodes (23): Mode, Control, Setup, clip(), HEIGHT, k(), levels(), load_meter() (+15 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (75): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+67 more)

### Community 23 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 25 - "AppState"
Cohesion: 0.14
Nodes (7): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.15
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - ".add_scene"
Cohesion: 0.21
Nodes (3): duplicate_zone_copies_scenes_behind_original(), move_scenes_between_banks(), Zone

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 36 - "Timeline"
Cohesion: 0.13
Nodes (13): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+5 more)

### Community 37 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "stage.rs"
Cohesion: 0.05
Nodes (47): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+39 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "Fader"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "super"
Cohesion: 0.08
Nodes (27): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password() (+19 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.21
Nodes (3): DmxCraftApp, t(), delete()

### Community 48 - "settings_window.rs"
Cohesion: 0.13
Nodes (24): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+16 more)

### Community 50 - "Layer"
Cohesion: 0.11
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 51 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "Scene"
Cohesion: 0.13
Nodes (7): super_scene(), toggle_play(), free_scenes_get_a_bank(), Scene, SceneSettings, Step, steps_insert_remove_and_normalize()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 70 - "app.rs"
Cohesion: 0.15
Nodes (13): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, UpdateBadge, Available, Busy (+5 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.12
Nodes (19): block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward, Outward (+11 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 78 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.15
Nodes (10): Button, Dialog, FOOTER_H, HEADER_H, INPUT_H, Kind, Cancel, Primary (+2 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - "FixtureType"
Cohesion: 0.27
Nodes (3): ChannelDef, FixtureType, ft()

### Community 94 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 95 - "egui"
Cohesion: 0.10
Nodes (16): HANDLE, Knob, SIZE, SWEEP, Grip, Body, Corner, EdgeX (+8 more)

### Community 96 - "PatchGrid"
Cohesion: 0.21
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
Nodes (12): effect_tiles(), EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings (+4 more)

### Community 106 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 108 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 109 - "control/timeline.rs"
Cohesion: 0.11
Nodes (20): paint_fixture(), paint_shape_icon(), add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, fmt_ms() (+12 more)

### Community 113 - "ViewCtx"
Cohesion: 0.15
Nodes (10): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView (+2 more)

### Community 117 - "mapping.rs"
Cohesion: 0.18
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 118 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 123 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): SceneProgress, FixtureLimits, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "settings.rs"
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 137 - "t"
Cohesion: 0.16
Nodes (22): advanced(), banks_of(), bar_slider(), block_props(), effect_header(), fit_mapping(), FX_EXT, FxDialog (+14 more)

### Community 140 - "demo.rs"
Cohesion: 0.31
Nodes (5): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (18): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+10 more)

### Community 146 - "ClipAction"
Cohesion: 0.13
Nodes (14): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split (+6 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 153 - "IconButton"
Cohesion: 0.12
Nodes (4): FooterButton, ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "Step"
Cohesion: 0.18
Nodes (7): Fullscreen, SETTLE, Step, Enter, Maximize, resize_handles(), mark_clipboard()

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 170 - "BankScope"
Cohesion: 0.29
Nodes (6): BankScope, All, Bank, Banks, Off, OutsideBank

### Community 171 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 174 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **319 isolated node(s):** `Select`, `Edit`, `Round`, `Square`, `Matrix` (+314 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 773 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **95 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `engine.rs`, `DmxFrame`, `audio.rs`, `stage.rs`, `super`, `.fade_state`, `DmxCraftApp`, `settings_window.rs`, `Layer`, `Scene`, `app.rs`, `Show`, `control/timeline.rs`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.140) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `Round` to the rest of the system?**
  _319 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.07542087542087542 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `engine.rs`, `demo.rs`, `stage_canvas.rs`, `.add_scene`, `stage.rs`, `super`, `DmxCraftApp`, `Scene`, `app.rs`, `feature.rs`, `motion.rs`, `show.rs`, `FixtureType`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.139) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `MenuItem<'a>`, `PatchGrid`, `programmer.rs`, `stage.rs`, `t`, `t`, `scene_card.rs`, `Fader`, `control/timeline.rs`, `stage_canvas.rs`, `settings_window.rs`, `ViewCtx`, `titlebar.rs`, `dialog.rs`, `IconButton`, `menu.rs`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._