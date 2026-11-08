# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 124 files · ~163,498 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3216 nodes · 7831 edges · 248 communities (137 shown, 111 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 413 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `be4d26d3`
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
- Scene
- rig.rs
- scene_props.rs
- host
- menu.rs
- with
- ensure_linux_menu_entry
- ofl/src/lib.rs
- fx_color.rs
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
- FixtureLimits
- .row
- .object_props
- window.rs
- stage.rs
- Host
- Show
- feature.rs
- OutputKind
- motion.rs
- View
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
- hashmap
- qxf.rs
- file_dialog.rs
- en.rs
- profiles.rs
- Rect
- DeviceModel
- ev2.rs
- knob.rs
- EffectKind
- outline.rs
- convert.rs
- model.rs
- Snapshot
- CurveShape
- ColorPreset
- DmxFrame
- BankScope
- t
- stage_canvas.rs
- Drag
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- gltf_import.rs
- DvcGoldUsb
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- Step
- Shape
- Transform
- ChaserPreset
- VizView
- Problem
- swatch.rs
- plan
- UpdateBadge
- Capability
- Venue
- Vec2
- stage_editor.rs
- src/mapping.rs
- lock.rs
- tools.rs
- dip.rs
- Dialog<'a>
- demo.rs
- ProfileChannel
- time_field.rs
- audio.rs
- midi.rs
- Settings
- color_picker.rs
- PaletteStore
- mapping/mod.rs
- Mapper
- qlc-bundle.rs
- ShowError
- arc
- DialogKind
- apply
- SlotKind
- Tab
- Lang
- Category
- Tab
- Timeline
- T
- .fine_names
- .channel
- FooterButton<'a>
- .check_placement
- Ui
- Grip
- Box<dyn serialport::SerialPort>
- Mode
- OutlineTab
- resize_handles
- Tastatur- und MIDI-Zuordnung

## God Nodes (most connected - your core abstractions)
1. `Engine` - 187 edges
2. `t()` - 175 edges
3. `Show` - 131 edges
4. `Icon` - 105 edges
5. `AppState` - 75 edges
6. `ChannelKind` - 70 edges
7. `ViewCtx` - 61 edges
8. `ChannelRef` - 48 edges
9. `FixtureType` - 46 edges
10. `DmxFrame` - 43 edges

## Surprising Connections (you probably didn't know these)
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `palette_button()` --calls--> `color32()`  [INFERRED]
  crates/app/src/mapping/window.rs → crates/app/src/midi_device.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (248 total, 111 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.20
Nodes (32): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+24 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.14
Nodes (10): LibraryDrag, drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH, patch() (+2 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "theme.rs"
Cohesion: 0.07
Nodes (49): accent(), accent_dim(), accent_hover(), active(), apply(), bg(), body(), border() (+41 more)

### Community 5 - "programmer.rs"
Cohesion: 0.10
Nodes (31): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+23 more)

### Community 6 - "AppState"
Cohesion: 0.13
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.22
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), feature_levels(), grayscale_has_equal_channels(), hard_edges_use_palette_colors_only() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "ChannelDef"
Cohesion: 0.15
Nodes (11): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+3 more)

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.17
Nodes (8): all_texts_translated(), EN, fill(), fill_replaces_names(), LANG, tr!, texts_in_source(), ACTIVE

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

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

### Community 23 - "Scene"
Cohesion: 0.13
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.08
Nodes (41): Step, Enter, Maximize, button(), advanced(), banks_of(), bar_slider(), block_props() (+33 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.16
Nodes (11): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+3 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_color.rs"
Cohesion: 0.22
Nodes (12): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), ROW_H, slider_row() (+4 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.14
Nodes (11): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, mul(), rotation(), rotation_order_x_then_y_then_z() (+3 more)

### Community 36 - "Ui"
Cohesion: 0.14
Nodes (20): kind_color(), card_row(), full_combo(), list_header(), list_row(), ListAction, Add, Duplicate (+12 more)

### Community 37 - "mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (31): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+23 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (17): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+9 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 66 - ".row"
Cohesion: 0.23
Nodes (13): DragObjects, Tree, TreeAction, Delete, Drop, Duplicate, GroupSelection, NewFolder (+5 more)

### Community 68 - ".object_props"
Cohesion: 0.16
Nodes (11): fold_section(), ViewCtx, renumber(), targets(), vec3_row(), apply_texture(), TexAction, Builtin (+3 more)

### Community 69 - "window.rs"
Cohesion: 0.13
Nodes (19): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.17
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Host"
Cohesion: 0.14
Nodes (13): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+5 more)

### Community 72 - "Show"
Cohesion: 0.23
Nodes (4): .VIRTUAL_DIMMER, Fixture, Group, Show

### Community 73 - "feature.rs"
Cohesion: 0.16
Nodes (14): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+6 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "View"
Cohesion: 0.13
Nodes (9): View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView, ZonesView (+1 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.24
Nodes (13): combo(), curve_graph(), effect_body(), features_ui(), grid(), mode_row(), move_editor(), point_row() (+5 more)

### Community 79 - "egui"
Cohesion: 0.09
Nodes (23): SETTLE, DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP (+15 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.10
Nodes (12): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+4 more)

### Community 89 - "profile.rs"
Cohesion: 0.13
Nodes (12): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX, ofl_round_trip() (+4 more)

### Community 90 - "app.rs"
Cohesion: 0.27
Nodes (5): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, viewport_input()

### Community 92 - "profile_builder.rs"
Cohesion: 0.14
Nodes (15): capability_fields(), color_label(), FOOT_H, hex_field(), kind_label(), LIST_W, MAX_FORM_W, paint_range_block() (+7 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.14
Nodes (17): AXIS_COLORS, decode_texture(), Handles, handles(), hit_y(), Missing, model_tex_id(), project() (+9 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.07
Nodes (54): ControlViews, select_all(), CARD_H, COMPACT_H, programmer_values(), ProgrammerView, Action, Bank (+46 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.14
Nodes (11): BuilderRequest, Edit, New, Open, Template, FileJob, Export, Import (+3 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.24
Nodes (4): clip(), SceneCard, SceneCardResponse, STRIP_W

### Community 101 - "hashmap"
Cohesion: 0.28
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "profiles.rs"
Cohesion: 0.18
Nodes (11): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+3 more)

### Community 109 - "Rect"
Cohesion: 0.25
Nodes (6): add_audio(), add_audio_dialog(), fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 110 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.10
Nodes (18): deg_quat(), euler_matches_core_rotation(), quat_deg(), drag_row(), DragFixtures, drop_marker(), FixtureSort, Address (+10 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "model.rs"
Cohesion: 0.09
Nodes (9): Physics, strobe_ranges(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart (+1 more)

### Community 128 - "Snapshot"
Cohesion: 0.26
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.16
Nodes (11): ColorFx, ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow (+3 more)

### Community 136 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 137 - "BankScope"
Cohesion: 0.11
Nodes (16): BankScope, All, Bank, Banks, Off, OutsideBank, LoopMode, Loop (+8 more)

### Community 140 - "t"
Cohesion: 0.22
Nodes (12): t(), text_field(), category_label(), check(), general(), hint(), manufacturer_picker(), physical() (+4 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.10
Nodes (28): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+20 more)

### Community 143 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (19): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete, Duplicate, ExpandAll (+11 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 152 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.22
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 159 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 160 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 161 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 162 - "VizView"
Cohesion: 0.23
Nodes (13): aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object, rotate_all() (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 168 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 169 - "Capability"
Cohesion: 0.15
Nodes (8): cap_for_kind(), Capability, fixture_types_detect_kinds_and_keep_keys(), own_format_round_trip(), problems_found(), spot(), template_from_types(), valid_profile_has_no_problems()

### Community 170 - "Venue"
Cohesion: 0.13
Nodes (8): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), Venue

### Community 172 - "stage_editor.rs"
Cohesion: 0.16
Nodes (11): CanvasMode, Edit, Select, Arrange, AlignH, AlignV, Grid, Reset (+3 more)

### Community 173 - "src/mapping.rs"
Cohesion: 0.08
Nodes (34): Overlay, MapFile, add_find_and_json(), ButtonMode, Toggle, Value, Control, BankStep (+26 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.05
Nodes (81): ChannelValue, Step, dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run() (+73 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "ProfileChannel"
Cohesion: 0.24
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 183 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 184 - "audio.rs"
Cohesion: 0.15
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 185 - "midi.rs"
Cohesion: 0.13
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 186 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 187 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 190 - "PaletteStore"
Cohesion: 0.29
Nodes (8): builtin(), load_user(), named(), NamedPalette, path(), save_user(), PaletteStore, BodyCtx

### Community 192 - "mapping/mod.rs"
Cohesion: 0.16
Nodes (22): bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fader(), fire() (+14 more)

### Community 193 - "Mapper"
Cohesion: 0.15
Nodes (10): default_feedback(), Mapper, overlay_id(), Pick, pick(), pick_id(), report(), Sent (+2 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.27
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 207 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 217 - "apply"
Cohesion: 0.21
Nodes (7): apply(), clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 218 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 219 - "Tab"
Cohesion: 0.20
Nodes (9): SettingsWindow, sidebar(), Tab, Ai, .ALL, General, Midi, Outputs (+1 more)

### Community 220 - "Lang"
Cohesion: 0.25
Nodes (6): Lang, lang(), .ALL, De, En, set_lang()

### Community 221 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 223 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 230 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 233 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 238 - "Mode"
Cohesion: 0.67
Nodes (3): Mode, Build, Live

### Community 240 - "OutlineTab"
Cohesion: 0.67
Nodes (3): OutlineTab, Fixtures, Objects

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **472 isolated node(s):** `None`, `Release`, `ToggleFx`, `Dimmer`, `Color` (+467 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1104 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **111 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `engine.rs`, `runtime.rs`, `programmer.rs`, `DmxFrame`, `BankScope`, `t`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `audio.rs`, `mapping/mod.rs`, `Mapper`, `.object_props`, `window.rs`, `stage.rs`, `Host`, `Show`, `View`, `apply`, `app.rs`, `profile_builder.rs`, `scene_grid.rs`, `ProfileBuilder`, `hashmap`, `Rect`?**
  _High betweenness centrality (0.141) - this node is a cross-community bridge._
- **Are the 144 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 144 INFERRED edges - model-reasoned connections that need verification._
- **What connects `None`, `Release`, `ToggleFx` to the rest of the system?**
  _472 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13548387096774195 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `library_panel.rs`, `theme.rs`, `programmer.rs`, `i18n/mod.rs`, `viz/library.rs`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `.viewport`, `fx_color.rs`, `VizView`, `Ui`, `FixtureAction`, `textures.rs`, `Fader`, `stage_editor.rs`, `DmxCraftApp`, `settings_window.rs`, `Dialog<'a>`, `Settings`, `color_picker.rs`, `PaletteStore`, `mapping/mod.rs`, `.row`, `.object_props`, `window.rs`, `stage.rs`, `View`, `fx_ui.rs`, `egui`, `dialog.rs`, `apply`, `app.rs`, `Lang`, `profile_builder.rs`, `viz/mod.rs`, `scene_grid.rs`, `ProfileBuilder`, `file_dialog.rs`, `profiles.rs`, `Rect`, `outline.rs`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Show` (e.g. with `Mappings` and `Venue`) actually correct?**
  _`Show` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11692307692307692 - nodes in this community are weakly interconnected._