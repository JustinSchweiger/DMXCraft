# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~92,020 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2038 nodes · 4724 edges · 177 communities (100 shown, 77 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 223 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `7985c68a`
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
- menu.rs
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
- time_field.rs
- Timeline
- titlebar.rs
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
- MenuAction
- library_panel.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- TitleInfo
- widgets/mod.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- Layer
- FixtureLimits
- T
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- Category
- en.rs
- Effect
- .ui
- MoveShape
- Settings
- ViewCtx
- mapping.rs
- control/timeline.rs
- DvcGoldUsb
- BankScope
- FixtureType
- hashmap
- CurveShape
- ColorPreset
- ChaserPreset
- t
- app.rs
- swatch.rs
- stage_canvas.rs
- knob.rs
- DmxFrame
- resize_handles
- ClipAction
- Theme
- lock.rs
- CurveFx
- IconButton
- egui
- plan.rs
- Transform
- PickTarget
- sense
- Pos2
- plan
- Fullscreen
- Arrange
- dip.rs
- Grip
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
6. `AppState` - 46 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 38 edges
10. `ViewCtx` - 34 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `AppState` --references--> `AudioPlayer`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/audio.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs

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

## Communities (177 total, 77 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 1 - "AppState"
Cohesion: 0.07
Nodes (27): ControlViews, AppState, Focus, Bank, Effect, Scenes, Timeline, Mode (+19 more)

### Community 2 - "runtime.rs"
Cohesion: 0.10
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (26): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+18 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (20): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+12 more)

### Community 6 - "pal"
Cohesion: 0.20
Nodes (18): load_meter(), show(), accent(), accent_dim(), apply(), bg(), body(), border() (+10 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.22
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), feature_levels(), grayscale_has_equal_channels(), hard_edges_use_palette_colors_only() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (57): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+49 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.23
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

### Community 19 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (74): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+66 more)

### Community 23 - "super"
Cohesion: 0.18
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 25 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "qxf.rs"
Cohesion: 0.26
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.12
Nodes (25): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+17 more)

### Community 31 - ".add_scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

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
Nodes (13): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+5 more)

### Community 37 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.15
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "stage.rs"
Cohesion: 0.10
Nodes (24): apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV (+16 more)

### Community 41 - "drag_number"
Cohesion: 0.29
Nodes (4): SETTLE, drag_number(), endless_delta(), raw_available()

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
Cohesion: 0.13
Nodes (24): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+16 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "egui"
Cohesion: 0.22
Nodes (3): section_header(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.16
Nodes (10): paint_fixture(), paint_shape_icon(), PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix (+2 more)

### Community 66 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (12): SceneProgress, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx (+4 more)

### Community 68 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 69 - "programmer.rs"
Cohesion: 0.18
Nodes (14): ProgrammerMode, Blind, Edit, Live, elide(), fader(), fixture_block(), fx_target_id() (+6 more)

### Community 70 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 71 - "library_panel.rs"
Cohesion: 0.16
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 72 - "Show"
Cohesion: 0.17
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.26
Nodes (10): apply_levels(), block(), chaser_levels(), ChaserFx, fill_ends_full(), fixture_beams(), FixtureBeam, inward_is_symmetric() (+2 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.25
Nodes (7): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, points_jump()

### Community 77 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

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
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 90 - "demo.rs"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 93 - "Layer"
Cohesion: 0.23
Nodes (13): color_bar(), current(), fader_groups(), FaderGroup, fixture_blocks(), FixtureBlock, merged_block(), merged_or() (+5 more)

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

### Community 108 - "Effect"
Cohesion: 0.27
Nodes (6): Show, effect_header(), effect_tiles(), fit_mapping(), FxPreview, Effect

### Community 109 - ".ui"
Cohesion: 0.27
Nodes (4): fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 110 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 113 - "ViewCtx"
Cohesion: 0.16
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.14
Nodes (13): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, mark_clipboard() (+5 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 121 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 123 - "FixtureType"
Cohesion: 0.27
Nodes (3): ChannelDef, FixtureType, ft()

### Community 128 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.16
Nodes (11): ColorFx, ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow (+3 more)

### Community 136 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 137 - "t"
Cohesion: 0.18
Nodes (21): t(), advanced(), banks_of(), bar_slider(), block_props(), FX_EXT, FxDialog, header() (+13 more)

### Community 138 - "app.rs"
Cohesion: 0.31
Nodes (4): edit_preview(), load_step(), LockDialog, OUTPUT_HZ

### Community 141 - "stage_canvas.rs"
Cohesion: 0.23
Nodes (11): BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, StageInteraction, strobe_hz() (+3 more)

### Community 142 - "knob.rs"
Cohesion: 0.20
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 143 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 151 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 152 - "CurveFx"
Cohesion: 0.38
Nodes (4): curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value()

### Community 154 - "egui"
Cohesion: 0.11
Nodes (17): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, ICON_FAMILY (+9 more)

### Community 155 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 156 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 157 - "PickTarget"
Cohesion: 0.21
Nodes (8): PickTarget, Jump, Protect, Release, Tab, Effects, Properties, Third

### Community 161 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 165 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 167 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 168 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

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
Cohesion: 0.29
Nodes (5): Step, Enter, Maximize, FxFile, import_fx()

## Knowledge Gaps
- **314 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+309 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 754 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **77 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `theme.rs`, `fx.rs`, `app.rs`, `t`, `stage_canvas.rs`, `.add_scene`, `stage.rs`, `DmxCraftApp`, `core/src/lib.rs`, `feature.rs`, `motion.rs`, `widgets/mod.rs`, `show.rs`, `demo.rs`, `ShowError`, `EffectKind`, `BankScope`, `FixtureType`?**
  _High betweenness centrality (0.150) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _314 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.053838484546360914 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `hashmap`, `AppState`, `runtime.rs`, `ChannelKind`, `t`, `app.rs`, `t`, `audio.rs`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `core/src/lib.rs`, `programmer.rs`, `library_panel.rs`, `Show`, `Layer`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.07390648567119155 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `AppState`, `pal`, `app.rs`, `t`, `file_dialog.rs`, `fx_ui.rs`, `egui`, `titlebar.rs`, `stage.rs`, `HeaderAction`, `Fader`, `i18n/mod.rs`, `Step`, `DmxCraftApp`, `settings_window.rs`, `egui`, `programmer.rs`, `library_panel.rs`, `widgets/mod.rs`, `dialog.rs`, `Layer`, `PatchGrid`, `Dialog<'a>`, `Category`, `Effect`, `.ui`, `Settings`, `ViewCtx`, `control/timeline.rs`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._