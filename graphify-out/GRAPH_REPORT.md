# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 99 files · ~103,584 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 14 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2194 nodes · 5221 edges · 194 communities (89 shown, 105 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 235 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3788497a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- engine.rs
- AppState
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
- Snapshot
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
- send_frame
- host
- menu.rs
- with
- ensure_linux_menu_entry
- convert.rs
- Scene
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
- Layer
- ClipAction
- PlanShape
- demo.rs
- super
- .paint
- app.rs
- Transform
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- patch_grid.rs
- Host
- OpenDmx
- show.rs
- dialog.rs
- time_field.rs
- plan
- Toggle<'a>
- .fade_state
- ChannelKind
- limit_pad.rs
- call
- Dialog<'a>
- MapPreset
- ShowError
- EffectKind
- en.rs
- qlc-bundle.rs
- Rect
- ViewCtx
- mapping.rs
- i18n/mod.rs
- core/src/lib.rs
- ChannelValue
- CurveShape
- ColorPreset
- t
- server.rs
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Drag
- IconButton
- swatch.rs
- Vec2
- plan.rs
- FixtureLimits
- tools.rs
- icons.rs
- Settings
- lock.rs
- range_slider.rs
- dip.rs

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
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `SettingsCx` --references--> `McpStatus`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/mcp.rs
- `ai()` --calls--> `t()`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/i18n/mod.rs
- `ai()` --references--> `McpStatus`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/mcp.rs

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

## Communities (194 total, 105 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 1 - "AppState"
Cohesion: 0.05
Nodes (31): AppState, Focus, Bank, Effect, Scenes, Timeline, LibraryDrag, Mode (+23 more)

### Community 2 - "runtime.rs"
Cohesion: 0.19
Nodes (6): engine_keeps_running_without_device(), Failing, RECONNECT_DELAY, record_load(), Runtime, runtime_sends_rendered_frames()

### Community 3 - "open_dmx.rs"
Cohesion: 0.15
Nodes (11): BREAK, DMX_BAUD, FRAME_TIME, FTDI_VENDOR_ID, list_ports(), MAB, resolve(), RETRY (+3 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (27): accent_hover(), body(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+19 more)

### Community 5 - "programmer.rs"
Cohesion: 0.12
Nodes (33): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, category_list() (+25 more)

### Community 6 - "pal"
Cohesion: 0.25
Nodes (15): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+7 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.09
Nodes (10): NullOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputError, NotFound (+2 more)

### Community 10 - "t"
Cohesion: 0.09
Nodes (47): select_all(), programmer_values(), Action, Bank, CreateSuper, Delete, DuplicateScene, Flash (+39 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 12 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

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
Cohesion: 0.11
Nodes (16): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, Aufbau, Bauen und starten (+8 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.05
Nodes (53): Fullscreen, SETTLE, Step, Enter, Maximize, clip(), delete(), HEIGHT (+45 more)

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
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize(), Zone

### Community 24 - "ofl/src/lib.rs"
Cohesion: 0.13
Nodes (13): OflError, Invalid, Json, Skip, embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC (+5 more)

### Community 25 - "send_frame"
Cohesion: 0.17
Nodes (7): Box<dyn serialport::SerialPort>, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), packet(), Recorder, send_frame()

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

### Community 31 - "Scene"
Cohesion: 0.23
Nodes (4): LoopMode, Loop, Once, Scene

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "audio.rs"
Cohesion: 0.06
Nodes (24): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+16 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.13
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, fine_mark(), paint_symbol()

### Community 45 - "qxf.rs"
Cohesion: 0.23
Nodes (10): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), rejects_files_without_modes(), SAMPLE (+2 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.08
Nodes (14): DmxCraftApp, LockDialog, Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT (+6 more)

### Community 48 - "settings_window.rs"
Cohesion: 0.11
Nodes (27): Settings, ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs() (+19 more)

### Community 50 - "Layer"
Cohesion: 0.11
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 66 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 70 - "app.rs"
Cohesion: 0.15
Nodes (11): ControlViews, edit_preview(), load_step(), OUTPUT_HZ, CanvasMode, Edit, Select, StageInteraction (+3 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.19
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.09
Nodes (23): FxPreview, apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+15 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.11
Nodes (11): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation, OutputStatus, Connecting (+3 more)

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "Host"
Cohesion: 0.16
Nodes (17): AppEvent, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host, call_tool() (+9 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.19
Nodes (13): ChannelDef, address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range() (+5 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (13): Button, Dialog, DialogResult, FOOTER_H, Form, HEADER_H, INPUT_H, Kind (+5 more)

### Community 89 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 93 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 94 - "ChannelKind"
Cohesion: 0.09
Nodes (18): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+10 more)

### Community 95 - "limit_pad.rs"
Cohesion: 0.17
Nodes (7): Grip, Body, Corner, EdgeX, EdgeY, LimitPad, Segmented<'a, T>

### Community 96 - "call"
Cohesion: 0.32
Nodes (20): add_clips(), add_effect(), add_fixtures(), apply_scene_fields(), call(), create_scene(), create_super_scene(), existing_fixtures() (+12 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 104 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 108 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 109 - "Rect"
Cohesion: 0.30
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 113 - "ViewCtx"
Cohesion: 0.08
Nodes (20): section_header(), apply(), View, ViewCtx, body(), caption(), FixtureSettings, range_row() (+12 more)

### Community 117 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 118 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 123 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (15): SceneProgress, BankScope, All, Bank, Banks, Off, OutsideBank, Group (+7 more)

### Community 128 - "ChannelValue"
Cohesion: 0.31
Nodes (6): ChannelValue, Step, build_steps(), channel_kind(), collect_values(), resolve_channel()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 137 - "t"
Cohesion: 0.11
Nodes (34): t(), advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping() (+26 more)

### Community 140 - "server.rs"
Cohesion: 0.13
Nodes (13): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), header() (+5 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (15): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+7 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.10
Nodes (22): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, grid_options(), HEADER_W, HeaderAction (+14 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 152 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 158 - "Vec2"
Cohesion: 0.09
Nodes (15): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, SIZE (+7 more)

### Community 170 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (31): any_obj(), boolean(), clip_schema(), dmx_output(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+23 more)

### Community 195 - "icons.rs"
Cohesion: 0.17
Nodes (5): FooterButton, ICON_FAMILY, ICON_FILL_FAMILY, SceneCardResponse, PillResult

### Community 196 - "Settings"
Cohesion: 0.22
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 197 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 198 - "range_slider.rs"
Cohesion: 0.36
Nodes (6): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H

### Community 199 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

## Knowledge Gaps
- **327 isolated node(s):** `General`, `Outputs`, `Midi`, `Ai`, `Project` (+322 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 806 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **105 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `engine.rs`, `AppState`, `runtime.rs`, `programmer.rs`, `t`, `t`, `Snapshot`, `control/timeline.rs`, `audio.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Layer`, `app.rs`, `Show`, `Host`, `.fade_state`, `call`, `Rect`, `ViewCtx`, `core/src/lib.rs`?**
  _High betweenness centrality (0.149) - this node is a cross-community bridge._
- **Are the 79 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 79 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Outputs`, `Midi` to the rest of the system?**
  _327 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `AppState` be split into smaller, more focused modules?**
  _Cohesion score 0.05217391304347826 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `engine.rs`, `AppState`, `ChannelValue`, `runtime.rs`, `theme.rs`, `fx.rs`, `t`, `Engine`, `Snapshot`, `stage_canvas.rs`, `titlebar.rs`, `Scene`, `Scene`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `app.rs`, `feature.rs`, `motion.rs`, `show.rs`, `call`, `ShowError`, `EffectKind`, `core/src/lib.rs`?**
  _High betweenness centrality (0.131) - this node is a cross-community bridge._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14619883040935672 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `AppState`, `programmer.rs`, `pal`, `t`, `fx_ui.rs`, `control/timeline.rs`, `titlebar.rs`, `Vec2`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `app.rs`, `patch_grid.rs`, `dialog.rs`, `Rect`, `ViewCtx`, `i18n/mod.rs`?**
  _High betweenness centrality (0.077) - this node is a cross-community bridge._