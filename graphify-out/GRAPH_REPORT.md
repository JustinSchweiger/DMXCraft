# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~94,436 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2054 nodes · 4766 edges · 190 communities (90 shown, 100 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `42c29697`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- ofl/src/lib.rs
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
- qxf.rs
- qlc-bundle.rs
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
- stage.rs
- drag_number
- Vec2
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- hashmap
- MenuItem<'a>
- PlanShape
- Scene
- DvcGoldUsb
- ScenePropsView
- app.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- ClipAction
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- Settings
- plan
- Toggle<'a>
- Drag
- ChannelKind
- strokekind
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- OflError
- en.rs
- Tab
- Rect
- FxFile
- ViewCtx
- mapping.rs
- time_field.rs
- Effect
- core/src/lib.rs
- CurveShape
- ColorPreset
- settings.rs
- t
- demo.rs
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Category
- IconButton
- swatch.rs
- PickTarget
- toggle.rs
- ProgrammerMode
- FixtureLimits
- arc
- send_frame

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
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `hidden_banks_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/i18n/mod.rs
- `bank_menu()` --calls--> `color_swatches()`  [INFERRED]
  crates/app/src/views/control/scene_grid.rs → crates/app/src/ui/widgets/color_picker.rs

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

## Communities (190 total, 100 thin omitted)

### Community 0 - "ofl/src/lib.rs"
Cohesion: 0.16
Nodes (9): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, Library, LibraryFixture, Source, Ofl (+1 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (27): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+19 more)

### Community 5 - "programmer.rs"
Cohesion: 0.20
Nodes (15): elide(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock, fx_target_id() (+7 more)

### Community 6 - "pal"
Cohesion: 0.25
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels(), hard_edges_use_palette_colors_only() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.08
Nodes (48): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W, programmer_values(), Action, Bank (+40 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.20
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.09
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
Cohesion: 0.05
Nodes (55): Fullscreen, SETTLE, Step, Enter, Maximize, Mode, Control, Setup (+47 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (75): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+67 more)

### Community 23 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 24 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 25 - "AppState"
Cohesion: 0.13
Nodes (7): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

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

### Community 34 - "knob.rs"
Cohesion: 0.20
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "FixtureType"
Cohesion: 0.29
Nodes (3): ChannelDef, FixtureType, ft()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "stage.rs"
Cohesion: 0.06
Nodes (47): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+39 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "Vec2"
Cohesion: 0.05
Nodes (34): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+26 more)

### Community 45 - "super"
Cohesion: 0.05
Nodes (36): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+28 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.13
Nodes (24): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+16 more)

### Community 50 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 65 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "Scene"
Cohesion: 0.11
Nodes (7): ChannelValue, duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, Step, steps_insert_remove_and_normalize()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "ScenePropsView"
Cohesion: 0.33
Nodes (6): FxDialog, Picker, picker_window(), ScenePropsView, single_scene(), toggle()

### Community 70 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.16
Nodes (8): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show, Zone

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (22): apply_effect(), apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 78 - "stage_editor.rs"
Cohesion: 0.15
Nodes (12): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+4 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+5 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 95 - "strokekind"
Cohesion: 0.11
Nodes (9): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T> (+1 more)

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
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 106 - "OflError"
Cohesion: 0.40
Nodes (4): OflError, Invalid, Json, Skip

### Community 108 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 109 - "Rect"
Cohesion: 0.21
Nodes (9): paint_fixture(), paint_shape_icon(), add_audio(), add_audio_dialog(), fmt_ms(), grid_options(), paint_clip(), TimelineView (+1 more)

### Community 113 - "ViewCtx"
Cohesion: 0.13
Nodes (10): section_header(), View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView (+2 more)

### Community 117 - "mapping.rs"
Cohesion: 0.18
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 118 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 121 - "Effect"
Cohesion: 0.27
Nodes (6): Show, effect_header(), effect_tiles(), fit_mapping(), FxPreview, Effect

### Community 123 - "core/src/lib.rs"
Cohesion: 0.13
Nodes (12): SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+4 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "settings.rs"
Cohesion: 0.22
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 137 - "t"
Cohesion: 0.25
Nodes (13): advanced(), bar_slider(), block_props(), FX_EXT, header(), pick_tile(), PickColumn, properties() (+5 more)

### Community 140 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.20
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, StageCanvas (+6 more)

### Community 143 - "DmxFrame"
Cohesion: 0.18
Nodes (8): color_bar(), current(), set_group(), xy_pad(), Layer, Live, Programmer, DmxFrame

### Community 146 - "control/timeline.rs"
Cohesion: 0.13
Nodes (17): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete, Duplicate, ExpandAll (+9 more)

### Community 147 - "Theme"
Cohesion: 0.22
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.21
Nodes (11): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+3 more)

### Community 153 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 170 - "ProgrammerMode"
Cohesion: 0.24
Nodes (8): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch(), tool_separator(), toolbar()

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 177 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

## Knowledge Gaps
- **319 isolated node(s):** `All`, `Delete`, `New`, `Auto`, `Color` (+314 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 781 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **100 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Scene`, `runtime.rs`, `theme.rs`, `FixtureType`, `app.rs`, `ShowError`, `stage.rs`, `fx.rs`, `t`, `Engine`, `demo.rs`, `stage_canvas.rs`, `feature.rs`, `DmxCraftApp`, `motion.rs`, `EffectKind`, `show.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.169) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `All`, `Delete`, `New` to the rest of the system?**
  _319 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `DmxFrame`, `control/timeline.rs`, `Category`, `Timeline`, `audio.rs`, `stage.rs`, `ProgrammerMode`, `DmxCraftApp`, `settings_window.rs`, `hashmap`, `ScenePropsView`, `app.rs`, `Show`, `Rect`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.122) - this node is a cross-community bridge._
- **Should `theme.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0928030303030303 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `PatchGrid`, `programmer.rs`, `stage.rs`, `t`, `t`, `Vec2`, `stage_editor.rs`, `settings_window.rs`, `ViewCtx`, `control/timeline.rs`, `titlebar.rs`, `MenuItem<'a>`, `dialog.rs`, `IconButton`, `menu.rs`?**
  _High betweenness centrality (0.094) - this node is a cross-community bridge._