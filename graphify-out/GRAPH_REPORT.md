# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 124 files · ~164,939 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3228 nodes · 7881 edges · 243 communities (126 shown, 117 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 411 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `69aec93a`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- VizView
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
- Serialize
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
- audio.rs
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
- text_field
- .effects_tab
- egui
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- app.rs
- Toggle<'a>
- t
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
- Theme
- Rect
- DmxFrame
- ev2.rs
- knob.rs
- EffectKind
- outline.rs
- convert.rs
- SlotKind
- gltf_import.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- Profile
- stage_canvas.rs
- BankScope
- ScenePropsView
- plan.rs
- MenuItem<'a>
- Shape
- ModelPart
- IconButton
- mcp/src/venue.rs
- ShutterEffect
- .viewport
- Fullscreen
- FixtureLimits
- Transform
- ChaserPreset
- VizView
- Problem
- swatch.rs
- plan
- Scene
- .of
- palettes.rs
- Vec2
- stage_editor.rs
- src/mapping.rs
- lock.rs
- tools.rs
- dip.rs
- Picker
- demo.rs
- ProfileChannel
- Tab
- .check_placement
- midi.rs
- .apply_channel_action
- color_picker.rs
- resize_handles
- mapping/mod.rs
- qlc-bundle.rs
- ShowError
- arc
- DialogKind
- apply
- Timeline
- FooterButton<'a>
- Tastatur- und MIDI-Zuordnung

## God Nodes (most connected - your core abstractions)
1. `Engine` - 188 edges
2. `t()` - 175 edges
3. `Show` - 131 edges
4. `Icon` - 105 edges
5. `AppState` - 75 edges
6. `ChannelKind` - 70 edges
7. `ViewCtx` - 61 edges
8. `ChannelRef` - 50 edges
9. `FixtureType` - 46 edges
10. `DmxFrame` - 43 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `mapping_handles()` --references--> `Camera`  [INFERRED]
  crates/app/src/views/control/stage.rs → crates/app/src/ui/widgets/stage_canvas.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (243 total, 117 thin omitted)

### Community 0 - "VizView"
Cohesion: 0.10
Nodes (12): decode_texture(), hit_y(), Missing, Mode, Build, Live, model_tex_id(), QualityParams (+4 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (11): LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH (+3 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.09
Nodes (43): accent(), accent_dim(), accent_hover(), apply(), bg(), body(), border(), c() (+35 more)

### Community 5 - "programmer.rs"
Cohesion: 0.07
Nodes (51): button(), ProgrammerMode, Blind, Edit, Live, Category, All, Beam (+43 more)

### Community 6 - "AppState"
Cohesion: 0.13
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (27): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), fbm(), feature_levels(), grayscale_has_equal_channels() (+19 more)

### Community 9 - "OutputError"
Cohesion: 0.08
Nodes (12): NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput, OutputConfig (+4 more)

### Community 10 - "Deserialize"
Cohesion: 0.13
Nodes (12): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+4 more)

### Community 11 - "Engine"
Cohesion: 0.05
Nodes (45): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv() (+37 more)

### Community 12 - "Venue"
Cohesion: 0.16
Nodes (6): renumber(), fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

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

### Community 23 - "Serialize"
Cohesion: 0.11
Nodes (7): RECENT_MAX, Physics, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.15
Nodes (12): FX_EXT, FxDialog, FxFile, import_fx(), pick_tile(), PickColumn, picker_window(), remove_effect() (+4 more)

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
Cohesion: 0.11
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.12
Nodes (28): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+20 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.11
Nodes (14): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, hang_below_nearest_truss(), IDENTITY, Material (+6 more)

### Community 36 - "Ui"
Cohesion: 0.16
Nodes (22): form(), check(), full_combo(), general(), hint(), list_header(), list_row(), Duplicate (+14 more)

### Community 37 - "mapping.rs"
Cohesion: 0.21
Nodes (9): every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth(), text_level(), transform() (+1 more)

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
Cohesion: 0.09
Nodes (19): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+11 more)

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
Cohesion: 0.06
Nodes (39): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+31 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "audio.rs"
Cohesion: 0.14
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 65 - "BankScope"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize(), Zone

### Community 66 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 68 - "ViewCtx"
Cohesion: 0.30
Nodes (5): fold_section(), ViewCtx, vec3_row(), apply_texture(), VizView

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (25): color(), Overlay, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+17 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Host"
Cohesion: 0.13
Nodes (13): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+5 more)

### Community 72 - "Show"
Cohesion: 0.20
Nodes (3): .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (14): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+6 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "text_field"
Cohesion: 0.22
Nodes (3): text_field(), ZonesView, Windows

### Community 78 - ".effects_tab"
Cohesion: 0.26
Nodes (11): advanced(), bar_slider(), block_props(), effect_header(), effect_tiles(), header(), properties(), row() (+3 more)

### Community 79 - "egui"
Cohesion: 0.07
Nodes (18): ICON_FAMILY, ICON_FILL_FAMILY, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+10 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.15
Nodes (22): address_map_marks_ranges(), Group, groups_follow_fixture_removal(), json_roundtrip(), LoopMode, Loop, Once, id_type! (+14 more)

### Community 87 - "dialog.rs"
Cohesion: 0.07
Nodes (14): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+6 more)

### Community 89 - "profile.rs"
Cohesion: 0.09
Nodes (19): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges(), INTENSITY_COLORS, KEY_PREFIX (+11 more)

### Community 90 - "app.rs"
Cohesion: 0.18
Nodes (10): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, UpdateBadge, Available, Busy, Error (+2 more)

### Community 92 - "t"
Cohesion: 0.11
Nodes (20): t(), capability_fields(), category_label(), color_label(), FOOT_H, hex_field(), kind_label(), LIST_W (+12 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.16
Nodes (18): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), Handles, handles(), project(), proposals_for_easy_view_names(), propose_type() (+10 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.07
Nodes (51): select_all(), CARD_H, COMPACT_H, programmer_values(), Action, Bank, Compact, CreateSuper (+43 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.15
Nodes (10): BuilderRequest, Edit, New, Open, Template, FileJob, Export, Import (+2 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.24
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

### Community 108 - "Theme"
Cohesion: 0.22
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 109 - "Rect"
Cohesion: 0.16
Nodes (17): button_rect(), fader(), fader_channels(), in_rotated(), report(), rotated_quad(), target(), target_rotated() (+9 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 118 - "EffectKind"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (25): drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural(), natural_order() (+17 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "SlotKind"
Cohesion: 0.21
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 128 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.16
Nodes (11): ColorFx, ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow (+3 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.06
Nodes (32): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, ClipAction, Copy, Cut, Delete, Duplicate (+24 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 140 - "Profile"
Cohesion: 0.17
Nodes (11): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+3 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 143 - "BankScope"
Cohesion: 0.20
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 146 - "ScenePropsView"
Cohesion: 0.22
Nodes (7): ControlViews, ProgrammerView, ScenePropsView, Tab, Effects, Properties, Third

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 160 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 161 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 162 - "VizView"
Cohesion: 0.13
Nodes (19): aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object, sel_frame() (+11 more)

### Community 165 - "Problem"
Cohesion: 0.12
Nodes (20): fine_names(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+12 more)

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 169 - ".of"
Cohesion: 0.33
Nodes (3): Show, fit_mapping(), FxPreview

### Community 170 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 172 - "stage_editor.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+5 more)

### Community 173 - "src/mapping.rs"
Cohesion: 0.06
Nodes (37): add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control, BankStep, Blackout (+29 more)

### Community 174 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 176 - "tools.rs"
Cohesion: 0.06
Nodes (81): ChannelValue, Step, dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run() (+73 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "Picker"
Cohesion: 0.40
Nodes (5): Picker, PickTarget, Jump, Protect, Release

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "ProfileChannel"
Cohesion: 0.27
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 183 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 184 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 185 - "midi.rs"
Cohesion: 0.29
Nodes (4): MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 186 - ".apply_channel_action"
Cohesion: 0.40
Nodes (4): ListAction, Add, Remove, unique()

### Community 187 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 192 - "mapping/mod.rs"
Cohesion: 0.10
Nodes (29): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), default_feedback(), describe(), describe_mapping(), fire(), fires_id() (+21 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
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
Cohesion: 0.11
Nodes (15): apply(), View, SetupView, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag (+7 more)

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **473 isolated node(s):** `Select`, `Edit`, `New`, `Row`, `Grid` (+468 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1112 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **117 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `Engine`, `Venue`, `stage_canvas.rs`, `rig.rs`, `VizView`, `FixtureAction`, `Scene`, `stage_editor.rs`, `src/mapping.rs`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `.check_placement`, `mapping/mod.rs`, `BankScope`, `window.rs`, `stage.rs`, `feature.rs`, `ShowError`, `motion.rs`, `show.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`?**
  _High betweenness centrality (0.145) - this node is a cross-community bridge._
- **Are the 144 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 144 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Select`, `Edit`, `New` to the rest of the system?**
  _473 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `VizView` be split into smaller, more focused modules?**
  _Cohesion score 0.10344827586206896 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `Ui`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `audio.rs`, `mapping/mod.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `Host`, `Show`, `.effects_tab`, `show.rs`, `apply`, `app.rs`, `t`, `scene_grid.rs`, `Timeline`, `ProfileBuilder`, `Rect`, `DmxFrame`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Show` (e.g. with `Mappings` and `Venue`) actually correct?**
  _`Show` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11692307692307692 - nodes in this community are weakly interconnected._