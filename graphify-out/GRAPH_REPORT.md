# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 123 files · ~158,814 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3147 nodes · 7678 edges · 245 communities (140 shown, 105 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 400 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `29fac9fe`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- engine.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- ChannelDef
- Engine
- i18n/mod.rs
- viz/library.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- .add_scene
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- pal
- env
- dmxcraft
- Venue
- Ui
- mapping.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- textures.rs
- Fader
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- MenuAction
- ClipAction
- core/src/lib.rs
- .row
- .object_props
- window.rs
- stage.rs
- .fine_names
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- fx_ui.rs
- egui
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- app.rs
- Toggle<'a>
- profile_builder.rs
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- ProfileBuilder
- MapPreset
- scene_card.rs
- time_field.rs
- qxf.rs
- file_dialog.rs
- en.rs
- profiles.rs
- Rect
- quick.rs
- ev2.rs
- knob.rs
- Host
- Scene
- convert.rs
- model.rs
- Snapshot
- CurveShape
- ColorPreset
- Tab
- View
- text_field
- stage_canvas.rs
- Drag
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- server.rs
- .check_placement
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- Step
- call
- protocol.rs
- VizView
- Problem
- Venue
- Vec2
- Shape
- src/mapping.rs
- lock.rs
- tools.rs
- dip.rs
- PlanShape
- demo.rs
- .default
- Theme
- audio.rs
- midi.rs
- Settings
- color_picker.rs
- outline.rs
- Dialog<'a>
- Control
- Timeline
- mapping/mod.rs
- Mapper
- swatch.rs
- qlc-bundle.rs
- DmxFrame
- ShowError
- Capability
- arc
- DialogKind
- panels.rs
- SlotKind
- DvcGoldUsb
- Tab
- plan
- BankScope
- range_slider.rs
- palettes.rs
- Timeline
- UpdateBadge
- ChannelValue
- FooterButton<'a>
- widgets/mod.rs
- BuilderRequest
- T
- elide
- paint_symbol
- .fade_state
- Box<dyn serialport::SerialPort>
- MAPPING.md

## God Nodes (most connected - your core abstractions)
1. `Engine` - 186 edges
2. `t()` - 171 edges
3. `Show` - 126 edges
4. `Icon` - 105 edges
5. `AppState` - 75 edges
6. `ChannelKind` - 71 edges
7. `ViewCtx` - 61 edges
8. `FixtureType` - 45 edges
9. `DmxFrame` - 43 edges
10. `DmxCraftApp` - 42 edges

## Surprising Connections (you probably didn't know these)
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `DmxCraftApp` --references--> `Pending`  [INFERRED]
  crates/app/src/app.rs → crates/app/src/file_dialog.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (245 total, 105 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.22
Nodes (31): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+23 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (11): LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH (+3 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (30): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+22 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (39): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+31 more)

### Community 6 - "AppState"
Cohesion: 0.13
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "ChannelDef"
Cohesion: 0.13
Nodes (12): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+4 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.11
Nodes (18): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+10 more)

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
Cohesion: 0.21
Nodes (19): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+11 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.03
Nodes (77): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+69 more)

### Community 23 - ".add_scene"
Cohesion: 0.14
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), json_roundtrip(), move_scenes_between_banks(), steps_insert_remove_and_normalize(), Zone

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (51): Step, Enter, Maximize, Show, advanced(), banks_of(), bar_slider(), block_props() (+43 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (10): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+2 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "pal"
Cohesion: 0.16
Nodes (13): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), install_fonts() (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.11
Nodes (15): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, hang_below_nearest_truss(), IDENTITY, Material (+7 more)

### Community 36 - "Ui"
Cohesion: 0.16
Nodes (21): kind_color(), card_row(), check(), full_combo(), hint(), list_header(), list_row(), Duplicate (+13 more)

### Community 37 - "mapping.rs"
Cohesion: 0.12
Nodes (17): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+9 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (28): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+20 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.09
Nodes (26): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+18 more)

### Community 44 - "Fader"
Cohesion: 0.13
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fader_ranges()

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.20
Nodes (3): DmxCraftApp, fill_types(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (18): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+10 more)

### Community 50 - "MenuAction"
Cohesion: 0.08
Nodes (24): load_meter(), MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, Mapping, NewShow (+16 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (12): SceneProgress, FixtureLimits, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind (+4 more)

### Community 66 - ".row"
Cohesion: 0.12
Nodes (18): drag_row(), drop_marker(), Renaming, Folder, Object, Tree, TreeAction, Delete (+10 more)

### Community 68 - ".object_props"
Cohesion: 0.21
Nodes (7): fold_section(), vec3_row(), TexAction, Builtin, Load, Set, VizView

### Community 69 - "window.rs"
Cohesion: 0.13
Nodes (19): color(), describe(), apc_channel_labels(), channel_label(), combo(), hint(), LIST_H, MAP_EXT (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.21
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 72 - "Show"
Cohesion: 0.23
Nodes (3): Fixture, paste_fixtures_finds_free_addresses_and_names(), Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (21): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+13 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.12
Nodes (27): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+19 more)

### Community 79 - "egui"
Cohesion: 0.09
Nodes (20): Dialog, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+12 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.27
Nodes (10): address_map_marks_ranges(), groups_follow_fixture_removal(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes(), repatch_ignores_itself() (+2 more)

### Community 87 - "dialog.rs"
Cohesion: 0.09
Nodes (12): Button, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.10
Nodes (18): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX (+10 more)

### Community 90 - "app.rs"
Cohesion: 0.29
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 92 - "profile_builder.rs"
Cohesion: 0.12
Nodes (22): t(), capability_fields(), category_label(), color_label(), FOOT_H, general(), hex_field(), kind_label() (+14 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.19
Nodes (15): AXIS_COLORS, Handles, handles(), hit_y(), model_tex_id(), project(), QualityParams, ray_aabb_exit() (+7 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (50): CARD_H, COMPACT_H, programmer_values(), Action, Bank, Compact, CreateSuper, Delete (+42 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.14
Nodes (10): FileJob, Export, Import, import(), ListAction, Add, Remove, new_profile() (+2 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.27
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 101 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "profiles.rs"
Cohesion: 0.20
Nodes (10): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+2 more)

### Community 109 - "Rect"
Cohesion: 0.29
Nodes (4): fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 110 - "quick.rs"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.22
Nodes (3): raised_hover(), Knob, Knob<'a>

### Community 118 - "Host"
Cohesion: 0.16
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+6 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "model.rs"
Cohesion: 0.09
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 128 - "Snapshot"
Cohesion: 0.26
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 137 - "View"
Cohesion: 0.14
Nodes (13): MenuPlace, Patch, Stage, StageEditor, Arrange, AlignH, AlignV, Grid (+5 more)

### Community 140 - "text_field"
Cohesion: 0.12
Nodes (9): text_field(), View, body(), caption(), FixtureSettings, range_row(), SetupView, ZonesView (+1 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (20): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+12 more)

### Community 143 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (21): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete (+13 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.12
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 152 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "Step"
Cohesion: 0.25
Nodes (3): Fullscreen, SETTLE, mark_clipboard()

### Community 160 - "call"
Cohesion: 0.25
Nodes (25): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), channel_kind(), collect_values(), create_scene() (+17 more)

### Community 161 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 162 - "VizView"
Cohesion: 0.11
Nodes (11): decode_texture(), Missing, Mode, Build, Live, proposals_for_easy_view_names(), propose_type(), VizView (+3 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "Venue"
Cohesion: 0.16
Nodes (6): renumber(), fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

### Community 172 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 173 - "src/mapping.rs"
Cohesion: 0.16
Nodes (15): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, Feedback, FeedbackState (+7 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.11
Nodes (31): any_obj(), apply_scene_fields(), boolean(), clip_schema(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+23 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - ".default"
Cohesion: 0.40
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 183 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 184 - "audio.rs"
Cohesion: 0.14
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 185 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 186 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 187 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 188 - "outline.rs"
Cohesion: 0.14
Nodes (13): deg_quat(), euler_matches_core_rotation(), quat_deg(), DragFixtures, DragObjects, FixtureSort, Address, Name (+5 more)

### Community 190 - "Control"
Cohesion: 0.13
Nodes (15): Overlay, Control, BankStep, Blackout, Master, Scene, ShowBank, StopAll (+7 more)

### Community 191 - "Timeline"
Cohesion: 0.24
Nodes (3): Clip, Timeline, Track

### Community 192 - "mapping/mod.rs"
Cohesion: 0.29
Nodes (9): bank_step_wraps(), fire(), index_targets(), is_on(), mapping(), resolve(), setup(), toggle_and_flash() (+1 more)

### Community 193 - "Mapper"
Cohesion: 0.23
Nodes (4): Mapper, overlay_id(), pick_id(), target()

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 200 - "DmxFrame"
Cohesion: 0.16
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Capability"
Cohesion: 0.19
Nodes (3): cap_for_kind(), Capability, last_matching()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 207 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 210 - "panels.rs"
Cohesion: 0.32
Nodes (12): ViewCtx, aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object (+4 more)

### Community 211 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 212 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 216 - "Tab"
Cohesion: 0.22
Nodes (8): sidebar(), Tab, Ai, .ALL, General, Midi, Outputs, Project

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 218 - "BankScope"
Cohesion: 0.22
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 220 - "range_slider.rs"
Cohesion: 0.43
Nodes (5): from_pct(), pct(), RangeSlider, value_field(), VALUE_H

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "Timeline"
Cohesion: 0.18
Nodes (11): ClipContent, Audio, Scene, fades(), GridMode, Beat, Time, one() (+3 more)

### Community 225 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 230 - "widgets/mod.rs"
Cohesion: 0.40
Nodes (4): CanvasMode, Edit, Select, XyPad

### Community 231 - "BuilderRequest"
Cohesion: 0.40
Nodes (5): BuilderRequest, Edit, New, Open, Template

### Community 240 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

## Knowledge Gaps
- **468 isolated node(s):** `Setup`, `Control`, `Edit`, `Blind`, `Live` (+463 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1079 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **105 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `profile_builder.rs` to `library_panel.rs`, `programmer.rs`, `View`, `i18n/mod.rs`, `text_field`, `viz/library.rs`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `.viewport`, `pal`, `VizView`, `Ui`, `FixtureAction`, `textures.rs`, `Fader`, `DmxCraftApp`, `settings_window.rs`, `MenuAction`, `Settings`, `color_picker.rs`, `outline.rs`, `Dialog<'a>`, `.row`, `.object_props`, `window.rs`, `stage.rs`, `patch_grid.rs`, `fx_ui.rs`, `egui`, `panels.rs`, `dialog.rs`, `app.rs`, `viz/mod.rs`, `palettes.rs`, `scene_grid.rs`, `ProfileBuilder`, `file_dialog.rs`, `profiles.rs`, `Rect`?**
  _High betweenness centrality (0.128) - this node is a cross-community bridge._
- **Are the 140 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 140 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Setup`, `Control`, `Edit` to the rest of the system?**
  _468 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `engine.rs`, `runtime.rs`, `programmer.rs`, `text_field`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `call`, `Ui`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `audio.rs`, `mapping/mod.rs`, `Mapper`, `core/src/lib.rs`, `window.rs`, `stage.rs`, `DmxFrame`, `Show`, `panels.rs`, `app.rs`, `profile_builder.rs`, `scene_grid.rs`, `ProfileBuilder`, `Rect`, `quick.rs`, `.fade_state`, `Host`?**
  _High betweenness centrality (0.118) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Show` (e.g. with `Mappings` and `Venue`) actually correct?**
  _`Show` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Should `open_dmx.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11330049261083744 - nodes in this community are weakly interconnected._