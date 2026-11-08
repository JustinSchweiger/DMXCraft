# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 118 files · ~152,053 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 16 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3019 nodes · 7309 edges · 229 communities (121 shown, 108 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 375 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `d0a214ec`
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
- audio.rs
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
- .modes_tab
- mapping.rs
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
- MenuAction
- ClipAction
- core/src/lib.rs
- .row
- t
- gltf_import.rs
- stage.rs
- Profile
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
- Ui
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
- ChaserPreset
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
- body
- protocol.rs
- TexAction
- Problem
- GridMode
- Vec2
- Shape
- lock.rs
- tools.rs
- dip.rs
- PlanShape
- ChannelValue
- .default
- Theme
- widgets/mod.rs
- qlc-bundle.rs
- ShowError
- Capability
- arc
- DialogKind
- panels.rs
- FixtureLimits
- plan
- Timeline
- FooterButton<'a>
- T

## God Nodes (most connected - your core abstractions)
1. `Engine` - 172 edges
2. `t()` - 161 edges
3. `Show` - 121 edges
4. `Icon` - 104 edges
5. `ChannelKind` - 71 edges
6. `ViewCtx` - 61 edges
7. `AppState` - 57 edges
8. `FixtureType` - 45 edges
9. `DmxFrame` - 43 edges
10. `ChannelRef` - 42 edges

## Surprising Connections (you probably didn't know these)
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `ai()` --references--> `McpStatus`  [INFERRED]
  crates/app/src/settings_window.rs → crates/app/src/mcp.rs

## Import Cycles
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (229 total, 108 thin omitted)

### Community 0 - "engine.rs"
Cohesion: 0.22
Nodes (31): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+23 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.11
Nodes (17): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+9 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "theme.rs"
Cohesion: 0.10
Nodes (25): accent_hover(), c(), DANGER, DIP_BODY, fixture_bar(), FS_BODY, FS_CAPTION, FS_HEADING (+17 more)

### Community 5 - "programmer.rs"
Cohesion: 0.09
Nodes (39): ProgrammerMode, Blind, Edit, Live, Category, All, Beam, Color (+31 more)

### Community 6 - "AppState"
Cohesion: 0.14
Nodes (10): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+2 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.17
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), ColorFx, feature_levels(), grayscale_has_equal_channels() (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.07
Nodes (13): DmxFrame, NullOutput, RecordingOutput, DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS, DmxOutput (+5 more)

### Community 10 - "ChannelDef"
Cohesion: 0.10
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "audio.rs"
Cohesion: 0.06
Nodes (24): AudioPlayer, duration_ms(), read_wave(), wav(), Waveform, waveform_and_duration(), WaveState, Failed (+16 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.14
Nodes (15): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+7 more)

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

### Community 23 - ".add_scene"
Cohesion: 0.16
Nodes (5): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), SceneSettings, steps_insert_remove_and_normalize()

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.08
Nodes (42): Show, advanced(), banks_of(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping() (+34 more)

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

### Community 31 - "pal"
Cohesion: 0.22
Nodes (13): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), pal() (+5 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.08
Nodes (20): renumber(), aim_at_points_beam_to_target(), apply(), CameraPreset, close(), Environment, fixture_folders_are_exclusive(), FixtureFolder (+12 more)

### Community 36 - ".modes_tab"
Cohesion: 0.18
Nodes (14): list_header(), ListAction, Add, Duplicate, Remove, ModeOp, Add, Down (+6 more)

### Community 37 - "mapping.rs"
Cohesion: 0.18
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

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
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "fader.rs"
Cohesion: 0.07
Nodes (20): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+12 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.19
Nodes (3): DmxCraftApp, fill_types(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.05
Nodes (33): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings, ai(), full_combo(), general(), HEIGHT (+25 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "ClipAction"
Cohesion: 0.25
Nodes (8): ClipAction, Copy, Cut, Delete, Duplicate, EditScene, ResetSpeed, Split

### Community 65 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): UNIVERSE_SIZE, Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 66 - ".row"
Cohesion: 0.09
Nodes (28): drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name, move_objects() (+20 more)

### Community 68 - "t"
Cohesion: 0.13
Nodes (9): t(), fold_section(), section_header(), ViewCtx, ZonesView, Windows, save_user(), vec3_row() (+1 more)

### Community 69 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (15): ControlViews, ProgrammerView, fx_button(), group_pill(), GroupAction, AddSelection, Delete, New (+7 more)

### Community 71 - "Profile"
Cohesion: 0.19
Nodes (4): import_real_ofl_with_16bit_ranges(), ofl_round_trip(), Profile, rename_and_resolution_update_modes()

### Community 72 - "Show"
Cohesion: 0.19
Nodes (3): Fixture, Show, Zone

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (14): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+6 more)

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
Cohesion: 0.10
Nodes (33): builtin(), load_user(), named(), NamedPalette, path(), save_user(), color_fx_ui(), edit_colors() (+25 more)

### Community 79 - "egui"
Cohesion: 0.11
Nodes (16): SIZE, SWEEP, Grip, Body, Corner, EdgeX, EdgeY, LimitPad (+8 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.12
Nodes (11): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+3 more)

### Community 89 - "profile.rs"
Cohesion: 0.11
Nodes (17): CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), INTENSITY_COLORS, KEY_PREFIX (+9 more)

### Community 90 - "app.rs"
Cohesion: 0.18
Nodes (10): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, UpdateBadge, Available, Busy, Error (+2 more)

### Community 92 - "profile_builder.rs"
Cohesion: 0.09
Nodes (24): capability_fields(), category_label(), color_label(), FOOT_H, hex_field(), kind_label(), LIST_W, MAX_FORM_W (+16 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.10
Nodes (24): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), Handles, handles(), hit_y(), Missing (+16 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.06
Nodes (63): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+55 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.17
Nodes (8): FileJob, Export, Import, import(), new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "Ui"
Cohesion: 0.25
Nodes (12): SceneCardResponse, PillResult, check(), full_combo(), general(), hint(), list_row(), manufacturer_picker() (+4 more)

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
Cohesion: 0.23
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 109 - "Rect"
Cohesion: 0.19
Nodes (7): add_audio(), add_audio_dialog(), fmt_ms(), grid_options(), paint_clip(), TimelineView, Timeline

### Community 110 - "quick.rs"
Cohesion: 0.32
Nodes (5): Quick, quick_action(), Center, Off, On

### Community 113 - "ev2.rs"
Cohesion: 0.19
Nodes (13): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+5 more)

### Community 118 - "Host"
Cohesion: 0.13
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+6 more)

### Community 121 - "convert.rs"
Cohesion: 0.12
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "model.rs"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 128 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (17): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+9 more)

### Community 136 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 137 - "View"
Cohesion: 0.13
Nodes (15): View, body(), caption(), FixtureSettings, range_row(), SetupView, Arrange, AlignH (+7 more)

### Community 140 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (20): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+12 more)

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

### Community 152 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 153 - "IconButton"
Cohesion: 0.14
Nodes (3): ICON_FAMILY, ICON_FILL_FAMILY, IconButton

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.12
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.22
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 158 - "Step"
Cohesion: 0.20
Nodes (6): Fullscreen, SETTLE, Step, Enter, Maximize, mark_clipboard()

### Community 160 - "body"
Cohesion: 0.40
Nodes (4): body(), heading_font(), heading_text(), title()

### Community 161 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 162 - "TexAction"
Cohesion: 0.50
Nodes (4): TexAction, Builtin, Load, Set

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 170 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 172 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 174 - "lock.rs"
Cohesion: 0.24
Nodes (6): digest(), hash_and_verify(), hash_password(), hex(), verify(), addresses_are_one_based()

### Community 176 - "tools.rs"
Cohesion: 0.08
Nodes (66): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+58 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 180 - "ChannelValue"
Cohesion: 0.22
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 181 - ".default"
Cohesion: 0.29
Nodes (5): add_range_until_every_value_is_used(), normalize_repairs_gaps(), ProfileChannel, ranges(), ranges_always_cover_0_to_255()

### Community 183 - "Theme"
Cohesion: 0.25
Nodes (6): active(), set_theme(), Theme, .ALL, Gray, Navy

### Community 196 - "widgets/mod.rs"
Cohesion: 0.15
Nodes (8): DipSwitch, clip(), SceneCard, CanvasMode, Edit, Select, Swatch, XyPad

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 203 - "Capability"
Cohesion: 0.25
Nodes (3): cap_for_kind(), Capability, last_matching()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 207 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 210 - "panels.rs"
Cohesion: 0.17
Nodes (16): aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object, rotate_all() (+8 more)

### Community 217 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 224 - "Timeline"
Cohesion: 0.16
Nodes (10): Clip, ClipContent, Audio, Scene, fades(), one(), paste_finds_free_place_and_keeps_gaps(), split_and_length() (+2 more)

## Knowledge Gaps
- **448 isolated node(s):** `Off`, `Error`, `General`, `Outputs`, `Midi` (+443 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1054 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **108 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `t()` connect `t` to `library_panel.rs`, `programmer.rs`, `View`, `audio.rs`, `viz/library.rs`, `control/timeline.rs`, `titlebar.rs`, `scene_props.rs`, `pal`, `.modes_tab`, `FixtureAction`, `textures.rs`, `fader.rs`, `DmxCraftApp`, `settings_window.rs`, `.row`, `stage.rs`, `patch_grid.rs`, `fx_ui.rs`, `egui`, `panels.rs`, `dialog.rs`, `app.rs`, `profile_builder.rs`, `viz/mod.rs`, `scene_grid.rs`, `ProfileBuilder`, `Ui`, `file_dialog.rs`, `profiles.rs`, `Rect`?**
  _High betweenness centrality (0.127) - this node is a cross-community bridge._
- **Are the 131 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 131 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Error`, `General` to the rest of the system?**
  _448 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1126984126984127 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `Snapshot`, `engine.rs`, `library_panel.rs`, `runtime.rs`, `theme.rs`, `fx.rs`, `Engine`, `stage_canvas.rs`, `.add_scene`, `rig.rs`, `.check_placement`, `Venue`, `FixtureAction`, `DmxCraftApp`, `tools.rs`, `ChannelValue`, `core/src/lib.rs`, `feature.rs`, `ShowError`, `motion.rs`, `panels.rs`, `show.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `Scene`?**
  _High betweenness centrality (0.106) - this node is a cross-community bridge._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `Snapshot`, `library_panel.rs`, `engine.rs`, `runtime.rs`, `programmer.rs`, `audio.rs`, `control/timeline.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `core/src/lib.rs`, `t`, `stage.rs`, `Show`, `app.rs`, `profile_builder.rs`, `scene_grid.rs`, `ProfileBuilder`, `Rect`, `quick.rs`, `Host`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._