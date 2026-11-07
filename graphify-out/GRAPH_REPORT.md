# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~93,165 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2040 nodes · 4740 edges · 167 communities (90 shown, 77 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 225 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f68774ea`
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
- color_picker.rs
- MenuAction
- PlanShape
- ChannelValue
- src/timeline.rs
- programmer.rs
- Tab
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- knob.rs
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- plan
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
- core/src/lib.rs
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
- UpdateBadge
- .check_placement
- elide
- IconButton
- Vec2
- sense
- HeaderAction
- egui

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

## Communities (167 total, 77 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 1 - "AppState"
Cohesion: 0.06
Nodes (19): AppState, Focus, Bank, Effect, Scenes, Timeline, LibraryDrag, Mode (+11 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (25): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+17 more)

### Community 5 - "ChannelKind"
Cohesion: 0.08
Nodes (19): FxTarget, ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic (+11 more)

### Community 6 - "pal"
Cohesion: 0.23
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
Cohesion: 0.10
Nodes (43): programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash, FocusBank (+35 more)

### Community 11 - "stage.rs"
Cohesion: 0.16
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

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

### Community 19 - "titlebar.rs"
Cohesion: 0.18
Nodes (21): delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu(), modifier() (+13 more)

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
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 25 - "scene_card.rs"
Cohesion: 0.22
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.14
Nodes (14): heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

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
Cohesion: 0.15
Nodes (6): Scene, Clip, fades(), paste_finds_free_place_and_keeps_gaps(), split_and_length(), Timeline

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
Cohesion: 0.08
Nodes (29): fixture_pos(), apply(), FixtureAction, AddToGroup, CopyLayout, Delete, InvertOrder, MirrorH (+21 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "Fader"
Cohesion: 0.11
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+6 more)

### Community 45 - "super"
Cohesion: 0.05
Nodes (36): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+28 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 50 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 51 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "ChannelValue"
Cohesion: 0.20
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 68 - "src/timeline.rs"
Cohesion: 0.18
Nodes (9): ClipContent, Audio, Scene, GridMode, Beat, Time, one(), super_scene_from_scenes_keeps_fades() (+1 more)

### Community 69 - "programmer.rs"
Cohesion: 0.32
Nodes (12): fader(), fader_groups(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock, fx_target_id(), kind_color() (+4 more)

### Community 70 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 71 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.24
Nodes (3): Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.13
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "knob.rs"
Cohesion: 0.29
Nodes (3): Knob, SIZE, SWEEP

### Community 78 - "stage_editor.rs"
Cohesion: 0.21
Nodes (8): Arrange, AlignH, AlignV, Grid, Reset, Row, set_pos(), tool_sep()

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.13
Nodes (6): FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, LABEL_W

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - "Layer"
Cohesion: 0.39
Nodes (7): color_bar(), current(), set_group(), xy_pad(), Layer, Live, Programmer

### Community 95 - "limit_pad.rs"
Cohesion: 0.11
Nodes (9): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T> (+1 more)

### Community 96 - "PatchGrid"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.13
Nodes (7): Button, Dialog<'a>, DialogResult, Kind, Cancel, Primary, Secondary

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
Cohesion: 0.26
Nodes (7): fmt_ms(), grid_options(), paint_clip(), row_h(), TimelineView, track_h(), track_header()

### Community 113 - "ViewCtx"
Cohesion: 0.10
Nodes (13): Dialog, text_field(), section_header(), View, ViewCtx, body(), caption(), FixtureSettings (+5 more)

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.11
Nodes (18): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, Drag, FadeIn, FadeOut, Left (+10 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 121 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 123 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (11): SceneProgress, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+3 more)

### Community 128 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 137 - "t"
Cohesion: 0.11
Nodes (33): t(), Show, advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles() (+25 more)

### Community 138 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 141 - "stage_canvas.rs"
Cohesion: 0.14
Nodes (18): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_rect(), live_colors() (+10 more)

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 151 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 153 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "Vec2"
Cohesion: 0.29
Nodes (7): HANDLE, from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 171 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 172 - "egui"
Cohesion: 0.10
Nodes (9): Fullscreen, SETTLE, Step, Enter, Maximize, ICON_PNG, texture(), window_icon() (+1 more)

## Knowledge Gaps
- **314 isolated node(s):** `Move`, `Left`, `Right`, `FadeIn`, `FadeOut` (+309 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 756 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **77 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `AppState`, `runtime.rs`, `theme.rs`, `fx.rs`, `app.rs`, `t`, `stage.rs`, `stage_canvas.rs`, `.check_placement`, `.add_scene`, `Timeline`, `stage.rs`, `DmxCraftApp`, `ChannelValue`, `feature.rs`, `motion.rs`, `show.rs`, `ShowError`, `EffectKind`, `BankScope`, `core/src/lib.rs`?**
  _High betweenness centrality (0.142) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Move`, `Left`, `Right` to the rest of the system?**
  _314 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.053838484546360914 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `hashmap`, `AppState`, `runtime.rs`, `t`, `app.rs`, `t`, `stage.rs`, `Timeline`, `ProgrammerMode`, `audio.rs`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `programmer.rs`, `Show`, `Layer`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.06458635703918723 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `AppState`, `pal`, `app.rs`, `t`, `file_dialog.rs`, `fx_ui.rs`, `stage.rs`, `titlebar.rs`, `super`, `ProgrammerMode`, `stage.rs`, `Fader`, `super`, `DmxCraftApp`, `settings_window.rs`, `color_picker.rs`, `programmer.rs`, `stage_editor.rs`, `dialog.rs`, `Layer`, `PatchGrid`, `Dialog<'a>`, `Category`, `.ui`, `ViewCtx`, `control/timeline.rs`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._