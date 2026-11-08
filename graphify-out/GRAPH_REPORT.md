# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 124 files · ~163,026 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3210 nodes · 7813 edges · 229 communities (126 shown, 103 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 410 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `73fb22d5`
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
- dip_switch.rs
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
- .load
- MapPreset
- scene_card.rs
- Layer
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
- core/src/lib.rs
- BankScope
- text_field
- stage_canvas.rs
- Drag
- control/timeline.rs
- plan.rs
- MenuItem<'a>
- server.rs
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
- TexAction
- Venue
- Vec2
- Arrange
- src/mapping.rs
- super
- tools.rs
- dip.rs
- PlanShape
- ChannelValue
- .fade_state
- Theme
- audio.rs
- midi.rs
- Settings
- color_picker.rs
- Control
- mapping/mod.rs
- .feedback
- qlc-bundle.rs
- ShowError
- arc
- VizView
- Timeline
- FooterButton<'a>
- Tastatur- und MIDI-Zuordnung

## God Nodes (most connected - your core abstractions)
1. `Engine` - 186 edges
2. `t()` - 174 edges
3. `Show` - 129 edges
4. `Icon` - 105 edges
5. `AppState` - 75 edges
6. `ChannelKind` - 70 edges
7. `ViewCtx` - 61 edges
8. `FixtureType` - 45 edges
9. `ChannelRef` - 44 edges
10. `DmxFrame` - 43 edges

## Surprising Connections (you probably didn't know these)
- `palette_button()` --calls--> `color32()`  [INFERRED]
  crates/app/src/mapping/window.rs → crates/app/src/midi_device.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (229 total, 103 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.22
Nodes (31): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+23 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.12
Nodes (16): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+8 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.11
Nodes (26): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+18 more)

### Community 5 - "programmer.rs"
Cohesion: 0.12
Nodes (34): button(), Category, All, Beam, Color, Dimmer, Gobo, .ITEMS (+26 more)

### Community 6 - "AppState"
Cohesion: 0.12
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (23): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+15 more)

### Community 9 - "OutputError"
Cohesion: 0.09
Nodes (9): DmxFrame, NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented (+1 more)

### Community 10 - "ChannelDef"
Cohesion: 0.10
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (4): Engine, lerp(), PlayingScene, ChannelRef

### Community 12 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

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
Cohesion: 0.15
Nodes (23): clip(), delete(), HEIGHT, k(), levels(), load_meter(), lock(), main_menu() (+15 more)

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
Cohesion: 0.11
Nodes (10): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneKind, Fx, Static, Super (+2 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (52): Step, Enter, Maximize, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+44 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "menu.rs"
Cohesion: 0.14
Nodes (14): body(), caption(), heading_font(), begin(), context(), dropdown(), ICON_SLOT, IconFn (+6 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "dip_switch.rs"
Cohesion: 0.17
Nodes (14): accent(), accent_dim(), apply(), bg(), border(), install_fonts(), raised(), raised_hover() (+6 more)

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
Cohesion: 0.09
Nodes (32): kind_color(), card_row(), FileJob, Export, Import, full_combo(), list_header(), list_row() (+24 more)

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
Cohesion: 0.06
Nodes (33): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+25 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.09
Nodes (26): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+18 more)

### Community 44 - "Fader"
Cohesion: 0.11
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.12
Nodes (25): ai(), full_combo(), general(), HEIGHT, hint(), midi(), outputs(), project() (+17 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "ClipAction"
Cohesion: 0.22
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 66 - ".row"
Cohesion: 0.12
Nodes (19): heading_text(), drag_row(), drop_marker(), Renaming, Folder, Object, Tree, TreeAction (+11 more)

### Community 68 - ".object_props"
Cohesion: 0.16
Nodes (6): fold_section(), ViewCtx, proposals_for_easy_view_names(), propose_type(), vec3_row(), VizView

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (19): Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.19
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Host"
Cohesion: 0.16
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+6 more)

### Community 72 - "Show"
Cohesion: 0.19
Nodes (7): Fixture, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType, Show

### Community 73 - "feature.rs"
Cohesion: 0.15
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "patch_grid.rs"
Cohesion: 0.12
Nodes (17): View, body(), caption(), FixtureSettings, range_row(), SetupView, CELL_H_MAX, CELL_H_MIN (+9 more)

### Community 78 - "fx_ui.rs"
Cohesion: 0.11
Nodes (32): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+24 more)

### Community 79 - "egui"
Cohesion: 0.07
Nodes (26): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, ICON_FAMILY (+18 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.07
Nodes (16): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, form() (+8 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (26): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+18 more)

### Community 90 - "app.rs"
Cohesion: 0.27
Nodes (5): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, viewport_input()

### Community 92 - "profile_builder.rs"
Cohesion: 0.12
Nodes (26): t(), capability_fields(), category_label(), check(), color_label(), FOOT_H, general(), hex_field() (+18 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.19
Nodes (15): AXIS_COLORS, Handles, handles(), hit_y(), model_tex_id(), project(), QualityParams, ray_aabb_exit() (+7 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.08
Nodes (48): ControlViews, ProgrammerView, Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+40 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.18
Nodes (7): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W, COLUMN_W

### Community 101 - "Layer"
Cohesion: 0.10
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "profiles.rs"
Cohesion: 0.13
Nodes (12): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+4 more)

### Community 109 - "Rect"
Cohesion: 0.25
Nodes (6): add_audio(), add_audio_dialog(), fmt_ms(), grid_options(), paint_clip(), TimelineView

### Community 110 - "DeviceModel"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 118 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.14
Nodes (13): deg_quat(), euler_matches_core_rotation(), quat_deg(), DragFixtures, DragObjects, FixtureSort, Address, Name (+5 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "model.rs"
Cohesion: 0.11
Nodes (9): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Model, ModelPart, part_roundtrip_through_json() (+1 more)

### Community 128 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "core/src/lib.rs"
Cohesion: 0.18
Nodes (8): UNIVERSE_SIZE, SceneProgress, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, UNIVERSES

### Community 137 - "BankScope"
Cohesion: 0.19
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 140 - "text_field"
Cohesion: 0.20
Nodes (3): text_field(), ZonesView, Windows

### Community 141 - "stage_canvas.rs"
Cohesion: 0.11
Nodes (24): AMBER, BOX, Camera, CanvasMode, Edit, Select, fit(), fixture_pos() (+16 more)

### Community 143 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 146 - "control/timeline.rs"
Cohesion: 0.11
Nodes (19): AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete, Duplicate, ExpandAll (+11 more)

### Community 147 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 150 - "server.rs"
Cohesion: 0.12
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 152 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 158 - "Step"
Cohesion: 0.25
Nodes (3): Fullscreen, SETTLE, mark_clipboard()

### Community 159 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 160 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 161 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 162 - "VizView"
Cohesion: 0.18
Nodes (14): aim_selected(), apply_placement(), decode_texture(), GizmoDrag, move_all(), Pick, Fixture, Object (+6 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 168 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 169 - "TexAction"
Cohesion: 0.50
Nodes (4): TexAction, Builtin, Load, Set

### Community 170 - "Venue"
Cohesion: 0.16
Nodes (6): renumber(), fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), Venue

### Community 172 - "Arrange"
Cohesion: 0.33
Nodes (6): Arrange, AlignH, AlignV, Grid, Reset, Row

### Community 173 - "src/mapping.rs"
Cohesion: 0.15
Nodes (17): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, Feedback, FeedbackState (+9 more)

### Community 174 - "super"
Cohesion: 0.15
Nodes (16): digest(), hash_and_verify(), hash_password(), hex(), verify(), addresses_are_one_based(), call_tool(), error() (+8 more)

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (55): add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), boolean(), build_steps(), call() (+47 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 180 - "ChannelValue"
Cohesion: 0.20
Nodes (7): cv(), demo_show(), empty_show(), ensure_builtin_types(), programmer_values(), ChannelValue, Step

### Community 181 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 183 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 184 - "audio.rs"
Cohesion: 0.14
Nodes (10): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+2 more)

### Community 185 - "midi.rs"
Cohesion: 0.14
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 186 - "Settings"
Cohesion: 0.13
Nodes (5): MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 187 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 190 - "Control"
Cohesion: 0.11
Nodes (18): Overlay, Control, BankStep, Blackout, Channel, Fixture, Master, Scene (+10 more)

### Community 192 - "mapping/mod.rs"
Cohesion: 0.11
Nodes (29): bank_step_wraps(), button_rect(), channel_mapping_sets_live_value(), channel_name(), color(), default_feedback(), describe(), describe_mapping() (+21 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 207 - "VizView"
Cohesion: 0.12
Nodes (14): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, Missing, Mode (+6 more)

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (14): Clip, ClipContent, Audio, Scene, fades(), GridMode, Beat, Time (+6 more)

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **473 isolated node(s):** `New`, `Keyboard`, `Midi`, `Scene`, `ShowBank` (+468 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1104 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **103 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `engine.rs`, `runtime.rs`, `programmer.rs`, `core/src/lib.rs`, `OutputError`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `ChannelValue`, `.fade_state`, `audio.rs`, `mapping/mod.rs`, `.feedback`, `.object_props`, `window.rs`, `stage.rs`, `Host`, `Show`, `app.rs`, `profile_builder.rs`, `scene_grid.rs`, `scene_card.rs`, `Layer`, `Rect`?**
  _High betweenness centrality (0.149) - this node is a cross-community bridge._
- **Are the 143 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 143 INFERRED edges - model-reasoned connections that need verification._
- **What connects `New`, `Keyboard`, `Midi` to the rest of the system?**
  _473 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11764705882352941 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `profile_builder.rs` to `library_panel.rs`, `programmer.rs`, `core/src/lib.rs`, `i18n/mod.rs`, `stage_canvas.rs`, `text_field`, `viz/library.rs`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `dip_switch.rs`, `VizView`, `Ui`, `FixtureAction`, `textures.rs`, `Fader`, `DmxCraftApp`, `settings_window.rs`, `Settings`, `color_picker.rs`, `mapping/mod.rs`, `.row`, `.object_props`, `window.rs`, `stage.rs`, `patch_grid.rs`, `fx_ui.rs`, `egui`, `dialog.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `.load`, `scene_card.rs`, `file_dialog.rs`, `profiles.rs`, `Rect`, `outline.rs`?**
  _High betweenness centrality (0.128) - this node is a cross-community bridge._
- **Are the 2 inferred relationships involving `Show` (e.g. with `Mappings` and `Venue`) actually correct?**
  _`Show` has 2 INFERRED edges - model-reasoned connections that need verification._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._