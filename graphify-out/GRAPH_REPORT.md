# Graph Report - dmxcraft-src  (2026-11-07)

## Corpus Check
- 110 files · ~128,241 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2683 nodes · 6412 edges · 232 communities (116 shown, 116 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 304 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c57611ed`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- venue.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- pal
- steps
- fx.rs
- OutputError
- scene_grid.rs
- Engine
- i18n/mod.rs
- fx_ui.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- engine.rs
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- audio.rs
- env
- dmxcraft
- knob.rs
- Timeline
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- Layer
- ClipAction
- FixtureType
- demo.rs
- ViewCtx
- .paint
- stage.rs
- Transform
- Show
- feature.rs
- output/src/lib.rs
- motion.rs
- patch_grid.rs
- fx_color.rs
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- plan
- Toggle<'a>
- t
- viz/mod.rs
- ChannelKind
- convert.rs
- CapabilityKind
- Dialog<'a>
- MapPreset
- AppState
- qxf.rs
- en.rs
- Profile
- Rect
- View
- .ui
- Action
- mcp.rs
- EffectKind
- convert.rs
- Step
- Category
- CurveShape
- ColorPreset
- Tab
- stage_editor.rs
- bank_menu
- stage_canvas.rs
- DmxFrame
- control/timeline.rs
- Theme
- Snapshot
- server.rs
- bridge.rs
- IconButton
- palettes.rs
- ShutterEffect
- Quality
- egui
- Scene
- Host
- MenuAction
- Problem
- protocol.rs
- OrbitCamera
- plan.rs
- .show
- tools.rs
- lock.rs
- Tab
- qlc-bundle.rs
- time_field.rs
- UpdateBadge
- app.rs
- ShowError
- dip.rs
- arc
- Settings
- Effect
- Drag
- .ui
- Ui
- DialogKind
- .fade_state
- Mode
- GridMode

## God Nodes (most connected - your core abstractions)
1. `Engine` - 165 edges
2. `t()` - 137 edges
3. `Show` - 116 edges
4. `Icon` - 103 edges
5. `ChannelKind` - 61 edges
6. `AppState` - 55 edges
7. `ViewCtx` - 43 edges
8. `Profile` - 42 edges
9. `ChannelRef` - 42 edges
10. `DmxFrame` - 42 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `AppState` --references--> `Quality`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `AppState` --references--> `Fixture`  [INFERRED]
  crates/app/src/state.rs → crates/app/src/viz/mod.rs
- `DmxCraftApp` --references--> `VizView`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/viz/mod.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`

## Communities (232 total, 116 thin omitted)

### Community 0 - "venue.rs"
Cohesion: 0.06
Nodes (36): renumber(), apply(), CameraPreset, close(), Environment, argb(), DEFAULT_TRUSS_LEN, Ev2Fixture (+28 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (15): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (30): accent_dim(), accent_hover(), apply(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY (+22 more)

### Community 5 - "programmer.rs"
Cohesion: 0.13
Nodes (28): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+20 more)

### Community 6 - "pal"
Cohesion: 0.26
Nodes (7): accent(), bg(), border(), caption(), raised(), text_dim(), text_faint()

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, fbm(), feature_levels() (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig (+4 more)

### Community 10 - "scene_grid.rs"
Cohesion: 0.19
Nodes (18): CARD_H, programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow() (+10 more)

### Community 11 - "Engine"
Cohesion: 0.08
Nodes (4): Engine, lerp(), PlayingScene, SceneProgress

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "fx_ui.rs"
Cohesion: 0.23
Nodes (11): BodyCtx, combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor() (+3 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "DmxCraft"
Cohesion: 0.08
Nodes (22): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+14 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.17
Nodes (22): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+14 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - "engine.rs"
Cohesion: 0.24
Nodes (29): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+21 more)

### Community 24 - "rig.rs"
Cohesion: 0.11
Nodes (20): Beam, build(), cell_light(), cube(), Kind, kind(), Bar, Blinder (+12 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (44): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), Step (+36 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.17
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.15
Nodes (11): embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL, EMBEDDED_QLC, fill_physical(), Library, LibraryFixture, Source (+3 more)

### Community 31 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 36 - "Timeline"
Cohesion: 0.13
Nodes (12): grid_options(), Clip, ClipContent, Audio, Scene, fades(), one(), paste_finds_free_place_and_keeps_gaps() (+4 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.14
Nodes (14): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+6 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 44 - "fader.rs"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+7 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (38): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+30 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.20
Nodes (19): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+11 more)

### Community 50 - "Layer"
Cohesion: 0.11
Nodes (9): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On (+1 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "FixtureType"
Cohesion: 0.08
Nodes (19): UNIVERSE_SIZE, ChannelDef, FixtureType, Physics, builtin_fixture_types(), ft(), PLAN_SIZE_RANGE, PlanShape (+11 more)

### Community 66 - "demo.rs"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 68 - "ViewCtx"
Cohesion: 0.33
Nodes (4): ViewCtx, fixture_props(), vec3_row(), VizView

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 72 - "Show"
Cohesion: 0.18
Nodes (8): Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "output/src/lib.rs"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "fx_color.rs"
Cohesion: 0.20
Nodes (12): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+4 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.18
Nodes (13): address_map_marks_ranges(), FixtureLimits, groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range() (+5 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (21): cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+13 more)

### Community 90 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 92 - "t"
Cohesion: 0.10
Nodes (40): t(), text_field(), capability_card(), capability_fields(), category_label(), check(), color_label(), full_combo() (+32 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.21
Nodes (12): hit_y(), object_tree(), Pick, Fixture, Object, ray_aabb_exit(), ray_box(), ray_sphere() (+4 more)

### Community 94 - "ChannelKind"
Cohesion: 0.08
Nodes (19): ChannelKind, .ALL, Amber, Blue, ColorWheel, Dimmer, Generic, Gobo (+11 more)

### Community 95 - "convert.rs"
Cohesion: 0.20
Nodes (15): convert_fixture(), kinds(), matrix_pixel_count_and_order(), matrix_pixel_keys_abc_and_groups(), moving_head_fine_channels_and_wheels(), OflError, Invalid, Json (+7 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.07
Nodes (31): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+23 more)

### Community 100 - "AppState"
Cohesion: 0.14
Nodes (12): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+4 more)

### Community 104 - "qxf.rs"
Cohesion: 0.21
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "Profile"
Cohesion: 0.11
Nodes (13): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+5 more)

### Community 109 - "Rect"
Cohesion: 0.29
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "View"
Cohesion: 0.20
Nodes (7): View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView

### Community 117 - "Action"
Cohesion: 0.18
Nodes (18): Action, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump, Mark (+10 more)

### Community 118 - "mcp.rs"
Cohesion: 0.20
Nodes (5): McpLink, McpStatus, Error, Off, Running

### Community 119 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.17
Nodes (12): capabilities(), channel_kind(), default_value(), fine_aliases(), Lookup, Lookup<'a>, Matrix, natural_cmp() (+4 more)

### Community 128 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 137 - "stage_editor.rs"
Cohesion: 0.16
Nodes (11): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 140 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (16): BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_fixture() (+8 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (21): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete (+13 more)

### Community 147 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 149 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 150 - "server.rs"
Cohesion: 0.24
Nodes (6): header(), http_roundtrip(), McpServer, origin_ok(), post(), serve()

### Community 152 - "bridge.rs"
Cohesion: 0.24
Nodes (7): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run()

### Community 153 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - "Quality"
Cohesion: 0.20
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 158 - "egui"
Cohesion: 0.08
Nodes (21): DipSwitch, SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY (+13 more)

### Community 160 - "Scene"
Cohesion: 0.07
Nodes (20): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank() (+12 more)

### Community 161 - "Host"
Cohesion: 0.24
Nodes (7): AppEvent, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host

### Community 162 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 172 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 174 - ".show"
Cohesion: 0.17
Nodes (9): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder, ZonesView (+1 more)

### Community 176 - "tools.rs"
Cohesion: 0.14
Nodes (52): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+44 more)

### Community 195 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 196 - "Tab"
Cohesion: 0.25
Nodes (7): Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 199 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 200 - "app.rs"
Cohesion: 0.22
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 201 - "ShowError"
Cohesion: 0.31
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 206 - "Effect"
Cohesion: 0.33
Nodes (3): Show, FxPreview, Effect

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 215 - ".ui"
Cohesion: 0.21
Nodes (6): body(), heading_font(), heading_text(), text(), title(), Toggle

### Community 217 - "DialogKind"
Cohesion: 0.50
Nodes (4): DialogKind, ImportEv2, LoadVenue, SaveVenue

### Community 226 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 227 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

### Community 228 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

## Knowledge Gaps
- **424 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+419 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 973 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **116 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `venue.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `scene_grid.rs`, `Engine`, `bank_menu`, `stage_canvas.rs`, `Snapshot`, `engine.rs`, `rig.rs`, `Scene`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `FixtureType`, `demo.rs`, `app.rs`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `viz/mod.rs`, `EffectKind`?**
  _High betweenness centrality (0.119) - this node is a cross-community bridge._
- **Are the 111 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 111 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _424 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.061581920903954805 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `scene_grid.rs`, `DmxFrame`, `control/timeline.rs`, `Snapshot`, `engine.rs`, `rig.rs`, `scene_props.rs`, `audio.rs`, `Scene`, `Host`, `FixtureAction`, `.show`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `Layer`, `FixtureType`, `ViewCtx`, `stage.rs`, `app.rs`, `Show`, `t`, `.fade_state`, `AppState`, `Profile`, `Rect`, `View`, `Action`, `mcp.rs`?**
  _High betweenness centrality (0.098) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14039408866995073 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `pal`, `stage_editor.rs`, `scene_grid.rs`, `i18n/mod.rs`, `fx_ui.rs`, `bank_menu`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `palettes.rs`, `Timeline`, `color_picker.rs`, `FixtureAction`, `fader.rs`, `.show`, `DmxCraftApp`, `settings_window.rs`, `ViewCtx`, `stage.rs`, `app.rs`, `Settings`, `fx_color.rs`, `patch_grid.rs`, `dialog.rs`, `viz/mod.rs`, `Dialog<'a>`, `Profile`, `Rect`, `View`, `.ui`, `Action`?**
  _High betweenness centrality (0.093) - this node is a cross-community bridge._