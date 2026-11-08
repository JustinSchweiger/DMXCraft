# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 128 files · ~175,173 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3382 nodes · 8536 edges · 239 communities (135 shown, 104 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 441 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e7b44166`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- views/touch.rs
- library_panel.rs
- runtime.rs
- open_dmx.rs
- theme.rs
- programmer.rs
- AppState
- steps
- fx.rs
- OutputError
- Deserialize
- Engine
- Venue
- viz/library.rs
- jobs
- steps
- DmxCraft
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Ui
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- fx_ui.rs
- env
- dmxcraft
- Venue
- t
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
- Capability
- BankScope
- PlanShape
- ViewCtx
- window.rs
- stage.rs
- Host
- Show
- feature.rs
- OutputKind
- motion.rs
- egui
- engine.rs
- Vec2
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- app.rs
- Toggle<'a>
- profiles.rs
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- ProfileBuilder
- MapPreset
- scene_card.rs
- DeviceModel
- qxf.rs
- file_dialog.rs
- en.rs
- VizView
- Rect
- DmxFrame
- ev2.rs
- Context
- EffectKind
- outline.rs
- convert.rs
- Snapshot
- gltf_import.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- audio.rs
- Color32
- stage_canvas.rs
- core/src/lib.rs
- Box<dyn serialport::SerialPort>
- panels.rs
- MenuItem<'a>
- plan.rs
- SceneId
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- FixtureLimits
- Shape
- HeaderAction
- color_picker.rs
- Problem
- DialogKind
- plan
- Scene
- fixture_settings.rs
- Category
- Vec2
- stage_editor.rs
- src/mapping.rs
- lock.rs
- tools.rs
- dip.rs
- ClipAction
- demo.rs
- Settings
- Dialog<'a>
- Tab
- Grip
- midi.rs
- Drag
- Touch-Oberfläche
- target_shape
- mapping/mod.rs
- paint_symbol
- qlc-bundle.rs
- Fullscreen
- resize_handles
- ShowError
- arc
- icon_library.rs
- Tab
- ProgrammerMode
- Mapper
- apply
- .ui
- UpdateBadge
- .check_placement
- Timeline
- FooterButton<'a>
- Tastatur- und MIDI-Zuordnung

## God Nodes (most connected - your core abstractions)
1. `Engine` - 196 edges
2. `t()` - 189 edges
3. `Show` - 139 edges
4. `Icon` - 106 edges
5. `AppState` - 79 edges
6. `ChannelKind` - 72 edges
7. `ViewCtx` - 71 edges
8. `ChannelRef` - 55 edges
9. `FixtureType` - 46 edges
10. `DmxFrame` - 43 edges

## Surprising Connections (you probably didn't know these)
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

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

## Communities (239 total, 104 thin omitted)

### Community 0 - "views/touch.rs"
Cohesion: 0.09
Nodes (36): apply_pick(), channels_of(), COLORS, content_rect(), current_color(), current_position(), display_label(), Drag (+28 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (12): LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH (+4 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.05
Nodes (42): active(), BLACK, CONTRAST, DANGER, DASLIGHT, DIP_BODY, FS_BODY, FS_CAPTION (+34 more)

### Community 5 - "programmer.rs"
Cohesion: 0.20
Nodes (16): elide(), fader(), fader_groups(), fader_label(), FaderGroup, fixture_block(), fixture_blocks(), FixtureBlock (+8 more)

### Community 6 - "AppState"
Cohesion: 0.12
Nodes (11): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (28): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+20 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig, OutputError (+4 more)

### Community 10 - "Deserialize"
Cohesion: 0.05
Nodes (25): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+17 more)

### Community 11 - "Engine"
Cohesion: 0.10
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.15
Nodes (7): renumber(), fixture_folders_are_exclusive(), FixtureFolder, mul(), Object, room_and_tree(), Venue

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
Cohesion: 0.16
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

### Community 23 - "Ui"
Cohesion: 0.13
Nodes (21): kind_color(), card_row(), full_combo(), list_header(), list_row(), ListAction, Add, Duplicate (+13 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.05
Nodes (54): Step, Enter, Maximize, Knob, Knob<'a>, COMPACT_W, FINE_STEP_MS, format() (+46 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.20
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.11
Nodes (33): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+25 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.12
Nodes (13): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, hang_below_nearest_truss(), IDENTITY, Material (+5 more)

### Community 36 - "t"
Cohesion: 0.11
Nodes (27): t(), text_field(), capability_fields(), category_label(), check(), color_label(), FOOT_H, general() (+19 more)

### Community 37 - "mapping.rs"
Cohesion: 0.19
Nodes (11): fbm(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level() (+3 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.18
Nodes (17): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+9 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fader_ranges()

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.21
Nodes (3): DmxCraftApp, fill_types(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.23
Nodes (19): ai(), customize(), full_combo(), general(), HEIGHT, hint(), midi(), outputs() (+11 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "Capability"
Cohesion: 0.16
Nodes (7): add_range_until_every_value_is_used(), cap_for_kind(), Capability, normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 65 - "BankScope"
Cohesion: 0.14
Nodes (8): BankScope, All, Bank, Banks, Off, OutsideBank, duplicate_zone_copies_scenes_behind_original(), Zone

### Community 66 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 68 - "ViewCtx"
Cohesion: 0.21
Nodes (6): fold_section(), ViewCtx, proposals_for_easy_view_names(), propose_type(), vec3_row(), VizView

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (25): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.16
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Host"
Cohesion: 0.14
Nodes (13): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+5 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (5): .VIRTUAL_DIMMER, Fixture, Group, Show, Placement

### Community 73 - "feature.rs"
Cohesion: 0.11
Nodes (22): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+14 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "egui"
Cohesion: 0.19
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 78 - "engine.rs"
Cohesion: 0.15
Nodes (31): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+23 more)

### Community 79 - "Vec2"
Cohesion: 0.05
Nodes (29): DipSwitch, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+21 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.28
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, Form, HEADER_H, INPUT_H, Kind (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (20): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS (+12 more)

### Community 90 - "app.rs"
Cohesion: 0.25
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 92 - "profiles.rs"
Cohesion: 0.21
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.12
Nodes (25): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), GizmoDrag, Handles, handles(), hit_y() (+17 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (49): zone_color(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+41 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.13
Nodes (12): BuilderRequest, Edit, New, Open, Template, FileJob, Export, Import (+4 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 101 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "VizView"
Cohesion: 0.14
Nodes (11): Missing, Mode, Build, Live, VizView, FixtureSort, Address, Name (+3 more)

### Community 109 - "Rect"
Cohesion: 0.28
Nodes (5): fmt_ms(), grid_options(), paint_clip(), TimelineView, track_header()

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Context"
Cohesion: 0.22
Nodes (11): overlay_id(), Pick, pick(), pick_full(), pick_id(), states_id(), take_touch_picks(), touch_pick() (+3 more)

### Community 118 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (25): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 128 - "gltf_import.rs"
Cohesion: 0.18
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.14
Nodes (14): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, row_h() (+6 more)

### Community 137 - "audio.rs"
Cohesion: 0.06
Nodes (24): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+16 more)

### Community 140 - "Color32"
Cohesion: 0.11
Nodes (37): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+29 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.13
Nodes (20): AMBER, BOX, Camera, fit(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+12 more)

### Community 143 - "core/src/lib.rs"
Cohesion: 0.17
Nodes (9): LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static, Super (+1 more)

### Community 147 - "panels.rs"
Cohesion: 0.23
Nodes (10): aim_selected(), apply_placement(), Pick, Fixture, Object, targets(), TexAction, Builtin (+2 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 152 - "SceneId"
Cohesion: 0.12
Nodes (5): FadeState, In, Out, pause_and_step_jumps(), SceneProgress

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.29
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 160 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 161 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 162 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 168 - "Scene"
Cohesion: 0.15
Nodes (5): free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 169 - "fixture_settings.rs"
Cohesion: 0.17
Nodes (12): Layer, Live, Programmer, Quick, quick_action(), Center, Off, On (+4 more)

### Community 170 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 172 - "stage_editor.rs"
Cohesion: 0.09
Nodes (18): CanvasMode, Edit, Select, fixture_pos(), apply(), MenuPlace, Patch, Stage (+10 more)

### Community 173 - "src/mapping.rs"
Cohesion: 0.07
Nodes (32): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, BankStep, Blackout (+24 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.06
Nodes (79): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), Host, call_tool() (+71 more)

### Community 178 - "dip.rs"
Cohesion: 0.29
Nodes (7): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches(), addresses_are_one_based()

### Community 179 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 180 - "demo.rs"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 181 - "Settings"
Cohesion: 0.16
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 183 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 184 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 185 - "midi.rs"
Cohesion: 0.15
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "Touch-Oberfläche"
Cohesion: 0.40
Nodes (4): Bearbeiten, Bedienen, Elemente hinzufügen, Touch-Oberfläche

### Community 189 - "target_shape"
Cohesion: 0.23
Nodes (14): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), report(), rotated_quad(), t_touch_hint() (+6 more)

### Community 192 - "mapping/mod.rs"
Cohesion: 0.18
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fires_id(), fixed_channels_work_without_selection() (+12 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Fullscreen"
Cohesion: 0.25
Nodes (3): Fullscreen, SETTLE, mark_clipboard()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "icon_library.rs"
Cohesion: 0.29
Nodes (4): glyph(), LIBRARY, names_are_unique(), paint()

### Community 207 - "Tab"
Cohesion: 0.17
Nodes (11): SettingsWindow, sidebar(), Tab, Ai, .ALL, Customize, General, Midi (+3 more)

### Community 210 - "ProgrammerMode"
Cohesion: 0.29
Nodes (5): ProgrammerMode, Blind, Edit, Live, items_mode()

### Community 216 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

### Community 217 - "apply"
Cohesion: 0.29
Nodes (6): clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 219 - ".ui"
Cohesion: 0.31
Nodes (10): button(), category_list(), color_bar(), current(), fixtures_of(), merge_groups(), mode_switch(), tool_separator() (+2 more)

### Community 221 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 223 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **492 isolated node(s):** `General`, `Customize`, `Startup`, `Outputs`, `Midi` (+487 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1128 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **104 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `views/touch.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `audio.rs`, `Color32`, `core/src/lib.rs`, `rig.rs`, `scene_props.rs`, `SceneId`, `mcp/src/venue.rs`, `t`, `fixture_settings.rs`, `stage_editor.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `mapping/mod.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `Host`, `Show`, `egui`, `engine.rs`, `Mapper`, `app.rs`, `.ui`, `scene_grid.rs`, `ProfileBuilder`, `Rect`, `Snapshot`?**
  _High betweenness centrality (0.145) - this node is a cross-community bridge._
- **Are the 157 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 157 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Customize`, `Startup` to the rest of the system?**
  _492 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `views/touch.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0858843537414966 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `views/touch.rs`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `Deserialize`, `Engine`, `Color32`, `stage_canvas.rs`, `Venue`, `rig.rs`, `FixtureAction`, `Scene`, `stage_editor.rs`, `src/mapping.rs`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `mapping/mod.rs`, `BankScope`, `window.rs`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `show.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `.check_placement`, `Context`, `EffectKind`, `Snapshot`?**
  _High betweenness centrality (0.119) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.14193548387096774 - nodes in this community are weakly interconnected._