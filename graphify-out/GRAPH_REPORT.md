# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 117 files · ~147,264 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 2953 nodes · 7145 edges · 242 communities (124 shown, 118 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 362 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `f71a43d4`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- core/src/venue.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- Action
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
- audio.rs
- env
- dmxcraft
- Venue
- pal
- color_picker.rs
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag_number
- textures.rs
- fader.rs
- render.rs
- DmxCraft – Hinweise für KI-Assistenten
- DmxCraftApp
- settings_window.rs
- VizView
- ClipAction
- core/src/lib.rs
- .row
- ViewCtx
- engine.rs
- stage.rs
- gltf_import.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- patch_grid.rs
- fx_ui.rs
- Pos2
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- t
- Toggle<'a>
- profile_builder.rs
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- Dialog<'a>
- MapPreset
- app.rs
- qxf.rs
- en.rs
- profiles.rs
- Rect
- fixture_settings.rs
- ev2.rs
- knob.rs
- mcp.rs
- EffectKind
- convert.rs
- model.rs
- Snapshot
- CurveShape
- ColorPreset
- Material
- stage_editor.rs
- Scene
- stage_canvas.rs
- bank_menu
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- server.rs
- .effects_tab
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- time_field.rs
- ZoneId
- protocol.rs
- super
- Problem
- Step
- Vec2
- Shape
- lock.rs
- tools.rs
- file_dialog.rs
- dip.rs
- .of
- demo.rs
- palettes.rs
- Category
- swatch.rs
- qlc-bundle.rs
- Camera
- ShowError
- Arrange
- arc
- Settings
- Drag
- DialogKind
- FixtureLimits
- plan
- .check_placement
- DmxFrame
- Grip
- FxFile
- scene_brief
- SceneCard
- Timeline
- ChannelValue
- ok
- FooterButton<'a>
- PickTarget
- T
- Tab

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 154 edges
3. `Show` - 120 edges
4. `Icon` - 104 edges
5. `ChannelKind` - 69 edges
6. `ViewCtx` - 61 edges
7. `AppState` - 55 edges
8. `FixtureType` - 43 edges
9. `DmxFrame` - 43 edges
10. `Venue` - 42 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `load()` --calls--> `b64_decode()`  [INFERRED]
  crates/app/src/viz/gltf_import.rs → crates/core/src/venue/model.rs
- `VizView` --references--> `Prefab`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/library.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/control/programmer.rs -> crates/app/src/views/control/programmer.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (242 total, 118 thin omitted)

### Community 0 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (11): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, mul(), rotation(), rotation_order_x_then_y_then_z() (+3 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (18): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+10 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.08
Nodes (30): accent_hover(), active(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION (+22 more)

### Community 5 - "programmer.rs"
Cohesion: 0.10
Nodes (31): ProgrammerMode, Blind, Edit, Live, kind_color(), category_list(), color_bar(), current() (+23 more)

### Community 6 - "AppState"
Cohesion: 0.13
Nodes (11): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (25): all_presets_render_every_beam(), apply(), Beam, beams(), ColorFx, fbm(), grayscale_has_equal_channels(), hard_edges_use_palette_colors_only() (+17 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig (+4 more)

### Community 10 - "Action"
Cohesion: 0.18
Nodes (20): Action, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump, Mark (+12 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (14): BLACK, builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user(), METAL, Prefab (+6 more)

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
Cohesion: 0.06
Nodes (48): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+40 more)

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
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (22): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), Kind, kind() (+14 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.22
Nodes (13): bar_slider(), FX_EXT, FxDialog, pick_tile(), PickColumn, Picker, picker_window(), properties() (+5 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.14
Nodes (14): heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn, item() (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (16): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+8 more)

### Community 31 - "audio.rs"
Cohesion: 0.13
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.16
Nodes (5): fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

### Community 36 - "pal"
Cohesion: 0.22
Nodes (16): accent(), accent_dim(), apply(), bg(), body(), border(), caption(), install_fonts() (+8 more)

### Community 37 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

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

### Community 43 - "textures.rs"
Cohesion: 0.19
Nodes (16): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), make(), mix() (+8 more)

### Community 44 - "fader.rs"
Cohesion: 0.11
Nodes (13): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+5 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.14
Nodes (26): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+18 more)

### Community 50 - "VizView"
Cohesion: 0.10
Nodes (14): decode_texture(), Missing, Mode, Build, Live, model_tex_id(), QualityParams, VizView (+6 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "core/src/lib.rs"
Cohesion: 0.10
Nodes (17): LoopMode, Loop, Once, PLAN_SIZE_RANGE, PlanShape, .ALL, BarHorizontal, BarVertical (+9 more)

### Community 66 - ".row"
Cohesion: 0.09
Nodes (26): renumber(), drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural() (+18 more)

### Community 68 - "ViewCtx"
Cohesion: 0.22
Nodes (12): fold_section(), ViewCtx, aim_selected(), apply_placement(), targets(), vec3_row(), apply_texture(), TexAction (+4 more)

### Community 69 - "engine.rs"
Cohesion: 0.23
Nodes (30): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+22 more)

### Community 70 - "stage.rs"
Cohesion: 0.20
Nodes (14): select_all(), fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename (+6 more)

### Community 71 - "gltf_import.rs"
Cohesion: 0.18
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (3): Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (25): apply_effect(), apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.14
Nodes (14): along(), apply(), catmull(), circle_goes_round(), local_roundtrip(), MoveFx, MoveShape, .ALL (+6 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.16
Nodes (12): Segmented, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output (+4 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.12
Nodes (24): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+16 more)

### Community 79 - "Pos2"
Cohesion: 0.11
Nodes (9): ICON_FAMILY, ICON_FILL_FAMILY, LimitPad, CARD_H, STRIP_W, Toggle, XyPad, Handles (+1 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.08
Nodes (26): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+18 more)

### Community 90 - "t"
Cohesion: 0.13
Nodes (11): t(), section_header(), body(), caption(), FixtureSettings, range_row(), ZonesView, Windows (+3 more)

### Community 92 - "profile_builder.rs"
Cohesion: 0.05
Nodes (61): text_field(), fine_mark(), paint_symbol(), SceneCardResponse, capability_fields(), card_row(), category_label(), check() (+53 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.15
Nodes (25): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, handles(), hit_y(), move_all(), Pick (+17 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.19
Nodes (15): programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow(), chip() (+7 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.06
Nodes (38): default_palette(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, MapPreset, .ALL, Bounce (+30 more)

### Community 100 - "app.rs"
Cohesion: 0.19
Nodes (9): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView, StageView, View (+1 more)

### Community 104 - "qxf.rs"
Cohesion: 0.23
Nodes (11): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), rejects_files_without_modes() (+3 more)

### Community 108 - "profiles.rs"
Cohesion: 0.17
Nodes (10): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+2 more)

### Community 109 - "Rect"
Cohesion: 0.32
Nodes (3): fmt_ms(), paint_clip(), TimelineView

### Community 110 - "fixture_settings.rs"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "mcp.rs"
Cohesion: 0.13
Nodes (12): McpLink, McpStatus, Error, Off, Running, AppEvent, Saved, SelectScene (+4 more)

### Community 119 - "EffectKind"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 121 - "convert.rs"
Cohesion: 0.15
Nodes (21): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+13 more)

### Community 123 - "model.rs"
Cohesion: 0.09
Nodes (8): Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 128 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "Material"
Cohesion: 0.22
Nodes (5): Builder, mat(), Environment, hang_below_nearest_truss(), Material

### Community 137 - "stage_editor.rs"
Cohesion: 0.18
Nodes (9): CanvasMode, Edit, Select, StageCanvas, StageInteraction, Underlay, set_pos(), StageEditorView (+1 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.19
Nodes (16): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+8 more)

### Community 143 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (21): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, grid_options(), HEADER_W, HeaderAction (+13 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.12
Nodes (13): bridge_forwards_to_server(), dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), header() (+5 more)

### Community 152 - ".effects_tab"
Cohesion: 0.26
Nodes (8): advanced(), block_props(), effect_header(), effect_tiles(), header(), row(), scope_row(), steps_table()

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.39
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 160 - "ZoneId"
Cohesion: 0.19
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 161 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 162 - "super"
Cohesion: 0.18
Nodes (5): save_as_gets_show_extension(), SHOW_EXT, with_show_extension(), RECENT_MAX, addresses_are_one_based()

### Community 165 - "Problem"
Cohesion: 0.11
Nodes (21): problem_text(), fine_names(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode (+13 more)

### Community 170 - "Step"
Cohesion: 0.20
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, mark_clipboard()

### Community 171 - "Vec2"
Cohesion: 0.17
Nodes (8): from_pct(), HANDLE, pct(), RangeSlider, value_field(), VALUE_H, OrbitCamera, ray_through_center_hits_target()

### Community 172 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.13
Nodes (50): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+42 more)

### Community 177 - "file_dialog.rs"
Cohesion: 0.38
Nodes (4): Pending, Purpose, OpenShow, SaveShowAs

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - ".of"
Cohesion: 0.33
Nodes (3): Show, fit_mapping(), FxPreview

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 195 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 201 - "ShowError"
Cohesion: 0.16
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "Settings"
Cohesion: 0.19
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), Settings

### Community 207 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 210 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 218 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 220 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 226 - "ok"
Cohesion: 0.67
Nodes (4): full_workflow(), helpful_errors(), host(), ok()

### Community 231 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 234 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

## Knowledge Gaps
- **442 isolated node(s):** `Low`, `Medium`, `High`, `Ultra`, `Live` (+437 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1035 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **118 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `Snapshot`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `stage_editor.rs`, `Engine`, `Scene`, `stage_canvas.rs`, `bank_menu`, `.add_scene`, `rig.rs`, `ZoneId`, `Venue`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `core/src/lib.rs`, `engine.rs`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `.check_placement`, `viz/mod.rs`, `scene_grid.rs`, `app.rs`?**
  _High betweenness centrality (0.123) - this node is a cross-community bridge._
- **Are the 127 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 127 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Low`, `Medium`, `High` to the rest of the system?**
  _442 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `core/src/venue.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14035087719298245 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `stage_editor.rs`, `Action`, `i18n/mod.rs`, `viz/library.rs`, `bank_menu`, `control/timeline.rs`, `titlebar.rs`, `.effects_tab`, `scene_props.rs`, `.viewport`, `pal`, `color_picker.rs`, `Problem`, `FixtureAction`, `textures.rs`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `file_dialog.rs`, `VizView`, `palettes.rs`, `.row`, `ViewCtx`, `stage.rs`, `Settings`, `fx_ui.rs`, `patch_grid.rs`, `dialog.rs`, `profile_builder.rs`, `FxFile`, `viz/mod.rs`, `scene_grid.rs`, `Dialog<'a>`, `app.rs`, `profiles.rs`, `Rect`?**
  _High betweenness centrality (0.103) - this node is a cross-community bridge._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10810810810810811 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `AppState`, `Action`, `control/timeline.rs`, `.effects_tab`, `scene_props.rs`, `rig.rs`, `mcp/src/venue.rs`, `audio.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `core/src/lib.rs`, `ViewCtx`, `engine.rs`, `stage.rs`, `Show`, `DmxFrame`, `profile_builder.rs`, `scene_brief`, `scene_grid.rs`, `app.rs`, `Rect`, `fixture_settings.rs`, `mcp.rs`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._