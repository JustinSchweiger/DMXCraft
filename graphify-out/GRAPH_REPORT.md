# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 137 files · ~203,106 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3808 nodes · 9527 edges · 257 communities (155 shown, 102 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 498 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `309f18b4`
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
- audio_triggers.rs
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
- app/src/audio.rs
- t
- .push
- DmxCraft – UI-Richtlinien
- UpdateState
- FixtureAction
- drag.rs
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
- Settings
- ViewCtx
- window.rs
- stage.rs
- missing_audio.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- Profile
- engine.rs
- sense
- OpenDmx
- show.rs
- dialog.rs
- profile.rs
- toolbar.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- Ui
- MapPreset
- audio_trigger_ui.rs
- fx/mapping.rs
- qxf.rs
- file_dialog.rs
- en.rs
- DialogKind
- Rect
- super
- server.rs
- ev2.rs
- Tab
- DeviceModel
- outline.rs
- convert.rs
- VizView
- View
- CurveShape
- ColorPreset
- control/timeline.rs
- super
- ChannelDef
- theme.rs
- stage_canvas.rs
- AudioGrid
- Action
- flush
- paintItem
- scene_card.rs
- menu.rs
- plan.rs
- DMXCraft
- mapping/mod.rs
- Dialog<'a>
- mcp/src/venue.rs
- views/touch.rs
- ShutterEffect
- .viewport
- Scene
- time_field.rs
- FixtureLimits
- color_picker.rs
- Problem
- Physics
- Profile
- gltf_import.rs
- fixture_settings.rs
- panels.rs
- assist.rs
- src/mapping.rs
- TouchKind
- tools.rs
- EffectKind
- core/src/venue.rs
- ClipAction
- demo.rs
- bank_menu
- knob.rs
- app.rs
- .show
- Drag
- Touch-Oberfläche
- ProfileBuilder
- StageInteraction
- fire
- Tab
- Segmented<'a, T>
- widgets/mod.rs
- qlc-bundle.rs
- analyze
- DvcGoldUsb
- ShowError
- trigger_cx
- Transform
- arc
- swatch.rs
- IconButton
- draw
- widgets/mod.rs
- PlanShape
- Shape
- OrbitCamera
- Category
- SlotKind
- Fullscreen
- UpdateBadge
- palettes.rs
- Timeline
- FooterButton<'a>
- Grip
- textOn
- PickTarget
- plan
- Tab
- .pending_grid
- resize_handles
- Tastatur- und MIDI-Zuordnung
- AudioAnalysis
- AudioPlayer

## God Nodes (most connected - your core abstractions)
1. `Engine` - 218 edges
2. `t()` - 196 edges
3. `Show` - 144 edges
4. `Icon` - 113 edges
5. `AppState` - 100 edges
6. `ViewCtx` - 82 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 57 edges
9. `Profile` - 53 edges
10. `FixtureType` - 50 edges

## Surprising Connections (you probably didn't know these)
- `Aufbau` --references--> `send()`  [INFERRED]
  README.md → crates/app/src/remote/web/app.js
- `block_props()` --references--> `Timeline`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/state.rs
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

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

## Communities (257 total, 102 thin omitted)

### Community 0 - "paint_item"
Cohesion: 0.09
Nodes (25): section_header(), add_page(), add_tile(), COLORS, Drag, Move, Resize, GAP (+17 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.13
Nodes (15): LibraryDrag, BuilderRequest, Edit, New, Open, Template, template(), drag_ghost() (+7 more)

### Community 2 - "runtime.rs"
Cohesion: 0.14
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (13): active(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.10
Nodes (34): ProgrammerMode, Blind, Edit, Live, category_list(), color_bar(), current(), elide() (+26 more)

### Community 6 - "AppState"
Cohesion: 0.10
Nodes (14): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+6 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.18
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "Deserialize"
Cohesion: 0.11
Nodes (7): B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart, part_roundtrip_through_json()

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.15
Nodes (6): fixture_folders_are_exclusive(), FixtureFolder, mul(), Object, room_and_tree(), Venue

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
Cohesion: 0.13
Nodes (12): Aufbau, Einschalten, KI-Assistent (MCP), Sicherheit, Verbinden, Werkzeuge, 3D-Visualisierer, Import aus Easy View 2 (+4 more)

### Community 19 - "titlebar.rs"
Cohesion: 0.15
Nodes (26): audio_control(), AudioInfo, clip(), delete(), HEIGHT, k(), levels(), load_meter() (+18 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (83): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Camera (+75 more)

### Community 23 - "audio_triggers.rs"
Cohesion: 0.14
Nodes (12): env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey, Events (+4 more)

### Community 24 - "rig.rs"
Cohesion: 0.12
Nodes (24): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), Kind (+16 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.10
Nodes (32): Step, Enter, Maximize, Show, advanced(), banks_of(), bar_slider(), block_props() (+24 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.13
Nodes (23): BPM_MAX, BPM_MIN, detect_grid(), detect_segments(), energy_curve(), ENERGY_FRAMES, event_at(), FRAME_MS (+15 more)

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

### Community 34 - "app/src/audio.rs"
Cohesion: 0.17
Nodes (11): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+3 more)

### Community 36 - "t"
Cohesion: 0.15
Nodes (24): t(), capability_fields(), category_label(), check(), color_label(), FOOT_H, general(), hex_field() (+16 more)

### Community 37 - ".push"
Cohesion: 0.18
Nodes (10): Analyzer, Biquad, Envelope, envelope_filters_band(), FilterKind, Band, High, Low (+2 more)

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.08
Nodes (28): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+20 more)

### Community 41 - "drag.rs"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.17
Nodes (18): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), load_file(), make() (+10 more)

### Community 44 - "Fader"
Cohesion: 0.10
Nodes (16): DOT_H, Fader, FADER_W, FaderEvent, Changed, None, Release, ToggleFx (+8 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.21
Nodes (22): ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint(), midi() (+14 more)

### Community 50 - "MenuAction"
Cohesion: 0.09
Nodes (23): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+15 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (32): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+24 more)

### Community 52 - "TouchLayout"
Cohesion: 0.21
Nodes (6): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 66 - "Settings"
Cohesion: 0.15
Nodes (4): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX, Settings

### Community 68 - "ViewCtx"
Cohesion: 0.37
Nodes (4): fold_section(), ViewCtx, vec3_row(), VizView

### Community 69 - "window.rs"
Cohesion: 0.11
Nodes (24): color(), Mapper, channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT (+16 more)

### Community 70 - "stage.rs"
Cohesion: 0.12
Nodes (17): ControlViews, text_field(), ProgrammerView, fx_button(), group_pill(), GroupAction, AddSelection, Delete (+9 more)

### Community 71 - "missing_audio.rs"
Cohesion: 0.07
Nodes (25): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+17 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (10): .VIRTUAL_DIMMER, Fixture, Group, PatchError, BadUniverse, OutOfRange, Overlap, UnknownType (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.10
Nodes (25): apply_levels(), block(), chaser_levels(), ChaserFx, ChaserPreset, .ALL, Fill, Inward (+17 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "Profile"
Cohesion: 0.23
Nodes (9): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+1 more)

### Community 78 - "engine.rs"
Cohesion: 0.21
Nodes (33): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), layered_start_keeps_other_scenes() (+25 more)

### Community 79 - "sense"
Cohesion: 0.09
Nodes (23): DipSwitch, SIZE, SWEEP, LimitPad, from_pct(), HANDLE, pct(), RangeSlider (+15 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 87 - "dialog.rs"
Cohesion: 0.11
Nodes (13): Button, Dialog, FOOTER_H, FooterButton, HEADER_H, INPUT_H, Kind, Cancel (+5 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (26): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fixture_types_detect_kinds_and_keep_keys() (+18 more)

### Community 90 - "toolbar.rs"
Cohesion: 0.28
Nodes (6): chip(), divider(), level(), point_chip(), point_width(), VizView

### Community 92 - "app.js"
Cohesion: 0.12
Nodes (21): buildTabs(), clamp(), contentRect(), cv, de, fieldMove(), g, hsToRgb() (+13 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.15
Nodes (18): AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), Handles, handles(), hit_y(), project(), proposals_for_easy_view_names() (+10 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (27): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+19 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.15
Nodes (16): programmer_values(), Bank, FocusBank, RenameBank, apply(), apply_bank(), arrow(), chip() (+8 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Ui"
Cohesion: 0.14
Nodes (20): kind_color(), card_row(), full_combo(), list_header(), list_row(), ListAction, Add, Duplicate (+12 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_trigger_ui.rs"
Cohesion: 0.12
Nodes (15): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+7 more)

### Community 101 - "fx/mapping.rs"
Cohesion: 0.18
Nodes (12): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+4 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 109 - "Rect"
Cohesion: 0.20
Nodes (8): fmt_ms(), grid_options(), paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers(), TimelineView

### Community 112 - "server.rs"
Cohesion: 0.13
Nodes (13): dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server(), header() (+5 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Tab"
Cohesion: 0.14
Nodes (13): SettingsWindow, sidebar(), Tab, Ai, .ALL, Audio, Customize, General (+5 more)

### Community 118 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.09
Nodes (26): renumber(), drag_row(), DragFixtures, DragObjects, drop_marker(), move_objects(), moving_into_rotated_group_keeps_world_place(), natural() (+18 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "VizView"
Cohesion: 0.10
Nodes (12): decode_texture(), Missing, Mode, Build, Live, model_tex_id(), QualityParams, upload_assets() (+4 more)

### Community 128 - "View"
Cohesion: 0.17
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (21): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction, Delete, Duplicate (+13 more)

### Community 137 - "super"
Cohesion: 0.06
Nodes (30): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), all_texts_translated(), EN, fill() (+22 more)

### Community 138 - "ChannelDef"
Cohesion: 0.07
Nodes (22): UNIVERSE_SIZE, ChannelDef, ChannelRange, FixtureType, RangeEffect, Closed, Color, Gobo (+14 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (61): accent(), accent_dim(), accent_hover(), apply(), BAND_HIGH, BAND_LOW, BAND_MID, bg() (+53 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.18
Nodes (17): AMBER, BOX, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn, paint_effects() (+9 more)

### Community 143 - "Action"
Cohesion: 0.18
Nodes (20): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+12 more)

### Community 144 - "flush"
Cohesion: 0.32
Nodes (12): api(), askPin(), flush(), loadLayout(), message(), poll(), pressDown(), release() (+4 more)

### Community 146 - "paintItem"
Cohesion: 0.24
Nodes (12): colorOf(), cover(), faderParts(), font(), paintItem(), panel(), rgbToHs(), rr() (+4 more)

### Community 147 - "scene_card.rs"
Cohesion: 0.20
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 149 - "menu.rs"
Cohesion: 0.11
Nodes (13): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+5 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 151 - "DMXCraft"
Cohesion: 0.17
Nodes (12): send(), 3D-Visualisierer, Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, KI-Assistent (MCP) (+4 more)

### Community 152 - "mapping/mod.rs"
Cohesion: 0.14
Nodes (29): alt_click(), button(), button_rect(), fader(), fader_channels(), fires_id(), in_rotated(), overlay_id() (+21 more)

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "views/touch.rs"
Cohesion: 0.18
Nodes (18): apply(), find_item(), state_json(), channels_of(), content_rect(), current_color(), current_position(), display_label() (+10 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 159 - "Scene"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "time_field.rs"
Cohesion: 0.27
Nodes (11): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+3 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.15
Nodes (19): problem_text(), Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName (+11 more)

### Community 166 - "Physics"
Cohesion: 0.40
Nodes (4): Physics, ButtonMode, Toggle, Value

### Community 167 - "Profile"
Cohesion: 0.09
Nodes (16): AppProfiles, import(), MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get() (+8 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.14
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+1 more)

### Community 170 - "panels.rs"
Cohesion: 0.15
Nodes (18): aim_selected(), apply_placement(), GizmoDrag, move_all(), Pick, Fixture, Object, rotate_all() (+10 more)

### Community 171 - "assist.rs"
Cohesion: 0.26
Nodes (7): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset

### Community 173 - "src/mapping.rs"
Cohesion: 0.06
Nodes (39): Overlay, add_find_and_json(), channels_key(), Control, AudioFade, AudioMaster, AudioMute, BankStep (+31 more)

### Community 174 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 176 - "tools.rs"
Cohesion: 0.06
Nodes (80): ChannelValue, Step, AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration (+72 more)

### Community 177 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 178 - "core/src/venue.rs"
Cohesion: 0.12
Nodes (12): aim_at_points_beam_to_target(), apply(), close(), Environment, hang_below_nearest_truss(), IDENTITY, Material, rotation() (+4 more)

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 184 - "app.rs"
Cohesion: 0.25
Nodes (6): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input()

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "Touch-Oberfläche"
Cohesion: 0.40
Nodes (4): Bearbeiten, Bedienen, Elemente hinzufügen, Touch-Oberfläche

### Community 189 - "ProfileBuilder"
Cohesion: 0.18
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 192 - "fire"
Cohesion: 0.13
Nodes (15): bank_step_wraps(), channel_mapping_sets_live_value(), default_feedback(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on() (+7 more)

### Community 193 - "Tab"
Cohesion: 0.25
Nodes (8): sidebar(), Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 196 - "widgets/mod.rs"
Cohesion: 0.29
Nodes (6): CanvasMode, Edit, Select, StageCanvas, Underlay, XyPad

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "analyze"
Cohesion: 0.39
Nodes (8): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), finds_tempo_and_downbeat(), mixed_file_splits_into_tracks(), one_song_stays_one_segment(), silence_has_no_grid(), track()

### Community 199 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "trigger_cx"
Cohesion: 0.38
Nodes (7): channel_name(), describe(), describe_mapping(), t_deleted(), apply_pick(), pick_adds_scene_button_and_assigns_to_selection(), programmer_fader_becomes_fixed_channels()

### Community 203 - "Transform"
Cohesion: 0.25
Nodes (7): Transform, .ALL, Center, HorizontalSymmetry, None, Reverse, VerticalSymmetry

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 207 - "IconButton"
Cohesion: 0.10
Nodes (8): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY, ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 209 - "draw"
Cohesion: 0.40
Nodes (6): draw(), hit(), image(), isHot(), page(), rectOf()

### Community 210 - "widgets/mod.rs"
Cohesion: 0.15
Nodes (12): MenuPlace, Patch, Stage, StageEditor, Arrange, AlignH, AlignV, Grid (+4 more)

### Community 211 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 212 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 213 - "OrbitCamera"
Cohesion: 0.19
Nodes (3): camera_json(), OrbitCamera, ray_through_center_hits_target()

### Community 215 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 216 - "SlotKind"
Cohesion: 0.29
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 221 - "UpdateBadge"
Cohesion: 0.33
Nodes (6): UpdateBadge, Available, Busy, Error, Installed, None

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "Timeline"
Cohesion: 0.10
Nodes (18): add_audio(), add_audio_dialog(), has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Scene (+10 more)

### Community 230 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 231 - "textOn"
Cohesion: 0.50
Nodes (4): applyTheme(), dim(), hexRgb(), textOn()

### Community 234 - "PickTarget"
Cohesion: 0.50
Nodes (4): PickTarget, Jump, Protect, Release

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 238 - "Tab"
Cohesion: 0.50
Nodes (4): Tab, Effects, Properties, Third

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

### Community 250 - "AudioAnalysis"
Cohesion: 0.21
Nodes (6): WaveState, Failed, Loading, Ready, AudioAnalysis, AudioSegment

### Community 251 - "AudioPlayer"
Cohesion: 0.18
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

## Knowledge Gaps
- **532 isolated node(s):** `General`, `Customize`, `Startup`, `Outputs`, `Midi` (+527 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1237 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **102 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `View`, `library_panel.rs`, `paint_item`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `super`, `ChannelDef`, `Action`, `audio_triggers.rs`, `mapping/mod.rs`, `scene_props.rs`, `rig.rs`, `views/touch.rs`, `mcp/src/venue.rs`, `app/src/audio.rs`, `t`, `Profile`, `FixtureAction`, `fixture_settings.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `app.rs`, `ProfileBuilder`, `fire`, `ViewCtx`, `window.rs`, `stage.rs`, `missing_audio.rs`, `Show`, `trigger_cx`, `engine.rs`, `scene_grid.rs`, `audio_trigger_ui.rs`, `Rect`, `super`, `AudioPlayer`?**
  _High betweenness centrality (0.152) - this node is a cross-community bridge._
- **Are the 170 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 170 INFERRED edges - model-reasoned connections that need verification._
- **What connects `General`, `Customize`, `Startup` to the rest of the system?**
  _532 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `paint_item` be split into smaller, more focused modules?**
  _Cohesion score 0.09178743961352658 - nodes in this community are weakly interconnected._
- **Why does `Show` connect `Show` to `paint_item`, `runtime.rs`, `fx.rs`, `super`, `ChannelDef`, `Engine`, `theme.rs`, `stage_canvas.rs`, `Venue`, `mapping/mod.rs`, `rig.rs`, `views/touch.rs`, `Scene`, `FixtureAction`, `panels.rs`, `src/mapping.rs`, `DmxCraftApp`, `tools.rs`, `EffectKind`, `demo.rs`, `bank_menu`, `TouchLayout`, `app.rs`, `fire`, `BankScope`, `widgets/mod.rs`, `missing_audio.rs`, `feature.rs`, `trigger_cx`, `ShowError`, `motion.rs`, `engine.rs`, `show.rs`, `viz/mod.rs`, `scene_grid.rs`, `VizView`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13333333333333333 - nodes in this community are weakly interconnected._