# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 91 files · ~94,189 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2052 nodes · 4757 edges · 175 communities (91 shown, 84 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 226 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e27354d1`
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
- GridMode
- MenuAction
- PlanShape
- ChannelValue
- DvcGoldUsb
- segmented.rs
- app.rs
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
- FixtureType
- FixtureLimits
- Stroke
- PatchGrid
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- MoveShape
- en.rs
- Tab
- .ui
- Step
- ViewCtx
- mapping.rs
- control/timeline.rs
- Ui
- Effect
- core/src/lib.rs
- windows.rs
- CurveShape
- ColorPreset
- ChaserPreset
- t
- demo.rs
- stage_canvas.rs
- DmxFrame
- ClipAction
- Theme
- UpdateBadge
- CurveFx
- IconButton
- Fullscreen
- Drag
- PickTarget
- sense
- HeaderAction
- arc

## God Nodes (most connected - your core abstractions)
1. `Engine` - 139 edges
2. `t()` - 101 edges
3. `Show` - 100 edges
4. `Icon` - 98 edges
5. `ChannelKind` - 52 edges
6. `AppState` - 48 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 38 edges
10. `ViewCtx` - 34 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `AppState` --references--> `AudioPlayer`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/audio.rs
- `show()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `load_meter()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `main_menu()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs

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

## Communities (175 total, 84 thin omitted)

### Community 0 - "Engine"
Cohesion: 0.05
Nodes (44): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+36 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (6): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag()

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (24): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+16 more)

### Community 5 - "programmer.rs"
Cohesion: 0.05
Nodes (63): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+55 more)

### Community 6 - "pal"
Cohesion: 0.25
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

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
Nodes (59): ControlViews, color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS (+51 more)

### Community 11 - "stage.rs"
Cohesion: 0.20
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 13 - "fx_ui.rs"
Cohesion: 0.08
Nodes (31): builtin(), load_user(), named(), NamedPalette, path(), save_user(), Swatch, color_fx_ui() (+23 more)

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
Cohesion: 0.20
Nodes (21): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+13 more)

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
Cohesion: 0.13
Nodes (14): heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 31 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize(), Zone

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
Cohesion: 0.14
Nodes (10): Clip, ClipContent, Audio, Scene, fades(), paste_finds_free_place_and_keeps_gaps(), split_and_length(), super_scene_from_scenes_keeps_fades() (+2 more)

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
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag_number"
Cohesion: 0.21
Nodes (5): SETTLE, drag_number(), endless_delta(), raw_available(), section_header()

### Community 44 - "Fader"
Cohesion: 0.06
Nodes (26): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+18 more)

### Community 45 - "super"
Cohesion: 0.08
Nodes (32): digest(), hash_and_verify(), hash_password(), hex(), verify(), COMPACT_W, FINE_STEP_MS, format() (+24 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.07
Nodes (28): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings, full_combo(), general(), HEIGHT, hint() (+20 more)

### Community 50 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 51 - "MenuAction"
Cohesion: 0.11
Nodes (18): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+10 more)

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "ChannelValue"
Cohesion: 0.20
Nodes (3): ChannelValue, Scene, Step

### Community 68 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 69 - "segmented.rs"
Cohesion: 0.25
Nodes (3): body(), Segmented, Segmented<'a, T>

### Community 70 - "app.rs"
Cohesion: 0.24
Nodes (7): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, Mode, Control, Setup

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.19
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

### Community 77 - "knob.rs"
Cohesion: 0.20
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 78 - "stage_editor.rs"
Cohesion: 0.16
Nodes (12): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+4 more)

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

### Community 95 - "Stroke"
Cohesion: 0.15
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, XyPad

### Community 96 - "PatchGrid"
Cohesion: 0.17
Nodes (13): t(), apply(), CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+5 more)

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

### Community 106 - "MoveShape"
Cohesion: 0.22
Nodes (7): MoveShape, .ALL, Circle, Curve, Line, Points, Polygon

### Community 108 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 109 - ".ui"
Cohesion: 0.26
Nodes (7): fmt_ms(), grid_options(), paint_clip(), row_h(), TimelineView, track_h(), track_header()

### Community 110 - "Step"
Cohesion: 0.29
Nodes (5): Step, Enter, Maximize, FxFile, import_fx()

### Community 113 - "ViewCtx"
Cohesion: 0.19
Nodes (8): View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "control/timeline.rs"
Cohesion: 0.14
Nodes (13): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, mark_clipboard() (+5 more)

### Community 121 - "Effect"
Cohesion: 0.25
Nodes (4): Show, effect_header(), FxPreview, Effect

### Community 123 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (16): BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode, Loop (+8 more)

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
Cohesion: 0.17
Nodes (22): advanced(), banks_of(), bar_slider(), block_props(), effect_tiles(), fit_mapping(), FX_EXT, FxDialog (+14 more)

### Community 140 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+6 more)

### Community 146 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 152 - "CurveFx"
Cohesion: 0.38
Nodes (4): curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value()

### Community 153 - "IconButton"
Cohesion: 0.11
Nodes (4): FooterButton, ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 156 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 171 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

## Knowledge Gaps
- **319 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+314 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 766 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **84 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Engine`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `demo.rs`, `stage_canvas.rs`, `.add_scene`, `stage.rs`, `DmxCraftApp`, `ChannelValue`, `app.rs`, `feature.rs`, `motion.rs`, `stage_editor.rs`, `show.rs`, `FixtureType`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.145) - this node is a cross-community bridge._
- **Are the 78 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 78 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _319 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Engine` be split into smaller, more focused modules?**
  _Cohesion score 0.05121257282211082 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `stage.rs`, `DmxFrame`, `Timeline`, `audio.rs`, `stage.rs`, `DmxCraftApp`, `settings_window.rs`, `ChannelValue`, `app.rs`, `Show`, `PatchGrid`, `.ui`, `ViewCtx`, `control/timeline.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.114) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10804597701149425 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `PatchGrid` to `windows.rs`, `library_panel.rs`, `programmer.rs`, `pal`, `t`, `t`, `stage.rs`, `file_dialog.rs`, `fx_ui.rs`, `titlebar.rs`, `i18n/mod.rs`, `stage.rs`, `Fader`, `DmxCraftApp`, `settings_window.rs`, `app.rs`, `stage_editor.rs`, `dialog.rs`, `Dialog<'a>`, `.ui`, `Step`, `ViewCtx`, `control/timeline.rs`, `core/src/lib.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._