# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 88 files · ~82,852 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 1884 nodes · 4348 edges · 167 communities (87 shown, 80 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 199 edges (avg confidence: 0.82)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `8bd18be6`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- engine.rs
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
- time_field.rs
- titlebar.rs
- DmxCraft – UI-Richtlinien
- Stroke
- stage.rs
- drag_number
- Fader
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- arc
- widgets/mod.rs
- PlanShape
- core/src/lib.rs
- qlc-bundle.rs
- programmer.rs
- MenuAction
- Engine
- Show
- feature.rs
- OutputKind
- motion.rs
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
- strokekind
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- en.rs
- Layer
- BankScope
- mapping.rs
- Snapshot
- DvcGoldUsb
- ChannelRef
- Vec2
- CurveShape
- ColorPreset
- output/src/lib.rs
- t
- app.rs
- fixture_menu.rs
- stage_canvas.rs
- knob.rs
- DmxFrame
- resize_handles
- icons.rs
- super
- elide
- Tab
- IconButton
- range_slider.rs
- plan.rs
- Transform
- accent
- sense
- dip.rs
- Dialog
- Fullscreen

## God Nodes (most connected - your core abstractions)
1. `Engine` - 123 edges
2. `Show` - 95 edges
3. `t()` - 92 edges
4. `Icon` - 87 edges
5. `ChannelKind` - 53 edges
6. `DmxFrame` - 40 edges
7. `ChannelRef` - 39 edges
8. `DmxCraftApp` - 38 edges
9. `AppState` - 38 edges
10. `ViewCtx` - 34 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `builtin()` --calls--> `t()`  [INFERRED]
  crates/app/src/palettes.rs → crates/app/src/i18n/mod.rs
- `save_user()` --calls--> `t()`  [INFERRED]
  crates/app/src/palettes.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs

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

## Communities (167 total, 80 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.22
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 1 - "AppState"
Cohesion: 0.06
Nodes (28): AppState, Focus, Bank, Effect, Scenes, LibraryDrag, Mode, Control (+20 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (36): accent_dim(), accent_hover(), active(), apply(), c(), DANGER, DIP_BODY, fixture_bar() (+28 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (20): ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+12 more)

### Community 6 - "text_dim"
Cohesion: 0.26
Nodes (9): load_meter(), show(), body(), caption(), heading_font(), heading_text(), text(), text_dim() (+1 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.14
Nodes (7): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (57): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+49 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_color.rs"
Cohesion: 0.10
Nodes (30): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+22 more)

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
Cohesion: 0.20
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

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
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 25 - "scene_card.rs"
Cohesion: 0.22
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
Cohesion: 0.15
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Step, steps_insert_remove_and_normalize()

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 37 - "titlebar.rs"
Cohesion: 0.27
Nodes (17): delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier(), new() (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "Stroke"
Cohesion: 0.21
Nodes (4): border(), raised(), Swatch, XyPad

### Community 40 - "stage.rs"
Cohesion: 0.11
Nodes (22): FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH, MirrorV, NewGroup (+14 more)

### Community 41 - "drag_number"
Cohesion: 0.27
Nodes (4): SETTLE, drag_number(), endless_delta(), raw_available()

### Community 44 - "Fader"
Cohesion: 0.16
Nodes (6): Fader, FaderEvent, Changed, None, Release, ToggleFx

### Community 45 - "i18n/mod.rs"
Cohesion: 0.09
Nodes (15): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+7 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "widgets/mod.rs"
Cohesion: 0.32
Nodes (5): section_header(), CanvasMode, Edit, Select, StageCanvas

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
Cohesion: 0.19
Nodes (23): category_list(), color_bar(), current(), fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks() (+15 more)

### Community 70 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 71 - "Engine"
Cohesion: 0.09
Nodes (5): Engine, FadeState, In, Out, PlayingScene

### Community 72 - "Show"
Cohesion: 0.18
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.13
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "TitleInfo"
Cohesion: 0.22
Nodes (7): TitleInfo, UpdateBadge, Available, Busy, Error, Installed, None

### Community 78 - "stage_editor.rs"
Cohesion: 0.18
Nodes (10): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep() (+2 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.14
Nodes (7): FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H, LABEL_W

### Community 90 - "demo.rs"
Cohesion: 0.31
Nodes (5): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue

### Community 93 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 95 - "strokekind"
Cohesion: 0.13
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
Cohesion: 0.14
Nodes (10): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+2 more)

### Community 109 - "Layer"
Cohesion: 0.11
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 113 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 128 - "Vec2"
Cohesion: 0.20
Nodes (10): DOT_H, FADER_W, fine_mark(), HANDLE, NUM_H, paint_symbol(), SYMBOL_H, VALUE_H (+2 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "output/src/lib.rs"
Cohesion: 0.17
Nodes (6): OutputConfig, OutputStatus, Connecting, Error, Running, Stopped

### Community 137 - "t"
Cohesion: 0.06
Nodes (47): Step, Enter, Maximize, t(), advanced(), banks_of(), bar_slider(), effect_header() (+39 more)

### Community 138 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 140 - "fixture_menu.rs"
Cohesion: 0.22
Nodes (7): fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (13): BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_fixture(), paint_shape_icon() (+5 more)

### Community 142 - "knob.rs"
Cohesion: 0.19
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 147 - "super"
Cohesion: 0.27
Nodes (6): digest(), hash_and_verify(), hash_password(), hex(), verify(), UNIVERSE_SIZE

### Community 152 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 154 - "range_slider.rs"
Cohesion: 0.33
Nodes (5): from_pct(), pct(), RangeSlider, value_field(), VALUE_H

### Community 155 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 156 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 157 - "accent"
Cohesion: 0.46
Nodes (3): accent(), bg(), DipSwitch

### Community 160 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 161 - "Dialog"
Cohesion: 0.33
Nodes (6): Button, Dialog, Kind, Cancel, Primary, Secondary

## Knowledge Gaps
- **280 isolated node(s):** `De`, `En`, `None`, `Release`, `ToggleFx` (+275 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 703 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **80 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `engine.rs`, `AppState`, `runtime.rs`, `theme.rs`, `fx.rs`, `app.rs`, `t`, `fixture_menu.rs`, `stage_canvas.rs`, `duplicate_zone_copies_scenes_behind_original`, `stage.rs`, `DmxCraftApp`, `widgets/mod.rs`, `core/src/lib.rs`, `Engine`, `feature.rs`, `motion.rs`, `show.rs`, `demo.rs`, `ShowError`, `EffectKind`, `BankScope`, `Snapshot`, `ChannelRef`?**
  _High betweenness centrality (0.123) - this node is a cross-community bridge._
- **Are the 71 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 71 INFERRED edges - model-reasoned connections that need verification._
- **What connects `De`, `En`, `None` to the rest of the system?**
  _280 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.05548654244306418 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `Vec2`, `AppState`, `text_dim`, `app.rs`, `t`, `file_dialog.rs`, `fx_color.rs`, `fixture_menu.rs`, `super`, `titlebar.rs`, `stage.rs`, `Fader`, `i18n/mod.rs`, `DmxCraftApp`, `settings_window.rs`, `programmer.rs`, `stage_editor.rs`, `dialog.rs`, `strokekind`, `PatchGrid`, `Dialog<'a>`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14130434782608695 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `MenuItem<'a>`, `AppState`, `PatchGrid`, `titlebar.rs`, `programmer.rs`, `t`, `t`, `fixture_menu.rs`, `stage_editor.rs`, `settings_window.rs`, `icons.rs`, `menu.rs`, `widgets/mod.rs`, `dialog.rs`, `Tab`, `IconButton`, `scene_card.rs`?**
  _High betweenness centrality (0.084) - this node is a cross-community bridge._