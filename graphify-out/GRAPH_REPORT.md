# Graph Report - dmxcraft-src  (2026-11-09)

## Corpus Check
- 139 files · ~207,563 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 17 file(s) not represented in the graph (top: (none) 5, .ttf 3, .ico 2)

## Summary
- 3886 nodes · 9768 edges · 274 communities (159 shown, 115 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 512 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a8508b2f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- VizView
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
- app/src/obs.rs
- viz/library.rs
- jobs
- steps
- Tastatur- und MIDI-Zuordnung
- titlebar.rs
- announce
- build-local-artifacts
- Icon
- Settings
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
- super
- remote/mod.rs
- TouchLayout
- mcp.rs
- MenuAction
- ViewCtx
- window.rs
- stage.rs
- Ui
- Show
- feature.rs
- OutputKind
- motion.rs
- engine.rs
- fixture_settings.rs
- Vec2
- OpenDmx
- core/src/lib.rs
- text_field
- profile.rs
- toolbar.rs
- Toggle<'a>
- app.js
- viz/mod.rs
- ChannelKind
- scene_grid.rs
- CapabilityKind
- OrbitCamera
- MapPreset
- audio_triggers.rs
- TouchView
- Deserialize
- fx/mapping.rs
- en.rs
- Control
- Rect
- app.rs
- server.rs
- ev2.rs
- Venue
- Dialog<'a>
- outline.rs
- convert.rs
- Timeline
- core/src/venue.rs
- CurveShape
- ColorPreset
- control/timeline.rs
- i18n/mod.rs
- scene_card.rs
- theme.rs
- stage_canvas.rs
- AudioGrid
- missing_audio.rs
- patch_grid.rs
- Mapper
- EffectKind
- menu.rs
- Rect
- Action
- target_shape
- icon_library.rs
- mcp/src/venue.rs
- audio_trigger_ui.rs
- ShutterEffect
- .viewport
- Scene
- show.rs
- Tab
- color_picker.rs
- Problem
- analyze
- Profile
- gltf_import.rs
- BankScope
- Drag
- Fullscreen
- src/mapping.rs
- core/src/obs.rs
- tools.rs
- PlanShape
- widgets/mod.rs
- ClipAction
- demo.rs
- qxf.rs
- Category
- egui
- TouchKind
- Shape
- bank_menu
- super
- ProfileBuilder
- MenuItem<'a>
- fire
- Tab
- qlc-bundle.rs
- lock.rs
- DmxFrame
- AudioTrigger
- ShowError
- views/touch.rs
- Purpose
- arc
- plan.rs
- FixtureLimits
- IconButton
- StageCanvas
- Grip
- stage_editor.rs
- Material
- OrbitCamera
- .check_placement
- Knob<'a>
- palettes.rs
- FilterKind
- FooterButton<'a>
- resize_handles
- ChaserPreset
- plan
- Pos2
- WaveState
- AudioPlayer
- Kind
- SlotKind
- DialogKind
- Status
- Box<dyn serialport::SerialPort>

## God Nodes (most connected - your core abstractions)
1. `Engine` - 227 edges
2. `t()` - 200 edges
3. `Show` - 148 edges
4. `Icon` - 113 edges
5. `AppState` - 101 edges
6. `ViewCtx` - 82 edges
7. `ChannelKind` - 72 edges
8. `ChannelRef` - 58 edges
9. `Profile` - 53 edges
10. `FixtureType` - 50 edges

## Surprising Connections (you probably didn't know these)
- `delete()` --calls--> `t()`  [INFERRED]
  crates/app/src/titlebar.rs → crates/app/src/i18n/mod.rs
- `VizView` --references--> `Model`  [INFERRED]
  crates/app/src/viz/mod.rs → crates/app/src/viz/render.rs
- `SceneEdit` --references--> `Value`  [INFERRED]
  crates/app/src/views/control/scene_props.rs → crates/app/src/remote/mod.rs
- `release_modes_and_protection()` --calls--> `one()`  [INFERRED]
  crates/core/src/engine.rs → crates/core/src/timeline.rs
- `lamp()` --calls--> `full()`  [INFERRED]
  crates/app/src/ui/theme.rs → crates/core/src/fx/feature.rs

## Import Cycles
- 1-file cycle: `crates/app/src/views/fixture_menu.rs -> crates/app/src/views/fixture_menu.rs`
- 1-file cycle: `crates/core/src/fixture.rs -> crates/core/src/fixture.rs`
- 2-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/convert.rs`
- 2-file cycle: `crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/lib.rs -> crates/output/src/open_dmx.rs -> crates/output/src/lib.rs`
- 2-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/backends.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/profile.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/ofl/src/convert.rs -> crates/ofl/src/lib.rs -> crates/ofl/src/qxf.rs -> crates/ofl/src/convert.rs`
- 3-file cycle: `crates/output/src/backends.rs -> crates/output/src/lib.rs -> crates/output/src/runtime.rs -> crates/output/src/backends.rs`

## Communities (274 total, 115 thin omitted)

### Community 0 - "VizView"
Cohesion: 0.10
Nodes (14): decode_texture(), hit_y(), Missing, Mode, Build, Live, model_tex_id(), proposals_for_easy_view_names() (+6 more)

### Community 1 - "library_panel.rs"
Cohesion: 0.09
Nodes (25): delete(), file(), load_all(), load_user(), save(), sort(), sort_user(), today() (+17 more)

### Community 2 - "runtime.rs"
Cohesion: 0.12
Nodes (10): engine_keeps_running_without_device(), OutputStatus, Connecting, Error, Running, Stopped, RECONNECT_DELAY, record_load() (+2 more)

### Community 3 - "open_dmx.rs"
Cohesion: 0.11
Nodes (17): BREAK, DMX_BAUD, DmxLine, frame_is_break_mark_startcode_and_512_channels(), frame_over_pseudo_terminal(), FRAME_TIME, FTDI_VENDOR_ID, list_ports() (+9 more)

### Community 4 - "Theme"
Cohesion: 0.13
Nodes (13): active(), set_theme(), Theme, .ALL, Black, Contrast, Daslight, Graphite (+5 more)

### Community 5 - "programmer.rs"
Cohesion: 0.15
Nodes (28): button(), kind_color(), category_list(), color_bar(), current(), elide(), fader(), fader_groups() (+20 more)

### Community 6 - "AppState"
Cohesion: 0.09
Nodes (14): AppState, Focus, Bank, Effect, Scenes, Timeline, Mode, Builder (+6 more)

### Community 7 - "steps"
Cohesion: 0.09
Nodes (22): env, CARGO_TERM_COLOR, jobs, test, os, name, on, pull_request (+14 more)

### Community 8 - "fx.rs"
Cohesion: 0.20
Nodes (24): all_presets_render_every_beam(), apply(), apply_effect(), Beam, beams(), burst_width_widens_rings(), color_span(), ColorFx (+16 more)

### Community 9 - "OutputError"
Cohesion: 0.11
Nodes (8): NullOutput, RecordingOutput, DmxOutput, OutputConfig, OutputError, NotFound, Transfer, Failing

### Community 10 - "Deserialize"
Cohesion: 0.10
Nodes (10): load_file(), B64, b64_decode(), b64_encode(), base64_roundtrip(), Blob, Model, ModelPart (+2 more)

### Community 11 - "Engine"
Cohesion: 0.07
Nodes (8): Engine, FadeState, In, Out, lerp(), PlayingScene, SceneProgress, ChannelRef

### Community 12 - "app/src/obs.rs"
Cohesion: 0.11
Nodes (24): auth_matches_obs_websocket_example(), auth_response(), b64(), connects_and_plays_video_block(), engine(), fake_obs(), fire(), Link (+16 more)

### Community 13 - "viz/library.rs"
Cohesion: 0.13
Nodes (16): BLACK, Builder, builtin(), builtins_insert_with_fresh_ids(), DARK, EXT, GREY, load_user() (+8 more)

### Community 14 - "jobs"
Cohesion: 0.17
Nodes (11): needs, runs-on, jobs, build-global-artifacts, name, on, pull_request, push (+3 more)

### Community 15 - "steps"
Cohesion: 0.40
Nodes (11): steps, steps, steps, steps, steps, id, if, name (+3 more)

### Community 17 - "Tastatur- und MIDI-Zuordnung"
Cohesion: 0.05
Nodes (33): Fenster, MIDI-Controller einrichten (Einstellungen → MIDI), Neue Elemente zuordenbar machen, Tastatur- und MIDI-Zuordnung, Zuordenbare Elemente, Zuordnung einschalten, Aufbau, Einschalten (+25 more)

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

### Community 23 - "Settings"
Cohesion: 0.20
Nodes (3): cable_belongs_to_one_universe(), default_instance_name(), RECENT_MAX

### Community 24 - "rig.rs"
Cohesion: 0.18
Nodes (17): Beam, build(), cell_light(), color_wheel(), cube(), gobo(), gobo_of(), kind() (+9 more)

### Community 25 - "scene_props.rs"
Cohesion: 0.07
Nodes (53): COMPACT_W, FINE_STEP_MS, format(), MAX_MS, PX_PER_STEP, split(), split_and_format_carry_over(), STEPS_MS (+45 more)

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
Cohesion: 0.12
Nodes (28): canvas(), canvas_line(), color_fx_ui(), edit_colors(), label_row(), LABEL_W, paint_strip(), palette_editor() (+20 more)

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
Cohesion: 0.14
Nodes (25): t(), capability_fields(), category_label(), check(), color_label(), FOOT_H, general(), hex_field() (+17 more)

### Community 37 - ".push"
Cohesion: 0.25
Nodes (6): Analyzer, Biquad, Envelope, envelope_filters_band(), percentile(), speed_six_minutes()

### Community 38 - "DmxCraft – UI-Richtlinien"
Cohesion: 0.20
Nodes (9): 1. Grundprinzipien, 2. Design-Tokens (`ui/theme.rs`), 3. Layout, 4. Komponenten (`ui/widgets/`), 5. Interaktion, 6. Text & Symbole, 7. Architektur der Oberfläche, 8. Prüfen vor jedem Commit (+1 more)

### Community 39 - "UpdateState"
Cohesion: 0.13
Nodes (11): APP_NAME, Updater, UpdateState, Available, Checking, Error, Idle, Installed (+3 more)

### Community 40 - "FixtureAction"
Cohesion: 0.07
Nodes (32): apply(), FixtureAction, AddToGroup, Copy, CopyLayout, Cut, Delete, Duplicate (+24 more)

### Community 41 - "drag.rs"
Cohesion: 0.44
Nodes (3): drag_number(), endless_delta(), raw_available()

### Community 43 - "textures.rs"
Cohesion: 0.19
Nodes (16): brick(), builtin(), builtins_are_seamless_and_decode(), carpet(), concrete(), fbm(), make(), mix() (+8 more)

### Community 44 - "Fader"
Cohesion: 0.12
Nodes (10): Fader, FaderEvent, Changed, None, Release, ToggleFx, FaderRange, fine_mark() (+2 more)

### Community 45 - "render.rs"
Cohesion: 0.05
Nodes (39): cone(), cube(), cylinder(), Mesh, meshes_are_valid(), plane(), truss(), tube() (+31 more)

### Community 46 - "DmxCraft – Hinweise für KI-Assistenten"
Cohesion: 0.40
Nodes (4): DMXCraft – Hinweise für KI-Assistenten, Orientierung (Tokens sparen), Regeln, Struktur

### Community 48 - "settings_window.rs"
Cohesion: 0.15
Nodes (27): MidiDevice, Settings, ai(), audio(), customize(), full_combo(), general(), HEIGHT (+19 more)

### Community 50 - "super"
Cohesion: 0.14
Nodes (10): APC_MK2_PALETTE, color32(), detect_and_match(), DeviceModel, .ALL, ApcMiniMk2, Generic, matching_output() (+2 more)

### Community 51 - "remote/mod.rs"
Cohesion: 0.07
Nodes (33): addresses(), APP_JS, CLIENT_TIMEOUT, Cmd, Color, Press, Value, Xy (+25 more)

### Community 52 - "TouchLayout"
Cohesion: 0.20
Nodes (7): find_item(), images_are_pruned_and_old_integer_positions_load(), items_find_free_spots(), json_roundtrip_and_cleanup(), TouchImage, TouchLayout, TouchPage

### Community 65 - "mcp.rs"
Cohesion: 0.06
Nodes (32): bridge_command(), installed_bridge(), McpLink, McpStatus, Error, Off, Running, VizJob (+24 more)

### Community 66 - "MenuAction"
Cohesion: 0.08
Nodes (24): MenuAction, About, AudioFade, AudioMute, CheckUpdates, DmxLevels, InstallUpdate, MissingAudio (+16 more)

### Community 68 - "ViewCtx"
Cohesion: 0.25
Nodes (8): fold_section(), ViewCtx, vec3_row(), TexAction, Builtin, Load, Set, VizView

### Community 69 - "window.rs"
Cohesion: 0.09
Nodes (22): color(), channel_label(), channel_labels(), combo(), hint(), LIST_H, MAP_EXT, palette_button() (+14 more)

### Community 70 - "stage.rs"
Cohesion: 0.19
Nodes (14): fx_button(), group_pill(), GroupAction, AddSelection, Delete, New, Rename, Select (+6 more)

### Community 71 - "Ui"
Cohesion: 0.14
Nodes (20): card_row(), full_combo(), list_header(), list_row(), ListAction, Add, Duplicate, Remove (+12 more)

### Community 72 - "Show"
Cohesion: 0.21
Nodes (3): .VIRTUAL_DIMMER, Fixture, Show

### Community 73 - "feature.rs"
Cohesion: 0.17
Nodes (15): apply_levels(), block(), chaser_levels(), ChaserFx, curve_levels(), curve_phasing_shifts_beams(), CurveFx, custom_value() (+7 more)

### Community 74 - "OutputKind"
Cohesion: 0.33
Nodes (4): OutputKind, .ALL, OpenDmx, Simulation

### Community 76 - "motion.rs"
Cohesion: 0.09
Nodes (23): along(), apply(), catmull(), circle_goes_round(), dist(), insert_and_remove_points(), lerp(), local_roundtrip() (+15 more)

### Community 77 - "engine.rs"
Cohesion: 0.21
Nodes (35): blind_keeps_output_but_monitor_shows_edits(), color_fx_drives_rgb_and_moves(), cv(), defaults_are_rendered(), end_scene_follows_jump_to(), fades_between_steps_and_loops(), finished_super_scene_restarts_on_play(), first_step_fades_intensity_in() (+27 more)

### Community 78 - "fixture_settings.rs"
Cohesion: 0.10
Nodes (16): ProgrammerMode, Blind, Edit, Live, Layer, Live, Programmer, Quick (+8 more)

### Community 79 - "Vec2"
Cohesion: 0.08
Nodes (19): DOT_H, FADER_W, HANDLE, NUM_H, SYMBOL_H, VALUE_H, WHEEL_STEP, Knob (+11 more)

### Community 80 - "OpenDmx"
Cohesion: 0.27
Nodes (3): missing_cable_is_reported_not_fatal(), open_without_assignment_fails_clearly(), OpenDmx

### Community 86 - "core/src/lib.rs"
Cohesion: 0.15
Nodes (10): Group, LoopMode, Loop, Once, PLAN_SIZE_RANGE, SceneKind, Fx, Static (+2 more)

### Community 89 - "profile.rs"
Cohesion: 0.07
Nodes (27): add_range_until_every_value_is_used(), cap_for_kind(), Capability, CATEGORIES, DMX_CONNECTORS, entity(), entity_text(), fine_names() (+19 more)

### Community 90 - "toolbar.rs"
Cohesion: 0.22
Nodes (9): select_all(), chip(), divider(), FLY_SECS, H, level(), point_chip(), point_width() (+1 more)

### Community 92 - "app.js"
Cohesion: 0.07
Nodes (56): api(), applyTheme(), askPin(), buildTabs(), clamp(), colorOf(), contentRect(), cover() (+48 more)

### Community 93 - "viz/mod.rs"
Cohesion: 0.14
Nodes (24): aim_selected(), apply_placement(), AXIS_COLORS, deg_quat(), euler_matches_core_rotation(), GizmoDrag, handles(), move_all() (+16 more)

### Community 94 - "ChannelKind"
Cohesion: 0.06
Nodes (26): ChannelKind, .ALL, Amber, Blue, ColorWheel, Control, Dimmer, Fan (+18 more)

### Community 95 - "scene_grid.rs"
Cohesion: 0.12
Nodes (19): ControlViews, programmer_values(), ProgrammerView, Bank, FocusBank, MoveScenes, RenameBank, apply() (+11 more)

### Community 96 - "CapabilityKind"
Cohesion: 0.06
Nodes (30): CapabilityKind, .ALL, ColorIntensity, ColorPreset, ColorTemperature, Effect, EffectSpeed, Focus (+22 more)

### Community 97 - "OrbitCamera"
Cohesion: 0.19
Nodes (3): camera_json(), OrbitCamera, ray_through_center_hits_target()

### Community 99 - "MapPreset"
Cohesion: 0.09
Nodes (22): default_palette(), MapPreset, .ALL, Bounce, Burst, Butterfly, Explosion, Fire (+14 more)

### Community 100 - "audio_triggers.rs"
Cohesion: 0.14
Nodes (13): compute(), env_key(), EnvKey, EnvState, Failed, Loading, Ready, EventKey (+5 more)

### Community 101 - "TouchView"
Cohesion: 0.14
Nodes (13): section_header(), add_page(), add_tile(), icon_picker(), ImageTarget, Item, Page, pick_image() (+5 more)

### Community 104 - "Deserialize"
Cohesion: 0.11
Nodes (13): ChannelDef, ChannelRange, Physics, RangeEffect, Closed, Color, Gobo, Open (+5 more)

### Community 106 - "fx/mapping.rs"
Cohesion: 0.15
Nodes (15): fbm(), hash01(), every_preset_gives_colors(), glyph(), local_inside_and_outside(), MappingFx, mix(), smooth() (+7 more)

### Community 108 - "Control"
Cohesion: 0.09
Nodes (23): Overlay, Control, AudioFade, AudioMaster, AudioMute, BankStep, Blackout, Channel (+15 more)

### Community 109 - "Rect"
Cohesion: 0.19
Nodes (9): add_audio(), add_audio_dialog(), add_video(), fmt_ms(), grid_options(), has_audio(), is_video(), mark_clipboard() (+1 more)

### Community 110 - "app.rs"
Cohesion: 0.14
Nodes (12): edit_preview(), load_step(), LockDialog, moved_out(), OUTPUT_HZ, viewport_input(), UpdateBadge, Available (+4 more)

### Community 112 - "server.rs"
Cohesion: 0.11
Nodes (14): Running, dechunk(), dechunks(), error_for(), forward(), not_running_gives_error_with_id(), run(), bridge_forwards_to_server() (+6 more)

### Community 113 - "ev2.rs"
Cohesion: 0.24
Nodes (12): argb(), DEFAULT_TRUSS_LEN, Ev2Fixture, Ev2Import, import(), imports_geometry_and_fixtures(), node(), RECT_SIZE (+4 more)

### Community 117 - "Venue"
Cohesion: 0.15
Nodes (6): fixture_folders_are_exclusive(), FixtureFolder, Object, room_and_tree(), TrussHang, Venue

### Community 118 - "Dialog<'a>"
Cohesion: 0.08
Nodes (10): Button, Dialog, Dialog<'a>, DialogResult, FooterButton, Form, Kind, Cancel (+2 more)

### Community 119 - "outline.rs"
Cohesion: 0.08
Nodes (29): renumber(), drag_row(), DragFixtures, DragObjects, drop_marker(), FixtureSort, Address, Name (+21 more)

### Community 121 - "convert.rs"
Cohesion: 0.13
Nodes (25): apply_texture(), capabilities(), channel_kind(), convert_fixture(), default_value(), fine_aliases(), kinds(), Lookup (+17 more)

### Community 123 - "Timeline"
Cohesion: 0.10
Nodes (18): audio_grid_sets_tempo_and_offset(), Clip, ClipContent, Audio, Obs, Scene, Video, fades() (+10 more)

### Community 128 - "core/src/venue.rs"
Cohesion: 0.14
Nodes (10): aim_at_points_beam_to_target(), apply(), close(), IDENTITY, mul(), Placement, rotation(), rotation_order_x_then_y_then_z() (+2 more)

### Community 132 - "CurveShape"
Cohesion: 0.14
Nodes (13): CurveShape, .ALL, Custom, InverseRamp, Pulse, Ramp, Random, Sine (+5 more)

### Community 133 - "ColorPreset"
Cohesion: 0.09
Nodes (18): ColorPreset, .ALL, Burst, KnightRider, Perlin, Plasma, Rainbow, RandomFill (+10 more)

### Community 136 - "control/timeline.rs"
Cohesion: 0.11
Nodes (16): audio_marks(), AUDIO_TRACK_H, DEFAULT_CLIP_MS, EDGE, HEADER_W, LANE_H, Mark, OBS_COLOR (+8 more)

### Community 137 - "i18n/mod.rs"
Cohesion: 0.10
Nodes (14): all_texts_translated(), EN, fill(), fill_replaces_names(), Lang, lang(), .ALL, De (+6 more)

### Community 138 - "scene_card.rs"
Cohesion: 0.20
Nodes (6): CARD_H, clip(), COMPACT_H, SceneCard, SceneCardResponse, STRIP_W

### Community 140 - "theme.rs"
Cohesion: 0.07
Nodes (60): accent(), accent_dim(), accent_hover(), apply(), BAND_HIGH, BAND_LOW, BAND_MID, bg() (+52 more)

### Community 141 - "stage_canvas.rs"
Cohesion: 0.15
Nodes (17): AMBER, BOX, Camera, fixture_rect(), live_colors(), MenuFn, paint_effects(), paint_fixture() (+9 more)

### Community 142 - "AudioGrid"
Cohesion: 0.19
Nodes (4): AudioAnalysis, AudioGrid, AudioSegment, grid_helpers()

### Community 143 - "missing_audio.rs"
Cohesion: 0.20
Nodes (12): AUDIO_EXT, CHECK_EVERY, file_name(), finds_missing_and_relinks_keeping_cut(), folder_search_matches_file_names(), index_folder(), missing_id(), MissingAudio (+4 more)

### Community 144 - "patch_grid.rs"
Cohesion: 0.12
Nodes (12): Segmented, Segmented<'a, T>, CELL_H_MAX, CELL_H_MIN, clip(), COLUMNS, Drag, GridMode (+4 more)

### Community 146 - "Mapper"
Cohesion: 0.26
Nodes (3): default_feedback(), Mapper, Sent

### Community 147 - "EffectKind"
Cohesion: 0.12
Nodes (11): EffectKind, .ALL, ChaserFx, ColorFx, ColorMappings, CurveFx, Mappings, MoveFx (+3 more)

### Community 149 - "menu.rs"
Cohesion: 0.19
Nodes (12): begin(), context(), dropdown(), ICON_SLOT, IconFn, item(), ITEM_H, MenuItem (+4 more)

### Community 150 - "Rect"
Cohesion: 0.16
Nodes (14): HeaderAction, Delete, Duplicate, ExpandAll, Insert, Toggle, paint_bands(), paint_beats() (+6 more)

### Community 151 - "Action"
Cohesion: 0.19
Nodes (19): Action, Compact, CreateSuper, Delete, DuplicateScene, Flash, HideScenes, Jump (+11 more)

### Community 152 - "target_shape"
Cohesion: 0.13
Nodes (32): alt_click(), button_rect(), channel_name(), describe(), describe_mapping(), fader(), fader_channels(), fires_id() (+24 more)

### Community 153 - "icon_library.rs"
Cohesion: 0.25
Nodes (5): glyph(), LIBRARY, names_are_unique(), paint(), ICON_FAMILY

### Community 154 - "mcp/src/venue.rs"
Cohesion: 0.44
Nodes (12): add_object(), add_truss_and_hang_fixture(), apply_object(), delete_objects(), get_venue(), num(), object_json(), place_fixtures() (+4 more)

### Community 155 - "audio_trigger_ui.rs"
Cohesion: 0.26
Nodes (9): apply_trigger_pick(), trigger_cx(), trigger_ui(), TriggerAction, CancelPick, None, Pick, TriggerCx (+1 more)

### Community 156 - "ShutterEffect"
Cohesion: 0.15
Nodes (12): ShutterEffect, .ALL, Burst, Closed, Lightning, Open, Pulse, RampDown (+4 more)

### Community 157 - ".viewport"
Cohesion: 0.20
Nodes (7): Quality, .ALL, High, Low, Medium, Ultra, QualityParams

### Community 159 - "Scene"
Cohesion: 0.12
Nodes (6): duplicate_zone_copies_scenes_behind_original(), free_scenes_get_a_bank(), move_scenes_between_banks(), Scene, SceneSettings, steps_insert_remove_and_normalize()

### Community 160 - "show.rs"
Cohesion: 0.30
Nodes (12): address_map_marks_ranges(), groups_follow_fixture_removal(), json_roundtrip(), id_type!, next_free_address_skips_used(), paste_fixtures_finds_free_addresses_and_names(), patch_rejects_overlap_and_range(), plan_shape_is_guessed_from_type() (+4 more)

### Community 161 - "Tab"
Cohesion: 0.15
Nodes (12): Tab, Ai, .ALL, Audio, Customize, General, Midi, Obs (+4 more)

### Community 162 - "color_picker.rs"
Cohesion: 0.15
Nodes (13): color_picker(), color_swatches(), hex_parsing(), hsv_color(), HUE_H, parse_hex(), PRESET_COLORS, SV_H (+5 more)

### Community 165 - "Problem"
Cohesion: 0.16
Nodes (18): Problem, DuplicateChannel, DuplicateInMode, DuplicateMode, EmptyChannelName, EmptyMode, EmptyModeName, Gap (+10 more)

### Community 166 - "analyze"
Cohesion: 0.39
Nodes (8): analyze(), bands_separate_kick_and_hats(), bassline_and_intro_keep_the_grid(), finds_tempo_and_downbeat(), mixed_file_splits_into_tracks(), one_song_stays_one_segment(), silence_has_no_grid(), track()

### Community 167 - "Profile"
Cohesion: 0.09
Nodes (15): AppProfiles, MemoryProfiles, ProfileStore, create_list_update_delete(), delete(), format(), get(), list() (+7 more)

### Community 168 - "gltf_import.rs"
Cohesion: 0.17
Nodes (9): linear_to_srgb(), load(), loads_base_color_texture(), loads_embedded_triangle_with_node_transform(), MAX_TRIANGLES, percent_decode(), percent_decoding(), RawPart (+1 more)

### Community 169 - "BankScope"
Cohesion: 0.21
Nodes (7): BankScope, All, Bank, Banks, Off, OutsideBank, Zone

### Community 170 - "Drag"
Cohesion: 0.33
Nodes (6): Drag, FadeIn, FadeOut, Left, Move, Right

### Community 171 - "Fullscreen"
Cohesion: 0.22
Nodes (5): Fullscreen, SETTLE, Step, Enter, Maximize

### Community 173 - "src/mapping.rs"
Cohesion: 0.10
Nodes (24): MapFile, WindowState, add_find_and_json(), ButtonMode, Toggle, Value, channels_key(), Feedback (+16 more)

### Community 174 - "core/src/obs.rs"
Cohesion: 0.31
Nodes (7): bx(), find_box(), mp4_duration_ms(), ObsAction, Scene, Source, reads_mp4_length()

### Community 176 - "tools.rs"
Cohesion: 0.09
Nodes (63): ChannelValue, Step, add_clips(), add_effect(), add_fixtures(), any_obj(), apply_scene_fields(), base64() (+55 more)

### Community 177 - "PlanShape"
Cohesion: 0.20
Nodes (8): PlanShape, .ALL, BarHorizontal, BarVertical, Effect, Matrix, Round, Square

### Community 179 - "ClipAction"
Cohesion: 0.12
Nodes (16): ClipAction, ApplyGrid, Copy, Cut, Delete, Duplicate, EditScene, Grid (+8 more)

### Community 180 - "demo.rs"
Cohesion: 0.48
Nodes (4): cv(), demo_show(), empty_show(), ensure_builtin_types()

### Community 181 - "qxf.rs"
Cohesion: 0.20
Nodes (12): channel_def(), child(), convert_qxf(), converts_modes_and_kinds(), kind_from_group(), kind_from_preset(), qlc_physical(), qlc_ranges() (+4 more)

### Community 182 - "Category"
Cohesion: 0.22
Nodes (9): Category, All, Beam, Color, Dimmer, Gobo, .ITEMS, Other (+1 more)

### Community 183 - "egui"
Cohesion: 0.19
Nodes (5): View, SetupView, StageEditorView, ZonesView, Windows

### Community 184 - "TouchKind"
Cohesion: 0.20
Nodes (7): TouchKind, Button, Color, Fader, Knob, Label, Xy

### Community 185 - "Shape"
Cohesion: 0.20
Nodes (10): Shape, Box, Cylinder, Group, Model, Plane, Truss, TrussProfile (+2 more)

### Community 186 - "bank_menu"
Cohesion: 0.24
Nodes (13): bank_menu(), BankAction, ColorToScenes, Delete, Duplicate, Hide, New, Rename (+5 more)

### Community 187 - "super"
Cohesion: 0.17
Nodes (11): History, LIMIT, live_values_are_undoable(), Snapshot, undo_redo_roundtrip(), known_addresses(), on_switches(), roundtrip() (+3 more)

### Community 189 - "ProfileBuilder"
Cohesion: 0.19
Nodes (7): FileJob, Export, Import, new_profile(), pick_export(), pick_import(), ProfileBuilder

### Community 192 - "fire"
Cohesion: 0.23
Nodes (13): bank_step_wraps(), channel_mapping_sets_live_value(), fire(), fixed_channels_work_without_selection(), group_set(), index_targets(), is_on(), mapping() (+5 more)

### Community 193 - "Tab"
Cohesion: 0.29
Nodes (7): Tab, .ALL, Channels, Check, General, Modes, Physical

### Community 197 - "qlc-bundle.rs"
Cohesion: 0.26
Nodes (8): git(), main(), run(), dedupe_keys(), git(), main(), qxf_files(), run()

### Community 198 - "lock.rs"
Cohesion: 0.43
Nodes (5): digest(), hash_and_verify(), hash_password(), hex(), verify()

### Community 199 - "DmxFrame"
Cohesion: 0.18
Nodes (3): addresses_are_one_based(), DmxFrame, UNIVERSE_SIZE

### Community 200 - "AudioTrigger"
Cohesion: 0.22
Nodes (6): AudioTrigger, TriggerBand, .ALL, High, Low, Mid

### Community 201 - "ShowError"
Cohesion: 0.27
Nodes (5): old_show_files_without_new_fields_load(), ShowError, Format, Io, Version

### Community 202 - "views/touch.rs"
Cohesion: 0.12
Nodes (30): apply(), apply_pick(), channels_of(), COLORS, content_rect(), current_color(), current_position(), display_label() (+22 more)

### Community 203 - "Purpose"
Cohesion: 0.23
Nodes (7): Pending, Purpose, OpenShow, SaveShowAs, save_as_gets_show_extension(), SHOW_EXT, with_show_extension()

### Community 204 - "arc"
Cohesion: 0.25
Nodes (3): ICON_PNG, texture(), window_icon()

### Community 205 - "plan.rs"
Cohesion: 0.29
Nodes (9): clamp(), default_position(), grid(), grid_fits_inside(), MARGIN, row(), row_is_evenly_spread(), snap() (+1 more)

### Community 207 - "IconButton"
Cohesion: 0.15
Nodes (3): ICON_FILL_FAMILY, IconButton, paint_alert()

### Community 208 - "StageCanvas"
Cohesion: 0.33
Nodes (5): CanvasMode, Edit, Select, StageCanvas, Underlay

### Community 209 - "Grip"
Cohesion: 0.40
Nodes (5): Grip, Body, Corner, EdgeX, EdgeY

### Community 210 - "stage_editor.rs"
Cohesion: 0.19
Nodes (10): fit(), fixture_pos(), Arrange, AlignH, AlignV, Grid, Reset, Row (+2 more)

### Community 211 - "Material"
Cohesion: 0.36
Nodes (3): Environment, hang_below_nearest_truss(), Material

### Community 213 - "OrbitCamera"
Cohesion: 0.26
Nodes (7): apply_camera(), camera_views_and_values(), find_point(), quality(), quality_key(), VizView, CameraPreset

### Community 217 - ".check_placement"
Cohesion: 0.53
Nodes (5): PatchError, BadUniverse, OutOfRange, Overlap, UnknownType

### Community 223 - "palettes.rs"
Cohesion: 0.62
Nodes (6): builtin(), load_user(), named(), NamedPalette, path(), save_user()

### Community 224 - "FilterKind"
Cohesion: 0.50
Nodes (4): FilterKind, Band, High, Low

### Community 233 - "ChaserPreset"
Cohesion: 0.25
Nodes (7): ChaserPreset, .ALL, Fill, Inward, Outward, Random, Single

### Community 237 - "plan"
Cohesion: 0.29
Nodes (7): plan, publishing, tag, tag-flag, env, outputs, runs-on

### Community 250 - "WaveState"
Cohesion: 0.60
Nodes (4): WaveState, Failed, Loading, Ready

### Community 251 - "AudioPlayer"
Cohesion: 0.20
Nodes (4): AudioPlayer, fader_pickup_after_reset(), has_audio(), mute_and_fade()

### Community 262 - "Kind"
Cohesion: 0.29
Nodes (7): Kind, Bar, Blinder, Fog, Laser, Moving, Par

### Community 264 - "SlotKind"
Cohesion: 0.29
Nodes (7): SlotKind, .ALL, Color, Frost, Gobo, Open, Prism

### Community 265 - "DialogKind"
Cohesion: 0.33
Nodes (6): DialogKind, ImportEv2, ImportModel, LoadTexture, LoadVenue, SaveVenue

### Community 266 - "Status"
Cohesion: 0.40
Nodes (5): Status, Connected, Connecting, Failed, Off

## Knowledge Gaps
- **537 isolated node(s):** `Off`, `Connecting`, `Connected`, `General`, `Customize` (+532 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1250 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **115 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Engine` connect `Engine` to `library_panel.rs`, `runtime.rs`, `programmer.rs`, `control/timeline.rs`, `app/src/obs.rs`, `Mapper`, `Action`, `target_shape`, `scene_props.rs`, `rig.rs`, `audio_trigger_ui.rs`, `mcp/src/venue.rs`, `app/src/audio.rs`, `t`, `Profile`, `FixtureAction`, `DmxCraftApp`, `settings_window.rs`, `tools.rs`, `remote/mod.rs`, `egui`, `super`, `ProfileBuilder`, `fire`, `mcp.rs`, `ViewCtx`, `window.rs`, `stage.rs`, `DmxFrame`, `Show`, `views/touch.rs`, `engine.rs`, `fixture_settings.rs`, `core/src/lib.rs`, `scene_grid.rs`, `audio_triggers.rs`, `Timeline`, `Rect`, `app.rs`, `AudioPlayer`?**
  _High betweenness centrality (0.130) - this node is a cross-community bridge._
- **Are the 172 inferred relationships involving `t()` (e.g. with `.apply_menu()` and `.dialogs()`) actually correct?**
  _`t()` has 172 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Off`, `Connecting`, `Connected` to the rest of the system?**
  _537 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `VizView` be split into smaller, more focused modules?**
  _Cohesion score 0.10461538461538461 - nodes in this community are weakly interconnected._
- **Why does `t()` connect `t` to `VizView`, `library_panel.rs`, `programmer.rs`, `i18n/mod.rs`, `Deserialize`, `theme.rs`, `viz/library.rs`, `missing_audio.rs`, `patch_grid.rs`, `titlebar.rs`, `Rect`, `Action`, `target_shape`, `scene_props.rs`, `audio_trigger_ui.rs`, `.viewport`, `fx_ui.rs`, `color_picker.rs`, `FixtureAction`, `textures.rs`, `Fader`, `DmxCraftApp`, `settings_window.rs`, `egui`, `bank_menu`, `ProfileBuilder`, `ViewCtx`, `window.rs`, `stage.rs`, `Ui`, `views/touch.rs`, `Purpose`, `fixture_settings.rs`, `Vec2`, `stage_editor.rs`, `text_field`, `toolbar.rs`, `viz/mod.rs`, `palettes.rs`, `scene_grid.rs`, `TouchView`, `Rect`, `app.rs`, `Dialog<'a>`, `outline.rs`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `Show` (e.g. with `Mappings` and `ObsSettings`) actually correct?**
  _`Show` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `library_panel.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.09178743961352658 - nodes in this community are weakly interconnected._