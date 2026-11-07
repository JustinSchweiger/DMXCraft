# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 99 files · ~103,574 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2194 nodes · 5221 edges · 212 communities (110 shown, 102 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 235 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a7855bd7`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- stage.rs
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
- Scene
- ofl/src/lib.rs
- AppState
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- Action
- env
- dmxcraft
- knob.rs
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- audio.rs
- FixtureAction
- drag_number
- fader.rs
- qxf.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- hashmap
- t
- PlanShape
- BankScope
- Timeline
- .paint
- app.rs
- Transform
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- Host
- OpenDmx
- show.rs
- dialog.rs
- time_field.rs
- plan
- Toggle<'a>
- fx_color.rs
- ChannelKind
- T
- call
- Dialog<'a>
- MapPreset
- ShowError
- UpdateState
- EffectKind
- Grip
- en.rs
- qlc-bundle.rs
- Rect
- mcp.rs
- ViewCtx
- mapping.rs
- i18n/mod.rs
- Arrange
- core/src/lib.rs
- ChannelRef
- CurveShape
- ColorPreset
- palettes.rs
- t
- server.rs
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Category
- Step
- Drag
- IconButton
- swatch.rs
- PickTarget
- egui
- plan.rs
- FixtureLimits
- arc
- Tab
- tools.rs
- DvcGoldUsb
- bridge.rs
- Tab
- scene_card.rs
- Settings
- lock.rs
- range_slider.rs
- dip.rs
- widgets/mod.rs
- fixture_menu.rs
- ProgrammerMode
- ChaserPreset
- OutputStatus
- Context
- ok

## God Nodes (most connected - your core abstractions)
1. `Engine` - 160 edges
2. `Show` - 110 edges
3. `t()` - 102 edges
4. `Icon` - 99 edges
5. `ChannelKind` - 54 edges
6. `AppState` - 48 edges
7. `ChannelRef` - 41 edges
8. `DmxFrame` - 40 edges
9. `DmxCraftApp` - 39 edges
10. `Timeline` - 36 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs
- `DmxCraftApp` --references--> `McpLink`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/mcp.rs
- `SettingsCx` --references--> `McpStatus`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/mcp.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (212 total, 102 thin omitted)

### Community 0 - "stage.rs"
Cohesion: 0.20
Nodes (15): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+7 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (7): LibraryDrag, drag_ghost(), LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch(), source_tag(), FixtureType

### Community 2 - "runtime.rs"
Cohesion: 0.16
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (24): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+16 more)

### Community 5 - "programmer.rs"
Cohesion: 0.19
Nodes (21): category_list(), color_bar(), current(), elide(), fader(), fader_groups(), FaderGroup, fixture_block() (+13 more)

### Community 6 - "pal"
Cohesion: 0.21
Nodes (14): accent(), accent_dim(), apply(), bg(), border(), install_fonts(), pal(), raised() (+6 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "t"
Cohesion: 0.12
Nodes (27): CARD_H, programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow() (+19 more)

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (46): cv(), demo_show(), empty_show(), ensure_builtin_types(), History, LIMIT, live_values_are_undoable(), Snapshot (+38 more)

### Community 12 - "file_dialog.rs"
Cohesion: 0.24
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
Cohesion: 0.11
Nodes (16): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, Aufbau, Bauen und starten (+8 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.07
Nodes (48): Mode, Control, Setup, clip(), delete(), HEIGHT, k(), levels() (+40 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (76): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+68 more)

### Community 23 - "Scene"
Cohesion: 0.13
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.14
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 25 - "AppState"
Cohesion: 0.14
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

### Community 31 - "Action"
Cohesion: 0.16
Nodes (23): edit_preview(), load_step(), Action, CreateSuper, Delete, DuplicateScene, Flash, HideScenes (+15 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.16
Nodes (11): ClipContent, Audio, Scene, fades(), GridMode, Beat, Time, one() (+3 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.15
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "qxf.rs"
Cohesion: 0.17
Nodes (12): ChannelDef, ft(), channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset() (+4 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.20
Nodes (17): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+9 more)

### Community 50 - "hashmap"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 51 - "t"
Cohesion: 0.24
Nodes (3): t(), ZonesView, Windows

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 68 - "Timeline"
Cohesion: 0.21
Nodes (5): add_audio(), add_audio_dialog(), Clip, Timeline, Track

### Community 69 - ".paint"
Cohesion: 0.14
Nodes (5): body(), caption(), heading_font(), heading_text(), MenuItem<'a>

### Community 70 - "app.rs"
Cohesion: 0.31
Nodes (5): ControlViews, LockDialog, OUTPUT_HZ, ProgrammerView, SceneGridView

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.17
Nodes (6): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 73 - "feature.rs"
Cohesion: 0.16
Nodes (14): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+6 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 78 - "Host"
Cohesion: 0.16
Nodes (17): AppEvent, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host, call_tool() (+9 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - "fx_color.rs"
Cohesion: 0.18
Nodes (10): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+2 more)

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (20): FxTarget, toggle_feature(), ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer (+12 more)

### Community 96 - "call"
Cohesion: 0.26
Nodes (23): add_clips(), add_effect(), add_fixtures(), call(), create_scene(), create_super_scene(), dmx_output(), existing_fixtures() (+15 more)

### Community 97 - "Dialog<'a>"
Cohesion: 0.14
Nodes (3): Dialog<'a>, DialogResult, Form

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

### Community 108 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 109 - "Rect"
Cohesion: 0.25
Nodes (5): paint_fixture(), paint_shape_icon(), fmt_ms(), paint_clip(), TimelineView

### Community 110 - "mcp.rs"
Cohesion: 0.20
Nodes (5): McpLink, McpStatus, Error, Off, Running

### Community 113 - "ViewCtx"
Cohesion: 0.15
Nodes (11): apply(), View, ViewCtx, body(), caption(), FixtureSettings, range_row(), SetupView (+3 more)

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 121 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 123 - "core/src/lib.rs"
Cohesion: 0.14
Nodes (11): SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 128 - "ChannelRef"
Cohesion: 0.15
Nodes (4): Layer, Live, Programmer, ChannelRef

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "palettes.rs"
Cohesion: 0.38
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 137 - "t"
Cohesion: 0.14
Nodes (26): Show, advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping() (+18 more)

### Community 140 - "server.rs"
Cohesion: 0.24
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 141 - "stage_canvas.rs"
Cohesion: 0.20
Nodes (14): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, StageCanvas (+6 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.08
Nodes (25): AUDIO_TRACK_H, ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed (+17 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 150 - "Step"
Cohesion: 0.15
Nodes (7): Fullscreen, SETTLE, Step, Enter, Maximize, FxFile, import_fx()

### Community 152 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 157 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 158 - "egui"
Cohesion: 0.12
Nodes (14): ICON_FAMILY, ICON_FILL_FAMILY, SIZE, SWEEP, LimitPad, HANDLE, section_header(), Segmented (+6 more)

### Community 170 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 172 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 174 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 176 - "tools.rs"
Cohesion: 0.11
Nodes (30): ChannelValue, Step, any_obj(), apply_scene_fields(), boolean(), build_steps(), channel_kind(), clip_schema() (+22 more)

### Community 177 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 183 - "bridge.rs"
Cohesion: 0.22
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 194 - "Tab"
Cohesion: 0.22
Nodes (9): SettingsWindow, sidebar(), Tab, Ai, .ALL, General, Midi, Outputs (+1 more)

### Community 195 - "scene_card.rs"
Cohesion: 0.29
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 196 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 197 - "lock.rs"
Cohesion: 0.27
Nodes (6): digest(), hash_and_verify(), hash_password(), hex(), verify(), UNIVERSE_SIZE

### Community 198 - "range_slider.rs"
Cohesion: 0.43
Nodes (5): from_pct(), pct(), RangeSlider, value_field(), VALUE_H

### Community 199 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 200 - "widgets/mod.rs"
Cohesion: 0.67
Nodes (3): CanvasMode, Edit, Select

### Community 201 - "fixture_menu.rs"
Cohesion: 0.22
Nodes (5): MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 203 - "ProgrammerMode"
Cohesion: 0.29
Nodes (6): ProgrammerMode, Blind, Edit, Live, items_mode(), mode_switch()

### Community 204 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 205 - "OutputStatus"
Cohesion: 0.29
Nodes (5): OutputStatus, Connecting, Error, Running, Stopped

### Community 206 - "Context"
Cohesion: 0.50
Nodes (3): resize_handles(), drag_ghost(), mark_clipboard()

### Community 207 - "ok"
Cohesion: 0.67
Nodes (4): full_workflow(), helpful_errors(), host(), ok()

## Knowledge Gaps
- **327 isolated node(s):** `General`, `Outputs`, `Midi`, `Ai`, `Project` (+322 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 805 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **102 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Engine` to `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `stage_canvas.rs`, `Scene`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `BankScope`, `app.rs`, `Show`, `fixture_menu.rs`, `feature.rs`, `motion.rs`, `Context`, `show.rs`, `call`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.167) - this node is a cross-community bridge._
- **Are the 79 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 79 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Outputs`, `Midi` to the rest of the system?**
  _327 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14153846153846153 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `stage.rs`, `ChannelRef`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `t`, `t`, `DmxFrame`, `control/timeline.rs`, `Action`, `audio.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `hashmap`, `app.rs`, `fixture_menu.rs`, `Host`, `ChannelKind`, `call`, `Rect`, `mcp.rs`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.156) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0946969696969697 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `stage.rs`, `library_panel.rs`, `programmer.rs`, `pal`, `palettes.rs`, `t`, `t`, `file_dialog.rs`, `fx_ui.rs`, `control/timeline.rs`, `titlebar.rs`, `Step`, `egui`, `Action`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `Settings`, `Timeline`, `app.rs`, `fixture_menu.rs`, `ProgrammerMode`, `patch_grid.rs`, `dialog.rs`, `fx_color.rs`, `Dialog<'a>`, `Rect`, `ViewCtx`, `i18n/mod.rs`?**
  _High betweenness centrality (0.074) - this node is a cross-community bridge._