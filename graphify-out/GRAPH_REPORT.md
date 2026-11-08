# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 131 files · ~187,487 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3581 nodes · 9047 edges · 253 communities (151 shown, 102 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 467 edges (avg confidence: 0.81)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `ed63b42f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- paint_item
- library_panel.rs
- runtime.rs
- open_dmx.rs
- Theme
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
- ListAction
- rig.rs
- scene_props.rs
- host
- core/src/audio.rs
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
- remote/mod.rs
- TouchLayout
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
- View
- engine.rs
- egui
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- app.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- ProfileBuilder
- MapPreset
- scene_card.rs
- super
- qxf.rs
- file_dialog.rs
- en.rs
- panels.rs
- Rect
- super
- server.rs
- ev2.rs
- call
- EffectKind
- outline.rs
- convert.rs
- Snapshot
- gltf_import.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- ChannelDef
- theme.rs
- stage_canvas.rs
- bank_menu
- core/src/lib.rs
- flush
- paintItem
- panels.rs
- MenuItem<'a>
- plan.rs
- DMXCraft
- target_shape
- mcp/src/venue.rs
- .show
- ShutterEffect
- .viewport
- Scene
- protocol.rs
- HeaderAction
- color_picker.rs
- Problem
- DvcGoldUsb
- plan
- Scene
- Layer
- Category
- outline.rs
- fixture_menu.rs
- src/mapping.rs
- TouchKind
- tools.rs
- dip.rs
- ClipAction
- demo.rs
- menu.rs
- knob.rs
- Tab
- time_field.rs
- midi.rs
- Drag
- Touch-Oberfläche
- profiles.rs
- views/touch.rs
- mapping/mod.rs
- widgets/mod.rs
- qlc-bundle.rs
- Fullscreen
- Input
- ShowError
- ChaserPreset
- arc
- icon_library.rs
- IconButton
- T
- draw
- stage_editor.rs
- .end_frame
- apply
- Transform
- UpdateBadge
- palettes.rs
- Timeline
- FooterButton<'a>
- Cmd
- textOn
- swatch.rs
- GridMode
- Tastatur- und MIDI-Zuordnung
- Box<dyn serialport::SerialPort>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 201 edges
2. `t()` - 186 edges
3. `Show` - 140 edges
4. `Icon` - 107 edges
5. `AppState` - 88 edges
6. `ViewCtx` - 72 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 56 edges
9. `FixtureType` - 46 edges
10. `DmxFrame` - 43 edges

## Surprising Connections (you probably didn't know these)
- `Aufbau` --references--> `send()`  [INFERRED]
  README.md → crates/app/src/remote/web/app.js
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
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 2-file cycle: `crates/output/src/dvc_gold.rs -> crates/output/src/lib.rs -> crates/output/src/dvc_gold.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`

## Communities (253 total, 102 thin omitted)

### Community 0 - "paint_item"
Cohesion: 0.14
Nodes (15): rgb(), add_page(), content_rect(), fader_parts(), ImageTarget, Item, Page, paint_cover() (+7 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.17
Nodes (11): LibraryDrag, template(), drag_ghost(), LibAction, Builder, Delete, LibraryPanel, MAX_FIXTURES_PER_SEARCH (+3 more)

### Community 2 - "runtime.rs"
Cohesion: 0.11
Nodes (11): engine_keeps_running_without_device(), Failing, OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY (+3 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.12
Nodes (13): main(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.16
Nodes (27): FADER_W, category_list(), color_bar(), current(), elide(), fader(), fader_groups(), fader_label() (+19 more)

### Community 6 - "AppState"
Cohesion: 0.11
Nodes (11): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+3 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, NotImplemented, Transfer

### Community 10 - "Deserialize"
Cohesion: 0.12
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.12
Nodes (9): Environment, fixture_folders_are_exclusive(), FixtureFolder, hang_below_nearest_truss(), Material, Object, room_and_tree(), TrussHang (+1 more)

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
Cohesion: 0.14
Nodes (11): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+3 more)

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
Nodes (78): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+70 more)

### Community 23 - "ListAction"
Cohesion: 0.67
Nodes (3): ListAction, Add, Remove

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (23): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+15 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.09
Nodes (37): button(), Show, advanced(), bar_slider(), block_props(), effect_header(), effect_tiles(), fit_mapping() (+29 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.05
Nodes (46): analysis(), analyze_file(), AudioPlayer, cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player (+38 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.13
Nodes (25): canvas(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore (+17 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "Venue"
Cohesion: 0.14
Nodes (11): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, mul(), Placement, rotation() (+3 more)

### Community 36 - "t"
Cohesion: 0.10
Nodes (41): t(), kind_color(), capability_fields(), card_row(), category_label(), check(), color_label(), FOOT_H (+33 more)

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
Cohesion: 0.09
Nodes (26): FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate, InvertOrder (+18 more)

### Community 41 - "drag_number"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Fader"
Cohesion: 0.10
Nodes (15): DOT_H, Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange (+7 more)

### Community 45 - "render.rs"
Cohesion: 0.06
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.06
Nodes (42): DeviceModel, .ALL, ApcMiniMk2, Generic, MidiDevice, cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX (+34 more)

### Community 50 - "MenuAction"
Cohesion: 0.10
Nodes (20): MenuAction, About, CheckUpdates, DmxLevels, InstallUpdate, NewShow, OpenRecent, OpenShow (+12 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.08
Nodes (30): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+22 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "BankScope"
Cohesion: 0.20
Nodes (8): banks_of(), BankScope, All, Bank, Banks, Off, OutsideBank, SceneSettings

### Community 66 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 68 - "ViewCtx"
Cohesion: 0.22
Nodes (5): ViewCtx, proposals_for_easy_view_names(), propose_type(), vec3_row(), VizView

### Community 69 - "window.rs"
Cohesion: 0.16
Nodes (22): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+14 more)

### Community 70 - "stage.rs"
Cohesion: 0.15
Nodes (17): ControlViews, title(), ProgrammerView, fx_button(), group_pill(), GroupAction, AddSelection, Delete (+9 more)

### Community 71 - "Host"
Cohesion: 0.15
Nodes (14): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, AppEvent (+6 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (10): .VIRTUAL_DIMMER, Fixture, Group, paste_fixtures_finds_free_addresses_and_names(), PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.15
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.29
Nodes (5): OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "View"
Cohesion: 0.11
Nodes (10): text_field(), View, body(), caption(), FixtureSettings, range_row(), SetupView, StageEditorView (+2 more)

### Community 78 - "engine.rs"
Cohesion: 0.22
Nodes (32): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), limits_cap_dimmer_and_position() (+24 more)

### Community 79 - "egui"
Cohesion: 0.12
Nodes (14): HANDLE, ICON_FILL_FAMILY, Grip, Body, Corner, EdgeX, EdgeY, from_pct() (+6 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.06
Nodes (14): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+6 more)

### Community 89 - "profile.rs"
Cohesion: 0.06
Nodes (27): add_range_until_every_value_is_used(), CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names(), fixture_types_detect_kinds_and_keep_keys(), import_real_ofl_with_16bit_ranges() (+19 more)

### Community 90 - "app.rs"
Cohesion: 0.27
Nodes (5): edit_preview(), load_step(), LockDialog, OUTPUT_HZ, viewport_input()

### Community 92 - "app.js"
Cohesion: 0.12
Nodes (21): buildTabs(), clamp(), contentRect(), cv, de, fieldMove(), g, hsToRgb() (+13 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.11
Nodes (20): AXIS_COLORS, decode_texture(), deg_quat(), euler_matches_core_rotation(), Handles, handles(), Missing, Mode (+12 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.10
Nodes (38): zone_color(), select_all(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete (+30 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (32): cap_for_kind(), Capability, CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect (+24 more)

### Community 97 - "ProfileBuilder"
Cohesion: 0.12
Nodes (13): BuilderRequest, Edit, New, Open, Template, FileJob, Export, Import (+5 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "scene_card.rs"
Cohesion: 0.24
Nodes (5): CARD_H, clip(), COMPACT_H, SceneCardResponse, STRIP_W

### Community 101 - "super"
Cohesion: 0.17
Nodes (10): digest(), hash_and_verify(), hash_password(), hex(), verify(), APC_MK2_PALETTE, detect_and_match(), matching_output() (+2 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "panels.rs"
Cohesion: 0.11
Nodes (15): Segmented, DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue, renumber() (+7 more)

### Community 109 - "Rect"
Cohesion: 0.17
Nodes (8): fmt_ms(), grid_options(), has_audio(), mark_clipboard(), paint_bands(), paint_beats(), paint_clip(), TimelineView

### Community 112 - "server.rs"
Cohesion: 0.15
Nodes (10): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), http_roundtrip() (+2 more)

### Community 113 - "ev2.rs"
Cohesion: 0.12
Nodes (22): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+14 more)

### Community 117 - "call"
Cohesion: 0.25
Nodes (25): add_clips(), add_effect(), add_fixtures(), build_steps(), call(), channel_kind(), collect_values(), create_scene() (+17 more)

### Community 118 - "EffectKind"
Cohesion: 0.15
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.12
Nodes (19): drag_row(), DragObjects, drop_marker(), Renaming, Folder, Object, Tree, TreeAction (+11 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "Snapshot"
Cohesion: 0.28
Nodes (5): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip()

### Community 128 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.13
Nodes (14): add_audio(), add_audio_dialog(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, row_h() (+6 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "ChannelDef"
Cohesion: 0.11
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (58): sidebar(), accent(), accent_dim(), accent_hover(), active(), apply(), BAND_HIGH, BAND_LOW (+50 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (12): AMBER, BOX, MenuFn, paint_effects(), paint_fixture(), paint_live_corner(), paint_shape_icon(), UV (+4 more)

### Community 142 - "bank_menu"
Cohesion: 0.16
Nodes (19): begin(), context(), IconFn, item(), MenuItem, separator(), sub(), bank_menu() (+11 more)

### Community 143 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): UNIVERSE_SIZE, FixtureLimits, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx (+3 more)

### Community 144 - "flush"
Cohesion: 0.32
Nodes (12): api(), askPin(), flush(), loadLayout(), message(), poll(), pressDown(), release() (+4 more)

### Community 146 - "paintItem"
Cohesion: 0.24
Nodes (12): colorOf(), cover(), faderParts(), font(), paintItem(), panel(), rgbToHs(), rr() (+4 more)

### Community 147 - "panels.rs"
Cohesion: 0.23
Nodes (15): aim_selected(), apply_placement(), GizmoDrag, hit_y(), move_all(), Pick, Fixture, Object (+7 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 151 - "DMXCraft"
Cohesion: 0.17
Nodes (12): send(), 3D-Visualisierer, Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, KI-Assistent (MCP) (+4 more)

### Community 152 - "target_shape"
Cohesion: 0.23
Nodes (14): alt_click(), button_rect(), fader(), fader_channels(), in_rotated(), report(), rotated_quad(), t_touch_hint() (+6 more)

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - ".show"
Cohesion: 0.30
Nodes (5): Camera, fixture_rect(), live_colors(), StageInteraction, strobe_hz()

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.20
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 160 - "protocol.rs"
Cohesion: 0.36
Nodes (10): call_tool(), error(), handle(), handle_one(), host(), initialize(), initialize_and_list(), INSTRUCTIONS (+2 more)

### Community 161 - "HeaderAction"
Cohesion: 0.33
Nodes (6): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle

### Community 162 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 167 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 168 - "Scene"
Cohesion: 0.18
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Zone

### Community 169 - "Layer"
Cohesion: 0.10
Nodes (12): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+4 more)

### Community 170 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 171 - "outline.rs"
Cohesion: 0.12
Nodes (11): OrbitCamera, ray_through_center_hits_target(), DragFixtures, FixtureSort, Address, Name, move_objects(), moving_into_rotated_group_keeps_world_place() (+3 more)

### Community 172 - "fixture_menu.rs"
Cohesion: 0.17
Nodes (8): fit(), fixture_pos(), apply(), MenuPlace, Patch, Stage, StageEditor, SIZES

### Community 173 - "src/mapping.rs"
Cohesion: 0.07
Nodes (35): Overlay, MapFile, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Control (+27 more)

### Community 174 - "TouchKind"
Cohesion: 0.15
Nodes (14): state_json(), display_label(), item_target(), item_value(), mapping_of(), target_entry(), TouchItem, TouchKind (+6 more)

### Community 176 - "tools.rs"
Cohesion: 0.12
Nodes (31): any_obj(), apply_scene_fields(), boolean(), clip_schema(), find_type(), fixture_json(), full_workflow(), helpful_errors() (+23 more)

### Community 178 - "dip.rs"
Cohesion: 0.57
Nodes (6): known_addresses(), on_switches(), roundtrip(), switch_value(), SWITCHES, switches()

### Community 179 - "ClipAction"
Cohesion: 0.11
Nodes (17): audio_grid(), ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene (+9 more)

### Community 180 - "demo.rs"
Cohesion: 0.26
Nodes (6): cv(), demo_show(), empty_show(), ensure_builtin_types(), ChannelValue, Step

### Community 181 - "menu.rs"
Cohesion: 0.18
Nodes (10): c(), canvas_dim(), canvas_faint(), canvas_line(), handle(), is_light(), ICON_SLOT, ITEM_H (+2 more)

### Community 182 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 183 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 184 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 185 - "midi.rs"
Cohesion: 0.18
Nodes (5): default_feedback(), MidiEvent, MidiHub, parse(), parses_note_and_cc()

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "Touch-Oberfläche"
Cohesion: 0.40
Nodes (4): Bearbeiten, Bedienen, Elemente hinzufügen, Touch-Oberfläche

### Community 189 - "profiles.rs"
Cohesion: 0.26
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 190 - "views/touch.rs"
Cohesion: 0.11
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 192 - "mapping/mod.rs"
Cohesion: 0.16
Nodes (20): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fires_id(), fixed_channels_work_without_selection() (+12 more)

### Community 193 - "widgets/mod.rs"
Cohesion: 0.24
Nodes (5): LimitPad, SceneCard, fold_section(), section_header(), XyPad

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 199 - "Input"
Cohesion: 0.22
Nodes (8): Input, Key, Midi, MidiMessage, Cc, Note, note_name(), note_names()

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 209 - "draw"
Cohesion: 0.40
Nodes (6): draw(), hit(), image(), isHot(), page(), rectOf()

### Community 210 - "stage_editor.rs"
Cohesion: 0.14
Nodes (13): CanvasMode, Edit, Select, StageCanvas, Underlay, Arrange, AlignH, AlignV (+5 more)

### Community 216 - ".end_frame"
Cohesion: 0.13
Nodes (12): overlay_id(), Pick, pick(), pick_full(), pick_id(), Sent, states_id(), take_touch_picks() (+4 more)

### Community 217 - "apply"
Cohesion: 0.21
Nodes (10): CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode, Output, Patch (+2 more)

### Community 219 - "Transform"
Cohesion: 0.22
Nodes (8): order_coords(), Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 221 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "Timeline"
Cohesion: 0.12
Nodes (12): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Scene, fades(), one(), paste_finds_free_place_and_keeps_gaps() (+4 more)

### Community 230 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 231 - "textOn"
Cohesion: 0.50
Nodes (4): applyTheme(), dim(), hexRgb(), textOn()

### Community 237 - "GridMode"
Cohesion: 0.67
Nodes (3): GridMode, Beat, Time

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

## Knowledge Gaps
- **510 isolated node(s):** `Loading`, `OpenShow`, `SaveShowAs`, `ImportEv2`, `ImportModel` (+505 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1178 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **102 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `paint_item`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `core/src/lib.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `core/src/audio.rs`, `t`, `Layer`, `fixture_menu.rs`, `TouchKind`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `midi.rs`, `views/touch.rs`, `mapping/mod.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `Host`, `Show`, `View`, `engine.rs`, `.end_frame`, `app.rs`, `scene_grid.rs`, `ProfileBuilder`, `Rect`, `super`, `call`, `Snapshot`?**
  _High betweenness centrality (0.124) - this node is a cross-community bridge._
- **Are the 159 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 159 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Loading`, `OpenShow`, `SaveShowAs` to the rest of the system?**
  _510 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `paint_item` be split into smaller, more focused modules?**
  _Cohesion score 0.14393939393939395 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `paint_item`, `library_panel.rs`, `runtime.rs`, `fx.rs`, `Engine`, `theme.rs`, `stage_canvas.rs`, `bank_menu`, `core/src/lib.rs`, `Venue`, `panels.rs`, `rig.rs`, `.show`, `Scene`, `FixtureAction`, `Scene`, `fixture_menu.rs`, `src/mapping.rs`, `TouchKind`, `DmxCraftApp`, `tools.rs`, `demo.rs`, `TouchLayout`, `views/touch.rs`, `mapping/mod.rs`, `window.rs`, `feature.rs`, `ShowError`, `motion.rs`, `engine.rs`, `stage_editor.rs`, `show.rs`, `.end_frame`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `call`, `Snapshot`?**
  _High betweenness centrality (0.124) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1111111111111111 - nodes in this community are weakly interconnected._