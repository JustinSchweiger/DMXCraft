# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~91,340 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2027 nodes · 4690 edges · 177 communities (106 shown, 71 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 219 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `dcadfdd9`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Engine
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
- stage_editor.rs
- OpenDmx
- show.rs
- dialog.rs
- MenuItem<'a>
- demo.rs
- Toggle<'a>
- fx_color.rs
- FixtureLimits
- T
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- color_picker.rs
- en.rs
- Effect
- .ui
- Scene
- send_frame
- ViewCtx
- mapping.rs
- control/timeline.rs
- DvcGoldUsb
- BankScope
- FixtureType
- show
- CurveShape
- ColorPreset
- output/src/lib.rs
- t
- app.rs
- Lang
- stage_canvas.rs
- knob.rs
- DmxFrame
- resize_handles
- ClipAction
- Theme
- lock.rs
- Tab
- IconButton
- Vec2
- plan.rs
- Transform
- PickTarget
- sense
- text_field
- plan
- Fullscreen
- palettes.rs
- dip.rs
- Grip
- Source
- Drag
- HeaderAction
- Step
- super
- apply

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `t()` - 98 edges
3. `Icon` - 97 edges
4. `Show` - 97 edges
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
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `AppState` --references--> `AudioPlayer`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/audio.rs
- `effect_body()` --calls--> `t()`  [INFERRED]
  crates/app/src/views/control/fx_ui.rs → crates/app/src/i18n/mod.rs

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

## Communities (177 total, 71 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (43): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+35 more)

### Community 1 - "AppState"
Cohesion: 0.16
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 2 - "runtime.rs"
Cohesion: 0.17
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.15
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.11
Nodes (25): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+17 more)

### Community 5 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 6 - "text_dim"
Cohesion: 0.32
Nodes (7): accent(), bg(), border(), caption(), raised(), text_dim(), DipSwitch

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
Nodes (51): AppState, Focus, Bank, Effect, Scenes, Mode, Control, Setup (+43 more)

### Community 11 - "ofl/src/lib.rs"
Cohesion: 0.17
Nodes (9): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+1 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.22
Nodes (14): toggle_row(), BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mapping_ui() (+6 more)

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
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

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

### Community 27 - "qxf.rs"
Cohesion: 0.27
Nodes (7): channel_def(), child(), kind_from_group(), kind_from_preset(), SAMPLE, slug(), text()

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - ".add_scene"
Cohesion: 0.20
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings

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
Nodes (12): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+4 more)

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
Cohesion: 0.20
Nodes (5): SETTLE, drag_number(), endless_delta(), raw_available(), XyPad

### Community 44 - "Fader"
Cohesion: 0.11
Nodes (14): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+6 more)

### Community 45 - "i18n/mod.rs"
Cohesion: 0.16
Nodes (8): all_texts_translated(), EN, fill(), fill_replaces_names(), LANG, tr!, texts_in_source(), ACTIVE

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.19
Nodes (15): full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project(), SCALES (+7 more)

### Community 50 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 51 - "egui"
Cohesion: 0.22
Nodes (3): section_header(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "core/src/lib.rs"
Cohesion: 0.18
Nodes (9): LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super (+1 more)

### Community 68 - "qlc-bundle.rs"
Cohesion: 0.20
Nodes (12): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run() (+4 more)

### Community 69 - "programmer.rs"
Cohesion: 0.08
Nodes (45): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+37 more)

### Community 70 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 71 - "library_panel.rs"
Cohesion: 0.16
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 72 - "Show"
Cohesion: 0.21
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
Cohesion: 0.15
Nodes (12): MenuPlace, Patch, Stage, StageEditor, Arrange, AlignH, AlignV, Grid (+4 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, Form, form(), HEADER_H, INPUT_H (+5 more)

### Community 90 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 93 - "fx_color.rs"
Cohesion: 0.18
Nodes (10): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+2 more)

### Community 96 - "PatchGrid"
Cohesion: 0.19
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.17
Nodes (3): Dialog<'a>, DialogResult, select_all()

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

### Community 106 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 108 - "Effect"
Cohesion: 0.27
Nodes (6): Show, effect_header(), effect_tiles(), fit_mapping(), FxPreview, Effect

### Community 109 - ".ui"
Cohesion: 0.28
Nodes (5): fmt_ms(), grid_options(), paint_clip(), TimelineView, track_header()

### Community 110 - "Scene"
Cohesion: 0.18
Nodes (4): ChannelValue, Scene, Step, steps_insert_remove_and_normalize()

### Community 112 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

### Community 113 - "ViewCtx"
Cohesion: 0.16
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.16
Nodes (11): add_audio(), add_audio_dialog(), DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, RULER_H, super_scene() (+3 more)

### Community 119 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 121 - "BankScope"
Cohesion: 0.19
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 123 - "FixtureType"
Cohesion: 0.27
Nodes (3): ChannelDef, FixtureType, ft()

### Community 128 - "show"
Cohesion: 0.24
Nodes (8): load_meter(), show(), accent_dim(), body(), heading_font(), heading_text(), text(), title()

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
Cohesion: 0.18
Nodes (21): t(), advanced(), banks_of(), bar_slider(), block_props(), FX_EXT, FxDialog, header() (+13 more)

### Community 138 - "app.rs"
Cohesion: 0.23
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 140 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+6 more)

### Community 142 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 151 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 152 - "Tab"
Cohesion: 0.22
Nodes (8): SettingsWindow, sidebar(), Tab, .ALL, General, Midi, Outputs, Project

### Community 153 - "IconButton"
Cohesion: 0.13
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "Vec2"
Cohesion: 0.14
Nodes (8): HANDLE, from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H, Swatch

### Community 155 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 156 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 157 - "PickTarget"
Cohesion: 0.21
Nodes (8): PickTarget, Jump, Protect, Release, Tab, Effects, Properties, Third

### Community 158 - "sense"
Cohesion: 0.33
Nodes (3): raised_hover(), text_faint(), Toggle

### Community 160 - "text_field"
Cohesion: 0.29
Nodes (6): text_field(), CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 161 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 165 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 167 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 168 - "Grip"
Cohesion: 0.25
Nodes (6): Grip, Body, Corner, EdgeX, EdgeY, LimitPad

### Community 169 - "Source"
Cohesion: 0.50
Nodes (3): Source, Ofl, Qlc

### Community 170 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 171 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 172 - "Step"
Cohesion: 0.29
Nodes (5): Step, Enter, Maximize, FxFile, import_fx()

### Community 174 - "apply"
Cohesion: 0.67
Nodes (3): apply(), install_fonts(), surface()

## Knowledge Gaps
- **312 isolated node(s):** `Loading`, `Setup`, `Control`, `Edit`, `Blind` (+307 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 752 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **71 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `AppState`, `runtime.rs`, `theme.rs`, `fx.rs`, `app.rs`, `t`, `stage_canvas.rs`, `.add_scene`, `text_field`, `stage.rs`, `DmxCraftApp`, `core/src/lib.rs`, `feature.rs`, `motion.rs`, `show.rs`, `demo.rs`, `ShowError`, `EffectKind`, `Scene`, `BankScope`, `FixtureType`?**
  _High betweenness centrality (0.136) - this node is a cross-community bridge._
- **Are the 76 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 76 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `Setup`, `Control` to the rest of the system?**
  _312 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.0516190218127625 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `AppState`, `core/src/lib.rs`, `runtime.rs`, `Timeline`, `programmer.rs`, `audio.rs`, `stage.rs`, `t`, `app.rs`, `t`, `library_panel.rs`, `.ui`, `Show`, `DmxCraftApp`, `settings_window.rs`, `ViewCtx`, `DmxFrame`, `control/timeline.rs`?**
  _High betweenness centrality (0.129) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14619883040935672 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `show`, `AppState`, `text_dim`, `app.rs`, `t`, `Lang`, `file_dialog.rs`, `fx_ui.rs`, `super`, `palettes.rs`, `titlebar.rs`, `stage.rs`, `Fader`, `i18n/mod.rs`, `Step`, `DmxCraftApp`, `settings_window.rs`, `egui`, `programmer.rs`, `library_panel.rs`, `stage_editor.rs`, `dialog.rs`, `fx_color.rs`, `PatchGrid`, `Dialog<'a>`, `color_picker.rs`, `Effect`, `.ui`, `ViewCtx`, `control/timeline.rs`?**
  _High betweenness centrality (0.099) - this node is a cross-community bridge._