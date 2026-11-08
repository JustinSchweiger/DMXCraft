# Graph Report - dmxcraft-src  (2026-11-08)

## Corpus Check
- 135 files · ~198,741 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3748 nodes · 9394 edges · 257 communities (167 shown, 90 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 488 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `b27f1f91`
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
- Settings
- ViewCtx
- window.rs
- stage.rs
- mcp.rs
- Show
- feature.rs
- OutputKind
- motion.rs
- .grid
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
- super
- zones.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- ChannelDef
- theme.rs
- stage_canvas.rs
- AudioGrid
- core/src/lib.rs
- flush
- paintItem
- call
- menu.rs
- plan.rs
- DMXCraft
- target_shape
- text_dim
- mcp/src/venue.rs
- views/touch.rs
- ShutterEffect
- .viewport
- Scene
- Host
- FixtureLimits
- color_picker.rs
- Problem
- Mapping
- mcp/src/profiles.rs
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
- outline.rs
- knob.rs
- midi.rs
- Drag
- Touch-Oberfläche
- ProfileBuilder
- TouchItem
- mapping/mod.rs
- scene_card.rs
- Context
- qlc-bundle.rs
- bank_menu
- DvcGoldUsb
- ShowError
- ChannelValue
- Material
- arc
- icon_library.rs
- Rect
- IconButton
- T
- draw
- widgets/mod.rs
- PlanShape
- Shape
- Category
- Mapper
- Fullscreen
- Transform
- UpdateBadge
- palettes.rs
- Timeline
- FooterButton<'a>
- ChaserPreset
- textOn
- VizView
- Tab
- Kind
- plan
- Cmd
- FilterKind
- .fade_state
- Tastatur- und MIDI-Zuordnung
- WaveState
- AudioPlayer

## God Nodes (most connected - your core abstractions)
1. `Engine` - 215 edges
2. `t()` - 195 edges
3. `Show` - 140 edges
4. `Icon` - 112 edges
5. `AppState` - 97 edges
6. `ViewCtx` - 75 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 58 edges
9. `Profile` - 53 edges
10. `FixtureType` - 50 edges

## Surprising Connections (you probably didn't know these)
- `Aufbau` --references--> `send()`  [INFERRED]
  README.md → crates/app/src/remote/web/app.js
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

## Communities (257 total, 90 thin omitted)

### Community 0 - "paint_item"
Cohesion: 0.15
Nodes (13): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.09
Nodes (25): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+17 more)

### Community 2 - "runtime.rs"
Cohesion: 0.13
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.09
Nodes (18): Box<dyn serialport::SerialPort>, BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID (+10 more)

### Community 4 - "Theme"
Cohesion: 0.14
Nodes (12): set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite, Gray (+4 more)

### Community 5 - "programmer.rs"
Cohesion: 0.11
Nodes (34): ProgrammerMode, Blind, Edit, Live, FADER_W, category_list(), color_bar(), current() (+26 more)

### Community 6 - "AppState"
Cohesion: 0.10
Nodes (12): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Control (+4 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.19
Nodes (26): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+18 more)

### Community 9 - "OutputError"
Cohesion: 0.12
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputError, NotFound, NotImplemented, Transfer, Failing

### Community 10 - "Deserialize"
Cohesion: 0.09
Nodes (9): Physics, TouchImage, B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, ModelPart (+1 more)

### Community 11 - "Engine"
Cohesion: 0.06
Nodes (5): Engine, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "Venue"
Cohesion: 0.15
Nodes (7): renumber(), fixture_folders_are_exclusive(), FixtureFolder, mul(), Object, room_and_tree(), Venue

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
Cohesion: 0.21
Nodes (19): clip(), delete(), HEIGHT, k(), levels(), lock(), main_menu(), modifier() (+11 more)

### Community 20 - "announce"
Cohesion: 0.25
Nodes (9): env, if, needs, runs-on, env, env, BUILD_MANIFEST_NAME, GH_TOKEN (+1 more)

### Community 21 - "build-local-artifacts"
Cohesion: 0.22
Nodes (9): container, if, name, needs, runs-on, strategy, build-local-artifacts, fail-fast (+1 more)

### Community 22 - "Icon"
Cohesion: 0.02
Nodes (82): Icon, AlignH, AlignV, ArrangeGrid, ArrangeRow, ArrowsH, Builder, Center (+74 more)

### Community 23 - "audio_triggers.rs"
Cohesion: 0.14
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 24 - "rig.rs"
Cohesion: 0.18
Nodes (17): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), kind() (+9 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.06
Nodes (51): Step, Enter, Maximize, COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP (+43 more)

### Community 26 - "host"
Cohesion: 0.29
Nodes (7): env, if, needs, outputs, runs-on, host, val

### Community 27 - "core/src/audio.rs"
Cohesion: 0.13
Nodes (23): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), BPM_MAX, BPM_MIN, ENERGY_FRAMES, event_at(), finds_tempo_and_downbeat() (+15 more)

### Community 28 - "with"
Cohesion: 0.29
Nodes (7): with, merge-multiple, name, path, pattern, persist-credentials, submodules

### Community 30 - "ofl/src/lib.rs"
Cohesion: 0.12
Nodes (17): OflError, Invalid, Json, Skip, best_match_finds_easy_view_names(), embedded_library_has_ofl_and_qlc(), embedded_library_loads(), EMBEDDED_OFL (+9 more)

### Community 31 - "fx_ui.rs"
Cohesion: 0.15
Nodes (18): color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor(), PaletteStore, ROW_H (+10 more)

### Community 32 - "env"
Cohesion: 0.40
Nodes (5): ANNOUNCEMENT_BODY, ANNOUNCEMENT_TITLE, PRERELEASE_FLAG, RELEASE_COMMIT, env

### Community 33 - "dmxcraft"
Cohesion: 0.80
Nodes (5): dmxcraft, dmxcraft-core, dmxcraft-mcp, dmxcraft-ofl, dmxcraft-output

### Community 34 - "app/src/audio.rs"
Cohesion: 0.18
Nodes (10): analysis(), analyze_file(), cache_file(), duration_ms(), mp3_decoder_can_seek(), open_decoder(), Player, start_player() (+2 more)

### Community 36 - "t"
Cohesion: 0.13
Nodes (27): t(), text_field(), capability_fields(), category_label(), check(), color_label(), FOOT_H, full_combo() (+19 more)

### Community 37 - ".push"
Cohesion: 0.22
Nodes (8): Analyzer, Biquad, energy_curve(), Envelope, envelope_filters_band(), percentile(), smooth(), speed_six_minutes()

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
Nodes (8): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fader_ranges()

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 47 - "DmxCraftApp"
Cohesion: 0.19
Nodes (3): DmxCraftApp, fill_types(), viewport_input()

### Community 48 - "settings_window.rs"
Cohesion: 0.15
Nodes (25): Settings, ai(), audio(), customize(), full_combo(), general(), HEIGHT, hint() (+17 more)

### Community 50 - "MenuAction"
Cohesion: 0.07
Nodes (29): audio_control(), AudioInfo, load_meter(), MenuAction, About, AudioFade, AudioMaster, AudioMute (+21 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.09
Nodes (26): addresses(), APP_JS, CLIENT_TIMEOUT, DEFAULT_PORT, FONT_ICONS, FONT_TEXT, header(), hex() (+18 more)

### Community 52 - "TouchLayout"
Cohesion: 0.23
Nodes (5): images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchLayout, TouchPage

### Community 65 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 66 - "Settings"
Cohesion: 0.20
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 68 - "ViewCtx"
Cohesion: 0.21
Nodes (6): fold_section(), ViewCtx, drop_marker(), Folder, VizView, VizView

### Community 69 - "window.rs"
Cohesion: 0.12
Nodes (19): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+11 more)

### Community 70 - "stage.rs"
Cohesion: 0.18
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "mcp.rs"
Cohesion: 0.15
Nodes (9): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+1 more)

### Community 72 - "Show"
Cohesion: 0.15
Nodes (10): .VIRTUAL_DIMMER, Fixture, Group, paste_fixtures_finds_free_addresses_and_names(), PatchError, BadUniverse, OutOfRange, Overlap (+2 more)

### Community 73 - "feature.rs"
Cohesion: 0.15
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.18
Nodes (6): OutputConfig, OutputKind, .ALL, DvcGoldUsb, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - ".grid"
Cohesion: 0.21
Nodes (7): apply(), clip(), Drag, GridMode, Output, Patch, PatchGrid

### Community 78 - "engine.rs"
Cohesion: 0.19
Nodes (33): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), fades_between_steps_and_loops(), first_step_fades_intensity_in(), latest_scene_wins_for_stored_channels(), layered_start_keeps_other_scenes() (+25 more)

### Community 79 - "egui"
Cohesion: 0.08
Nodes (26): DOT_H, fine_mark(), HANDLE, NUM_H, paint_symbol(), SYMBOL_H, VALUE_H, WHEEL_STEP (+18 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "show.rs"
Cohesion: 0.32
Nodes (11): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type(), removing_fixture_cleans_scenes() (+3 more)

### Community 87 - "dialog.rs"
Cohesion: 0.07
Nodes (15): Button, Dialog, Dialog<'a>, DialogResult, FOOTER_H, FooterButton, Form, HEADER_H (+7 more)

### Community 89 - "profile.rs"
Cohesion: 0.05
Nodes (31): AppProfiles, MemoryProfiles, add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity() (+23 more)

### Community 90 - "app.rs"
Cohesion: 0.23
Nodes (6): ControlViews, edit_preview(), load_step(), LockDialog, OUTPUT_HZ, ProgrammerView

### Community 92 - "app.js"
Cohesion: 0.12
Nodes (21): buildTabs(), clamp(), contentRect(), cv, de, fieldMove(), g, hsToRgb() (+13 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.19
Nodes (14): AXIS_COLORS, GizmoDrag, Handles, handles(), project(), QualityParams, ray_aabb_exit(), ray_box() (+6 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.10
Nodes (37): zone_color(), programmer_values(), Action, Bank, Compact, CreateSuper, Delete, DuplicateScene (+29 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "Ui"
Cohesion: 0.13
Nodes (20): kind_color(), card_row(), list_header(), list_row(), ListAction, Add, Duplicate, Remove (+12 more)

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_trigger_ui.rs"
Cohesion: 0.13
Nodes (14): apply_trigger_pick(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx, triggers_ui() (+6 more)

### Community 101 - "fx/mapping.rs"
Cohesion: 0.17
Nodes (13): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+5 more)

### Community 104 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 106 - "file_dialog.rs"
Cohesion: 0.22
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 108 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 109 - "Rect"
Cohesion: 0.24
Nodes (4): fmt_ms(), grid_options(), mark_clipboard(), TimelineView

### Community 110 - "super"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 112 - "server.rs"
Cohesion: 0.12
Nodes (14): Running, dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server() (+6 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Tab"
Cohesion: 0.17
Nodes (11): Tab, Ai, .ALL, Audio, Customize, General, Midi, Outputs (+3 more)

### Community 118 - "DeviceModel"
Cohesion: 0.13
Nodes (11): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+3 more)

### Community 119 - "outline.rs"
Cohesion: 0.14
Nodes (16): drag_row(), DragObjects, Renaming, Object, Tree, TreeAction, Delete, Drop (+8 more)

### Community 121 - "convert.rs"
Cohesion: 0.14
Nodes (24): capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup, Lookup<'a> (+16 more)

### Community 123 - "super"
Cohesion: 0.12
Nodes (16): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), digest(), hash_and_verify(), hash_password() (+8 more)

### Community 128 - "zones.rs"
Cohesion: 0.16
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.17
Nodes (10): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+2 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.09
Nodes (23): add_audio(), add_audio_dialog(), audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, HeaderAction (+15 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.11
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "ChannelDef"
Cohesion: 0.13
Nodes (12): ChannelDef, ChannelRange, RangeEffect, Closed, Color, Gobo, Open, Other (+4 more)

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (55): accent_hover(), active(), BAND_HIGH, BAND_LOW, BAND_MID, BLACK, c(), canvas() (+47 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (19): AMBER, BOX, Camera, fit(), fixture_pos(), fixture_rect(), live_colors(), MenuFn (+11 more)

### Community 142 - "AudioGrid"
Cohesion: 0.15
Nodes (10): AudioAnalysis, AudioGrid, AudioSegment, detect_grid(), detect_segments(), grid_helpers(), local_tempo(), onsets() (+2 more)

### Community 143 - "core/src/lib.rs"
Cohesion: 0.12
Nodes (11): LoopMode, Loop, Once, PLAN_SIZE_RANGE, Scene, SceneKind, Fx, Static (+3 more)

### Community 144 - "flush"
Cohesion: 0.32
Nodes (12): api(), askPin(), flush(), loadLayout(), message(), poll(), pressDown(), release() (+4 more)

### Community 146 - "paintItem"
Cohesion: 0.24
Nodes (12): colorOf(), cover(), faderParts(), font(), paintItem(), panel(), rgbToHs(), rr() (+4 more)

### Community 147 - "call"
Cohesion: 0.31
Nodes (21): add_clips(), add_effect(), add_fixtures(), apply_scene_fields(), call(), create_scene(), create_super_scene(), dmx_output() (+13 more)

### Community 149 - "menu.rs"
Cohesion: 0.09
Nodes (17): body(), heading_font(), heading_text(), begin(), context(), dropdown(), ICON_SLOT, IconFn (+9 more)

### Community 150 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 151 - "DMXCraft"
Cohesion: 0.17
Nodes (12): send(), 3D-Visualisierer, Aufbau, Bauen und starten, Bedienung in Kürze, DMXCraft, Installieren, KI-Assistent (MCP) (+4 more)

### Community 152 - "target_shape"
Cohesion: 0.21
Nodes (15): alt_click(), button(), button_rect(), fader(), fader_channels(), in_rotated(), rotated_quad(), t_touch_hint() (+7 more)

### Community 153 - "text_dim"
Cohesion: 0.21
Nodes (11): accent(), accent_dim(), apply(), bg(), border(), caption(), install_fonts(), raised() (+3 more)

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "views/touch.rs"
Cohesion: 0.13
Nodes (21): apply(), apply_pick(), channels_of(), COLORS, current_color(), current_position(), Drag, Move (+13 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.09
Nodes (19): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+11 more)

### Community 157 - ".viewport"
Cohesion: 0.25
Nodes (6): Quality, .ALL, High, Low, Medium, Ultra

### Community 159 - "Scene"
Cohesion: 0.19
Nodes (4): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), steps_insert_remove_and_normalize()

### Community 160 - "Host"
Cohesion: 0.15
Nodes (19): AppEvent, ProfilesChanged, Saved, SelectScene, ToolCalled, AudioDuration, DEFAULT_PORT, Host (+11 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.16
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "Mapping"
Cohesion: 0.20
Nodes (11): Overlay, MapFile, add_find_and_json(), ButtonMode, Toggle, Value, MapKind, Keyboard (+3 more)

### Community 167 - "mcp/src/profiles.rs"
Cohesion: 0.19
Nodes (11): ProfileStore, create_list_update_delete(), delete(), format(), get(), list(), problem_text(), R (+3 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.15
Nodes (10): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+2 more)

### Community 169 - "fixture_settings.rs"
Cohesion: 0.20
Nodes (9): Quick, quick_action(), Center, Off, On, body(), caption(), FixtureSettings (+1 more)

### Community 170 - "panels.rs"
Cohesion: 0.14
Nodes (18): aim_selected(), apply_placement(), move_all(), Pick, Fixture, Object, sel_frame(), SelFrame (+10 more)

### Community 171 - "assist.rs"
Cohesion: 0.14
Nodes (8): apply_camera(), camera_json(), camera_views_and_values(), quality(), quality_key(), VizView, OrbitCamera, ray_through_center_hits_target()

### Community 173 - "src/mapping.rs"
Cohesion: 0.07
Nodes (29): channels_key(), Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+21 more)

### Community 174 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 176 - "tools.rs"
Cohesion: 0.10
Nodes (32): any_obj(), base64(), boolean(), camera_schema(), clip_schema(), find_type(), fixture_json(), full_workflow() (+24 more)

### Community 177 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 178 - "core/src/venue.rs"
Cohesion: 0.15
Nodes (10): aim_at_points_beam_to_target(), apply(), CameraPreset, close(), IDENTITY, Placement, rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "outline.rs"
Cohesion: 0.15
Nodes (13): deg_quat(), euler_matches_core_rotation(), quat_deg(), rotate_all(), DragFixtures, FixtureSort, Address, Name (+5 more)

### Community 182 - "knob.rs"
Cohesion: 0.18
Nodes (4): Knob, Knob<'a>, SIZE, SWEEP

### Community 185 - "midi.rs"
Cohesion: 0.13
Nodes (10): MidiEvent, MidiHub, parse(), parses_note_and_cc(), Input, Key, Midi, MidiMessage (+2 more)

### Community 186 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 187 - "Touch-Oberfläche"
Cohesion: 0.40
Nodes (4): Bearbeiten, Bedienen, Elemente hinzufügen, Touch-Oberfläche

### Community 189 - "ProfileBuilder"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 190 - "TouchItem"
Cohesion: 0.24
Nodes (12): find_item(), state_json(), content_rect(), display_label(), fader_parts(), item_target(), item_value(), mapping_of() (+4 more)

### Community 192 - "mapping/mod.rs"
Cohesion: 0.17
Nodes (21): bank_step_wraps(), channel_mapping_sets_live_value(), channel_name(), describe(), describe_mapping(), fire(), fires_id(), fixed_channels_work_without_selection() (+13 more)

### Community 195 - "scene_card.rs"
Cohesion: 0.19
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 196 - "Context"
Cohesion: 0.21
Nodes (11): overlay_id(), Pick, pick(), pick_full(), pick_id(), report(), states_id(), take_touch_picks() (+3 more)

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "bank_menu"
Cohesion: 0.30
Nodes (12): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+4 more)

### Community 199 - "DvcGoldUsb"
Cohesion: 0.18
Nodes (4): DVC_GOLD_PRODUCT_ID, DVC_GOLD_UNIVERSES, DvcGoldUsb, NICOLAUDIE_VENDOR_ID_GUESS

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "ChannelValue"
Cohesion: 0.27
Nodes (7): ChannelValue, Step, build_steps(), channel_kind(), collect_values(), existing_fixtures(), resolve_channel()

### Community 203 - "Material"
Cohesion: 0.27
Nodes (4): Environment, hang_below_nearest_truss(), Material, TrussHang

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 206 - "Rect"
Cohesion: 0.38
Nodes (5): paint_bands(), paint_beats(), paint_clip(), paint_segments(), paint_triggers()

### Community 207 - "IconButton"
Cohesion: 0.11
Nodes (3): ICON_FILL_FAMILY, IconButton, Toggle

### Community 209 - "draw"
Cohesion: 0.40
Nodes (6): draw(), hit(), image(), isHot(), page(), rectOf()

### Community 210 - "widgets/mod.rs"
Cohesion: 0.13
Nodes (14): CanvasMode, Edit, Select, StageCanvas, Underlay, XyPad, Arrange, AlignH (+6 more)

### Community 211 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 212 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 215 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 216 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

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
Cohesion: 0.11
Nodes (16): has_audio(), audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Scene, fades(), GridMode (+8 more)

### Community 230 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 231 - "textOn"
Cohesion: 0.50
Nodes (4): applyTheme(), dim(), hexRgb(), textOn()

### Community 232 - "VizView"
Cohesion: 0.11
Nodes (11): decode_texture(), hit_y(), Missing, Mode, Build, Live, model_tex_id(), proposals_for_easy_view_names() (+3 more)

### Community 233 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 234 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 238 - "Cmd"
Cohesion: 0.40
Nodes (5): Cmd, Color, Press, Value, Xy

### Community 239 - "FilterKind"
Cohesion: 0.50
Nodes (4): FilterKind, Band, High, Low

### Community 240 - ".fade_state"
Cohesion: 0.83
Nodes (3): FadeState, In, Out

### Community 242 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.29
Nodes (6): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten

### Community 250 - "WaveState"
Cohesion: 0.47
Nodes (4): WaveState, Failed, Loading, Ready

### Community 251 - "AudioPlayer"
Cohesion: 0.18
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

## Knowledge Gaps
- **528 isolated node(s):** `Off`, `Error`, `Move`, `Left`, `Right` (+523 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1218 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **90 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Show` connect `Show` to `paint_item`, `library_panel.rs`, `runtime.rs`, `fx.rs`, `Engine`, `theme.rs`, `stage_canvas.rs`, `Venue`, `core/src/lib.rs`, `rig.rs`, `views/touch.rs`, `Scene`, `Mapping`, `FixtureAction`, `panels.rs`, `DmxCraftApp`, `tools.rs`, `EffectKind`, `demo.rs`, `TouchLayout`, `TouchItem`, `mapping/mod.rs`, `BankScope`, `Context`, `bank_menu`, `feature.rs`, `ShowError`, `ChannelValue`, `motion.rs`, `engine.rs`, `widgets/mod.rs`, `show.rs`, `app.rs`, `viz/mod.rs`, `scene_grid.rs`, `VizView`, `super`?**
  _High betweenness centrality (0.120) - this node is a cross-community bridge._
- **Are the 165 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 165 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Error`, `Move` to the rest of the system?**
  _528 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09494949494949495 - nodes in this community are weakly interconnected._
- **Why does `Engine` connect `Engine` to `zones.rs`, `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `core/src/lib.rs`, `call`, `audio_triggers.rs`, `rig.rs`, `scene_props.rs`, `mcp/src/venue.rs`, `views/touch.rs`, `Host`, `app/src/audio.rs`, `t`, `mcp/src/profiles.rs`, `FixtureAction`, `fixture_settings.rs`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `ProfileBuilder`, `TouchItem`, `mapping/mod.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `mcp.rs`, `Show`, `.grid`, `engine.rs`, `Mapper`, `app.rs`, `scene_grid.rs`, `audio_trigger_ui.rs`, `Rect`, `super`, `AudioPlayer`, `.fade_state`, `super`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._
- **Are the 3 inferred relationships involving `Show` (e.g. with `Mappings` and `TouchLayout`) actually correct?**
  _`Show` has 3 INFERRED edges - model-reasoned connections that need verification._
- **Should `runtime.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.13 - nodes in this community are weakly interconnected._