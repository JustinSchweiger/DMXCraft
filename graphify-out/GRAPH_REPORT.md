# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~95,037 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2058 nodes · 4774 edges · 182 communities (84 shown, 98 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a150240e`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- fx_ui.rs
- color_picker.rs
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
- fx_color.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Tab
- form
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
- i18n/mod.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- Layer
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
- ClipAction
- ChaserPreset
- OpenDmx
- show.rs
- dialog.rs
- palettes.rs
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
- Rect
- ViewCtx
- mapping.rs
- super
- Arrange
- core/src/lib.rs
- CurveShape
- ColorPreset
- Settings
- t
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Category
- IconButton
- swatch.rs
- PickTarget
- egui
- ProgrammerMode
- FixtureLimits
- arc
- Box<dyn serialport::SerialPort>

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
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `general()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs
- `general()` --calls--> `form()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/ui/widgets/dialog.rs

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

## Communities (182 total, 98 thin omitted)

### Community 0 - "fx_ui.rs"
Cohesion: 0.20
Nodes (14): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor() (+6 more)

### Community 1 - "color_picker.rs"
Cohesion: 0.13
Nodes (14): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+6 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (30): accent_dim(), accent_hover(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+22 more)

### Community 5 - "programmer.rs"
Cohesion: 0.19
Nodes (22): XyPad, category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup (+14 more)

### Community 6 - "pal"
Cohesion: 0.26
Nodes (7): accent(), bg(), border(), caption(), raised(), text_dim(), DipSwitch

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), fbm(), feature_levels(), grayscale_has_equal_channels() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "t"
Cohesion: 0.09
Nodes (48): CARD_H, programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash (+40 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (42): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+34 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_color.rs"
Cohesion: 0.18
Nodes (12): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+4 more)

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

### Community 23 - "Tab"
Cohesion: 0.25
Nodes (7): sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 24 - "form"
Cohesion: 0.27
Nodes (3): DialogResult, Form, form()

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
Cohesion: 0.05
Nodes (52): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+44 more)

### Community 31 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.83
Nodes (4): dmxcraft, dmxcraft-core, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.11
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "FixtureType"
Cohesion: 0.16
Nodes (8): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelDef, FixtureType, builtin_fixture_types(), ft()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.07
Nodes (16): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+8 more)

### Community 40 - "FixtureAction"
Cohesion: 0.06
Nodes (46): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+38 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.18
Nodes (16): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+8 more)

### Community 50 - "Layer"
Cohesion: 0.15
Nodes (8): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On

### Community 51 - "t"
Cohesion: 0.18
Nodes (4): t(), section_header(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.22
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "Scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize(), Zone

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - ".paint"
Cohesion: 0.14
Nodes (5): body(), heading_font(), heading_text(), text(), MenuItem<'a>

### Community 70 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.21
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.18
Nodes (13): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), custom_value(), fill_ends_full() (+5 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 78 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (20): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+12 more)

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

### Community 106 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 109 - "Rect"
Cohesion: 0.29
Nodes (5): clip(), fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 113 - "ViewCtx"
Cohesion: 0.11
Nodes (16): apply(), View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView (+8 more)

### Community 117 - "mapping.rs"
Cohesion: 0.21
Nodes (9): every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level(), transform() (+1 more)

### Community 118 - "super"
Cohesion: 0.08
Nodes (32): digest(), hash_and_verify(), hash_password(), hex(), verify(), COMPACT_W, FINE_STEP_MS, format() (+24 more)

### Community 121 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 123 - "core/src/lib.rs"
Cohesion: 0.11
Nodes (14): UNIVERSE_SIZE, ChannelValue, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene (+6 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.16
Nodes (11): ColorFx, ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow (+3 more)

### Community 136 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 137 - "t"
Cohesion: 0.11
Nodes (29): advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping(), FX_EXT (+21 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.12
Nodes (21): SceneCard, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+13 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.08
Nodes (27): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, Drag, FadeIn, FadeOut, Left (+19 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 158 - "egui"
Cohesion: 0.12
Nodes (17): ICON_FAMILY, ICON_FILL_FAMILY, SIZE, SWEEP, LimitPad, from_pct(), HANDLE, pct() (+9 more)

### Community 170 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

## Knowledge Gaps
- **320 isolated node(s):** `General`, `Outputs`, `Midi`, `Project`, `Setup` (+315 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 780 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **98 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Scene`, `runtime.rs`, `theme.rs`, `FixtureType`, `app.rs`, `ShowError`, `FixtureAction`, `fx.rs`, `t`, `Engine`, `feature.rs`, `stage_canvas.rs`, `motion.rs`, `DmxCraftApp`, `EffectKind`, `show.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.143) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Outputs`, `Midi` to the rest of the system?**
  _320 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `color_picker.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12987012987012986 - nodes in this community are weakly interconnected._
- **Why does `Icon` connect `Icon` to `color_picker.rs`, `.paint`, `programmer.rs`, `FixtureAction`, `t`, `t`, `.paint`, `stage_canvas.rs`, `settings_window.rs`, `ViewCtx`, `control/timeline.rs`, `titlebar.rs`, `Tab`, `dialog.rs`, `IconButton`, `menu.rs`, `egui`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.12615384615384614 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `runtime.rs`, `Timeline`, `programmer.rs`, `app.rs`, `audio.rs`, `FixtureAction`, `t`, `t`, `Show`, `Rect`, `DmxCraftApp`, `settings_window.rs`, `ViewCtx`, `control/timeline.rs`, `Layer`, `DmxFrame`, `core/src/lib.rs`, `ChannelKind`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._