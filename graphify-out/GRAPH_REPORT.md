# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~94,431 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2054 nodes · 4766 edges · 189 communities (98 shown, 91 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ce44f9d3`
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
- fx_color.rs
- Snapshot
- AppState
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- .add_scene
- env
- dmxcraft
- Mode
- Timeline
- stage.rs
- DmxCraft – UI-Richtlinien
- audio.rs
- FixtureAction
- drag_number
- Fader
- super
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- hashmap
- MenuAction
- PlanShape
- Scene
- DvcGoldUsb
- PaletteStore
- app.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- Tab
- Arrange
- OpenDmx
- show.rs
- dialog.rs
- .group_bar
- plan
- Toggle<'a>
- range_slider.rs
- ChannelKind
- strokekind
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- scene_card.rs
- en.rs
- Tab
- Rect
- Step
- ViewCtx
- mapping.rs
- time_field.rs
- form
- Effect
- core/src/lib.rs
- t
- CurveShape
- ColorPreset
- settings.rs
- t
- demo.rs
- stage_canvas.rs
- ChannelRef
- control/timeline.rs
- Theme
- Category
- fixture_menu.rs
- IconButton
- Step
- Vec2
- PickTarget
- .ui
- ProgrammerMode
- FixtureLimits
- arc
- .fade_state
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
10. `Timeline` - 34 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SetupView` --references--> `LibraryPanel`  [INFERRED]
  crates/app/src/views/setup/mod.rs → crates/app/src/views/setup/library_panel.rs
- `ProgrammerView` --references--> `TimelineView`  [INFERRED]
  crates/app/src/views/control/programmer.rs → crates/app/src/views/control/timeline.rs
- `Effect` --references--> `Timeline`  [INFERRED]
  crates/core/src/show.rs → crates/core/src/timeline.rs

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

## Communities (189 total, 91 thin omitted)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (7): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.16
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (31): accent_dim(), accent_hover(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+23 more)

### Community 5 - "programmer.rs"
Cohesion: 0.18
Nodes (23): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+15 more)

### Community 6 - "pal"
Cohesion: 0.18
Nodes (9): accent(), bg(), border(), caption(), raised(), text_dim(), DipSwitch, Knob<'a> (+1 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.06
Nodes (58): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+50 more)

### Community 11 - "engine.rs"
Cohesion: 0.24
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.23
Nodes (12): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor() (+4 more)

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
Cohesion: 0.25
Nodes (18): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+10 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (75): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+67 more)

### Community 23 - "fx_color.rs"
Cohesion: 0.22
Nodes (11): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), ROW_H, slider_row() (+3 more)

### Community 24 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 25 - "AppState"
Cohesion: 0.12
Nodes (8): AppState, Focus, Bank, Effect, Scenes, Timeline, PatchForm, super_scene()

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.11
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 31 - ".add_scene"
Cohesion: 0.13
Nodes (9): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), move_scenes_between_banks() (+1 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Mode"
Cohesion: 0.16
Nodes (12): Mode, Control, Setup, load_meter(), show(), TitleInfo, UpdateBadge, Available (+4 more)

### Community 36 - "Timeline"
Cohesion: 0.11
Nodes (15): grid_options(), Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat (+7 more)

### Community 37 - "stage.rs"
Cohesion: 0.31
Nodes (6): fx_button(), mapping_handles(), mapping_target(), paint_mapping(), PillResult, StageView

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
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 51 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "Scene"
Cohesion: 0.16
Nodes (5): free_scenes_get_a_bank(), Scene, SceneSettings, Step, steps_insert_remove_and_normalize()

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "PaletteStore"
Cohesion: 0.33
Nodes (7): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore

### Community 70 - "app.rs"
Cohesion: 0.23
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 71 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

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

### Community 77 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

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
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 89 - ".group_bar"
Cohesion: 0.33
Nodes (8): group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select, SetName

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - "range_slider.rs"
Cohesion: 0.36
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (22): FxTarget, toggle_feature(), ChannelDef, ChannelKind, .ALL, Amber, Blue, ColorWheel (+14 more)

### Community 95 - "strokekind"
Cohesion: 0.14
Nodes (8): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented, Segmented<'a, T>

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

### Community 106 - "scene_card.rs"
Cohesion: 0.27
Nodes (5): CARD_H, clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 108 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 109 - "Rect"
Cohesion: 0.25
Nodes (5): paint_fixture(), paint_shape_icon(), fmt_ms(), paint_clip(), TimelineView

### Community 110 - "Step"
Cohesion: 0.20
Nodes (6): SETTLE, Step, Enter, Maximize, FxFile, import_fx()

### Community 113 - "ViewCtx"
Cohesion: 0.16
Nodes (10): apply(), View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView (+2 more)

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 119 - "form"
Cohesion: 0.31
Nodes (3): DialogResult, Form, form()

### Community 121 - "Effect"
Cohesion: 0.29
Nodes (4): Show, effect_header(), FxPreview, Effect

### Community 123 - "core/src/lib.rs"
Cohesion: 0.17
Nodes (9): LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super (+1 more)

### Community 128 - "t"
Cohesion: 0.24
Nodes (4): t(), section_header(), ZonesView, Windows

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "settings.rs"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 137 - "t"
Cohesion: 0.16
Nodes (22): advanced(), banks_of(), bar_slider(), block_props(), effect_tiles(), fit_mapping(), FX_EXT, FxDialog (+14 more)

### Community 140 - "demo.rs"
Cohesion: 0.43
Nodes (5): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (18): BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos(), fixture_rect() (+10 more)

### Community 143 - "ChannelRef"
Cohesion: 0.09
Nodes (4): DmxFrame, lerp(), PlayingScene, ChannelRef

### Community 146 - "control/timeline.rs"
Cohesion: 0.06
Nodes (34): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, ClipAction, Copy, Cut, Delete, Duplicate (+26 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 152 - "fixture_menu.rs"
Cohesion: 0.25
Nodes (5): MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 154 - "Step"
Cohesion: 0.33
Nodes (3): Fullscreen, resize_handles(), mark_clipboard()

### Community 156 - "Vec2"
Cohesion: 0.14
Nodes (7): HANDLE, ICON_FAMILY, ICON_FILL_FAMILY, Knob, SIZE, SWEEP, Swatch

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 158 - ".ui"
Cohesion: 0.21
Nodes (6): body(), heading_font(), heading_text(), text(), title(), Toggle

### Community 170 - "ProgrammerMode"
Cohesion: 0.33
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 174 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 177 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

## Knowledge Gaps
- **319 isolated node(s):** `Auto`, `Color`, `Custom`, `Delete`, `New` (+314 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 777 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **91 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `engine.rs`, `demo.rs`, `stage_canvas.rs`, `ChannelRef`, `Snapshot`, `fixture_menu.rs`, `.add_scene`, `FixtureAction`, `DmxCraftApp`, `Scene`, `app.rs`, `feature.rs`, `motion.rs`, `show.rs`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.122) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Auto`, `Color`, `Custom` to the rest of the system?**
  _319 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.0782051282051282 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `engine.rs`, `ChannelRef`, `control/timeline.rs`, `Snapshot`, `AppState`, `fixture_menu.rs`, `stage.rs`, `audio.rs`, `.fade_state`, `DmxCraftApp`, `settings_window.rs`, `hashmap`, `app.rs`, `Show`, `.group_bar`, `ChannelKind`, `Rect`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.111) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14153846153846153 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `pal`, `settings.rs`, `t`, `t`, `file_dialog.rs`, `fx_ui.rs`, `stage_canvas.rs`, `control/timeline.rs`, `titlebar.rs`, `fx_color.rs`, `fixture_menu.rs`, `Mode`, `Timeline`, `stage.rs`, `FixtureAction`, `ProgrammerMode`, `Fader`, `super`, `DmxCraftApp`, `settings_window.rs`, `PaletteStore`, `app.rs`, `dialog.rs`, `.group_bar`, `PatchGrid`, `Rect`, `Step`, `ViewCtx`, `form`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._